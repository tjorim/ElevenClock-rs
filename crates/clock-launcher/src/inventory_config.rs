//! Administrator-owned central configuration. Never cached locally: offline or
//! unreadable configuration means no collection. Read only inside the supervised
//! inventory helper, because even opening a share file can block indefinitely.
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub const FILE_NAME: &str = "elevenclock-rs-launcher.json";
pub const MAX_BYTES: usize = 4096;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    schema_version: u32,
    pub local_inventory_enabled: bool,
    #[serde(default)]
    pub include_clock_format: bool,
    #[serde(default)]
    pub report_directory: Option<PathBuf>,
}

pub fn parse(bytes: &[u8]) -> Option<Config> {
    if bytes.len() > MAX_BYTES {
        return None;
    }
    let config: Config = serde_json::from_slice(bytes).ok()?;
    if config.schema_version != 1 {
        return None;
    }
    if let Some(path) = &config.report_directory {
        let text = path.to_str()?;
        // Runtime paths follow Windows semantics; Unix absolute paths also allow
        // the publishing policy to be exercised with real files on portable hosts.
        let unc = text.strip_prefix("\\\\").is_some_and(|rest| {
            let mut parts = rest.split('\\');
            let server = parts.next().unwrap_or("");
            let share = parts.next().unwrap_or("");
            !server.is_empty() && server != "." && server != "?" && !share.is_empty()
        });
        let drive = text.as_bytes().get(..3).is_some_and(|prefix| {
            prefix[0].is_ascii_alphabetic()
                && prefix[1] == b':'
                && matches!(prefix[2], b'\\' | b'/')
        });
        if text.len() > 2048
            || text.chars().any(char::is_control)
            || !(unc || drive || (cfg!(unix) && path.is_absolute()))
            || text
                .split(['\\', '/'])
                .any(|part| part == ".." || part == ".")
        {
            return None;
        }
    }
    Some(config)
}

pub fn enabled_bytes(bytes: &[u8]) -> bool {
    parse(bytes).is_some_and(|config| config.local_inventory_enabled)
}

/// The source must be a different directory from the local executable directory,
/// including aliases. All lookup failures disable inventory. No permissions change.
pub fn read(source: &Path, destination: &Path) -> Option<Config> {
    let read = || -> io::Result<Vec<u8>> {
        if source.canonicalize()? == destination.canonicalize()? {
            return Err(io::Error::other("inventory source is local"));
        }
        let mut bytes = Vec::new();
        File::open(source.join(FILE_NAME))?
            .take(MAX_BYTES as u64 + 1)
            .read_to_end(&mut bytes)?;
        Ok(bytes)
    };
    parse(&read().ok()?)
}

pub fn enabled(source: &Path, destination: &Path) -> bool {
    read(source, destination).is_some_and(|config| config.local_inventory_enabled)
}

#[cfg(test)]
mod tests {
    use super::*;
    const ON: &[u8] = br#"{"schema_version":1,"local_inventory_enabled":true}"#;
    #[test]
    fn only_explicit_supported_configuration_enables_inventory() {
        assert!(enabled_bytes(ON));
        for bytes in [b"".as_slice(), b"{}", b"null", b"invalid",
            br#"{"schema_version":2,"local_inventory_enabled":true}"#,
            br#"{"schema_version":1,"local_inventory_enabled":false}"#,
            br#"{"schema_version":1,"local_inventory_enabled":"true"}"#,
            br#"{"schema_version":1,"local_inventory_enabled":true,"destination":"share"}"#,
            br#"{"schema_version":1,"local_inventory_enabled":true,"local_inventory_enabled":false}"#] {
            assert!(!enabled_bytes(bytes));
        }
        assert!(!enabled_bytes(&vec![b' '; MAX_BYTES + 1]));
    }
    #[test]
    fn each_read_observes_changes_and_failure_never_reuses_enabled_state() {
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("share");
        let destination = root.path().join("local");
        std::fs::create_dir(&source).unwrap();
        std::fs::create_dir(&destination).unwrap();
        let path = source.join(FILE_NAME);
        assert!(!enabled(&source, &destination));
        std::fs::write(&path, ON).unwrap();
        assert!(enabled(&source, &destination));
        assert!(!enabled(&source, &source));
        std::fs::write(&path, b"invalid").unwrap();
        assert!(!enabled(&source, &destination));
        std::fs::write(&path, ON).unwrap();
        assert!(enabled(&source, &destination));
        std::fs::remove_file(path).unwrap();
        assert!(!enabled(&source, &destination));
        std::fs::remove_dir(source.clone()).unwrap();
        assert!(!enabled(&source, &destination));
    }
}

#[cfg(test)]
mod destination_tests {
    use super::*;
    #[test]
    fn optional_report_destination_accepts_absolute_paths_and_rejects_ambiguous_ones() {
        let config = |path: &str| {
            serde_json::to_vec(&serde_json::json!({
            "schema_version":1, "local_inventory_enabled":true, "report_directory":path}))
            .unwrap()
        };
        for path in [r"\\server\reports\clocks", r"C:\Reports\Clocks"] {
            let parsed = parse(&config(path)).unwrap();
            assert_eq!(parsed.report_directory, Some(path.into()));
        }
        for path in [
            "",
            "relative",
            r"C:reports",
            r"\\?\C:\reports",
            r"\\.\pipe\reports",
            r"\\server",
            r"\\server\share\..\other",
            "bad\npath",
        ] {
            assert!(parse(&config(path)).is_none(), "{path}");
        }
        assert_eq!(
            parse(br#"{"schema_version":1,"local_inventory_enabled":true}"#)
                .unwrap()
                .report_directory,
            None
        );
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;
    #[test]
    fn format_reporting_requires_its_own_explicit_boolean_setting() {
        assert!(
            !parse(br#"{"schema_version":1,"local_inventory_enabled":true}"#)
                .unwrap()
                .include_clock_format
        );
        assert!(parse(br#"{"schema_version":1,"local_inventory_enabled":true,"include_clock_format":true}"#)
            .unwrap().include_clock_format);
        assert!(parse(br#"{"schema_version":1,"local_inventory_enabled":true,"include_clock_format":"true"}"#).is_none());
    }
}
