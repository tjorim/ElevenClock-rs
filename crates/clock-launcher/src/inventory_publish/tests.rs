use super::*;
use crate::{
    inventory_support::Code,
    launch::{Exit, LaunchOutcome, Observation, RefreshOutcome},
};
use std::fs;

const NAME: &str = "S-1-5-21-123.12345678-1234-4234-8234-123456789abc.json";

fn snapshot() -> Snapshot {
    let mut value = Snapshot::from_observation(Observation {
        local_launcher_current: false,
        exit: Exit::Done,
        refresh: RefreshOutcome::Updated,
        launch: LaunchOutcome::ProcessCreated,
        fallback: false,
    });
    value.sid = Some("S-1-5-21-123".into());
    value.installation_id = Some("12345678-1234-4234-8234-123456789abc".into());
    value.observed_utc = Some(100);
    value
}

#[test]
fn one_named_file_is_written_in_place_without_staging_or_directories() {
    let root = tempfile::tempdir().unwrap();
    let mut value = snapshot();
    publish(root.path(), &value, || Ok(())).unwrap();
    let target = root.path().join(NAME);
    assert_eq!(fs::read(&target).unwrap(), value.encode().unwrap());
    // A shorter rewrite must not leave trailing bytes from the longer one.
    value.support_issues = Some(Vec::new());
    value.observed_utc = Some(200);
    publish(root.path(), &value, || Ok(())).unwrap();
    assert_eq!(fs::read(&target).unwrap(), value.encode().unwrap());
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
}

#[test]
fn identity_time_folder_and_unchanged_configuration_are_required() {
    let root = tempfile::tempdir().unwrap();
    let mut unknown = snapshot();
    unknown.sid = None;
    assert!(publish(root.path(), &unknown, || Ok(())).is_err());
    unknown = snapshot();
    unknown.observed_utc = None;
    assert!(publish(root.path(), &unknown, || Ok(())).is_err());
    assert!(publish(&root.path().join("missing"), &snapshot(), || Ok(())).is_err());
    assert!(
        publish(root.path(), &snapshot(), || Err(io::Error::other(
            "revoked"
        )))
        .is_err()
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    assert!(!root.path().join("missing").exists());
}

#[test]
fn simultaneous_sessions_leave_one_file_and_a_later_write_repairs_it() {
    let root = tempfile::tempdir().unwrap();
    std::thread::scope(|scope| {
        let writers: Vec<_> = (100..108)
            .map(|time| {
                let root = root.path();
                scope.spawn(move || {
                    let mut value = snapshot();
                    value.observed_utc = Some(time);
                    publish(root, &value, || Ok(()))
                })
            })
            .collect();
        for writer in writers {
            // Overlapping writes may mix bytes or hit a sharing violation. They may not
            // leave extra files, and the next launch rewrites a complete report.
            let _ = writer.join().unwrap();
        }
    });
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
    publish(root.path(), &snapshot(), || Ok(())).unwrap();
    assert!(Snapshot::decode(&fs::read(root.path().join(NAME)).unwrap()).is_ok());
}

#[test]
fn publication_status_is_local_on_failure_and_cleared_on_success() {
    let root = tempfile::tempdir().unwrap();
    let local = tempfile::tempdir().unwrap();
    let mut value = snapshot();
    value.add_issue(Code::PublicationPending);
    let saved = crate::inventory::record_local(local.path(), value, || snapshot().installation_id)
        .unwrap()
        .unwrap();
    let absent = root.path().join("missing");
    assert!(publish_saved(&absent, saved.clone(), local.path(), || true).is_err());
    let local_path = local.path().join("latest.json");
    let failed = crate::inventory::read_latest(&local_path).unwrap();
    assert!(
        failed
            .support_issues
            .unwrap()
            .iter()
            .any(|i| i.code == Code::PublicationFailed)
    );
    publish_saved(root.path(), saved.clone(), local.path(), || true).unwrap();
    for path in [&local_path, &root.path().join(NAME)] {
        assert!(
            crate::inventory::read_latest(path)
                .unwrap()
                .support_issues
                .unwrap()
                .iter()
                .all(|i| !matches!(i.code, Code::PublicationFailed | Code::PublicationPending))
        );
    }
    // Revocation prevents any export/status rewrite; an unfinished local attempt
    // remains pending instead of being mislabeled as a known failure or success.
    crate::inventory::record_local(local.path(), saved.clone(), || None).unwrap();
    assert!(publish_saved(root.path(), saved.clone(), local.path(), || false).is_err());
    assert_eq!(crate::inventory::read_latest(&local_path).unwrap(), saved);
}

/// Manual check against a real drop-box folder (see docs/managed-deployment.md):
/// `ELEVENCLOCK_DROP_BOX=<folder> cargo test -p clock-launcher drop_box_folder -- --ignored`.
/// Publishes twice, so the second write exercises the rewrite-without-read path.
#[test]
#[ignore = "needs a configured drop-box folder"]
fn drop_box_folder_accepts_create_and_rewrite() {
    let root = std::env::var_os("ELEVENCLOCK_DROP_BOX").expect("set ELEVENCLOCK_DROP_BOX");
    let mut value = snapshot();
    publish(Path::new(&root), &value, || Ok(())).unwrap();
    value.observed_utc = Some(200);
    value.support_issues = Some(Vec::new());
    publish(Path::new(&root), &value, || Ok(())).unwrap();
}

#[test]
fn a_busy_publication_lock_or_a_newer_local_snapshot_writes_nothing() {
    let root = tempfile::tempdir().unwrap();
    let local = tempfile::tempdir().unwrap();
    let saved =
        crate::inventory::record_local(local.path(), snapshot(), || snapshot().installation_id)
            .unwrap()
            .unwrap();
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(local.path().join("publication.lock"))
        .unwrap();
    lock.lock().unwrap();
    let error = publish_saved(root.path(), saved.clone(), local.path(), || true).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::WouldBlock);
    drop(lock);
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    // A newer observation recorded meanwhile supersedes this one.
    let mut newer = snapshot();
    newer.observed_utc = Some(500);
    crate::inventory::record_local(local.path(), newer, || None).unwrap();
    publish_saved(root.path(), saved, local.path(), || true).unwrap();
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}
