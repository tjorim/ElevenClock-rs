use super::*;
use crate::launch::Exit;

fn sample() -> Snapshot {
    let mut snapshot = Snapshot::from_observation(Observation {
        exit: Exit::Done,
        refresh: RefreshOutcome::Unchanged,
        launch: LaunchOutcome::ProcessCreated,
        fallback: false,
    });
    snapshot.sid = Some("S-1-5-21-123".into());
    snapshot.installation_id = Some("12345678-1234-4234-8234-123456789abc".into());
    snapshot.observed_utc = Some(100);
    snapshot.app_version = Some("0.2.2.0".into());
    snapshot
}

#[test]
fn compatibility_and_bounds() {
    let snapshot = sample();
    assert_eq!(
        Snapshot::decode(&snapshot.encode().unwrap()).unwrap(),
        snapshot
    );
    let mut json = serde_json::to_value(&snapshot).unwrap();
    json["future_field"] = true.into();
    assert!(Snapshot::decode(&serde_json::to_vec(&json).unwrap()).is_ok());
    json["schema_version"] = 2.into();
    assert!(Snapshot::decode(&serde_json::to_vec(&json).unwrap()).is_err());
    assert!(Snapshot::decode(&vec![b' '; MAX_SNAPSHOT_BYTES + 1]).is_err());
    assert!(Snapshot::decode(b"{}").is_err());
    let mut bad = snapshot;
    bad.account_name = Some("secret\npath".into());
    assert!(bad.encode().is_err());
    bad.account_name = None;
    bad.installation_id = Some("not-an-id".into());
    assert!(bad.encode().is_err());
}

#[test]
fn unavailable_facts_are_explicit_and_not_grouped() {
    let snapshot = Snapshot::from_observation(Observation {
        exit: Exit::NoCopy,
        refresh: RefreshOutcome::SourceUnreachable,
        launch: LaunchOutcome::NotAttempted,
        fallback: false,
    });
    let bytes = snapshot.encode().unwrap();
    assert!(String::from_utf8_lossy(&bytes).contains("\"sid\":null"));
    let summary = aggregate([bytes.as_slice()], 100, 10).unwrap();
    assert!(summary.users.is_empty());
    assert!(summary.latest.is_empty());
    assert_eq!(summary.excluded, 1);
}

#[test]
fn failures_preserve_previous_snapshot_and_clean_up() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("latest.json");
    let old = sample();
    write_latest(&path, &old, || Ok(())).unwrap();
    let mut new = old.clone();
    new.observed_utc = Some(200);
    assert!(write_latest(&path, &new, || Err(io::Error::other("interrupted"))).is_err());
    assert_eq!(read_latest(&path).unwrap(), old);
    new.account_name = Some("x".repeat(1025));
    assert!(write_latest(&path, &new, || Ok(())).is_err());
    assert_eq!(read_latest(&path).unwrap(), old);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn latest_deduplicates_installations_and_preserves_per_copy_versions() {
    let old = sample();
    let mut newer = old.clone();
    newer.observed_utc = Some(200);
    newer.app_version = Some("0.3.0.0".into());
    let mut other = old.clone();
    other.installation_id = Some("aaaaaaaa-1234-4234-8234-123456789abc".into());
    let mut future = other.clone();
    future.observed_utc = Some(1000);
    let bytes: Vec<_> = [old, newer.clone(), newer, other, future]
        .iter()
        .map(|s| s.encode().unwrap())
        .collect();
    let mut records: Vec<_> = bytes.iter().map(Vec::as_slice).collect();
    records.push(b"malformed");
    let summary = aggregate(records.clone(), 220, 30).unwrap();
    assert_eq!(summary.users.len(), 1);
    assert_eq!(summary.latest.len(), 2);
    assert_eq!(summary.recent_users.len(), 1);
    assert_eq!(summary.excluded, 2);
    records.reverse();
    assert_eq!(aggregate(records, 220, 30).unwrap().latest, summary.latest);
    assert!(
        aggregate(bytes.iter().map(Vec::as_slice), 500, 30)
            .unwrap()
            .recent_users
            .is_empty()
    );
}

#[test]
fn simultaneous_writers_publish_only_complete_snapshots() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("latest.json");
    std::thread::scope(|scope| {
        for time in 0..8 {
            let path = &path;
            scope.spawn(move || {
                let mut snapshot = sample();
                snapshot.observed_utc = Some(time);
                write_latest(path, &snapshot, || Ok(())).unwrap();
            });
        }
    });
    assert!(read_latest(&path).unwrap().observed_utc.unwrap() < 8);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn local_record_retains_id_and_newer_snapshot_and_rotates_for_another_sid() {
    let dir = tempfile::tempdir().unwrap();
    let old = sample();
    let id = old.installation_id.clone();
    record_local(dir.path(), old.clone(), || id.clone()).unwrap();
    let mut next = old.clone();
    next.observed_utc = Some(200);
    next.installation_id = None;
    record_local(dir.path(), next, || panic!("must retain ID")).unwrap();
    let path = dir.path().join("latest.json");
    assert_eq!(read_latest(&path).unwrap().installation_id, id);
    record_local(dir.path(), old, || panic!("must retain ID")).unwrap();
    assert_eq!(read_latest(&path).unwrap().observed_utc, Some(200));
    let mut other = sample();
    other.sid = Some("S-1-5-21-456".into());
    other.observed_utc = Some(300);
    let rotated = "aaaaaaaa-1234-4234-8234-123456789abc".to_string();
    record_local(dir.path(), other, || Some(rotated.clone())).unwrap();
    assert_eq!(read_latest(&path).unwrap().installation_id, Some(rotated));
}

#[test]
fn local_lock_contention_does_not_wait_or_change_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    record_local(dir.path(), sample(), || sample().installation_id).unwrap();
    let lock = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(dir.path().join("inventory.lock"))
        .unwrap();
    lock.lock().unwrap();
    let error = record_local(dir.path(), sample(), || panic!("must not collect")).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::WouldBlock);
    assert_eq!(
        read_latest(&dir.path().join("latest.json")).unwrap(),
        sample()
    );
}

#[test]
fn retained_v1_fixture_and_unknown_outcomes() {
    let bytes = include_bytes!("../../tests/fixtures/inventory-v1.json");
    let snapshot = Snapshot::decode(bytes).unwrap();
    assert!(snapshot.fallback);
    assert_eq!(
        snapshot.account_name.as_deref(),
        Some("EXAMPLE\\clock-user")
    );
    let mut json: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    json["refresh"] = "future_outcome".into();
    assert!(Snapshot::decode(&serde_json::to_vec(&json).unwrap()).is_err());
    let mut json: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    json["sid"] = serde_json::Value::Null;
    assert!(Snapshot::decode(&serde_json::to_vec(&json).unwrap()).is_err());
}

#[test]
fn noncanonical_or_out_of_range_sids_cannot_split_user_counts() {
    for sid in [
        "S-1-05-21",
        "S-1-5-0001",
        "S-1-5-4294967296",
        "S-1-281474976710656-1",
        "S-1-5",
    ] {
        let mut snapshot = sample();
        snapshot.sid = Some(sid.into());
        assert!(snapshot.encode().is_err(), "{sid}");
    }
}

#[test]
fn format_collection_is_opted_in_validated_and_preserves_literal_text() {
    assert_eq!(
        collect_clock_format(false, || panic!("must not open preferences")),
        None
    );
    assert_eq!(collect_clock_format(true, || None), None);
    assert_eq!(collect_clock_format(true, || Some("%n".into())), None);
    let format = "Work 🕒 %H:%M\r\n%Y-%m-%d";
    let collected = collect_clock_format(true, || Some(format.into())).unwrap();
    assert_eq!(collected, "Work 🕒 %H:%M\n%Y-%m-%d");
    let mut snapshot = sample();
    snapshot.clock_format = Some(collected.clone());
    assert_eq!(
        Snapshot::decode(&snapshot.encode().unwrap())
            .unwrap()
            .clock_format,
        Some(collected)
    );
    snapshot.clock_format = Some("a".repeat(257));
    assert!(snapshot.encode().is_err());
    snapshot.clock_format = Some("bad\tformat".into());
    assert!(snapshot.encode().is_err());
    let old = include_bytes!("../../tests/fixtures/inventory-v1.json");
    assert_eq!(Snapshot::decode(old).unwrap().clock_format, None);
}

#[test]
fn popularity_counts_latest_users_and_installations_not_launcher_runs() {
    let mut a = sample();
    a.clock_format = Some("%H:%M\r\n%Y-%m-%d".into());
    let mut b = a.clone();
    b.clock_format = Some("%H:%M\n%Y-%m-%d".into());
    b.installation_id = Some("aaaaaaaa-1234-4234-8234-123456789abc".into());
    let mut other_user = a.clone();
    other_user.sid = Some("S-1-5-21-456".into());
    let mut changed = other_user.clone();
    changed.observed_utc = Some(200);
    changed.clock_format = Some(clock_core::format::PRESETS[0].format.into());
    let mut unavailable = a.clone();
    unavailable.installation_id = Some("bbbbbbbb-1234-4234-8234-123456789abc".into());
    unavailable.clock_format = None;
    let bytes: Vec<_> = [a.clone(), a, b, other_user, changed, unavailable]
        .iter()
        .map(|snapshot| snapshot.encode().unwrap())
        .collect();
    let result = aggregate(bytes.iter().map(Vec::as_slice), 210, 20).unwrap();
    let custom = &result.format_usage["%H:%M\n%Y-%m-%d"];
    assert_eq!(custom.installations, 2);
    assert_eq!(custom.users.len(), 1);
    assert_eq!(custom.recent_installations, 0);
    assert!(custom.recent_users.is_empty());
    assert_eq!(custom.preset, None);
    let built_in = &result.format_usage[clock_core::format::PRESETS[0].format];
    assert_eq!(built_in.installations, 1);
    assert_eq!(built_in.users.len(), 1);
    assert_eq!(built_in.recent_users.len(), 1);
    assert_eq!(built_in.recent_installations, 1);
    assert_eq!(built_in.preset, Some(clock_core::format::PRESETS[0].name));
    assert_eq!(result.format_unavailable_installations, 1);
}

#[test]
fn retained_v1_format_extension_fixture_is_compatible() {
    let snapshot = Snapshot::decode(include_bytes!(
        "../../tests/fixtures/inventory-v1-format.json"
    ))
    .unwrap();
    assert_eq!(
        snapshot.clock_format.as_deref(),
        Some("Work %H:%M\n%Y-%m-%d")
    );
    assert_eq!(
        Snapshot::decode(&snapshot.encode().unwrap()).unwrap(),
        snapshot
    );
}

#[test]
fn support_codes_are_bounded_structured_and_have_consistent_severity() {
    use crate::inventory_support::{Code, Issue, MAX_ISSUES, Severity};
    let mut snapshot = sample();
    snapshot.add_issue(Code::StartFailed);
    snapshot.add_issue(Code::StartFailed);
    let issues = snapshot.support_issues.as_ref().unwrap();
    assert_eq!(
        issues,
        &[Issue {
            code: Code::StartFailed,
            severity: Severity::Error
        }]
    );
    assert_eq!(
        Snapshot::decode(&snapshot.encode().unwrap()).unwrap(),
        snapshot
    );
    snapshot.support_issues.as_mut().unwrap()[0].severity = Severity::Warning;
    assert!(snapshot.encode().is_err());
    snapshot.support_issues = Some(vec![Code::CopyFailed.into(); MAX_ISSUES + 1]);
    assert!(snapshot.encode().is_err());
    snapshot.support_issues = Some(vec![Code::CopyFailed.into(); 2]);
    assert!(snapshot.encode().is_err());
    let mut json = serde_json::to_value(sample()).unwrap();
    json["support_issues"] =
        serde_json::json!([{"code":"C:\\private\\user\\error", "severity":"warning"}]);
    assert!(Snapshot::decode(&serde_json::to_vec(&json).unwrap()).is_err());
    assert_eq!(
        Snapshot::decode(include_bytes!("../../tests/fixtures/inventory-v1.json"))
            .unwrap()
            .support_issues,
        None
    );
}

#[test]
fn fact_failures_and_latest_support_counts_do_not_claim_live_app_health() {
    use crate::inventory_support::{Code, Severity};
    let mut a = sample();
    a.add_issue(Code::CopyFailed);
    let mut newer = a.clone();
    newer.observed_utc = Some(200);
    newer.remove_issue(Code::CopyFailed);
    newer.add_issue(Code::StartFailed);
    let mut second = a.clone();
    second.installation_id = Some("aaaaaaaa-1234-4234-8234-123456789abc".into());
    second.add_issue(Code::StartFailed);
    let mut old = second.clone();
    old.installation_id = Some("bbbbbbbb-1234-4234-8234-123456789abc".into());
    old.support_issues = None;
    let bytes: Vec<_> = [a, newer.clone(), newer, second, old]
        .iter()
        .map(|s| s.encode().unwrap())
        .collect();
    let summary = aggregate(bytes.iter().map(Vec::as_slice), 210, 20).unwrap();
    let starts = &summary.support_issue_usage[&Code::StartFailed];
    assert_eq!(starts.severity, Severity::Error);
    assert_eq!(starts.installations, 2);
    assert_eq!(starts.users.len(), 1);
    assert_eq!(starts.recent_installations, 1);
    assert_eq!(starts.recent_users.len(), 1);
    assert_eq!(
        summary.support_issue_usage[&Code::CopyFailed].installations,
        1
    );
    assert_eq!(summary.support_unavailable_installations, 1);
    let mut unknown = sample();
    unknown.sid = None;
    unknown.observed_utc = None;
    unknown.app_version = None;
    unknown.add_fact_issues();
    let codes: Vec<_> = unknown
        .support_issues
        .unwrap()
        .into_iter()
        .map(|i| i.code)
        .collect();
    assert!(codes.contains(&Code::AccountSidUnavailable));
    assert!(codes.contains(&Code::ObservationTimeUnavailable));
    assert!(codes.contains(&Code::AppVersionUnavailable));
    assert!(!codes.contains(&Code::AccountNameUnavailable));
}

#[test]
fn retained_support_fixture_preserves_known_codes_and_levels() {
    use crate::inventory_support::{Code, Severity};
    let snapshot = Snapshot::decode(include_bytes!(
        "../../tests/fixtures/inventory-v1-support.json"
    ))
    .unwrap();
    assert_eq!(snapshot.support_issues.as_ref().unwrap().len(), 3);
    assert!(
        snapshot
            .support_issues
            .as_ref()
            .unwrap()
            .iter()
            .any(|issue| issue.code == Code::StartFailed && issue.severity == Severity::Error)
    );
    assert_eq!(
        Snapshot::decode(&snapshot.encode().unwrap()).unwrap(),
        snapshot
    );
}
