//! Writing files so that an interruption never damages what is already there.
use crate::sha256;
use clock_windows::preferences_store::replace;
use std::{
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

/// Larger than any build of the app; stops a wrong file from filling memory.
pub const MAX_FILE_BYTES: u64 = 256 * 1024 * 1024;
/// Antivirus scanners often hold a freshly written executable for a moment.
const REPLACE_ATTEMPTS: u32 = 3;
const REPLACE_PAUSE: Duration = Duration::from_millis(200);

/// Reads a whole file, refusing one larger than [`MAX_FILE_BYTES`].
pub fn read_limited(path: &Path) -> io::Result<Vec<u8>> {
    let file = fs::File::open(path)?;
    if file.metadata()?.len() > MAX_FILE_BYTES {
        return Err(io::Error::other("file is too large"));
    }
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err(io::Error::other("file is too large"));
    }
    Ok(bytes)
}

/// The hash of the file at `path`, or `None` when it cannot be read.
pub fn hash_of(path: &Path) -> Option<sha256::Digest> {
    read_limited(path).ok().map(|bytes| sha256::digest(&bytes))
}

/// A new, empty file next to `target`, named so that it cannot clash with anything.
fn create_temporary(target: &Path) -> io::Result<(fs::File, PathBuf)> {
    let name = target
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("file");
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());
    for attempt in 0..8u32 {
        let temporary = target.with_file_name(format!(
            "{name}.tmp-{}-{nanos:x}-{attempt}",
            std::process::id()
        ));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => return Ok((file, temporary)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::other("no free temporary file name"))
}

/// Puts `bytes` at `target`: write a temporary file beside it, flush it, read it back
/// and compare hashes, then replace `target` in one step. `before_replace` runs just
/// before that step (tests use it to simulate an interruption). On any failure the
/// temporary file is removed and `target` is exactly as it was.
pub fn install(
    target: &Path,
    bytes: &[u8],
    before_replace: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    if let Some(folder) = target.parent() {
        fs::create_dir_all(folder)?;
    }
    let (mut file, temporary) = create_temporary(target)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        // Windows will not move a file that is still open here.
        drop(file);
        if hash_of(&temporary) != Some(sha256::digest(bytes)) {
            return Err(io::Error::other("the copy does not match its source"));
        }
        before_replace()?;
        let mut attempt = 1;
        loop {
            match replace(&temporary, target) {
                Err(_) if attempt < REPLACE_ATTEMPTS => {
                    attempt += 1;
                    std::thread::sleep(REPLACE_PAUSE);
                }
                other => return other,
            }
        }
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

/// Like [`install`], for a file that may be the running program itself. Windows will
/// not replace a running executable but does let it be renamed, so when the first try
/// fails the old file is moved to `parked` and the new one takes its place. `parked`
/// is deleted again on a later run, when the program that held it has ended.
pub fn install_running(
    target: &Path,
    parked: &Path,
    bytes: &[u8],
    mut before_replace: impl FnMut() -> io::Result<()>,
) -> io::Result<()> {
    let first = install(target, bytes, &mut before_replace);
    if first.is_ok() || !target.exists() {
        return first;
    }
    let _ = fs::remove_file(parked);
    fs::rename(target, parked).map_err(|_| first.unwrap_err())?;
    let second = install(target, bytes, before_replace);
    if second.is_err() {
        // Put the old file back rather than leave no launcher at all.
        let _ = fs::rename(parked, target);
    }
    second
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leftovers(folder: &Path) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(folder)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn install_creates_folders_and_replaces_content() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("a").join("b").join("app.exe");
        install(&target, b"one", || Ok(())).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"one");
        install(&target, b"two", || Ok(())).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"two");
        assert_eq!(leftovers(target.parent().unwrap()), ["app.exe"]);
    }

    #[test]
    fn an_interruption_before_the_replace_keeps_the_old_file_and_leaves_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("app.exe");
        fs::write(&target, b"old").unwrap();
        let error = install(&target, b"new", || Err(io::Error::other("cut")));
        assert!(error.is_err());
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert_eq!(leftovers(dir.path()), ["app.exe"]);
    }

    #[test]
    fn a_failed_replace_cleans_up() {
        let dir = tempfile::tempdir().unwrap();
        // A non-empty folder where the file should go cannot be replaced.
        let target = dir.path().join("app.exe");
        fs::create_dir(&target).unwrap();
        fs::write(target.join("keep"), b"x").unwrap();
        assert!(install(&target, b"new", || Ok(())).is_err());
        assert_eq!(leftovers(dir.path()), ["app.exe"]);
    }

    #[test]
    fn a_running_file_is_parked_and_replaced() {
        let dir = tempfile::tempdir().unwrap();
        let (target, parked) = (dir.path().join("l.exe"), dir.path().join("l.exe.old"));
        fs::write(&target, b"old").unwrap();
        fs::write(&parked, b"older").unwrap();
        // The first replace fails (as it does for a program that is running).
        let mut calls = 0;
        install_running(&target, &parked, b"new", || {
            calls += 1;
            if calls == 1 {
                Err(io::Error::other("in use"))
            } else {
                Ok(())
            }
        })
        .unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"new");
        assert_eq!(fs::read(&parked).unwrap(), b"old");
        assert_eq!(leftovers(dir.path()), ["l.exe", "l.exe.old"]);
    }

    #[test]
    fn when_parking_does_not_help_the_old_file_stays() {
        let dir = tempfile::tempdir().unwrap();
        let (target, parked) = (dir.path().join("l.exe"), dir.path().join("l.exe.old"));
        fs::write(&target, b"old").unwrap();
        let result = install_running(&target, &parked, b"new", || Err(io::Error::other("no")));
        assert!(result.is_err());
        assert_eq!(fs::read(&target).unwrap(), b"old");
        assert_eq!(leftovers(dir.path()), ["l.exe"]);
    }

    #[test]
    fn oversized_files_are_refused() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("big");
        let file = fs::File::create(&path).unwrap();
        file.set_len(MAX_FILE_BYTES + 1).unwrap();
        assert!(read_limited(&path).is_err());
        assert_eq!(hash_of(&path), None);
    }
}
