//! Opted-in managed inventory. Blocking fact acquisition and file writes run
//! in a one-shot, job-owned helper after the app start attempt. No retries/timers.
use crate::{
    args::Launch,
    inventory::{self, Snapshot},
    inventory_config,
    inventory_support::Code,
    launch::{Context, Exit, Layout, Observation},
    source,
    source::LAUNCHER_EXE,
};
use clock_windows::{
    inventory_facts as facts, preferences_store::app_data_dir, taskbar::worker::HelperJob,
};
use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
    time::Duration,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Request {
    snapshot: Snapshot,
    source: Option<std::path::PathBuf>,
}

pub const HELPER_MARKER: &str = "--elevenclock-rs-inventory-v1";
const DEADLINE: Duration = Duration::from_secs(3);
const TEARDOWN: Duration = Duration::from_secs(1);

fn helper_program(ctx: &Context, observation: Observation) -> Option<std::path::PathBuf> {
    if observation.exit == Exit::Refused {
        return None;
    }
    let local = ctx.destination.join(LAUNCHER_EXE);
    // The executing local launcher implements this protocol itself, independently
    // of whether the central app/launcher could be refreshed. Never infer protocol
    // support from the mere presence of an arbitrary older local executable.
    if source::same_folder(&ctx.own_exe, &local) {
        Some(ctx.own_exe.clone())
    } else {
        observation.local_launcher_current.then_some(local)
    }
}

/// Start only the fixed local launcher, never an executable on the central share.
/// The handshake gates all collection until the child is inside its kill-on-close
/// job. Dropping the job on parent exit or deadline removes a stuck child.
pub fn report(
    ctx: &Context,
    launch: &Launch,
    observation: Observation,
    extra_issues: &[Code],
) -> io::Result<()> {
    let Some(program) = helper_program(ctx, observation) else {
        return Ok(());
    };
    let mut snapshot = Snapshot::from_observation(observation);
    for code in extra_issues {
        snapshot.add_issue(*code);
    }
    snapshot.launcher_version = Some(env!("CARGO_PKG_VERSION").to_owned());
    // Pass only the explicit/resolved source hint. The local helper reads saved
    // source state itself when this launch used the local executable.
    let source = source::resolve(
        launch.source.as_deref(),
        ctx.own_exe.parent().unwrap_or(&ctx.destination),
        &ctx.destination,
        None,
    );
    let payload = serde_json::to_vec(&Request { snapshot, source })
        .map_err(|_| io::Error::other("inventory request unavailable"))?;
    supervise(
        &program,
        &[HELPER_MARKER],
        &ctx.destination,
        &payload,
        DEADLINE,
    )
}

fn supervise(
    program: &std::path::Path,
    args: &[&str],
    directory: &std::path::Path,
    payload: &[u8],
    deadline: Duration,
) -> io::Result<()> {
    if payload.len() > 3072 {
        return Err(io::Error::other("inventory handshake too large"));
    }
    let mut child = Command::new(program)
        .args(args)
        .current_dir(directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    let job = match HelperJob::attach(&child) {
        Ok(job) => job,
        Err(_) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(io::Error::other("inventory job unavailable"));
        }
    };
    // Payload contains a source hint and fixed outcomes/null facts (< 3 KiB), so it fits in the
    // anonymous pipe without waiting for the child to consume it.
    let sent = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("inventory pipe unavailable"))
        .and_then(|mut stdin| stdin.write_all(payload));
    // A waiter thread blocks in `wait`, so the launcher wakes the moment the helper exits
    // instead of polling. On deadline, closing the job kills the helper and unblocks the thread.
    let (done, finished) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = done.send(child.wait());
    });
    if sent.is_ok() {
        match finished.recv_timeout(deadline) {
            Ok(Ok(status)) if status.success() => return Ok(()),
            Ok(_) => return Err(io::Error::other("inventory failed")),
            Err(_) => {}
        }
    }
    drop(job);
    let _ = finished.recv_timeout(TEARDOWN);
    Err(io::Error::other("inventory deadline"))
}

/// Private helper role: it never launches the app, touches startup or shows UI.
pub fn collect() -> io::Result<()> {
    let mut payload = Vec::new();
    io::stdin().take(3073).read_to_end(&mut payload)?;
    if payload.len() > 3072 {
        return Err(io::Error::other("inventory disabled"));
    }
    let Request {
        mut snapshot,
        source: hint,
    } = serde_json::from_slice::<Request>(&payload)
        .map_err(|_| io::Error::other("inventory request unavailable"))?;
    snapshot.validate()?;
    let directory = app_data_dir().map_err(|_| io::Error::other("inventory folder unavailable"))?;
    let local_app = clock_windows::preferences_store::local_app_data()
        .map_err(|_| io::Error::other("inventory folder unavailable"))?
        .join("Programs")
        .join("ElevenClock-rs");
    // A helper launched from a share/staging location is not allowed to collect.
    let own = std::env::current_exe()?;
    if !crate::source::same_folder(&own, &local_app.join(LAUNCHER_EXE)) {
        return Err(io::Error::other("inventory requires local launcher"));
    }
    let state = inventory::read_source_state(&local_app.join(source::STATE_FILE));
    let Some(source) = source::resolve(hint.as_deref(), &local_app, &local_app, state.as_deref())
    else {
        return Ok(());
    };
    let Some(config) =
        inventory_config::read(&source, &local_app).filter(|config| config.local_inventory_enabled)
    else {
        return Ok(());
    };
    snapshot.observed_utc = facts::observed_utc();
    (snapshot.sid, snapshot.account_name) = facts::account();
    snapshot.app_version = facts::file_version(&Layout::new(&local_app).app());
    let mut preference_issue = None;
    snapshot.clock_format = inventory::collect_clock_format(config.include_clock_format, || {
        use clock_windows::preferences_store::{LocalPreferencesStore, StoreError};
        match LocalPreferencesStore::new().and_then(|store| store.saved_clock_format()) {
            Ok(format) => Some(format),
            Err(error) => {
                preference_issue = Some(match error {
                    StoreError::UnsupportedVersion(_) => Code::PreferencesUnsupported,
                    StoreError::DamagedData | StoreError::InvalidPreferences(_) => {
                        Code::PreferencesInvalid
                    }
                    _ => Code::PreferencesReadFailed,
                });
                None
            }
        }
    });
    if let Some(code) = preference_issue {
        snapshot.add_issue(code);
    }
    snapshot.add_fact_issues();
    // The parent is the same deployed launcher in normal operation. Its package
    // version is sent by the parent, not inferred from a potentially refreshed file.
    if inventory_config::read(&source, &local_app).as_ref() != Some(&config) {
        return Err(io::Error::other("inventory configuration changed"));
    }
    if config.report_directory.is_some() {
        snapshot.add_issue(Code::PublicationPending);
    }
    let saved = inventory::record_local(
        &directory.join("inventory"),
        snapshot,
        facts::new_installation_id,
    )?;
    if let (Some(root), Some(saved)) = (&config.report_directory, saved) {
        crate::inventory_publish::publish_saved(root, saved, &directory.join("inventory"), || {
            inventory_config::read(&source, &local_app).as_ref() == Some(&config)
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unverified_local_launcher_is_not_invoked_as_an_inventory_helper() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(LAUNCHER_EXE),
            b"old launcher without inventory support",
        )
        .unwrap();
        let ctx = Context {
            destination: dir.path().to_path_buf(),
            own_exe: dir.path().join("staging").join(LAUNCHER_EXE),
            probe_timeout: Duration::from_secs(1),
        };
        let observation = Observation {
            exit: Exit::Done,
            local_launcher_current: false,
            refresh: crate::launch::RefreshOutcome::SkippedRunning,
            launch: crate::launch::LaunchOutcome::ProcessCreated,
            fallback: false,
        };
        // Without the gate this would try to execute the old file and fail.
        assert!(report(&ctx, &Launch::default(), observation, &[]).is_ok());
    }

    #[test]
    fn executing_local_launcher_can_report_skipped_and_failed_refreshes() {
        let dir = tempfile::tempdir().unwrap();
        let ctx = Context {
            destination: dir.path().to_path_buf(),
            own_exe: dir.path().join(LAUNCHER_EXE),
            probe_timeout: Duration::from_secs(1),
        };
        for refresh in [
            crate::launch::RefreshOutcome::SkippedRunning,
            crate::launch::RefreshOutcome::SkippedCheckFailed,
            crate::launch::RefreshOutcome::SourceUnreachable,
            crate::launch::RefreshOutcome::SourceInvalid,
            crate::launch::RefreshOutcome::CopyFailed,
        ] {
            let observation = Observation {
                exit: Exit::Done,
                local_launcher_current: false,
                refresh,
                launch: crate::launch::LaunchOutcome::ProcessCreated,
                fallback: false,
            };
            assert_eq!(helper_program(&ctx, observation), Some(ctx.own_exe.clone()));
            assert_eq!(
                helper_program(
                    &ctx,
                    Observation {
                        exit: Exit::Refused,
                        ..observation
                    }
                ),
                None
            );
        }
    }

    #[test]
    fn supervised_exit_and_deadline_do_not_leave_a_helper_running() {
        let root = std::env::var_os("SystemRoot").unwrap();
        let powershell = std::path::Path::new(&root)
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        let dir = tempfile::tempdir().unwrap();
        assert!(
            supervise(
                &powershell,
                &[
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "Start-Sleep -Milliseconds 100; exit 0"
                ],
                dir.path(),
                b"{}",
                Duration::from_secs(10)
            )
            .is_ok()
        );
        let start = std::time::Instant::now();
        assert!(
            supervise(
                &powershell,
                &[
                    "-NoProfile",
                    "-NonInteractive",
                    "-Command",
                    "Start-Sleep -Seconds 30"
                ],
                dir.path(),
                b"{}",
                Duration::from_millis(100)
            )
            .is_err()
        );
        assert!(start.elapsed() < Duration::from_secs(5));
    }
}
