//! What the launcher does when it is started (ADR 0004).
//!
//! The flow is plain code over a [`Host`], the few things only the operating system
//! can answer: is the app running, does the installer's entry exist, start a program,
//! show a message. Files are real files, so the whole flow runs against a temporary
//! folder on any host, with a scripted `Host`, and on Windows against the real one.
//!
//! In order: refuse a mixed install; leave an app that is running alone; find, probe
//! and validate the central copy; copy it when its hash differs; refresh the local
//! launcher and remember the source; start the local app. Anything that goes wrong on
//! the way to a fresh copy falls back to the local copy that is already there.
use crate::{
    args::Launch,
    fsops::{self, install, install_running},
    inventory_support::Code,
    pe, sha256,
    source::{
        self, APP_EXE, LAUNCHER_EXE, LAUNCHER_OLD, Listed, Probe, STATE_FILE, SUMS, check_sums,
    },
};
use clock_windows::startup::RunKey;
use std::{
    any::Any,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

/// How long an unreachable central copy may hold the launch up.
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

pub const INSTALLED_COPY: &str = "ElevenClock-rs is already installed for this account; \
    uninstall it first. The managed copy and the installed copy cannot share a folder.";
const NO_SOURCE: &str = "ElevenClock-rs could not be started: the central copy cannot be \
    reached and this PC has no local copy yet. Connect to the company network or VPN and \
    start ElevenClock-rs again.";
const BAD_SOURCE: &str = "ElevenClock-rs could not be started: the central copy is not a \
    valid ElevenClock-rs program. Ask your administrator.";
const NOT_COPIED: &str = "ElevenClock-rs could not be started: copying it to this PC failed. \
    Try again, or ask your administrator.";
const NOT_STARTED: &str = "ElevenClock-rs could not be started.";
const NOT_RECORDED: &str = "ElevenClock-rs was started, but this PC could not remember where \
    the central copy is. It will not update at sign-in until you start ElevenClock-rs from the \
    central folder again.";

/// The process exit code: every outcome has one and a log line.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exit {
    Done,
    Internal,
    Usage,
    /// The installer's copy owns the folder.
    Refused,
    /// No usable central copy and no local copy.
    NoCopy,
    StartFailed,
    UninstallIncomplete,
}
impl Exit {
    pub fn code(self) -> u8 {
        match self {
            Self::Done => 0,
            Self::Internal => 1,
            Self::Usage => 2,
            Self::Refused => 3,
            Self::NoCopy => 4,
            Self::StartFailed => 5,
            Self::UninstallIncomplete => 6,
        }
    }
}

/// App refresh result, independent of diagnostic wording and launcher self-update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefreshOutcome {
    NotAttempted,
    Updated,
    Unchanged,
    SkippedRunning,
    SkippedCheckFailed,
    SourceUnreachable,
    SourceInvalid,
    CopyFailed,
}

/// Process creation does not establish that the clock is healthy or remains running.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchOutcome {
    NotAttempted,
    ProcessCreated,
    StartFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Observation {
    pub exit: Exit,
    /// This run established that the local launcher matches its refresh source.
    /// Freshness evidence only: this is not helper-protocol version negotiation.
    pub local_launcher_current: bool,
    pub refresh: RefreshOutcome,
    pub launch: LaunchOutcome,
    /// A refresh failed and an existing local executable was used instead.
    pub fallback: bool,
}

/// Whether the app is running for this user and session.
pub enum Claim {
    /// Not running. The value keeps the per-user instance claimed, so the app cannot
    /// start while its file is being replaced; drop it before starting the app.
    Free(Box<dyn Any>),
    Running,
    /// The check itself failed; treated like running, because replacing a file that
    /// might be in use is the one thing that must not happen.
    Unknown,
}

pub trait Host {
    /// The installer's uninstall entry exists for this account.
    fn installer_entry_present(&mut self) -> bool;
    fn claim_instance(&mut self) -> Claim;
    /// Start the app, detached, and let it take the foreground.
    fn start_app(&mut self, exe: &Path, args: &[OsString]) -> io::Result<()>;
    fn show_error(&mut self, message: &str);
    /// One decision per line, no paths or user names.
    fn log(&mut self, line: &str);
    /// Fixed support codes, independent of human log messages.
    fn support_issue(&mut self, _code: Code) {}
    /// Called just before `target` is swapped for a verified copy. Tests fail it to
    /// simulate an interrupted copy.
    fn before_replace(&mut self, _target: &Path) -> io::Result<()> {
        Ok(())
    }
    /// The Run value and its Windows flag.
    fn run_key(&mut self) -> &mut dyn RunKey;
    fn remove_template_handler(&mut self, _app: &Path) -> io::Result<bool> {
        Ok(false)
    }
    fn restore_template_handler(&mut self, _app: &Path) -> io::Result<()> {
        Ok(())
    }
}

/// The facts about this run that do not depend on the arguments.
pub struct Context {
    /// `%LOCALAPPDATA%\Programs\ElevenClock-rs`.
    pub destination: PathBuf,
    /// This launcher's own file.
    pub own_exe: PathBuf,
    pub probe_timeout: Duration,
}

/// The files the launcher manages in the destination folder, and no others.
pub struct Layout {
    destination: PathBuf,
}
impl Layout {
    pub fn new(destination: &Path) -> Self {
        Self {
            destination: destination.to_path_buf(),
        }
    }
    pub fn app(&self) -> PathBuf {
        self.destination.join(APP_EXE)
    }
    pub fn launcher(&self) -> PathBuf {
        self.destination.join(LAUNCHER_EXE)
    }
    pub fn launcher_old(&self) -> PathBuf {
        self.destination.join(LAUNCHER_OLD)
    }
    pub fn state(&self) -> PathBuf {
        self.destination.join(STATE_FILE)
    }
}

/// A short, path-free description of an I/O failure for the log.
pub fn describe(error: &io::Error) -> String {
    match error.raw_os_error() {
        Some(code) => format!("{:?}, os error {code}", error.kind()),
        None => format!("{:?}", error.kind()),
    }
}

/// Why there is no fresh copy.
enum Problem {
    Unreachable,
    Invalid,
    NotCopied,
}
impl Problem {
    fn outcome(&self) -> RefreshOutcome {
        match self {
            Self::Unreachable => RefreshOutcome::SourceUnreachable,
            Self::Invalid => RefreshOutcome::SourceInvalid,
            Self::NotCopied => RefreshOutcome::CopyFailed,
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::Unreachable => NO_SOURCE,
            Self::Invalid => BAD_SOURCE,
            Self::NotCopied => NOT_COPIED,
        }
    }
}

pub fn launch(host: &mut dyn Host, ctx: &Context, request: &Launch) -> Exit {
    observe_launch(host, ctx, request).exit
}

/// Execute best-effort reporting only after launch decisions have completed.
/// Neither an adapter error nor panic can change the launcher's exit code.
/// The native adapter chooses a known-compatible local helper before spawning it.
pub fn report_after(
    observation: Observation,
    report: impl FnOnce(Observation) -> io::Result<()>,
) -> Exit {
    if observation.exit != Exit::Refused {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| report(observation)));
    }
    observation.exit
}

/// Returns structured decisions without collecting identity, writing inventory or
/// invoking a reporting callback. The shipping entry point uses only the exit code.
pub fn observe_launch(host: &mut dyn Host, ctx: &Context, request: &Launch) -> Observation {
    let layout = Layout::new(&ctx.destination);
    if host.installer_entry_present() {
        host.log("refused: installed copy");
        // A sign-in start shows no message boxes, only the log.
        if !request.quiet {
            host.show_error(INSTALLED_COPY);
        }
        return Observation {
            exit: Exit::Refused,
            local_launcher_current: false,
            refresh: RefreshOutcome::NotAttempted,
            launch: LaunchOutcome::NotAttempted,
            fallback: false,
        };
    }
    // A launcher parked by an earlier self-update (`install_running`) can only be
    // deleted once the process that held it has ended. Best effort: it stays while
    // the old launcher is still the one running.
    let _ = fs::remove_file(layout.launcher_old());
    // Staged copies of an earlier, interrupted run (a kill or power loss skips the
    // cleanup that normally removes them).
    fsops::sweep_stale_staged(&ctx.destination);
    // Checked before the central copy is touched: a running app means no update, so
    // there is no reason to wait for a share that may be down.
    let claim = host.claim_instance();
    let local_copy = layout.app().is_file();
    if local_copy && !matches!(claim, Claim::Free(_)) {
        host.log(match claim {
            Claim::Running => "skipped: running",
            _ => "skipped: running check failed",
        });
        let refresh = if matches!(claim, Claim::Running) {
            RefreshOutcome::SkippedRunning
        } else {
            RefreshOutcome::SkippedCheckFailed
        };
        drop(claim);
        // The running app is already on screen: bring Settings forward first, and only
        // then look at the share, so a slow or absent share never delays the user.
        let observation = started(host, request, &layout, refresh, false, false);
        if refresh == RefreshOutcome::SkippedRunning {
            stage_update(host, ctx, request, &layout);
        }
        return observation;
    }
    let (refresh, fallback, local_launcher_current) = match refresh(host, ctx, request, &layout) {
        Ok((outcome, current)) => (outcome, false, current),
        Err(problem) if local_copy => {
            // The share is not there, but an update fetched while the app was running
            // is. The claim is still held, so the app cannot start mid-replace.
            if matches!(problem, Problem::Unreachable) {
                apply_staged_update(host, ctx, &layout);
            }
            host.log("using the local copy");
            (problem.outcome(), true, false)
        }
        Err(problem) => {
            drop(claim);
            host.log("no copy to start");
            if !request.quiet {
                host.show_error(problem.message());
            }
            return Observation {
                exit: Exit::NoCopy,
                local_launcher_current: false,
                refresh: problem.outcome(),
                launch: LaunchOutcome::NotAttempted,
                fallback: false,
            };
        }
    };
    drop(claim);
    started(
        host,
        request,
        &layout,
        refresh,
        fallback,
        local_launcher_current,
    )
}

fn started(
    host: &mut dyn Host,
    request: &Launch,
    layout: &Layout,
    refresh: RefreshOutcome,
    fallback: bool,
    local_launcher_current: bool,
) -> Observation {
    let exit = start(host, request, layout);
    Observation {
        exit,
        local_launcher_current,
        refresh,
        fallback,
        launch: if exit == Exit::Done {
            LaunchOutcome::ProcessCreated
        } else {
            LaunchOutcome::StartFailed
        },
    }
}

fn start(host: &mut dyn Host, request: &Launch, layout: &Layout) -> Exit {
    match host.start_app(&layout.app(), &request.app_args) {
        Ok(()) => {
            host.log("started");
            Exit::Done
        }
        Err(error) => {
            host.log(&format!("start failed: {}", describe(&error)));
            if !request.quiet {
                host.show_error(NOT_STARTED);
            }
            Exit::StartFailed
        }
    }
}

/// The central copy of the app, read and checked.
struct Central {
    folder: PathBuf,
    bytes: Vec<u8>,
    digest: sha256::Digest,
}

/// Finds, reads and validates the central copy of the app. Logs what it decides.
fn fetch_central(
    host: &mut dyn Host,
    ctx: &Context,
    request: &Launch,
    layout: &Layout,
) -> Result<Central, Problem> {
    let own_folder = ctx.own_exe.parent().unwrap_or(Path::new(""));
    let state = fsops::read_limited(&layout.state())
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok());
    let Some(folder) = source::resolve(
        request.source.as_deref(),
        own_folder,
        &ctx.destination,
        state.as_deref(),
    ) else {
        host.log("source unreachable: none known");
        return Err(Problem::Unreachable);
    };
    // The local folder is never a source: copying it onto itself would only ever
    // overwrite the record of where the real one is. The text comparison is the fast
    // path; the probe also catches junctions and short names.
    if source::same_folder(&folder, &ctx.destination) {
        host.log("source unreachable: it is the local folder");
        return Err(Problem::Unreachable);
    }
    let found = match source::probe(&folder, &ctx.destination, ctx.probe_timeout) {
        Probe::Found(found) => found,
        Probe::IsDestination => {
            host.log("source unreachable: it is the local folder");
            return Err(Problem::Unreachable);
        }
        Probe::Unreachable => {
            host.log("source unreachable");
            return Err(Problem::Unreachable);
        }
    };
    let bytes = match source::read_bounded(&found.path, ctx.probe_timeout) {
        Ok(bytes) => bytes,
        Err(error) => {
            host.log(&format!("source unreachable: {}", describe(&error)));
            return Err(Problem::Unreachable);
        }
    };
    let digest = sha256::digest(&bytes);
    if let Err(rejection) = pe::validate(&bytes, APP_EXE) {
        host.log(&format!("source invalid: {}", rejection.describe()));
        return Err(Problem::Invalid);
    }
    let sums = source::read_bounded(&folder.join(SUMS), ctx.probe_timeout)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok());
    if sums.is_some_and(|list| check_sums(&list, found.name, &digest) == Listed::Differs) {
        host.log("source invalid: does not match its checksum list");
        return Err(Problem::Invalid);
    }
    Ok(Central {
        folder,
        bytes,
        digest,
    })
}

/// Brings the local app up to the central copy, then the launcher and the record of
/// where the source is. Logs what it decides.
fn refresh(
    host: &mut dyn Host,
    ctx: &Context,
    request: &Launch,
    layout: &Layout,
) -> Result<(RefreshOutcome, bool), Problem> {
    let Central {
        folder,
        bytes,
        digest,
    } = fetch_central(host, ctx, request, layout)?;
    let app = layout.app();
    let outcome = if fsops::hash_of(&app) == Some(digest) {
        host.log("unchanged");
        RefreshOutcome::Unchanged
    } else {
        match install(&app, &bytes, || host.before_replace(&app)) {
            Ok(()) => {
                host.log("updated");
                RefreshOutcome::Updated
            }
            Err(error) => {
                host.log(&format!("update failed: {}", describe(&error)));
                return Err(Problem::NotCopied);
            }
        }
    };
    // The local copy now matches the share, so anything staged earlier is obsolete.
    fsops::discard_staged_updates(&ctx.destination, None);
    let local_launcher_current = refresh_launcher(host, ctx, layout, &folder);
    if !remember_source(host, layout, &folder) && !request.quiet {
        host.show_error(NOT_RECORDED);
    }
    Ok((outcome, local_launcher_current))
}

/// While the app runs it cannot be replaced, but the share may be reachable right now
/// and may not be at the next sign-in. Keep a verified copy beside the app, to be
/// swapped in at the next start (see [`apply_staged_update`]). Never touches the
/// running app; every failure is only logged.
fn stage_update(host: &mut dyn Host, ctx: &Context, request: &Launch, layout: &Layout) {
    let Ok(central) = fetch_central(host, ctx, request, layout) else {
        return;
    };
    let folder = &ctx.destination;
    if fsops::hash_of(&layout.app()) == Some(central.digest) {
        // Back to what is installed (a rollback on the share): nothing to apply.
        fsops::discard_staged_updates(folder, None);
        host.log("staging: the local copy is current");
        return;
    }
    let staged = fsops::staged_update_path(folder, &central.digest);
    if fsops::hash_of(&staged) == Some(central.digest) {
        fsops::discard_staged_updates(folder, Some(&central.digest));
        host.log("staging: already staged");
        return;
    }
    match install(&staged, &central.bytes, || host.before_replace(&staged)) {
        Ok(()) => {
            fsops::discard_staged_updates(folder, Some(&central.digest));
            host.log("update staged");
        }
        Err(error) => host.log(&format!("staging failed: {}", describe(&error))),
    }
}

/// With the app not running and the share out of reach, swap in an update staged by an
/// earlier run. The staged file is checked against the hash in its name and as an app
/// before it is used; a damaged one is removed. Call with the instance claimed.
fn apply_staged_update(host: &mut dyn Host, ctx: &Context, layout: &Layout) {
    let folder = &ctx.destination;
    let app = layout.app();
    let local = fsops::hash_of(&app);
    for (path, digest) in fsops::staged_updates(folder) {
        let bytes = fsops::read_limited(&path).ok();
        let usable = bytes.as_deref().is_some_and(|bytes| {
            sha256::digest(bytes) == digest && pe::validate(bytes, APP_EXE).is_ok()
        });
        let Some(bytes) = bytes.filter(|_| usable) else {
            host.log("staged update damaged: removed");
            let _ = fs::remove_file(&path);
            continue;
        };
        if local == Some(digest) {
            let _ = fs::remove_file(&path);
            continue;
        }
        match install(&app, &bytes, || host.before_replace(&app)) {
            Ok(()) => {
                host.log("staged update applied");
                fsops::discard_staged_updates(folder, None);
                return;
            }
            Err(error) => {
                host.log(&format!("staged update failed: {}", describe(&error)));
                return;
            }
        }
    }
}

/// Best effort: a launcher that cannot be refreshed never keeps the app from starting.
/// True only when this run established that the local file matches its source.
/// This does not establish support for any particular helper protocol.
fn refresh_launcher(host: &mut dyn Host, ctx: &Context, layout: &Layout, folder: &Path) -> bool {
    let timeout = ctx.probe_timeout;
    let central = folder.join(LAUNCHER_EXE);
    let started_from_share = !ctx
        .own_exe
        .parent()
        .is_some_and(|own| source::same_folder(own, &ctx.destination));
    let (from, read) = match source::read_bounded(&central, timeout) {
        Ok(bytes) => (central, Ok(bytes)),
        // Started from the share, which holds the launcher under another name.
        Err(_) if started_from_share => (
            ctx.own_exe.clone(),
            source::read_bounded(&ctx.own_exe, timeout),
        ),
        // The central folder has no launcher: nothing to refresh from.
        Err(error) if error.kind() == io::ErrorKind::NotFound => return false,
        Err(error) => (central, Err(error)),
    };
    let target = layout.launcher();
    if source::same_folder(&from, &target) {
        return read.is_ok();
    }
    let bytes = match read {
        Ok(bytes) => bytes,
        Err(error) => {
            host.support_issue(Code::LauncherRefreshFailed);
            host.log(&format!("launcher refresh skipped: {}", describe(&error)));
            return false;
        }
    };
    if let Err(rejection) = pe::validate(&bytes, LAUNCHER_EXE) {
        host.support_issue(Code::LauncherRefreshFailed);
        host.log(&format!(
            "launcher refresh skipped: {}",
            rejection.describe()
        ));
        return false;
    }
    if fsops::hash_of(&target) == Some(sha256::digest(&bytes)) {
        return true;
    }
    let result = install_running(&target, &layout.launcher_old(), &bytes, || {
        host.before_replace(&target)
    });
    match result {
        Ok(()) => {
            host.log("launcher updated");
            true
        }
        Err(error) => {
            host.support_issue(Code::LauncherRefreshFailed);
            host.log(&format!("launcher refresh failed: {}", describe(&error)));
            false
        }
    }
}

/// The local launcher needs to know where to look at the next login.
/// Whether the record is in place afterwards.
fn remember_source(host: &mut dyn Host, layout: &Layout, folder: &Path) -> bool {
    let Some(text) = source::state_text(folder) else {
        host.support_issue(Code::SourceNotRecorded);
        host.log("source not recorded: not storable as text");
        return false;
    };
    let state = layout.state();
    if fsops::read_limited(&state).is_ok_and(|current| current == text.as_bytes()) {
        return true;
    }
    match install(&state, text.as_bytes(), || host.before_replace(&state)) {
        Ok(()) => {
            host.log("source recorded");
            true
        }
        Err(error) => {
            host.support_issue(Code::SourceNotRecorded);
            host.log(&format!("source not recorded: {}", describe(&error)));
            false
        }
    }
}

#[cfg(test)]
mod tests;
