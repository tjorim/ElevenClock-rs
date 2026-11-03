//! Own-report replacement in administrator-provisioned per-SID directories, or blind
//! drop-box files written in place (see `ReportMode`).
//! No directory creation, ACL changes, append history or export without config.
use crate::{
    inventory::{MAX_SNAPSHOT_BYTES, Snapshot},
    inventory_config::ReportMode,
};
use clock_windows::preferences_store::replace;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

struct Pending(PathBuf);
impl Drop for Pending {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

/// Called only in the supervised helper after local persistence. The configured
/// root and its SID subdirectory must already exist. Locks, temp files and reports
/// have fixed per-installation names, including after helper termination.
pub fn publish(
    root: &Path,
    snapshot: &Snapshot,
    before_replace: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    let bytes = snapshot.encode()?;
    let (Some(sid), Some(id)) = (&snapshot.sid, &snapshot.installation_id) else {
        return Err(io::Error::other("inventory identity unavailable"));
    };
    if snapshot.observed_utc.is_none() {
        return Err(io::Error::other("inventory time unavailable"));
    }
    let folder = root.join(sid);
    // No create_dir_all: access and provisioning belong to the administrator.
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(folder.join(format!("{id}.lock")))?;
    lock.try_lock()
        .map_err(|_| io::Error::new(io::ErrorKind::WouldBlock, "inventory publication busy"))?;
    let target = folder.join(format!("{id}.json"));
    let old = match crate::inventory::read_latest(&target) {
        Ok(snapshot) => Some(snapshot),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error),
    };
    if let Some(old) = old {
        if old.sid != snapshot.sid || old.installation_id != snapshot.installation_id {
            return Err(io::Error::other("inventory identity conflict"));
        }
        if old.observed_utc > snapshot.observed_utc {
            return Ok(());
        }
    }
    let pending = Pending(folder.join(format!("{id}.pending")));
    // The stable lock excludes simultaneous sessions. A leftover .pending from a
    // terminated helper is truncated; no new orphan filenames accumulate.
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&pending.0)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    let mut verified = Vec::new();
    File::open(&pending.0)?
        .take(MAX_SNAPSHOT_BYTES as u64 + 1)
        .read_to_end(&mut verified)?;
    if verified != bytes {
        return Err(io::Error::other("inventory verification failed"));
    }
    before_replace()?;
    replace(&pending.0, &target)
}

/// Blind drop box: `<root><SID>.<installation-ID>.json`, written in place. Users are
/// expected to hold create and own-file write rights but no list, read, delete or
/// rename rights, so nothing is read back, locked, staged or renamed. A crash can leave a
/// short or mixed file; the summary excludes malformed reports and the next launch
/// rewrites the whole file. The summary checks the file owner against the SID inside.
pub fn publish_drop_box(
    root: &Path,
    snapshot: &Snapshot,
    before_write: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    let bytes = snapshot.encode()?;
    let (Some(sid), Some(id)) = (&snapshot.sid, &snapshot.installation_id) else {
        return Err(io::Error::other("inventory identity unavailable"));
    };
    if snapshot.observed_utc.is_none() {
        return Err(io::Error::other("inventory time unavailable"));
    }
    before_write()?;
    // No create_dir_all and no truncate-on-open: overwrite needs only write-data rights.
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(root.join(format!("{sid}.{id}.json")))?;
    file.write_all(&bytes)?;
    file.set_len(bytes.len() as u64)?;
    file.sync_all()
}

/// Publish an already committed local observation. The caller commits
/// `publication_pending` first; a process deadline can then leave an honest
/// incomplete status instead of falsely claiming success or a known failure.
pub fn publish_saved(
    root: &Path,
    mode: ReportMode,
    mut saved: Snapshot,
    local_directory: &Path,
    mut configuration_unchanged: impl FnMut() -> bool,
) -> io::Result<()> {
    use crate::inventory_support::Code;
    if !configuration_unchanged() {
        return Err(io::Error::other("inventory configuration changed"));
    }
    saved.remove_issue(Code::PublicationPending);
    let unchanged = || {
        if configuration_unchanged() {
            Ok(())
        } else {
            Err(io::Error::other("inventory configuration changed"))
        }
    };
    let published = match mode {
        ReportMode::PerUserDirectory => publish(root, &saved, unchanged),
        ReportMode::DropBox => publish_drop_box(root, &saved, unchanged),
    };
    if published.is_err() {
        saved.add_issue(Code::PublicationFailed);
    }
    // The next successful report describes that new observation, not an error
    // history. Failure to update local status leaves the original pending record.
    if configuration_unchanged() {
        let _ = crate::inventory::record_local(local_directory, saved, || None);
    }
    published
}

#[cfg(test)]
mod tests;
