//! Central reports: one `<SID>.<installation-ID>.json` file written in place into a
//! folder where users may create and rewrite their own files but not list or read.
//! No directory creation, ACL changes, append history or export without config.
use crate::inventory::Snapshot;
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

/// Called only in the supervised helper after local persistence. Users are expected to
/// hold create and own-file write rights but no list, read, delete or rename rights, so
/// nothing is read back, locked, staged or renamed. A crash can leave a short or mixed
/// file; the summary excludes malformed reports and the next launch rewrites the whole
/// file. The summary checks the file owner against the SID inside.
pub fn publish(
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
    mut saved: Snapshot,
    local_directory: &Path,
    mut configuration_unchanged: impl FnMut() -> bool,
) -> io::Result<()> {
    use crate::inventory_support::Code;
    if !configuration_unchanged() {
        return Err(io::Error::other("inventory configuration changed"));
    }
    saved.remove_issue(Code::PublicationPending);
    let published = publish(root, &saved, || {
        if configuration_unchanged() {
            Ok(())
        } else {
            Err(io::Error::other("inventory configuration changed"))
        }
    });
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
