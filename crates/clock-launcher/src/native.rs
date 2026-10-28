//! The Windows side: what only the operating system can answer, and the entry point.
//!
//! Win32 calls go through the bindings and helpers of `clock-windows`, so the launcher
//! and the app share the instance names, the folder rules, the log writer and the
//! atomic file replace, and cannot disagree about them.
use crate::{
    args::{self, Command, STARTUP, UNINSTALL},
    launch::{Claim, Context, Exit, Host, PROBE_TIMEOUT, launch},
    uninstall::uninstall,
};
use clock_windows::{
    bindings::{
        ACCESS_MASK, AllowSetForegroundWindow, HKEY, HKEY_CURRENT_USER, KEY_QUERY_VALUE,
        MB_ICONERROR, MB_OK, MessageBoxW, RegCloseKey, RegOpenKeyExW,
    },
    instance::{self, Acquired},
    logging::{self, RotatingLog},
    preferences_store::{app_data_dir, local_app_data},
    startup::{RegistryRunKey, RunKey, current_executable},
};
use std::{
    ffi::OsString,
    io,
    path::Path,
    process::{ExitCode, Stdio},
};
use windows_core::{PCWSTR, w};

/// The installer's uninstall entry (`AppId` `{97911B67-...}` plus Inno Setup's `_is1`).
const INSTALLER_ENTRY: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{97911B67-52D7-4D9B-B833-29303BC540A7}_is1";
/// The log's file name stem, beside the app's own log.
const LOG_NAME: &str = "launcher";

fn wide_nul(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

pub struct NativeHost {
    installer_entry: Vec<u16>,
    run_key: RegistryRunKey,
    log: Option<RotatingLog>,
}
impl NativeHost {
    pub fn new() -> Self {
        Self::with(
            INSTALLER_ENTRY,
            RegistryRunKey::new(),
            app_data_dir()
                .ok()
                .map(|folder| RotatingLog::named(folder.join("logs"), LOG_NAME)),
        )
    }
    /// Other registry locations and no log let the tests run in isolation.
    pub fn with(installer_entry: &str, run_key: RegistryRunKey, log: Option<RotatingLog>) -> Self {
        Self {
            installer_entry: wide_nul(installer_entry),
            run_key,
            log,
        }
    }
}
impl Default for NativeHost {
    fn default() -> Self {
        Self::new()
    }
}

impl Host for NativeHost {
    fn installer_entry_present(&mut self) -> bool {
        let mut key = HKEY::default();
        // SAFETY: opens an existing key read-only; the handle is closed right below.
        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(self.installer_entry.as_ptr()),
                None,
                ACCESS_MASK(KEY_QUERY_VALUE as u32),
                &mut key,
            )
        };
        if status.0 != 0 {
            return false;
        }
        // SAFETY: `key` was opened above and is not used afterwards.
        unsafe {
            let _ = RegCloseKey(key);
        }
        true
    }

    fn claim_instance(&mut self) -> Claim {
        match instance::acquire() {
            Ok(Acquired::Primary(guard)) => Claim::Free(Box::new(guard)),
            Ok(Acquired::Existing(_)) => Claim::Running,
            Err(_) => Claim::Unknown,
        }
    }

    fn start_app(&mut self, exe: &Path, args: &[OsString]) -> io::Result<()> {
        let mut command = std::process::Command::new(exe);
        command
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        // Not the launcher's folder, which may be the share.
        if let Some(folder) = exe.parent() {
            command.current_dir(folder);
        }
        let child = command.spawn()?;
        // A second launch asks the running copy to show Settings, which needs the
        // right to take the foreground that this click gave the launcher.
        // SAFETY: plain call with a process id; failure only means no foreground.
        unsafe {
            let _ = AllowSetForegroundWindow(child.id());
        }
        Ok(())
    }

    fn show_error(&mut self, message: &str) {
        let text = wide_nul(message);
        // SAFETY: `text` is NUL terminated and outlives this synchronous call.
        unsafe {
            MessageBoxW(
                None,
                PCWSTR(text.as_ptr()),
                w!("ElevenClock-rs"),
                (MB_OK | MB_ICONERROR) as u32,
            );
        }
    }

    fn log(&mut self, line: &str) {
        if let Some(log) = self.log.as_mut() {
            let _ = log.write_line(&logging::stamped(line));
        }
    }

    fn run_key(&mut self) -> &mut dyn RunKey {
        &mut self.run_key
    }
}

fn context() -> Option<Context> {
    Some(Context {
        destination: local_app_data()
            .ok()?
            .join("Programs")
            .join("ElevenClock-rs"),
        own_exe: current_executable()?,
        probe_timeout: PROBE_TIMEOUT,
    })
}

fn run() -> Exit {
    let mut host = NativeHost::new();
    let arguments: Vec<OsString> = std::env::args_os().skip(1).collect();
    // Login starts and the uninstall helper must never wait on a message box.
    let silent = arguments
        .iter()
        .any(|arg| arg == STARTUP || arg == UNINSTALL);
    let command = match args::parse(arguments) {
        Ok(command) => command,
        Err(error) => {
            host.log("usage error");
            if !silent {
                host.show_error(error.message());
            }
            return Exit::Usage;
        }
    };
    let Some(ctx) = context() else {
        host.log("folders unavailable");
        if !silent {
            host.show_error("ElevenClock-rs could not find your Windows folders.");
        }
        return Exit::Internal;
    };
    match command {
        Command::Launch(request) => launch(&mut host, &ctx, &request),
        Command::Uninstall => uninstall(&mut host, &ctx),
    }
}

/// Entry point. Every failure ends as an exit code and a log line; nothing here is
/// called back from Windows, so there is no foreign frame for a panic to cross, and
/// one that happens anyway is logged and turned into an exit code.
pub fn main() -> ExitCode {
    std::panic::set_hook(Box::new(|info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(|text| (*text).to_owned())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "no message".to_owned());
        NativeHost::new().log(&format!("internal error: {message}"));
    }));
    let exit = std::panic::catch_unwind(run).unwrap_or(Exit::Internal);
    ExitCode::from(exit.code())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::{APP_EXE, LAUNCHER_EXE};
    use clock_windows::bindings::{
        KEY_SET_VALUE, REG_SZ, RegCreateKeyExW, RegDeleteTreeW, RegSetValueExW,
    };
    use clock_windows::startup::{RunKey, VALUE_NAME, command_line};
    use std::fs;

    /// A throwaway key tree under HKCU, removed when the test ends.
    struct Scratch {
        root: String,
    }
    impl Scratch {
        fn new(tag: &str) -> Self {
            let unique = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |elapsed| elapsed.as_nanos());
            Self {
                root: format!(
                    r"Software\ElevenClock-rs-tests\launcher-{tag}-{}-{unique}",
                    std::process::id()
                ),
            }
        }
        fn sub(&self, name: &str) -> String {
            format!(r"{}\{name}", self.root)
        }
        /// Creates `subkey` (empty) the way the installer's entry exists.
        fn create(&self, subkey: &str) {
            let subkey = wide_nul(subkey);
            let mut key = HKEY::default();
            // SAFETY: creates a scratch key and closes the handle at once.
            unsafe {
                let status = RegCreateKeyExW(
                    HKEY_CURRENT_USER,
                    PCWSTR(subkey.as_ptr()),
                    None,
                    PCWSTR::null(),
                    0,
                    ACCESS_MASK(KEY_SET_VALUE as u32),
                    None,
                    &mut key,
                    None,
                );
                assert_eq!(status.0, 0);
                let _ = RegCloseKey(key);
            }
        }
        fn put_text(&self, subkey: &str, name: &str, text: &str) {
            let (subkey, name) = (wide_nul(subkey), wide_nul(name));
            let bytes: Vec<u8> = wide_nul(text)
                .iter()
                .flat_map(|unit| unit.to_le_bytes())
                .collect();
            let mut key = HKEY::default();
            // SAFETY: creates a scratch key, sets one value and closes the handle.
            unsafe {
                let status = RegCreateKeyExW(
                    HKEY_CURRENT_USER,
                    PCWSTR(subkey.as_ptr()),
                    None,
                    PCWSTR::null(),
                    0,
                    ACCESS_MASK(KEY_SET_VALUE as u32),
                    None,
                    &mut key,
                    None,
                );
                assert_eq!(status.0, 0);
                let status = RegSetValueExW(key, PCWSTR(name.as_ptr()), None, REG_SZ, Some(&bytes));
                assert_eq!(status.0, 0);
                let _ = RegCloseKey(key);
            }
        }
        fn run_key(&self) -> RegistryRunKey {
            RegistryRunKey::with_keys(&self.sub("Run"), &self.sub("Approved"), VALUE_NAME)
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let root = wide_nul(&self.root);
            // SAFETY: deletes only the scratch tree created by this test.
            unsafe {
                let _ = RegDeleteTreeW(HKEY_CURRENT_USER, PCWSTR(root.as_ptr()));
            }
        }
    }

    #[test]
    fn the_installers_entry_is_found_only_when_it_exists() {
        let scratch = Scratch::new("installer");
        let entry = scratch.sub("Uninstall");
        let mut host = NativeHost::with(&entry, scratch.run_key(), None);
        assert!(!host.installer_entry_present());
        scratch.create(&entry);
        assert!(host.installer_entry_present());
    }

    #[test]
    fn the_real_installer_entry_is_the_installers_app_id() {
        // Must match AppId in packaging/windows/ElevenClock-rs.iss, plus "_is1".
        assert!(INSTALLER_ENTRY.ends_with(r"\{97911B67-52D7-4D9B-B833-29303BC540A7}_is1"));
        let iss = include_str!("../../../packaging/windows/ElevenClock-rs.iss");
        assert!(iss.contains("AppId={{97911B67-52D7-4D9B-B833-29303BC540A7}"));
    }

    #[test]
    fn a_running_app_is_seen_through_the_per_user_instance() {
        let mut host = NativeHost::with("", RegistryRunKey::new(), None);
        // A real app on a developer machine would hold the instance; nothing to test.
        let Ok(Acquired::Primary(guard)) = instance::acquire() else {
            return;
        };
        assert!(matches!(host.claim_instance(), Claim::Running));
        drop(guard);
        assert!(matches!(host.claim_instance(), Claim::Free(_)));
    }

    #[test]
    fn starting_a_program_passes_its_arguments_and_reports_failure() {
        let mut host = NativeHost::with("", RegistryRunKey::new(), None);
        let system = std::env::var_os("SystemRoot").unwrap_or_else(|| "C:\\Windows".into());
        let cmd = Path::new(&system).join("System32").join("cmd.exe");
        host.start_app(&cmd, &[OsString::from("/c"), OsString::from("exit")])
            .unwrap();
        assert!(
            host.start_app(Path::new(r"C:\no\such\program.exe"), &[])
                .is_err()
        );
    }

    #[test]
    fn uninstall_on_the_real_registry_and_file_system_removes_only_what_is_ours() {
        // The instance check is the real one: skip when a real app is running here.
        let Ok(Acquired::Primary(guard)) = instance::acquire() else {
            return;
        };
        drop(guard);
        let scratch = Scratch::new("uninstall");
        let temporary = tempfile::tempdir().unwrap();
        let folder = temporary.path().to_path_buf();
        fs::write(folder.join(APP_EXE), b"app").unwrap();
        fs::write(folder.join(LAUNCHER_EXE), b"launcher").unwrap();
        fs::write(folder.join("notes.txt"), b"mine").unwrap();
        let ctx = Context {
            destination: folder.clone(),
            own_exe: folder.join("elsewhere").join(LAUNCHER_EXE),
            probe_timeout: PROBE_TIMEOUT,
        };
        let mut host = NativeHost::with(&scratch.sub("Uninstall"), scratch.run_key(), None);
        let mut other_program = scratch.run_key();
        scratch.put_text(&scratch.sub("Run"), "OtherApp", r#""C:\Other\app.exe" /x"#);

        // Another copy's value is left alone.
        other_program
            .write(&command_line(Path::new(r"C:\Elsewhere\elevenclock-rs.exe")).unwrap())
            .unwrap();
        assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
        assert!(other_program.read().unwrap().command.is_some());
        assert!(!folder.join(APP_EXE).exists());
        assert!(folder.join("notes.txt").exists());

        // This folder's launcher entry is removed, its neighbours stay.
        other_program
            .write(&command_line(&folder.join(LAUNCHER_EXE)).unwrap())
            .unwrap();
        assert_eq!(uninstall(&mut host, &ctx), Exit::Done);
        assert!(other_program.read().unwrap().command.is_none());
        assert!(!folder.join(LAUNCHER_EXE).exists() && folder.join("notes.txt").exists());
    }
}
