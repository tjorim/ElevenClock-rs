//! Where the central copy is, and what counts as one.
//!
//! The source is a folder. Which folder is decided by [`resolve`] from three inputs in
//! a fixed order, so an administrator never edits a config file (ADR 0004):
//!
//! 1. `--source <folder>`, for a launcher IT pushed to the machine;
//! 2. the launcher's own folder, when it was started from the share;
//! 3. the folder recorded in the state file, when it runs as the local copy.
use crate::sha256::{self, Digest};
use std::{
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

/// The app. The local copy always has this name.
pub const APP_EXE: &str = "elevenclock-rs.exe";
/// The app under a name that cannot be run from the share by mistake.
pub const APP_DATA: &str = "elevenclock-rs.data";
pub const LAUNCHER_EXE: &str = "elevenclock-rs-launcher.exe";
/// A just-replaced launcher that was running at the time; it can only go on a later run.
pub const LAUNCHER_OLD: &str = "elevenclock-rs-launcher.exe.old";
/// Optional `sha256sum` style list next to the central copy.
pub const SUMS: &str = "SHA256SUMS.txt";
/// The local launcher's record of where the central copy is.
pub const STATE_FILE: &str = "source.txt";

/// Windows compares paths without regard to case, and a trailing separator is noise.
pub fn same_folder(a: &Path, b: &Path) -> bool {
    let key = |path: &Path| {
        path.to_string_lossy()
            .trim_end_matches(['\\', '/'])
            .to_lowercase()
    };
    key(a) == key(b)
}

/// The folder to deploy from, or `None` when none can be known (a local launcher
/// without a state file).
pub fn resolve(
    argument: Option<&Path>,
    own_folder: &Path,
    destination: &Path,
    state: Option<&str>,
) -> Option<PathBuf> {
    if let Some(argument) = argument.filter(|path| !path.as_os_str().is_empty()) {
        return Some(std::path::absolute(argument).unwrap_or_else(|_| argument.to_path_buf()));
    }
    if !same_folder(own_folder, destination) {
        return Some(own_folder.to_path_buf());
    }
    state.and_then(parse_state)
}

/// The state file is one line: the source folder. Anything else is ignored, so a
/// damaged file can only make the launcher start the local copy.
pub fn parse_state(text: &str) -> Option<PathBuf> {
    let line = text.lines().next()?.trim();
    let path = Path::new(line);
    (!line.is_empty() && path.is_absolute() && !line.chars().any(char::is_control))
        .then(|| path.to_path_buf())
}

/// The text to store for `source`; `None` when it cannot be stored as one line of text.
pub fn state_text(source: &Path) -> Option<String> {
    let text = source.to_str()?;
    (!text.is_empty() && !text.chars().any(char::is_control)).then(|| format!("{text}\n"))
}

/// A central copy found in the source folder.
#[derive(Debug, Eq, PartialEq)]
pub struct Found {
    pub path: PathBuf,
    /// The file name it was found under: [`APP_EXE`] or [`APP_DATA`].
    pub name: &'static str,
}

/// Runs `work` on its own thread and gives up after `timeout`. An unreachable network
/// path can block a file system call for 20 s or more and cannot be cancelled, so the
/// abandoned thread is left to finish (or the process to end) on its own.
pub fn with_timeout<T: Send + 'static>(
    timeout: Duration,
    work: impl FnOnce() -> T + Send + 'static,
) -> Option<T> {
    let (sender, receiver) = mpsc::channel();
    std::thread::Builder::new()
        .name("source-probe".to_owned())
        .spawn(move || {
            let _ = sender.send(work());
        })
        .ok()?;
    receiver.recv_timeout(timeout).ok()
}

/// Looks for the app in `folder`, preferring the `.exe`. `None` when the folder is
/// unreachable or holds neither file.
pub fn probe(folder: &Path, timeout: Duration) -> Option<Found> {
    let folder = folder.to_path_buf();
    with_timeout(timeout, move || {
        [APP_EXE, APP_DATA].into_iter().find_map(|name| {
            let path = folder.join(name);
            path.is_file().then_some(Found { path, name })
        })
    })
    .flatten()
}

/// The result of comparing the central copy with `SHA256SUMS.txt`.
#[derive(Debug, Eq, PartialEq)]
pub enum Listed {
    /// The list has no line for this file, so it says nothing about it.
    NotListed,
    Matches,
    Differs,
}

/// Checks `digest` against a `sha256sum` style list. A line counts for the file when
/// its last path component is `name` (the file as found) or [`APP_EXE`], because a
/// list made for the release names the exe even when an administrator renamed it to
/// `.data`.
pub fn check_sums(list: &str, name: &str, digest: &Digest) -> Listed {
    let mut listed = Listed::NotListed;
    for line in list.lines() {
        let Some((hash, file)) = line.trim().split_once(char::is_whitespace) else {
            continue;
        };
        let file = file.trim_start().trim_start_matches('*');
        let file = file.rsplit(['/', '\\']).next().unwrap_or(file);
        if !file.eq_ignore_ascii_case(name) && !file.eq_ignore_ascii_case(APP_EXE) {
            continue;
        }
        match sha256::from_hex(hash) {
            Some(expected) if expected == *digest => {
                if listed == Listed::NotListed {
                    listed = Listed::Matches;
                }
            }
            _ => return Listed::Differs,
        }
    }
    listed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn path(text: &str) -> PathBuf {
        PathBuf::from(text)
    }

    #[test]
    fn the_argument_wins_over_everything() {
        let dir = tempfile::tempdir().unwrap();
        let (own, destination) = (dir.path().join("share"), dir.path().join("local"));
        let state = state_text(&dir.path().join("recorded")).unwrap();
        let chosen = dir.path().join("pushed");
        assert_eq!(
            resolve(Some(&chosen), &own, &destination, Some(&state)),
            Some(chosen)
        );
    }

    #[test]
    fn started_from_the_share_the_launchers_own_folder_is_the_source() {
        let dir = tempfile::tempdir().unwrap();
        let (own, destination) = (dir.path().join("share"), dir.path().join("local"));
        let state = state_text(&dir.path().join("recorded")).unwrap();
        assert_eq!(resolve(None, &own, &destination, Some(&state)), Some(own));
    }

    #[test]
    fn started_as_the_local_copy_the_recorded_folder_is_the_source() {
        let dir = tempfile::tempdir().unwrap();
        let destination = dir.path().join("local");
        let recorded = dir.path().join("recorded");
        let state = state_text(&recorded).unwrap();
        assert_eq!(
            resolve(None, &destination, &destination, Some(&state)),
            Some(recorded)
        );
        // Case and a trailing separator do not make the local folder a different one.
        let upper = path(&format!(
            "{}/",
            destination.to_string_lossy().to_uppercase()
        ));
        assert_eq!(resolve(None, &upper, &destination, None), None);
        // Nothing recorded: no source.
        assert_eq!(resolve(None, &destination, &destination, None), None);
    }

    #[test]
    fn a_damaged_state_file_gives_no_source() {
        let dir = tempfile::tempdir().unwrap();
        let destination = dir.path().join("local");
        for text in ["", "\n", "relative\\path\n", "a\u{1}b", "   "] {
            assert_eq!(parse_state(text), None, "{text:?}");
            assert_eq!(resolve(None, &destination, &destination, Some(text)), None);
        }
        // Only the first line counts, and surrounding spaces are trimmed.
        let source = dir.path().join("share");
        let text = format!("  {}  \nsecond line\n", source.display());
        assert_eq!(parse_state(&text), Some(source));
    }

    #[test]
    fn state_text_round_trips_one_line() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("share folder");
        let text = state_text(&source).unwrap();
        assert!(text.ends_with('\n') && text.matches('\n').count() == 1);
        assert_eq!(parse_state(&text), Some(source));
        assert_eq!(state_text(Path::new("")), None);
        assert_eq!(state_text(Path::new("a\nb")), None);
    }

    #[test]
    fn a_probe_prefers_the_exe_and_accepts_data() {
        let dir = tempfile::tempdir().unwrap();
        let timeout = Duration::from_secs(5);
        assert_eq!(probe(dir.path(), timeout), None);
        std::fs::write(dir.path().join(APP_DATA), b"x").unwrap();
        assert_eq!(probe(dir.path(), timeout).unwrap().name, APP_DATA);
        std::fs::write(dir.path().join(APP_EXE), b"x").unwrap();
        assert_eq!(probe(dir.path(), timeout).unwrap().name, APP_EXE);
        assert_eq!(probe(&dir.path().join("missing"), timeout), None);
    }

    #[test]
    fn a_blocked_probe_gives_up_at_the_timeout() {
        let started = Instant::now();
        let result = with_timeout(Duration::from_millis(50), || {
            std::thread::sleep(Duration::from_secs(3));
            1
        });
        assert_eq!(result, None);
        assert!(started.elapsed() < Duration::from_secs(2));
        assert_eq!(with_timeout(Duration::from_secs(5), || 7), Some(7));
    }

    #[test]
    fn sums_are_matched_by_file_name_and_may_name_the_exe_for_a_data_copy() {
        let digest = sha256::digest(b"program");
        let other = sha256::digest(b"another");
        let hex = sha256::to_hex(&digest);
        let wrong = sha256::to_hex(&other);
        let list = |hash: &str, file: &str| format!("{hash}  {file}\n");
        assert_eq!(check_sums("", APP_EXE, &digest), Listed::NotListed);
        assert_eq!(
            check_sums(&list(&hex, "payload/elevenclock-rs.exe"), APP_EXE, &digest),
            Listed::Matches
        );
        assert_eq!(
            check_sums(&list(&hex, "*ElevenClock-rs.EXE"), APP_DATA, &digest),
            Listed::Matches
        );
        assert_eq!(
            check_sums(&list(&hex, "elevenclock-rs.data"), APP_DATA, &digest),
            Listed::Matches
        );
        assert_eq!(
            check_sums(&list(&wrong, "elevenclock-rs.exe"), APP_EXE, &digest),
            Listed::Differs
        );
        // A malformed hash for our file is a mismatch, not a pass.
        assert_eq!(
            check_sums(&list("zz", "elevenclock-rs.exe"), APP_EXE, &digest),
            Listed::Differs
        );
        // Lines about other files say nothing.
        let others =
            format!("{wrong}  elevenclock-rs-setup.exe\n{wrong}  symbols\\elevenclock-rs.pdb\n");
        assert_eq!(check_sums(&others, APP_EXE, &digest), Listed::NotListed);
        // One bad line among good ones is still a mismatch.
        let mixed = format!("{hex}  elevenclock-rs.exe\n{wrong}  elevenclock-rs.exe\n");
        assert_eq!(check_sums(&mixed, APP_EXE, &digest), Listed::Differs);
    }
}
