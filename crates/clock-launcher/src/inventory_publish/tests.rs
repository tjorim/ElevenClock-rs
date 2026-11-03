use super::*;
use crate::launch::{Exit, LaunchOutcome, Observation, RefreshOutcome};

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
fn provision(root: &Path, value: &Snapshot) -> PathBuf {
    let folder = root.join(value.sid.as_ref().unwrap());
    fs::create_dir(&folder).unwrap();
    folder.join(format!("{}.json", value.installation_id.as_ref().unwrap()))
}

#[test]
fn replacement_is_bounded_and_failures_preserve_the_previous_report() {
    let root = tempfile::tempdir().unwrap();
    let value = snapshot();
    let target = provision(root.path(), &value);
    publish(root.path(), &value, || Ok(())).unwrap();
    let mut newer = value.clone();
    newer.observed_utc = Some(200);
    assert!(publish(root.path(), &newer, || Err(io::Error::other("interrupted"))).is_err());
    assert_eq!(crate::inventory::read_latest(&target).unwrap(), value);
    publish(root.path(), &newer, || Ok(())).unwrap();
    publish(root.path(), &value, || {
        panic!("older report must not replace newer")
    })
    .unwrap();
    assert_eq!(crate::inventory::read_latest(&target).unwrap(), newer);
    assert_eq!(fs::read_dir(target.parent().unwrap()).unwrap().count(), 2); // report + stable lock
}

#[test]
fn missing_destination_or_identity_does_not_create_directories_or_damage_local_data() {
    let root = tempfile::tempdir().unwrap();
    let local = root.path().join("latest.json");
    let value = snapshot();
    crate::inventory::write_latest(&local, &value, || Ok(())).unwrap();
    let missing = root.path().join("missing");
    assert!(publish(&missing, &value, || Ok(())).is_err());
    assert!(!missing.exists());
    assert_eq!(crate::inventory::read_latest(&local).unwrap(), value);
    let mut unknown = value;
    unknown.sid = None;
    assert!(publish(root.path(), &unknown, || Ok(())).is_err());
}

#[test]
fn contention_and_unknown_schema_preserve_existing_data() {
    let root = tempfile::tempdir().unwrap();
    let value = snapshot();
    let target = provision(root.path(), &value);
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(target.with_extension("lock"))
        .unwrap();
    lock.lock().unwrap();
    assert!(
        publish(root.path(), &value, || panic!(
            "must not replace while locked"
        ))
        .is_err()
    );
    drop(lock);
    fs::write(&target, b"{\"schema_version\":99}").unwrap();
    assert!(publish(root.path(), &value, || Ok(())).is_err());
    assert_eq!(fs::read(target).unwrap(), b"{\"schema_version\":99}");
}

#[test]
fn killed_helper_leftovers_are_reused_without_accumulating_files() {
    let root = tempfile::tempdir().unwrap();
    let value = snapshot();
    let target = provision(root.path(), &value);
    fs::write(target.with_extension("pending"), b"partial upload").unwrap();
    publish(root.path(), &value, || Ok(())).unwrap();
    assert_eq!(crate::inventory::read_latest(&target).unwrap(), value);
    assert!(!target.with_extension("pending").exists());
}

#[test]
fn simultaneous_sessions_never_publish_partial_json_or_accumulate_temp_files() {
    let root = tempfile::tempdir().unwrap();
    let value = snapshot();
    let target = provision(root.path(), &value);
    let results = std::thread::scope(|scope| {
        let mut writers = Vec::new();
        for time in 100..108 {
            let root = root.path();
            let mut value = value.clone();
            value.observed_utc = Some(time);
            writers.push(scope.spawn(move || publish(root, &value, || Ok(()))));
        }
        writers
            .into_iter()
            .map(|writer| writer.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(results.iter().any(Result::is_ok));
    for error in results.into_iter().filter_map(Result::err) {
        assert_eq!(error.kind(), io::ErrorKind::WouldBlock);
    }
    let saved = crate::inventory::read_latest(&target).unwrap();
    assert_eq!(saved.sid, value.sid);
    assert!((100..108).contains(&saved.observed_utc.unwrap()));
    assert_eq!(fs::read_dir(target.parent().unwrap()).unwrap().count(), 2);
}

#[test]
fn publication_status_is_local_on_failure_and_cleared_on_success() {
    use crate::inventory_support::Code;
    let root = tempfile::tempdir().unwrap();
    let local = tempfile::tempdir().unwrap();
    let mut value = snapshot();
    value.add_issue(Code::PublicationPending);
    let saved = crate::inventory::record_local(local.path(), value, || snapshot().installation_id)
        .unwrap()
        .unwrap();
    let absent = root.path().join("missing");
    assert!(
        publish_saved(
            &absent,
            ReportMode::PerUserDirectory,
            saved.clone(),
            local.path(),
            || true
        )
        .is_err()
    );
    let local_path = local.path().join("latest.json");
    let failed = crate::inventory::read_latest(&local_path).unwrap();
    assert!(
        failed
            .support_issues
            .unwrap()
            .iter()
            .any(|i| i.code == Code::PublicationFailed)
    );
    let target = provision(root.path(), &saved);
    publish_saved(
        root.path(),
        ReportMode::PerUserDirectory,
        saved.clone(),
        local.path(),
        || true,
    )
    .unwrap();
    for path in [&local_path, &target] {
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
    assert!(
        publish_saved(
            root.path(),
            ReportMode::PerUserDirectory,
            saved.clone(),
            local.path(),
            || false
        )
        .is_err()
    );
    assert_eq!(crate::inventory::read_latest(&local_path).unwrap(), saved);
}

#[test]
fn drop_box_writes_one_named_file_in_place_without_staging_or_directories() {
    let root = tempfile::tempdir().unwrap();
    let mut value = snapshot();
    publish_drop_box(root.path(), &value, || Ok(())).unwrap();
    let name = "S-1-5-21-123.12345678-1234-4234-8234-123456789abc.json";
    let target = root.path().join(name);
    assert_eq!(fs::read(&target).unwrap(), value.encode().unwrap());
    // A shorter rewrite must not leave trailing bytes from the longer one.
    value.support_issues = Some(Vec::new());
    value.observed_utc = Some(200);
    publish_drop_box(root.path(), &value, || Ok(())).unwrap();
    assert_eq!(fs::read(&target).unwrap(), value.encode().unwrap());
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 1);
}

#[test]
fn drop_box_requires_identity_time_an_existing_folder_and_unchanged_configuration() {
    let root = tempfile::tempdir().unwrap();
    let mut unknown = snapshot();
    unknown.sid = None;
    assert!(publish_drop_box(root.path(), &unknown, || Ok(())).is_err());
    unknown = snapshot();
    unknown.observed_utc = None;
    assert!(publish_drop_box(root.path(), &unknown, || Ok(())).is_err());
    assert!(publish_drop_box(&root.path().join("missing"), &snapshot(), || Ok(())).is_err());
    assert!(
        publish_drop_box(root.path(), &snapshot(), || Err(io::Error::other(
            "revoked"
        )))
        .is_err()
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn saved_publication_uses_the_selected_mode() {
    let root = tempfile::tempdir().unwrap();
    let local = tempfile::tempdir().unwrap();
    let saved =
        crate::inventory::record_local(local.path(), snapshot(), || snapshot().installation_id)
            .unwrap()
            .unwrap();
    publish_saved(
        root.path(),
        ReportMode::DropBox,
        saved,
        local.path(),
        || true,
    )
    .unwrap();
    assert!(
        root.path()
            .join("S-1-5-21-123.12345678-1234-4234-8234-123456789abc.json")
            .is_file()
    );
    assert!(!root.path().join("S-1-5-21-123").exists());
}
