//! Managed inventory primitives, invoked only after administrator opt-in.
//! Identity-bearing values must never be passed to logs.
use crate::{
    fsops,
    inventory_support::{self, Code, Issue, Severity},
    launch::{LaunchOutcome, Observation, RefreshOutcome},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{self, Read},
    path::Path,
};

pub const SCHEMA_VERSION: u32 = 1;
pub const MAX_SNAPSHOT_BYTES: usize = 16 * 1024;
pub const MAX_AGGREGATION_RECORDS: usize = 100_000;

/// All unavailable facts are JSON null. Times are UTC Unix seconds (not local time).
/// Fields are required on emission; readers accept additive fields for v1 only.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    pub schema_version: u32,
    pub sid: Option<String>,
    pub account_name: Option<String>,
    pub installation_id: Option<String>,
    pub observed_utc: Option<u64>,
    pub app_version: Option<String>,
    pub launcher_version: Option<String>,
    /// Saved format source only; never a rendered date/time or editable draft.
    pub clock_format: Option<String>,
    pub support_issues: Option<Vec<Issue>>,
    pub refresh: RefreshOutcome,
    pub launch: LaunchOutcome,
    pub fallback: bool,
}

fn invalid() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, "invalid inventory snapshot")
}

impl Snapshot {
    pub fn from_observation(observation: Observation) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            sid: None,
            account_name: None,
            installation_id: None,
            observed_utc: None,
            app_version: None,
            launcher_version: None,
            clock_format: None,
            support_issues: Some(inventory_support::for_observation(observation)),
            refresh: observation.refresh,
            launch: observation.launch,
            fallback: observation.fallback,
        }
    }

    /// Codes are inserted once and sorted for stable serialization/tie breaking.
    pub fn add_issue(&mut self, code: Code) {
        let issues = self.support_issues.get_or_insert_with(Vec::new);
        if !issues.iter().any(|issue| issue.code == code) {
            issues.push(code.into());
            issues.sort_by_key(|issue| issue.code);
        }
    }

    pub fn remove_issue(&mut self, code: Code) {
        if let Some(issues) = &mut self.support_issues {
            issues.retain(|issue| issue.code != code);
        }
    }

    /// Fact failures are separate from launch health and never change its result.
    pub fn add_fact_issues(&mut self) {
        if self.sid.is_none() {
            self.add_issue(Code::AccountSidUnavailable);
        } else if self.account_name.is_none() {
            self.add_issue(Code::AccountNameUnavailable);
        }
        if self.observed_utc.is_none() {
            self.add_issue(Code::ObservationTimeUnavailable);
        }
        if self.app_version.is_none() {
            self.add_issue(Code::AppVersionUnavailable);
        }
    }

    pub fn validate(&self) -> io::Result<()> {
        let text = |value: &Option<String>, max: usize| {
            value
                .as_ref()
                .is_none_or(|s| !s.is_empty() && s.len() <= max && !s.chars().any(char::is_control))
        };
        let sid = self.sid.as_ref().is_none_or(|s| {
            let Some(rest) = s.strip_prefix("S-1-") else {
                return false;
            };
            let mut parts = rest.split('-');
            let canonical_number = |part: &str, max: u64| {
                part.parse::<u64>()
                    .is_ok_and(|value| value <= max && value.to_string() == part)
            };
            let authority = parts
                .next()
                .is_some_and(|part| canonical_number(part, (1u64 << 48) - 1));
            let subs: Vec<_> = parts.collect();
            authority
                && !subs.is_empty()
                && subs.len() <= 15
                && subs
                    .iter()
                    .all(|part| canonical_number(part, u32::MAX as u64))
        });
        let id = self.installation_id.as_ref().is_none_or(|s| {
            s.len() == 36
                && s.bytes().enumerate().all(|(i, c)| {
                    if [8, 13, 18, 23].contains(&i) {
                        c == b'-'
                    } else {
                        c.is_ascii_hexdigit() && !c.is_ascii_uppercase()
                    }
                })
        });
        if self.schema_version != SCHEMA_VERSION
            || !sid
            || !id
            || !text(&self.sid, 184)
            || !text(&self.account_name, 1024)
            || !text(&self.app_version, 128)
            || !text(&self.launcher_version, 128)
            || self
                .clock_format
                .as_ref()
                .is_some_and(|format| clock_core::CompiledFormat::parse(format).is_err())
            || self.support_issues.as_ref().is_some_and(|issues| {
                issues.len() > inventory_support::MAX_ISSUES
                    || issues
                        .iter()
                        .any(|issue| issue.severity != issue.code.severity())
                    || issues
                        .iter()
                        .map(|issue| issue.code)
                        .collect::<BTreeSet<_>>()
                        .len()
                        != issues.len()
            })
            || (self.account_name.is_some() && self.sid.is_none())
            || self.observed_utc.is_some_and(|t| t > 253_402_300_799)
            || (self.fallback
                && !matches!(
                    self.refresh,
                    RefreshOutcome::SourceUnreachable
                        | RefreshOutcome::SourceInvalid
                        | RefreshOutcome::CopyFailed
                ))
        {
            return Err(invalid());
        }
        Ok(())
    }

    pub fn encode(&self) -> io::Result<Vec<u8>> {
        self.validate()?;
        let bytes = serde_json::to_vec(self).map_err(|_| invalid())?;
        if bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(invalid());
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> io::Result<Self> {
        if bytes.len() > MAX_SNAPSHOT_BYTES {
            return Err(invalid());
        }
        let snapshot: Self = serde_json::from_slice(bytes).map_err(|_| invalid())?;
        snapshot.validate()?;
        Ok(snapshot)
    }
}

/// Caller supplies a local per-user application-data path, never source.txt or a
/// deployed executable. This API does not enable collection or choose a destination.
pub fn write_latest(
    path: &Path,
    snapshot: &Snapshot,
    before_replace: impl FnOnce() -> io::Result<()>,
) -> io::Result<()> {
    fsops::install(path, &snapshot.encode()?, before_replace)
}

/// Local-only recording primitive for the opted-in helper. A stable lock
/// serializes per-user sessions;
/// contention fails immediately, leaving startup independent of reporting.
/// The ID survives executable refreshes; deletion of the inventory directory
/// deliberately resets it. A different known SID rotates a copied profile's ID.
pub fn record_local(
    directory: &Path,
    mut snapshot: Snapshot,
    new_id: impl FnOnce() -> Option<String>,
) -> io::Result<Option<Snapshot>> {
    std::fs::create_dir_all(directory)?;
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(directory.join("inventory.lock"))?;
    lock.try_lock()
        .map_err(|_| io::Error::new(io::ErrorKind::WouldBlock, "inventory busy"))?;
    let path = directory.join("latest.json");
    let previous = match read_latest(&path) {
        Ok(snapshot) => Some(snapshot),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        // Do not silently replace malformed or future-schema data.
        Err(error) => return Err(error),
    };
    let same_identity = previous
        .as_ref()
        .filter(|old| old.sid == snapshot.sid || snapshot.sid.is_none());
    snapshot.installation_id = same_identity
        .and_then(|old| old.installation_id.clone())
        .or_else(new_id);
    // A failed/rolled-back clock for the same identity must not overwrite a later
    // observation. A different known SID must rotate even if its clock is earlier.
    // No time means unavailable; retain the old identity's snapshot on lookup failure.
    if same_identity.is_some_and(|old| old.observed_utc > snapshot.observed_utc) {
        return Ok(None);
    }
    write_latest(&path, &snapshot, || Ok(()))?;
    Ok(Some(snapshot))
}

pub fn read_latest(path: &Path) -> io::Result<Snapshot> {
    let mut bytes = Vec::new();
    File::open(path)?
        .take(MAX_SNAPSHOT_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    Snapshot::decode(&bytes)
}

/// Latest attributable observation per (SID, installation ID). Unknown identities,
/// unknown times and future timestamps are excluded, never guessed from filenames.
/// Stale observations remain in `latest`, but do not contribute to `recent_users`.
#[derive(Debug, Default)]
pub struct Summary {
    pub latest: BTreeMap<(String, String), Snapshot>,
    pub users: BTreeSet<String>,
    pub recent_users: BTreeSet<String>,
    pub excluded: usize,
    pub format_usage: BTreeMap<String, FormatUsage>,
    /// Includes older reports, collection disabled, and unavailable preferences.
    pub format_unavailable_installations: usize,
    pub support_issue_usage: BTreeMap<Code, IssueUsage>,
    pub support_unavailable_installations: usize,
}

#[derive(Debug, Default)]
pub struct FormatUsage {
    /// Matched against the aggregator's current built-ins, never a stored index.
    pub preset: Option<&'static str>,
    pub installations: usize,
    pub users: BTreeSet<String>,
    pub recent_installations: usize,
    pub recent_users: BTreeSet<String>,
}

#[derive(Debug)]
pub struct IssueUsage {
    pub severity: Severity,
    pub installations: usize,
    pub users: BTreeSet<String>,
    pub recent_installations: usize,
    pub recent_users: BTreeSet<String>,
}

/// The disabled branch never opens the preference store. Read failures are
/// unavailable, rather than synthesized defaults or a failed inventory report.
pub fn collect_clock_format(
    enabled: bool,
    read: impl FnOnce() -> Option<String>,
) -> Option<String> {
    if !enabled {
        return None;
    }
    let source = read()?;
    clock_core::CompiledFormat::parse(&source).ok()?;
    Some(source.replace("\r\n", "\n"))
}

pub fn aggregate<'a>(
    records: impl IntoIterator<Item = &'a [u8]>,
    now: u64,
    recent_seconds: u64,
) -> io::Result<Summary> {
    let mut summary = Summary::default();
    for (index, bytes) in records.into_iter().enumerate() {
        if index >= MAX_AGGREGATION_RECORDS {
            return Err(invalid());
        }
        let Ok(snapshot) = Snapshot::decode(bytes) else {
            summary.excluded += 1;
            continue;
        };
        let (Some(sid), Some(id), Some(time)) = (
            &snapshot.sid,
            &snapshot.installation_id,
            snapshot.observed_utc,
        ) else {
            summary.excluded += 1;
            continue;
        };
        if time > now {
            summary.excluded += 1;
            continue;
        }
        let key = (sid.clone(), id.clone());
        // Canonical JSON breaks timestamp ties deterministically, independent of
        // directory iteration order. Conflicting reports are not authentication.
        let replace = summary.latest.get(&key).is_none_or(|old| {
            (snapshot.observed_utc, snapshot.encode().ok()) > (old.observed_utc, old.encode().ok())
        });
        if replace {
            summary.latest.insert(key, snapshot);
        }
    }
    for ((sid, _), snapshot) in &summary.latest {
        summary.users.insert(sid.clone());
        let recent = snapshot
            .observed_utc
            .is_some_and(|time| now - time <= recent_seconds);
        if recent {
            summary.recent_users.insert(sid.clone());
        }
        if let Some(issues) = &snapshot.support_issues {
            for issue in issues {
                let usage = summary
                    .support_issue_usage
                    .entry(issue.code)
                    .or_insert_with(|| IssueUsage {
                        severity: issue.severity,
                        installations: 0,
                        users: BTreeSet::new(),
                        recent_installations: 0,
                        recent_users: BTreeSet::new(),
                    });
                usage.installations += 1;
                usage.users.insert(sid.clone());
                if recent {
                    usage.recent_installations += 1;
                    usage.recent_users.insert(sid.clone());
                }
            }
        } else {
            summary.support_unavailable_installations += 1;
        }
        if let Some(source) = &snapshot.clock_format {
            let source = source.replace("\r\n", "\n");
            let usage = summary.format_usage.entry(source.clone()).or_default();
            usage.preset = clock_core::format::PRESETS
                .iter()
                .find(|preset| preset.format == source)
                .map(|preset| preset.name);
            usage.installations += 1;
            usage.users.insert(sid.clone());
            if recent {
                usage.recent_installations += 1;
                usage.recent_users.insert(sid.clone());
            }
        } else {
            summary.format_unavailable_installations += 1;
        }
    }
    Ok(summary)
}

#[cfg(test)]
mod tests;

/// Small local source-state read for the inventory helper. Unlike executable reads,
/// this never allocates up to the app's 256 MiB source-file limit.
#[cfg(windows)]
pub(crate) fn read_source_state(path: &Path) -> Option<String> {
    let mut bytes = Vec::new();
    File::open(path)
        .ok()?
        .take(4097)
        .read_to_end(&mut bytes)
        .ok()?;
    if bytes.len() > 4096 {
        return None;
    }
    String::from_utf8(bytes).ok()
}
