//! Explicit manager action: read existing reports and atomically write a
//! JSON summary outside the report tree. No app start, collection or registry I/O.
use crate::{
    fsops,
    inventory::{self, MAX_AGGREGATION_RECORDS, MAX_SNAPSHOT_BYTES},
};
use serde_json::{Value, json};
use std::{
    io::{self, Read},
    path::Path,
};

#[cfg(test)]
fn write(directory: &Path, output: &Path, now: u64, recent_seconds: u64) -> io::Result<()> {
    write_before_read(directory, output, now, recent_seconds, || {})
}

pub fn write_checked(
    directory: &Path,
    output: &Path,
    now: u64,
    recent_seconds: u64,
    verify_owners: bool,
) -> io::Result<()> {
    write_with(directory, output, now, recent_seconds, verify_owners, || {})
}

#[cfg(test)]
fn write_before_read(
    directory: &Path,
    output: &Path,
    now: u64,
    recent_seconds: u64,
    before_read: impl FnOnce(),
) -> io::Result<()> {
    write_with(directory, output, now, recent_seconds, true, before_read)
}

fn write_with(
    directory: &Path,
    output: &Path,
    now: u64,
    recent_seconds: u64,
    verify_owners: bool,
    before_read: impl FnOnce(),
) -> io::Result<()> {
    let root = directory.canonicalize()?;
    let root_directory = clock_windows::report_files::Directory::open(&root)?;
    let output = std::path::absolute(output)?;
    let parent = output
        .parent()
        .ok_or_else(|| io::Error::other("output folder unavailable"))?
        .canonicalize()?;
    if parent.starts_with(&root) {
        return Err(io::Error::other("summary must be outside report folder"));
    }
    let output_parent = clock_windows::summary_output::Parent::open(&parent)?;
    let output_name = output
        .file_name()
        .ok_or_else(|| io::Error::other("output name unavailable"))?;
    let mut paths = Vec::new();
    let mut visited = 0;
    let mut unrecognized = 0usize;
    // Exactly the root and its immediate per-SID directories. Never follow links
    // or recursively walk arbitrary client-provided directory trees.
    for entry in root_directory.entries()? {
        let entry = entry?;
        let kind = entry.file_type()?;
        // Anyone may create names in a drop box, so names that are not reports are
        // skipped without counting toward the entry limit or being opened.
        if kind.is_file() && !looks_like_report(&entry.file_name()) {
            unrecognized += 1;
            continue;
        }
        count_entry(&mut visited)?;
        if kind.is_dir() {
            let directory = root_directory.directory(&entry.file_name())?;
            for child in directory.entries()? {
                let child = child?;
                count_entry(&mut visited)?;
                if child.file_type()?.is_file()
                    && child.path().extension().is_some_and(|ext| ext == "json")
                {
                    paths.push((Some(entry.file_name()), child.file_name()));
                }
            }
        } else if kind.is_file() && entry.path().extension().is_some_and(|ext| ext == "json") {
            paths.push((None, entry.file_name()));
        }
    }
    before_read();
    let unverified = std::cell::Cell::new(0usize);
    let records = paths.iter().map(|(directory, name)| {
        let child = directory
            .as_ref()
            .map(|name| root_directory.directory(name))
            .transpose()?;
        let directory = child.as_ref().unwrap_or(&root_directory);
        let mut bytes = Vec::new();
        let file = directory.file(name)?;
        (&file)
            .take(MAX_SNAPSHOT_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        // Drop-box files sit in a folder every user can write to, so the claimed SID is only
        // believed when the file is owned by that account. Unreadable or unsupported owner
        // checks count as unverified rather than failing the whole summary.
        if verify_owners && child.is_none() && !owner_matches(&file, &bytes) {
            unverified.set(unverified.get() + 1);
            bytes.clear();
        }
        Ok(bytes)
    });
    // Files are read one at a time; malformed reports are counted as excluded,
    // while I/O failures abort instead of quietly publishing incomplete totals.
    let summary = inventory::aggregate_records(records, now, recent_seconds)?;
    let counts =
        |installations: usize, users: usize, recent_installations: usize, recent_users: usize| {
            json!({ "installations": installations, "users": users,
            "recent_installations": recent_installations, "recent_users": recent_users })
        };
    let mut formats: Vec<Value> = summary
        .format_usage
        .iter()
        .map(|(format, usage)| {
            let mut value = counts(
                usage.installations,
                usage.users.len(),
                usage.recent_installations,
                usage.recent_users.len(),
            );
            value["format"] = json!(format);
            value["preset"] = json!(usage.preset);
            value
        })
        .collect();
    formats.sort_by(|a, b| {
        b["recent_users"]
            .as_u64()
            .cmp(&a["recent_users"].as_u64())
            .then_with(|| a["format"].as_str().cmp(&b["format"].as_str()))
    });
    let issues: Vec<Value> = summary
        .support_issue_usage
        .iter()
        .map(|(code, usage)| {
            let mut value = counts(
                usage.installations,
                usage.users.len(),
                usage.recent_installations,
                usage.recent_users.len(),
            );
            value["code"] = json!(code);
            value["severity"] = json!(usage.severity);
            value
        })
        .collect();
    let recent_installations = summary
        .latest
        .values()
        .filter(|snapshot| {
            snapshot
                .observed_utc
                .is_some_and(|time| now - time <= recent_seconds)
        })
        .count();
    let new_installations = summary
        .latest
        .values()
        .filter(|snapshot| {
            snapshot
                .first_observed_utc
                .is_some_and(|time| now - time <= recent_seconds)
        })
        .count();
    let value = json!({
        "schema_version": 1, "generated_utc": now, "recent_seconds": recent_seconds,
        "new_installations": new_installations,
        "counts": counts(summary.latest.len(), summary.users.len(), recent_installations, summary.recent_users.len()),
        "excluded_reports": summary.excluded,
        "unverified_owner_reports": unverified.get(),
        "unrecognized_files": unrecognized,
        "format_unavailable_installations": summary.format_unavailable_installations,
        "support_unavailable_installations": summary.support_unavailable_installations,
        "versions": version_breakdown(summary.latest.values(), now, recent_seconds),
        "formats": formats, "support_issues": issues,
        "latest": summary.latest.values().collect::<Vec<_>>()
    });
    let bytes = serde_json::to_vec_pretty(&value).map_err(io::Error::other)?;
    if bytes.len() as u64 > fsops::MAX_FILE_BYTES {
        return Err(io::Error::other("summary is too large"));
    }
    output_parent.install(output_name, &bytes, || Ok(()))
}

/// Only a flat (drop-box) report needs this. Non-Windows hosts cannot read owners, so
/// they treat flat reports as unverified; the managed summary command is a Windows tool.
fn owner_matches(file: &std::fs::File, bytes: &[u8]) -> bool {
    let Ok(snapshot) = inventory::Snapshot::decode(bytes) else {
        // Malformed data is excluded by aggregation and counted there.
        return true;
    };
    let (Some(sid), Ok(owner)) = (snapshot.sid, clock_windows::report_files::owner_sid(file))
    else {
        return false;
    };
    owner == sid
}

/// Dotted numeric version (`0.2.2.0`), or `None` when it is not purely numeric.
fn parse_version(text: &str) -> Option<Vec<u64>> {
    text.split('.').map(|part| part.parse().ok()).collect()
}

/// Installations per local app version, newest first (unknown last), with the share of
/// all attributed installations. "Newest" is the highest version any report carries: the
/// central release is deliberately not collected, so this is relative to what was seen.
fn version_breakdown<'a>(
    latest: impl Iterator<Item = &'a inventory::Snapshot>,
    now: u64,
    recent_seconds: u64,
) -> Value {
    #[derive(Default)]
    struct Row {
        installations: usize,
        recent: usize,
        users: std::collections::BTreeSet<String>,
    }
    let mut rows: std::collections::BTreeMap<Option<String>, Row> = Default::default();
    let mut total = 0usize;
    for snapshot in latest {
        total += 1;
        let row = rows.entry(snapshot.app_version.clone()).or_default();
        row.installations += 1;
        if snapshot
            .observed_utc
            .is_some_and(|time| now - time <= recent_seconds)
        {
            row.recent += 1;
        }
        if let Some(sid) = &snapshot.sid {
            row.users.insert(sid.clone());
        }
    }
    let mut ordered: Vec<_> = rows.into_iter().collect();
    // Newest numeric version first; non-numeric text and unknown versions follow.
    ordered.sort_by(|(a, _), (b, _)| {
        let key = |version: &Option<String>| version.as_deref().and_then(parse_version);
        key(b).cmp(&key(a)).then_with(|| a.cmp(b))
    });
    let percent = |count: usize| {
        if total == 0 {
            0.0
        } else {
            (count as f64 * 1000.0 / total as f64).round() / 10.0
        }
    };
    let newest = ordered
        .iter()
        .find(|(version, _)| version.as_deref().and_then(parse_version).is_some())
        .map(|(version, row)| (version.clone(), row.installations));
    json!({
        "newest_observed": newest.as_ref().and_then(|(version, _)| version.clone()),
        "on_newest_percent": newest.as_ref().map_or(0.0, |(_, count)| percent(*count)),
        "by_version": ordered.iter().map(|(version, row)| json!({
            "app_version": version,
            "installations": row.installations,
            "recent_installations": row.recent,
            "users": row.users.len(),
            "percent": percent(row.installations),
        })).collect::<Vec<_>>(),
    })
}

/// `<SID>.<installation-ID>.json`, the only flat names the launcher writes.
fn looks_like_report(name: &std::ffi::OsStr) -> bool {
    let Some(stem) = name.to_str().and_then(|name| name.strip_suffix(".json")) else {
        return false;
    };
    stem.rsplit_once('.')
        .is_some_and(|(sid, id)| sid.starts_with("S-1-") && id.len() == 36)
}

fn count_entry(visited: &mut usize) -> io::Result<()> {
    *visited += 1;
    // Reports plus their stable lock/staging files and per-user directories.
    if *visited > MAX_AGGREGATION_RECORDS * 4 {
        return Err(io::Error::other("report directory entry limit exceeded"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn command_deduplicates_reports_counts_exclusions_and_preserves_sources() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("reports");
        let sid = root.join("S-1-5-21-123");
        fs::create_dir_all(&sid).unwrap();
        let record = include_bytes!("../tests/fixtures/inventory-v1-support.json");
        fs::write(sid.join("one.json"), record).unwrap();
        fs::write(sid.join("duplicate.json"), record).unwrap();
        fs::write(sid.join("broken.json"), b"invalid").unwrap();
        fs::write(sid.join("ignored.pending"), b"partial").unwrap();
        let output = temp.path().join("summary.json");
        write(&root, &output, 1_800_000_000, 31_536_000).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(value["counts"]["installations"], 1);
        assert_eq!(value["counts"]["users"], 1);
        assert_eq!(value["counts"]["recent_users"], 1);
        assert_eq!(value["excluded_reports"], 1);
        assert_eq!(value["latest"].as_array().unwrap().len(), 1);
        assert!(!value["support_issues"].as_array().unwrap().is_empty());
        assert_eq!(fs::read(sid.join("one.json")).unwrap(), record);
        assert!(write(&root, &sid.join("summary.json"), 1_800_000_000, 10).is_err());
    }

    #[test]
    fn new_installations_use_first_seen_inside_the_window() {
        let temp = tempfile::tempdir().unwrap();
        let sid = temp.path().join("reports").join("S-1-5-21-123");
        fs::create_dir_all(&sid).unwrap();
        let mut record: Value = serde_json::from_slice(include_bytes!(
            "../tests/fixtures/inventory-v1-support.json"
        ))
        .unwrap();
        let observed = record["observed_utc"].as_u64().unwrap();
        record["first_observed_utc"] = json!(observed - 50);
        fs::write(sid.join("one.json"), serde_json::to_vec(&record).unwrap()).unwrap();
        for (window, expected) in [(1_000, 1), (10, 0)] {
            let output = temp.path().join("summary.json");
            write(
                &temp.path().join("reports"),
                &output,
                observed + 100,
                window,
            )
            .unwrap();
            let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
            assert_eq!(value["new_installations"], expected);
            assert_eq!(value["latest"][0]["first_observed_utc"], observed - 50);
            fs::remove_file(&output).unwrap();
        }
    }

    /// Drop-box reports are believed only when the file's owner is the SID inside it.
    #[cfg(windows)]
    #[test]
    fn drop_box_reports_need_an_owner_matching_their_sid() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("reports");
        fs::create_dir(&root).unwrap();
        let mine = clock_windows::inventory_facts::account().0.unwrap();
        let mut record: Value = serde_json::from_slice(include_bytes!(
            "../tests/fixtures/inventory-v1-support.json"
        ))
        .unwrap();
        record["sid"] = json!(mine);
        fs::write(
            root.join(format!("{mine}.12345678-1234-4234-8234-123456789abc.json")),
            serde_json::to_vec(&record).unwrap(),
        )
        .unwrap();
        record["sid"] = json!("S-1-5-21-999");
        record["installation_id"] = json!("aaaaaaaa-1234-4234-8234-123456789abc");
        fs::write(
            root.join("S-1-5-21-999.aaaaaaaa-1234-4234-8234-123456789abc.json"),
            serde_json::to_vec(&record).unwrap(),
        )
        .unwrap();
        let output = temp.path().join("summary.json");
        write(&root, &output, 1_900_000_000, 31_536_000).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        // The forged report is owned by this account, not by the SID it claims.
        let owner =
            fs::File::open(root.join(format!("{mine}.12345678-1234-4234-8234-123456789abc.json")))
                .unwrap();
        if clock_windows::report_files::owner_sid(&owner).unwrap() == mine {
            assert_eq!(value["counts"]["installations"], 1);
            assert_eq!(value["unverified_owner_reports"], 1);
        } else {
            assert_eq!(value["counts"]["installations"], 0);
            assert_eq!(value["unverified_owner_reports"], 2);
        }
        // Copied-in local snapshots are owned by whoever copied them.
        write_checked(&root, &output, 1_900_000_000, 31_536_000, false).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(value["counts"]["installations"], 2);
        assert_eq!(value["unverified_owner_reports"], 0);
    }

    #[test]
    fn names_that_are_not_reports_are_skipped_without_being_opened_or_counted() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("reports");
        fs::create_dir(&root).unwrap();
        for name in [
            "notes.txt",
            "x.json",
            "S-1-5-21-1.json",
            "S-1-5-21-1.short.json",
        ] {
            fs::write(root.join(name), b"not a report").unwrap();
        }
        let output = temp.path().join("summary.json");
        write(&root, &output, 1_900_000_000, 31_536_000).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(value["unrecognized_files"], 4);
        assert_eq!(value["excluded_reports"], 0);
        assert_eq!(value["counts"]["installations"], 0);
        assert!(looks_like_report(std::ffi::OsStr::new(
            "S-1-5-21-1.12345678-1234-4234-8234-123456789abc.json"
        )));
    }

    #[test]
    fn versions_are_ranked_newest_first_with_percentages() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("reports");
        fs::create_dir(&root).unwrap();
        let base: Value = serde_json::from_slice(include_bytes!(
            "../tests/fixtures/inventory-v1-support.json"
        ))
        .unwrap();
        let observed = base["observed_utc"].as_u64().unwrap();
        let cases = [
            (
                "0.2.2.0",
                "S-1-5-21-1",
                "aaaaaaaa-1234-4234-8234-123456789abc",
                observed,
            ),
            (
                "0.2.2.0",
                "S-1-5-21-2",
                "bbbbbbbb-1234-4234-8234-123456789abc",
                observed,
            ),
            (
                "0.2.10.0",
                "S-1-5-21-3",
                "cccccccc-1234-4234-8234-123456789abc",
                observed,
            ),
            (
                "0.2.3.0",
                "S-1-5-21-3",
                "dddddddd-1234-4234-8234-123456789abc",
                observed - 1_000_000,
            ),
        ];
        for (version, sid, id, time) in cases {
            let mut record = base.clone();
            record["app_version"] = json!(version);
            record["sid"] = json!(sid);
            record["installation_id"] = json!(id);
            record["observed_utc"] = json!(time);
            record["first_observed_utc"] = Value::Null;
            fs::write(
                root.join(format!("{sid}.{id}.json")),
                serde_json::to_vec(&record).unwrap(),
            )
            .unwrap();
        }
        let output = temp.path().join("summary.json");
        write_checked(&root, &output, observed + 10, 100, false).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        let versions = &value["versions"];
        // Numeric, not textual: 0.2.10.0 is newer than 0.2.3.0.
        assert_eq!(versions["newest_observed"], "0.2.10.0");
        assert_eq!(versions["on_newest_percent"], 25.0);
        let rows = versions["by_version"].as_array().unwrap();
        let order: Vec<_> = rows
            .iter()
            .map(|row| row["app_version"].as_str().unwrap())
            .collect();
        assert_eq!(order, ["0.2.10.0", "0.2.3.0", "0.2.2.0"]);
        assert_eq!(rows[2]["installations"], 2);
        assert_eq!(rows[2]["percent"], 50.0);
        assert_eq!(rows[2]["recent_installations"], 2);
        assert_eq!(rows[1]["recent_installations"], 0);
        // One user (S-1-5-21-3) has two versions on two installations.
        assert_eq!(rows[0]["users"], 1);
        assert_eq!(rows[1]["users"], 1);
    }

    #[test]
    fn version_parsing_is_numeric_and_unknown_versions_do_not_count_as_newest() {
        assert!(parse_version("0.2.10.0") > parse_version("0.2.3.0"));
        assert_eq!(parse_version("0.2.x"), None);
        assert_eq!(parse_version(""), None);
        let value = version_breakdown(std::iter::empty(), 0, 0);
        assert_eq!(value["newest_observed"], Value::Null);
        assert_eq!(value["on_newest_percent"], 0.0);
    }

    #[test]
    fn unavailable_input_preserves_previous_output_and_future_reports_are_excluded() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("summary.json");
        fs::write(&output, b"previous").unwrap();
        assert!(write(&temp.path().join("missing"), &output, 0, 10).is_err());
        assert_eq!(fs::read(&output).unwrap(), b"previous");
        let root = temp.path().join("reports");
        fs::create_dir(&root).unwrap();
        fs::write(
            root.join("S-1-5-21-123.12345678-1234-4234-8234-123456789abc.json"),
            include_bytes!("../tests/fixtures/inventory-v1.json"),
        )
        .unwrap();
        fs::write(
            root.join("S-1-5-21-124.bbbbbbbb-1234-4234-8234-123456789abc.json"),
            vec![b' '; MAX_SNAPSHOT_BYTES + 1],
        )
        .unwrap();
        write(&root, &output, 0, 10).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(value["counts"]["installations"], 0);
        assert_eq!(value["excluded_reports"], 2);
    }

    fn directory_link(target: &Path, link: &Path) {
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)]
        assert!(
            std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J"])
                .arg(link)
                .arg(target)
                .output()
                .unwrap()
                .status
                .success()
        );
    }

    #[test]
    fn swapped_report_and_sid_links_abort_without_replacing_output() {
        for swap_directory in [false, true] {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().join("reports");
            let sid = root.join("S-1-5-21-123");
            let outside = temp.path().join("outside");
            fs::create_dir_all(&sid).unwrap();
            fs::create_dir(&outside).unwrap();
            let record = include_bytes!("../tests/fixtures/inventory-v1-support.json");
            fs::write(sid.join("one.json"), record).unwrap();
            fs::write(outside.join("one.json"), record).unwrap();
            let output = temp.path().join("summary.json");
            fs::write(&output, b"previous summary").unwrap();
            let mut blocked_swap = false;
            let result = write_before_read(&root, &output, 1_800_000_000, 31_536_000, || {
                if swap_directory {
                    match fs::rename(&sid, root.join("old-sid")) {
                        Ok(()) => directory_link(&outside, &sid),
                        Err(_) if cfg!(windows) => blocked_swap = true,
                        Err(error) => panic!("unexpected rename error: {error}"),
                    }
                } else {
                    fs::remove_file(sid.join("one.json")).unwrap();
                    // A junction is also a nonregular JSON leaf and requires no
                    // Windows developer-mode/symlink privilege in CI.
                    directory_link(&outside, &sid.join("one.json"));
                }
            });
            if blocked_swap {
                // The held Windows root guard can prevent the swap itself.
                result.unwrap();
                let value: Value = serde_json::from_slice(&fs::read(output).unwrap()).unwrap();
                assert_eq!(value["counts"]["installations"], 1);
            } else {
                assert!(result.is_err());
                assert_eq!(fs::read(output).unwrap(), b"previous summary");
            }
        }
    }

    #[test]
    fn ordinary_report_replacement_between_enumeration_and_read_is_supported() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("reports");
        let sid = root.join("S-1-5-21-123");
        fs::create_dir_all(&sid).unwrap();
        let report = sid.join("one.json");
        fs::write(&report, b"old report").unwrap();
        let staged = sid.join("one.pending");
        fs::write(
            &staged,
            include_bytes!("../tests/fixtures/inventory-v1-support.json"),
        )
        .unwrap();
        let output = temp.path().join("summary.json");
        write_before_read(&root, &output, 1_800_000_000, 31_536_000, || {
            fs::remove_file(&report).unwrap();
            fs::rename(staged, report).unwrap();
        })
        .unwrap();
        let value: Value = serde_json::from_slice(&fs::read(output).unwrap()).unwrap();
        assert_eq!(value["counts"]["installations"], 1);
    }

    #[test]
    fn replacing_the_output_alias_cannot_redirect_the_summary_into_reports() {
        let temp = tempfile::tempdir().unwrap();
        let reports = temp.path().join("reports");
        let outside = temp.path().join("outside");
        let alias = temp.path().join("output-alias");
        fs::create_dir(&reports).unwrap();
        fs::create_dir(&outside).unwrap();
        directory_link(&outside, &alias);
        let output = alias.join("summary.json");
        write_before_read(&reports, &output, 0, 10, || {
            #[cfg(windows)]
            fs::remove_dir(&alias).unwrap();
            #[cfg(unix)]
            fs::remove_file(&alias).unwrap();
            directory_link(&reports, &alias);
        })
        .unwrap();
        assert!(outside.join("summary.json").is_file());
        assert!(!reports.join("summary.json").exists());
    }
}
