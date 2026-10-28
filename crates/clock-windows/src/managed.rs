//! Managed mode (ADR 0004).
//!
//! The app runs in managed mode when the company launcher sits next to it, in the
//! app's own folder: the local copy the launcher deploys to
//! `%LOCALAPPDATA%\Programs\ElevenClock-rs`. The only input is that one file name;
//! no configuration, environment variable or registry value can switch the mode on
//! or off, and checking it has no side effects.
//!
//! In managed mode the app shows no update controls and never checks for updates
//! (updates come from the launcher at launch and sign-in), and the Startup option
//! registers the sibling launcher instead of the app.
use std::path::{Path, PathBuf};

/// The launcher's fixed file name. Shared with the launcher crate.
pub const LAUNCHER_FILE: &str = "elevenclock-rs-launcher.exe";

/// The words that mark a managed copy next to the version.
pub const MANAGED_NOTE: &str = "managed by your organization";

/// Where the launcher would sit for an app at `executable`: the same folder. Split on
/// either separator by hand so the rule is the same on every host.
pub fn launcher_beside(executable: &Path) -> Option<PathBuf> {
    let text = executable.to_str()?;
    let cut = text.rfind(['\\', '/'])?;
    Some(PathBuf::from(format!("{}{LAUNCHER_FILE}", &text[..=cut])))
}

/// Whether the app at `executable` is a managed copy. `exists` is injected so the rule
/// is testable without a file system.
pub fn is_managed(executable: &Path, exists: impl Fn(&Path) -> bool) -> bool {
    launcher_beside(executable).is_some_and(|launcher| exists(&launcher))
}

/// Whether the running app is a managed copy. A plain file-existence check.
#[cfg(windows)]
pub fn detect() -> bool {
    crate::startup::current_executable()
        .is_some_and(|executable| is_managed(&executable, |launcher| launcher.is_file()))
}

/// The version as shown to the person using the app.
pub fn version_text(version: &str, managed: bool) -> String {
    if managed {
        format!("{version} ({MANAGED_NOTE})")
    } else {
        version.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const APP: &str = r"C:\Users\x\AppData\Local\Programs\ElevenClock-rs\elevenclock-rs.exe";
    const LAUNCHER: &str =
        r"C:\Users\x\AppData\Local\Programs\ElevenClock-rs\elevenclock-rs-launcher.exe";

    #[test]
    fn the_launcher_is_looked_for_in_the_apps_own_folder_only() {
        assert_eq!(
            launcher_beside(Path::new(APP)),
            Some(PathBuf::from(LAUNCHER))
        );
        assert_eq!(
            launcher_beside(Path::new("D:/portable/elevenclock-rs.exe")),
            Some(PathBuf::from("D:/portable/elevenclock-rs-launcher.exe"))
        );
        assert_eq!(launcher_beside(Path::new("elevenclock-rs.exe")), None);
    }

    #[test]
    fn managed_only_when_that_exact_file_exists() {
        assert!(is_managed(Path::new(APP), |path| path == Path::new(LAUNCHER)));
        assert!(!is_managed(Path::new(APP), |_| false));
        // A launcher in some other folder does not make this copy managed.
        assert!(!is_managed(Path::new(APP), |path| path != Path::new(LAUNCHER)));
    }

    #[test]
    fn the_version_carries_the_managed_note() {
        assert_eq!(version_text("1.4.0", false), "1.4.0");
        assert_eq!(
            version_text("1.4.0", true),
            "1.4.0 (managed by your organization)"
        );
    }
}
