//! Explicit administrator action: read existing reports and atomically write a
//! JSON summary outside the report tree. No app start, collection or registry I/O.
use crate::{
    fsops,
    inventory::{self, MAX_AGGREGATION_RECORDS, MAX_SNAPSHOT_BYTES},
};
use serde_json::{Value, json};
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

pub fn write(directory: &Path, output: &Path, now: u64, recent_seconds: u64) -> io::Result<()> {
    let root = directory.canonicalize()?;
    let output = std::path::absolute(output)?;
    let parent = output
        .parent()
        .ok_or_else(|| io::Error::other("output folder unavailable"))?
        .canonicalize()?;
    if parent.starts_with(&root) {
        return Err(io::Error::other("summary must be outside report folder"));
    }
    let mut paths = Vec::new();
    let mut visited = 0;
    // Exactly the root and its immediate per-SID directories. Never follow links
    // or recursively walk arbitrary client-provided directory trees.
    for entry in fs::read_dir(&root)? {
        let entry = entry?;
        count_entry(&mut visited)?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            for child in fs::read_dir(entry.path())? {
                let child = child?;
                count_entry(&mut visited)?;
                if child.file_type()?.is_file()
                    && child.path().extension().is_some_and(|ext| ext == "json")
                {
                    paths.push(child.path());
                }
            }
        } else if kind.is_file() && entry.path().extension().is_some_and(|ext| ext == "json") {
            paths.push(entry.path());
        }
    }
    let records = paths.iter().map(|path| {
        let mut bytes = Vec::new();
        File::open(path)?
            .take(MAX_SNAPSHOT_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
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
    let value = json!({
        "schema_version": 1, "generated_utc": now, "recent_seconds": recent_seconds,
        "counts": counts(summary.latest.len(), summary.users.len(), recent_installations, summary.recent_users.len()),
        "excluded_reports": summary.excluded,
        "format_unavailable_installations": summary.format_unavailable_installations,
        "support_unavailable_installations": summary.support_unavailable_installations,
        "formats": formats, "support_issues": issues,
        "latest": summary.latest.values().collect::<Vec<_>>()
    });
    let bytes = serde_json::to_vec_pretty(&value).map_err(io::Error::other)?;
    fsops::install(&output, &bytes, || Ok(()))
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
    fn unavailable_input_preserves_previous_output_and_future_reports_are_excluded() {
        let temp = tempfile::tempdir().unwrap();
        let output = temp.path().join("summary.json");
        fs::write(&output, b"previous").unwrap();
        assert!(write(&temp.path().join("missing"), &output, 0, 10).is_err());
        assert_eq!(fs::read(&output).unwrap(), b"previous");
        let root = temp.path().join("reports");
        fs::create_dir(&root).unwrap();
        fs::write(
            root.join("future.json"),
            include_bytes!("../tests/fixtures/inventory-v1.json"),
        )
        .unwrap();
        fs::write(
            root.join("oversized.json"),
            vec![b' '; MAX_SNAPSHOT_BYTES + 1],
        )
        .unwrap();
        write(&root, &output, 0, 10).unwrap();
        let value: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert_eq!(value["counts"]["installations"], 0);
        assert_eq!(value["excluded_reports"], 2);
    }
}
