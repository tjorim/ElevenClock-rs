//! Sweeping files this app's own writers left behind (a killed process, a power
//! loss or a session that ended mid-write skips the drop guards that normally remove
//! them).
//!
//! Deliberately narrow: the caller passes a matcher for its own exact name pattern
//! (never a wildcard), only regular files directly inside the given folder are
//! touched, symlinks are left alone, and every failure is ignored. Nothing here
//! can make startup fail.
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

/// Files that are none of ours or that cannot be inspected are never returned.
fn candidates(directory: &Path, owned: &dyn Fn(&str) -> bool) -> Vec<(PathBuf, SystemTime)> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name().to_str().is_some_and(owned))
        .filter_map(|entry| {
            // `symlink_metadata`: a link is not a regular file, so it is skipped.
            let metadata = fs::symlink_metadata(entry.path()).ok()?;
            let modified = metadata.modified().ok()?;
            metadata.is_file().then(|| (entry.path(), modified))
        })
        .collect()
}

/// Remove owned files not modified for longer than `older_than`. Returns how many
/// were removed.
pub fn remove_older_than(
    directory: &Path,
    owned: &dyn Fn(&str) -> bool,
    now: SystemTime,
    older_than: Duration,
) -> usize {
    candidates(directory, owned)
        .into_iter()
        .filter(|(_, modified)| {
            now.duration_since(*modified)
                .is_ok_and(|age| age > older_than)
        })
        .filter(|(path, _)| fs::remove_file(path).is_ok())
        .count()
}

/// Remove every owned file. For a caller that is itself removing the files' owner.
pub fn remove_all(directory: &Path, owned: &dyn Fn(&str) -> bool) -> usize {
    candidates(directory, owned)
        .into_iter()
        .filter(|(path, _)| fs::remove_file(path).is_ok())
        .count()
}

/// Keep the `keep` most recently modified owned files and remove the rest (ties are
/// broken by name, so the result is deterministic). Returns how many were removed.
pub fn keep_newest(directory: &Path, owned: &dyn Fn(&str) -> bool, keep: usize) -> usize {
    let mut files = candidates(directory, owned);
    files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    files
        .into_iter()
        .skip(keep)
        .filter(|(path, _)| fs::remove_file(path).is_ok())
        .count()
}

/// `text` is one or more ASCII digits.
pub fn digits(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
}

/// `text` is one or more lowercase hexadecimal digits.
pub fn hex(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, time::Duration};

    fn file_aged(directory: &Path, name: &str, age: Duration, now: SystemTime) -> PathBuf {
        let path = directory.join(name);
        let file = File::create(&path).unwrap();
        file.set_modified(now - age).unwrap();
        path
    }
    fn names(directory: &Path) -> Vec<String> {
        let mut names: Vec<_> = fs::read_dir(directory)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        names.sort();
        names
    }
    fn is_tmp(name: &str) -> bool {
        name.strip_prefix("a.")
            .and_then(|r| r.strip_suffix(".tmp"))
            .is_some_and(digits)
    }

    #[test]
    fn only_owned_names_older_than_the_limit_are_removed() {
        let dir = tempfile::tempdir().unwrap();
        let now = SystemTime::now();
        let day = Duration::from_secs(86_400);
        file_aged(dir.path(), "a.1.tmp", 2 * day, now);
        file_aged(dir.path(), "a.2.tmp", Duration::from_secs(60), now);
        file_aged(dir.path(), "a.x.tmp", 2 * day, now);
        file_aged(dir.path(), "b.3.tmp", 2 * day, now);
        fs::create_dir(dir.path().join("a.4.tmp")).unwrap();
        assert_eq!(remove_older_than(dir.path(), &is_tmp, now, day), 1);
        assert_eq!(
            names(dir.path()),
            ["a.2.tmp", "a.4.tmp", "a.x.tmp", "b.3.tmp"]
        );
    }

    #[test]
    fn remove_all_removes_owned_files_regardless_of_age() {
        let dir = tempfile::tempdir().unwrap();
        let now = SystemTime::now();
        file_aged(dir.path(), "a.1.tmp", Duration::ZERO, now);
        file_aged(dir.path(), "a.2.tmp", Duration::from_secs(9), now);
        file_aged(dir.path(), "keep.txt", Duration::ZERO, now);
        assert_eq!(remove_all(dir.path(), &is_tmp), 2);
        assert_eq!(names(dir.path()), ["keep.txt"]);
    }

    #[test]
    fn the_newest_files_are_kept() {
        let dir = tempfile::tempdir().unwrap();
        let now = SystemTime::now();
        for (name, age) in [
            ("a.1.tmp", 50),
            ("a.2.tmp", 40),
            ("a.3.tmp", 30),
            ("a.4.tmp", 20),
            ("a.5.tmp", 10),
        ] {
            file_aged(dir.path(), name, Duration::from_secs(age), now);
        }
        file_aged(dir.path(), "other", Duration::from_secs(99), now);
        assert_eq!(keep_newest(dir.path(), &is_tmp, 3), 2);
        assert_eq!(
            names(dir.path()),
            ["a.3.tmp", "a.4.tmp", "a.5.tmp", "other"]
        );
        assert_eq!(keep_newest(dir.path(), &is_tmp, 3), 0);
    }

    #[test]
    fn a_missing_folder_is_not_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("absent");
        assert_eq!(remove_all(&missing, &is_tmp), 0);
        assert_eq!(keep_newest(&missing, &is_tmp, 1), 0);
    }
}
