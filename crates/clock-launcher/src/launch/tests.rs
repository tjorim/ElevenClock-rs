//! The launcher's behaviour end to end, on real files in a temporary folder and a
//! scripted [`Host`]. On Windows the same scenarios run with the real
//! `MoveFileExW`-based replace.
use super::*;
use crate::{
    args::Launch,
    pe::fixture::{app, launcher},
    sha256, source,
    uninstall::uninstall,
};
use clock_windows::startup::{RunEntry, StartupError, command_line};
use std::fs;

const OLD: &str = "build-1";
const NEW: &str = "build-2";

#[derive(Default)]
struct FakeKey {
    command: Option<String>,
    removed: bool,
}
impl RunKey for FakeKey {
    fn read(&mut self) -> Result<RunEntry, StartupError> {
        Ok(RunEntry {
            command: self.command.clone(),
            approved: true,
        })
    }
    fn write(&mut self, command: &str) -> Result<(), StartupError> {
        self.command = Some(command.to_owned());
        Ok(())
    }
    fn remove(&mut self) -> Result<(), StartupError> {
        self.command = None;
        self.removed = true;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Instance {
    Free,
    Running,
    Unknown,
}

struct FakeHost {
    installer: bool,
    instance: Instance,
    start_fails: bool,
    /// File names whose replace is interrupted.
    interrupted: Vec<&'static str>,
    started: Vec<(PathBuf, Vec<OsString>)>,
    errors: Vec<String>,
    logs: Vec<String>,
    key: FakeKey,
}
impl FakeHost {
    fn new() -> Self {
        Self {
            installer: false,
            instance: Instance::Free,
            start_fails: false,
            interrupted: Vec::new(),
            started: Vec::new(),
            errors: Vec::new(),
            logs: Vec::new(),
            key: FakeKey::default(),
        }
    }
    fn logged(&self, line: &str) -> bool {
        self.logs.iter().any(|logged| logged == line)
    }
    fn logged_starting(&self, prefix: &str) -> bool {
        self.logs.iter().any(|logged| logged.starts_with(prefix))
    }
}
impl Host for FakeHost {
    fn installer_entry_present(&mut self) -> bool {
        self.installer
    }
    fn claim_instance(&mut self) -> Claim {
        match self.instance {
            Instance::Free => Claim::Free(Box::new(())),
            Instance::Running => Claim::Running,
            Instance::Unknown => Claim::Unknown,
        }
    }
    fn start_app(&mut self, exe: &Path, args: &[OsString]) -> io::Result<()> {
        if self.start_fails {
            return Err(io::Error::other("no"));
        }
        self.started.push((exe.to_path_buf(), args.to_vec()));
        Ok(())
    }
    fn show_error(&mut self, message: &str) {
        self.errors.push(message.to_owned());
    }
    fn log(&mut self, line: &str) {
        self.logs.push(line.to_owned());
    }
    fn before_replace(&mut self, target: &Path) -> io::Result<()> {
        let name = target
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if self.interrupted.contains(&name) {
            Err(io::Error::other("power cut"))
        } else {
            Ok(())
        }
    }
    fn run_key(&mut self) -> &mut dyn RunKey {
        &mut self.key
    }
}

struct World {
    _dir: tempfile::TempDir,
    share: PathBuf,
    destination: PathBuf,
}
impl World {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let share = dir.path().join("share");
        fs::create_dir(&share).unwrap();
        let destination = dir
            .path()
            .join("local")
            .join("Programs")
            .join("ElevenClock-rs");
        Self {
            _dir: dir,
            share,
            destination,
        }
    }
    /// A world with the managed app on the share.
    fn with_app(build: &str) -> Self {
        let world = Self::new();
        world.publish(build);
        world
    }
    fn publish(&self, build: &str) {
        fs::write(self.share.join(source::APP_EXE), app(build)).unwrap();
    }
    fn publish_launcher(&self, build: &str) {
        fs::write(self.share.join(source::LAUNCHER_EXE), launcher(build)).unwrap();
    }
    fn layout(&self) -> Layout {
        Layout::new(&self.destination)
    }
    /// Started from the share, the way users open it.
    fn launched_from_share(&self) -> Context {
        self.publish_launcher("share");
        self.context(self.share.join(source::LAUNCHER_EXE))
    }
    /// Started as the local copy, the way a login start runs it.
    fn launched_locally(&self) -> Context {
        self.context(self.layout().launcher())
    }
    fn context(&self, own_exe: PathBuf) -> Context {
        Context {
            destination: self.destination.clone(),
            own_exe,
            probe_timeout: Duration::from_secs(10),
        }
    }
    fn local_app(&self) -> Option<Vec<u8>> {
        fs::read(self.layout().app()).ok()
    }
    /// Everything in the destination folder, to show nothing stray is left behind.
    fn files(&self) -> Vec<String> {
        let Ok(entries) = fs::read_dir(&self.destination) else {
            return Vec::new();
        };
        let mut names: Vec<String> = entries
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }
    fn remove_share(&self) {
        fs::remove_dir_all(&self.share).unwrap();
    }
}

fn interactive() -> Launch {
    Launch::default()
}
fn login() -> Launch {
    Launch {
        quiet: true,
        app_args: vec![OsString::from("--startup")],
        ..Launch::default()
    }
}

#[test]
fn a_first_launch_deploys_the_app_the_launcher_and_the_state_file_then_starts() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();

    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);

    assert_eq!(world.local_app().unwrap(), app(NEW));
    assert_eq!(
        fs::read(world.layout().launcher()).unwrap(),
        launcher("share")
    );
    let recorded = fs::read_to_string(world.layout().state()).unwrap();
    assert_eq!(source::parse_state(&recorded), Some(world.share.clone()));
    assert_eq!(
        world.files(),
        [
            "elevenclock-rs-launcher.exe",
            "elevenclock-rs.exe",
            "source.txt"
        ]
    );
    assert_eq!(host.started, [(world.layout().app(), Vec::new())]);
    assert!(host.errors.is_empty());
    assert!(
        host.logged("updated") && host.logged("started"),
        "{:?}",
        host.logs
    );
}

#[test]
fn an_update_and_a_rollback_both_follow_the_central_copy() {
    let world = World::with_app(OLD);
    let ctx = world.launched_from_share();
    launch(&mut FakeHost::new(), &ctx, &interactive());
    assert_eq!(world.local_app().unwrap(), app(OLD));

    world.publish(NEW);
    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(NEW));
    assert!(host.logged("updated"));

    // The comparison is by hash, not by "newer": an older build is rolled back to.
    world.publish(OLD);
    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(OLD));
    assert!(host.logged("updated"));
    assert_eq!(
        world.files(),
        [
            "elevenclock-rs-launcher.exe",
            "elevenclock-rs.exe",
            "source.txt"
        ]
    );
}

#[test]
fn an_identical_copy_is_left_alone() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    launch(&mut FakeHost::new(), &ctx, &interactive());

    // A replace would fail here, so a pass proves nothing was written.
    let mut host = FakeHost::new();
    host.interrupted = vec![
        "elevenclock-rs.exe",
        "source.txt",
        "elevenclock-rs-launcher.exe",
    ];
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert!(host.logged("unchanged"), "{:?}", host.logs);
    assert!(!host.logged("updated") && !host.logged_starting("update failed"));
    assert_eq!(host.started.len(), 1);
}

#[test]
fn a_running_app_is_never_replaced_only_started_to_bring_settings_forward() {
    for running in [Instance::Running, Instance::Unknown] {
        let world = World::with_app(OLD);
        let ctx = world.launched_from_share();
        launch(&mut FakeHost::new(), &ctx, &interactive());
        world.publish(NEW);

        let mut host = FakeHost::new();
        host.instance = running;
        assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);

        assert_eq!(world.local_app().unwrap(), app(OLD));
        assert_eq!(host.started, [(world.layout().app(), Vec::new())]);
        assert!(host.logged_starting("skipped: running"), "{:?}", host.logs);
        assert!(host.errors.is_empty());
    }
}

#[test]
fn a_running_app_that_is_not_the_local_copy_does_not_block_the_first_deploy() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();
    host.instance = Instance::Running;
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(NEW));
}

#[test]
fn an_unreachable_source_starts_the_local_copy_without_a_message() {
    let world = World::with_app(OLD);
    let ctx = world.launched_from_share();
    launch(&mut FakeHost::new(), &ctx, &interactive());
    world.remove_share();

    // Both ways of being asked: a manual launch from the local copy, and a login start.
    for request in [interactive(), login()] {
        let mut host = FakeHost::new();
        assert_eq!(
            launch(&mut host, &world.launched_locally(), &request),
            Exit::Done
        );
        assert_eq!(host.started.len(), 1);
        assert_eq!(host.started[0].1, request.app_args);
        assert!(host.errors.is_empty());
        assert!(host.logged("source unreachable"), "{:?}", host.logs);
        assert_eq!(world.local_app().unwrap(), app(OLD));
    }
}

#[test]
fn an_unreachable_source_without_a_local_copy_is_one_clear_message() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    world.remove_share();

    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::NoCopy);
    assert_eq!(host.errors, [NO_SOURCE]);
    assert!(host.started.is_empty());
    assert!(world.files().is_empty());

    // A login start only logs.
    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &login()), Exit::NoCopy);
    assert!(host.errors.is_empty() && host.started.is_empty());
    assert!(host.logged("no copy to start"));
}

#[test]
fn a_local_launcher_with_nothing_recorded_cannot_find_a_source() {
    let world = World::new();
    fs::create_dir_all(&world.destination).unwrap();
    let mut host = FakeHost::new();
    assert_eq!(
        launch(&mut host, &world.launched_locally(), &interactive()),
        Exit::NoCopy
    );
    assert!(host.logged("source unreachable: none known"));
}

#[test]
fn an_installed_copy_stops_everything() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    for request in [interactive(), login()] {
        let mut host = FakeHost::new();
        host.installer = true;
        assert_eq!(launch(&mut host, &ctx, &request), Exit::Refused);
        // A sign-in start shows no message boxes, only the log.
        if request.quiet {
            assert!(host.errors.is_empty());
        } else {
            assert_eq!(host.errors, [INSTALLED_COPY]);
        }
        assert!(host.started.is_empty());
        assert!(host.logged("refused: installed copy"));
        // Nothing was deployed: the folder does not even exist.
        assert!(!world.destination.exists());
    }
}

#[test]
fn a_record_that_cannot_be_written_is_reported_only_when_messages_are_allowed() {
    for (request, shown) in [(interactive(), true), (login(), false)] {
        let world = World::with_app(NEW);
        let ctx = world.launched_from_share();
        let mut host = FakeHost::new();
        host.interrupted = vec!["source.txt"];
        assert_eq!(launch(&mut host, &ctx, &request), Exit::Done);
        assert!(host.logged_starting("source not recorded"));
        assert_eq!(host.errors.len(), usize::from(shown), "{:?}", host.errors);
        // The app starts either way.
        assert_eq!(host.started.len(), 1);
    }
}

#[test]
fn an_interrupted_copy_leaves_the_old_exe_working() {
    let world = World::with_app(OLD);
    let ctx = world.launched_from_share();
    launch(&mut FakeHost::new(), &ctx, &interactive());
    world.publish(NEW);

    let mut host = FakeHost::new();
    host.interrupted = vec!["elevenclock-rs.exe"];
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(OLD));
    assert_eq!(host.started.len(), 1);
    assert!(host.errors.is_empty());
    assert!(host.logged_starting("update failed"), "{:?}", host.logs);
    // The temporary file is gone.
    assert_eq!(
        world.files(),
        [
            "elevenclock-rs-launcher.exe",
            "elevenclock-rs.exe",
            "source.txt"
        ]
    );

    // The next launch, no longer interrupted, completes the update.
    assert_eq!(
        launch(&mut FakeHost::new(), &ctx, &interactive()),
        Exit::Done
    );
    assert_eq!(world.local_app().unwrap(), app(NEW));
}

#[test]
fn an_interrupted_first_copy_leaves_nothing_and_says_so() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();
    host.interrupted = vec!["elevenclock-rs.exe"];
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::NoCopy);
    assert_eq!(host.errors, [NOT_COPIED]);
    assert!(world.files().is_empty());
}

#[test]
fn a_data_copy_is_deployed_as_the_exe() {
    let world = World::new();
    fs::write(world.share.join(source::APP_DATA), app(NEW)).unwrap();
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(NEW));
    assert!(!world.destination.join("elevenclock-rs.data").exists());

    // With both present the exe is the one that counts.
    world.publish(OLD);
    launch(&mut FakeHost::new(), &ctx, &interactive());
    assert_eq!(world.local_app().unwrap(), app(OLD));
}

#[test]
fn a_central_copy_that_is_not_the_app_is_refused_and_the_local_copy_survives() {
    let wrong_machine = crate::pe::fixture::program(
        0x14c,
        crate::pe::fixture::PE32_PLUS,
        Some(&[
            ("ProductName", "ElevenClock-rs"),
            ("OriginalFilename", "elevenclock-rs.exe"),
        ]),
        b"",
    );
    let other_product = crate::pe::fixture::program(
        crate::pe::fixture::AMD64,
        crate::pe::fixture::PE32_PLUS,
        Some(&[
            ("ProductName", "Other"),
            ("OriginalFilename", "elevenclock-rs.exe"),
        ]),
        b"",
    );
    let the_launcher = launcher("not the app");
    let candidates = [
        b"MZ garbage".to_vec(),
        Vec::new(),
        wrong_machine,
        other_product,
        the_launcher,
    ];
    for bad in candidates {
        let world = World::with_app(OLD);
        let ctx = world.launched_from_share();
        launch(&mut FakeHost::new(), &ctx, &interactive());
        fs::write(world.share.join(source::APP_EXE), &bad).unwrap();

        let mut host = FakeHost::new();
        assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
        assert_eq!(world.local_app().unwrap(), app(OLD));
        assert!(host.logged_starting("source invalid"), "{:?}", host.logs);
        assert!(host.errors.is_empty());

        // With no local copy there is nothing to fall back to.
        let fresh = World::new();
        fs::write(fresh.share.join(source::APP_EXE), &bad).unwrap();
        let mut host = FakeHost::new();
        assert_eq!(
            launch(&mut host, &fresh.launched_from_share(), &interactive()),
            Exit::NoCopy
        );
        assert_eq!(host.errors, [BAD_SOURCE]);
    }
}

#[test]
fn a_checksum_list_next_to_the_central_copy_is_honoured() {
    let world = World::with_app(OLD);
    let ctx = world.launched_from_share();
    let good = sha256::to_hex(&sha256::digest(&app(OLD)));
    fs::write(
        world.share.join(source::SUMS),
        format!("{good}  payload/elevenclock-rs.exe\n"),
    )
    .unwrap();
    assert_eq!(
        launch(&mut FakeHost::new(), &ctx, &interactive()),
        Exit::Done
    );
    assert_eq!(world.local_app().unwrap(), app(OLD));

    // The list no longer matches the file: refused, local copy kept.
    world.publish(NEW);
    let mut host = FakeHost::new();
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app(OLD));
    assert!(host.logged("source invalid: does not match its checksum list"));

    // A list that says nothing about the app does not block it.
    fs::write(
        world.share.join(source::SUMS),
        format!("{good}  something-else.zip\n"),
    )
    .unwrap();
    launch(&mut FakeHost::new(), &ctx, &interactive());
    assert_eq!(world.local_app().unwrap(), app(NEW));
}

#[test]
fn the_source_argument_beats_the_launchers_own_folder() {
    let world = World::with_app("from the share");
    let elsewhere = world.share.with_file_name("pushed");
    fs::create_dir(&elsewhere).unwrap();
    fs::write(elsewhere.join(source::APP_EXE), app("pushed")).unwrap();
    // IT pushed the launcher to the machine, not to the share.
    let own = elsewhere.join(source::LAUNCHER_EXE);
    fs::write(&own, launcher("pushed")).unwrap();
    let ctx = world.context(own);

    let request = Launch {
        source: Some(world.share.clone()),
        ..Launch::default()
    };
    assert_eq!(launch(&mut FakeHost::new(), &ctx, &request), Exit::Done);
    assert_eq!(world.local_app().unwrap(), app("from the share"));
    // The record lets the local launcher find the same source at login.
    let recorded = fs::read_to_string(world.layout().state()).unwrap();
    assert_eq!(source::parse_state(&recorded), Some(world.share.clone()));
}

#[test]
fn the_local_folder_is_never_taken_as_the_source() {
    let world = World::with_app(OLD);
    launch(
        &mut FakeHost::new(),
        &world.launched_from_share(),
        &interactive(),
    );
    let recorded = fs::read_to_string(world.layout().state()).unwrap();

    let request = Launch {
        source: Some(world.destination.clone()),
        ..Launch::default()
    };
    let mut host = FakeHost::new();
    assert_eq!(
        launch(&mut host, &world.launched_locally(), &request),
        Exit::Done
    );
    assert!(host.logged("source unreachable: it is the local folder"));
    assert_eq!(
        fs::read_to_string(world.layout().state()).unwrap(),
        recorded
    );
}

#[test]
fn the_local_launcher_updates_from_the_recorded_source_and_refreshes_itself() {
    let world = World::with_app(OLD);
    launch(
        &mut FakeHost::new(),
        &world.launched_from_share(),
        &interactive(),
    );
    world.publish(NEW);
    world.publish_launcher("newer launcher");

    let mut host = FakeHost::new();
    assert_eq!(
        launch(&mut host, &world.launched_locally(), &login()),
        Exit::Done
    );

    assert_eq!(world.local_app().unwrap(), app(NEW));
    assert_eq!(
        fs::read(world.layout().launcher()).unwrap(),
        launcher("newer launcher")
    );
    assert_eq!(host.started[0].1, [OsString::from("--startup")]);
    assert!(host.logged("launcher updated"));
}

#[test]
fn a_launcher_that_cannot_be_refreshed_never_blocks_the_app() {
    let world = World::with_app(OLD);
    launch(
        &mut FakeHost::new(),
        &world.launched_from_share(),
        &interactive(),
    );
    world.publish(NEW);
    world.publish_launcher("newer launcher");

    let mut host = FakeHost::new();
    host.interrupted = vec!["elevenclock-rs-launcher.exe", "source.txt"];
    assert_eq!(
        launch(&mut host, &world.launched_locally(), &login()),
        Exit::Done
    );
    assert_eq!(world.local_app().unwrap(), app(NEW));
    assert!(
        host.logged_starting("launcher refresh failed"),
        "{:?}",
        host.logs
    );
    assert_eq!(
        fs::read(world.layout().launcher()).unwrap(),
        launcher("share")
    );

    // A central launcher that is not a valid launcher is skipped.
    fs::write(world.share.join(source::LAUNCHER_EXE), b"not a program").unwrap();
    let mut host = FakeHost::new();
    assert_eq!(
        launch(&mut host, &world.launched_locally(), &login()),
        Exit::Done
    );
    assert!(host.logged_starting("launcher refresh skipped"));
}

#[test]
fn arguments_reach_the_app_unchanged() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    let request = Launch {
        quiet: true,
        app_args: ["--startup", "extra one"].map(OsString::from).to_vec(),
        ..Launch::default()
    };
    let mut host = FakeHost::new();
    launch(&mut host, &ctx, &request);
    assert_eq!(host.started[0].1, request.app_args);
}

#[test]
fn a_start_that_fails_is_reported_unless_it_is_a_login_start() {
    let world = World::with_app(NEW);
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();
    host.start_fails = true;
    assert_eq!(launch(&mut host, &ctx, &interactive()), Exit::StartFailed);
    assert_eq!(host.errors, [NOT_STARTED]);

    let mut host = FakeHost::new();
    host.start_fails = true;
    assert_eq!(launch(&mut host, &ctx, &login()), Exit::StartFailed);
    assert!(host.errors.is_empty() && host.logged_starting("start failed"));
}

/// Every combination of the facts that decide what a launch does, against a plain
/// statement of the rules.
#[test]
fn the_decision_table() {
    let flags = [false, true];
    for installed in flags {
        for running in flags {
            for local in flags {
                for reachable in flags {
                    for same in flags {
                        let world = World::with_app(NEW);
                        let ctx = world.launched_from_share();
                        if local {
                            let build = if same { NEW } else { OLD };
                            fs::create_dir_all(&world.destination).unwrap();
                            fs::write(world.layout().app(), app(build)).unwrap();
                        }
                        let before = world.local_app();
                        if !reachable {
                            world.remove_share();
                        }
                        let mut host = FakeHost::new();
                        host.installer = installed;
                        host.instance = if running {
                            Instance::Running
                        } else {
                            Instance::Free
                        };
                        let label = format!(
                            "installed={installed} running={running} local={local} \
                             reachable={reachable} same={same}"
                        );

                        let exit = launch(&mut host, &ctx, &interactive());

                        if installed {
                            assert_eq!(exit, Exit::Refused, "{label}");
                            assert!(host.started.is_empty(), "{label}");
                            assert_eq!(world.local_app(), before, "{label}");
                        } else if local && running {
                            assert_eq!(exit, Exit::Done, "{label}");
                            assert_eq!(world.local_app(), before, "{label}");
                            assert_eq!(host.started.len(), 1, "{label}");
                        } else if !reachable {
                            if local {
                                assert_eq!(exit, Exit::Done, "{label}");
                                assert_eq!(world.local_app(), before, "{label}");
                                assert_eq!(host.started.len(), 1, "{label}");
                            } else {
                                assert_eq!(exit, Exit::NoCopy, "{label}");
                                assert!(host.started.is_empty(), "{label}");
                                assert_eq!(host.errors.len(), 1, "{label}");
                            }
                        } else {
                            assert_eq!(exit, Exit::Done, "{label}");
                            assert_eq!(world.local_app().unwrap(), app(NEW), "{label}");
                            assert_eq!(host.started.len(), 1, "{label}");
                        }
                    }
                }
            }
        }
    }
}

fn deployed() -> World {
    let world = World::with_app(NEW);
    launch(
        &mut FakeHost::new(),
        &world.launched_from_share(),
        &interactive(),
    );
    world
}

#[test]
fn uninstall_removes_the_known_files_and_nothing_else() {
    let world = deployed();
    fs::write(world.layout().launcher_old(), b"parked").unwrap();
    // Neighbours that are not ours stay, whatever their names.
    fs::write(world.destination.join("notes.txt"), b"mine").unwrap();
    fs::write(world.destination.join("elevenclock-rs.exe.tmp-1"), b"x").unwrap();
    fs::create_dir(world.destination.join("elevenclock-rs.dir")).unwrap();

    let mut host = FakeHost::new();
    let ctx = world.launched_from_share();
    assert_eq!(uninstall(&mut host, &ctx), Exit::Done);

    assert_eq!(
        world.files(),
        [
            "elevenclock-rs.dir",
            "elevenclock-rs.exe.tmp-1",
            "notes.txt"
        ]
    );
    assert!(host.started.is_empty() && host.errors.is_empty());
    // Running it again is fine.
    assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
}

#[test]
fn uninstall_removes_the_run_value_only_when_it_is_ours() {
    let world = deployed();
    let ctx = world.launched_from_share();
    let app_command = command_line(&world.layout().app()).unwrap();
    let launcher_command = command_line(&world.layout().launcher()).unwrap();
    let foreign = [
        r#""C:\Other\elevenclock-rs.exe" --startup"#.to_owned(),
        format!("{app_command} extra"),
        app_command.replace('"', ""),
        "not a command".to_owned(),
        String::new(),
    ];
    for command in foreign {
        let mut host = FakeHost::new();
        host.key.command = Some(command.clone());
        assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
        assert!(!host.key.removed, "{command}");
        assert_eq!(host.key.command, Some(command));
        assert!(host.logged("startup entry left alone"));
    }
    // This folder's app or launcher, in any letter case, is removed with its flag.
    for command in [
        app_command.clone(),
        launcher_command,
        app_command.to_uppercase(),
    ] {
        let mut host = FakeHost::new();
        host.key.command = Some(format!("  {command}  "));
        assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
        assert!(host.key.removed && host.key.command.is_none(), "{command}");
    }
    // No value at all.
    let mut host = FakeHost::new();
    assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
    assert!(!host.key.removed);
}

#[test]
fn uninstall_does_not_touch_a_running_app_or_the_installers_files() {
    let world = deployed();
    let ctx = world.launched_from_share();
    let mut host = FakeHost::new();
    host.instance = Instance::Running;
    host.key.command = Some(command_line(&world.layout().app()).unwrap());
    assert_eq!(uninstall(&mut host, &ctx), Exit::UninstallIncomplete);
    assert!(world.local_app().is_some() && !host.key.removed);

    let mut host = FakeHost::new();
    host.installer = true;
    assert_eq!(uninstall(&mut host, &ctx), Exit::Refused);
    assert!(world.local_app().is_some());
}

#[test]
fn uninstall_reports_a_file_it_could_not_remove() {
    let world = deployed();
    // A folder cannot be removed as a file.
    fs::remove_file(world.layout().state()).unwrap();
    fs::create_dir(world.layout().state()).unwrap();
    let mut host = FakeHost::new();
    host.key.command = Some(command_line(&world.layout().app()).unwrap());
    assert_eq!(
        uninstall(&mut host, &world.launched_from_share()),
        Exit::UninstallIncomplete
    );
    // The rest was still done.
    assert!(world.local_app().is_none() && host.key.removed);
}
