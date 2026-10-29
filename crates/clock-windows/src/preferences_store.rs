//! Private per-user persistence. No configuration path or file schema is a user API.
use clock_core::{Monitors, Preferences, PreferencesError, Theme, ValidatedPreferences};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const VERSION: u64 = 2;
const MAX_BYTES: u64 = 64 * 1024;
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    DamagedData,
    InvalidPreferences(PreferencesError),
    UnsupportedVersion(u64),
    RecoveryRequired,
}
impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(_) => f.write_str("Preferences could not be read or saved. Your previous settings are preserved. Try again."),
            Self::DamagedData => f.write_str("Saved preferences are damaged. Defaults are in use. Settings can preserve the damaged data and save a new selection."),
            Self::InvalidPreferences(e) => write!(f, "Preferences are invalid: {e}"),
            Self::UnsupportedVersion(_) => f.write_str("Saved preferences use an unsupported version. Defaults are in use; saving is disabled. Use a compatible app version."),
            Self::RecoveryRequired => f.write_str("Confirm recovery in Settings before replacing damaged preferences."),
        }
    }
}
impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::InvalidPreferences(e) => Some(e),
            _ => None,
        }
    }
}
impl From<io::Error> for StoreError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Internal application state, separate from editable preferences and OS startup state.
#[derive(Clone, Debug)]
pub struct StoredPreferences {
    preferences: ValidatedPreferences,
    first_run_completed: bool,
    updates: UpdateHistory,
    needs_migration: bool,
}
impl Default for StoredPreferences {
    fn default() -> Self {
        Self {
            preferences: Preferences::default()
                .validate()
                .expect("built-in defaults are valid"),
            first_run_completed: false,
            updates: UpdateHistory::default(),
            needs_migration: false,
        }
    }
}
/// Bounded private update state; timestamps are UTC epoch seconds.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateHistory {
    pub last_update_check: Option<u64>,
    pub last_notified_version: Option<String>,
    pub backoff_until: u64,
}
/// Safe storage boundary. Implementations must leave prior data intact on failed save.
/// Missing storage returns defaults. Recovery preserves the unreadable original.
pub trait PreferencesStore {
    fn load(&self) -> Result<StoredPreferences, StoreError>;
    fn save(&mut self, value: &StoredPreferences) -> Result<(), StoreError>;
    fn preserve_for_recovery(&mut self) -> Result<(), StoreError>;
}
/// Settings owns a draft; only a successfully persisted value becomes active.
pub struct PreferenceSession<S> {
    store: S,
    active: StoredPreferences,
    load_error: Option<StoreError>,
}
impl<S: PreferencesStore> PreferenceSession<S> {
    pub fn open(mut store: S) -> Self {
        let (active, load_error) = match store.load() {
            Ok(mut value) => {
                let error = if value.needs_migration {
                    store.save(&value).err()
                } else {
                    None
                };
                if error.is_none() {
                    value.needs_migration = false;
                }
                (value, error)
            }
            Err(e) => (StoredPreferences::default(), Some(e)),
        };
        Self {
            store,
            active,
            load_error,
        }
    }
    pub fn update_history(&self) -> &UpdateHistory {
        &self.active.updates
    }
    /// Shares the Settings commit boundary; never publishes a failed write.
    pub fn save_update_history(&mut self, updates: UpdateHistory) -> Result<(), StoreError> {
        if self.load_error.is_some() {
            return Err(StoreError::RecoveryRequired);
        }
        let mut next = self.active.clone();
        next.updates = updates;
        self.store.save(&next)?;
        self.active = next;
        Ok(())
    }
    pub fn active(&self) -> &ValidatedPreferences {
        &self.active.preferences
    }
    pub fn load_error(&self) -> Option<&StoreError> {
        self.load_error.as_ref()
    }
    pub fn first_run_completed(&self) -> bool {
        self.active.first_run_completed
    }
    /// Reset never writes, applies, or changes first-run completion.
    pub fn reset_draft(&self) -> Preferences {
        Preferences::default()
    }
    pub fn apply(&mut self, draft: Preferences) -> Result<(), StoreError> {
        self.commit(draft, self.active.first_run_completed, false)
    }
    /// Explicit Settings recovery preserves the original before saving. Newer versions
    /// cannot be recovered by this app, even through this action.
    pub fn recover(&mut self, draft: Preferences) -> Result<(), StoreError> {
        self.commit(draft, self.active.first_run_completed, true)
    }
    /// Called internally by onboarding after its successful completion.
    pub fn complete_first_run(&mut self) -> Result<(), StoreError> {
        self.commit(self.active.preferences.draft(), true, false)
    }
    fn commit(
        &mut self,
        draft: Preferences,
        first_run_completed: bool,
        recover: bool,
    ) -> Result<(), StoreError> {
        let preferences = draft.validate().map_err(StoreError::InvalidPreferences)?;
        if let Some(StoreError::UnsupportedVersion(v)) = self.load_error {
            return Err(StoreError::UnsupportedVersion(v));
        }
        if self.load_error.is_some() {
            if !recover {
                return Err(StoreError::RecoveryRequired);
            }
            self.store.preserve_for_recovery()?;
        }
        let next = StoredPreferences {
            preferences,
            first_run_completed,
            updates: self.active.updates.clone(),
            needs_migration: false,
        };
        self.store.save(&next)?;
        self.active = next;
        self.load_error = None;
        Ok(())
    }
}

#[derive(Default)]
pub struct MemoryPreferencesStore {
    value: Option<StoredPreferences>,
}
impl PreferencesStore for MemoryPreferencesStore {
    fn load(&self) -> Result<StoredPreferences, StoreError> {
        Ok(self.value.clone().unwrap_or_default())
    }
    fn save(&mut self, value: &StoredPreferences) -> Result<(), StoreError> {
        self.value = Some(value.clone());
        Ok(())
    }
    fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
        Ok(())
    }
}

/// Stand-in when the per-user data folder cannot be resolved. The clock still runs
/// with defaults; Settings can show them, but every save fails visibly instead of
/// appearing to succeed and being lost at the next start.
pub struct UnavailableStore;
impl PreferencesStore for UnavailableStore {
    fn load(&self) -> Result<StoredPreferences, StoreError> {
        Ok(StoredPreferences::default())
    }
    fn save(&mut self, _: &StoredPreferences) -> Result<(), StoreError> {
        Err(StoreError::Io(io::Error::other("data folder unavailable")))
    }
    fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
        Err(StoreError::Io(io::Error::other("data folder unavailable")))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Document {
    version: u64,
    format: String,
    monitors: String,
    font_size_pt: i64,
    theme: String,
    first_run_completed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    update_check_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    updates: Option<UpdateHistory>,
}
fn decode(bytes: &[u8]) -> Result<StoredPreferences, StoreError> {
    // Read the version before decoding this version's fields, including when a newer
    // writer changed the entire payload shape. Never rewrite an unsupported version.
    let raw: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| StoreError::DamagedData)?;
    let version = raw
        .get("version")
        .and_then(|v| v.as_u64())
        .ok_or(StoreError::DamagedData)?;
    if version != 1 && version != VERSION {
        return Err(StoreError::UnsupportedVersion(version));
    }
    let doc: Document = serde_json::from_slice(bytes).map_err(|_| StoreError::DamagedData)?;
    if (version == 1 && (raw.get("update_check_enabled").is_some() || raw.get("updates").is_some()))
        || (version == 2 && (doc.update_check_enabled.is_none() || doc.updates.is_none()))
        || doc
            .updates
            .as_ref()
            .and_then(|u| u.last_notified_version.as_ref())
            .is_some_and(|v| {
                v.len() > 256 || crate::update_check::version(v).is_none_or(|(_, pre)| pre)
            })
    {
        return Err(StoreError::DamagedData);
    }
    let preferences = (|| {
        Preferences {
            format: doc.format,
            monitors: Monitors::try_from(doc.monitors.as_str())?,
            font_size_pt: u8::try_from(doc.font_size_pt)
                .map_err(|_| PreferencesError::FontSizeOutOfRange(doc.font_size_pt))?,
            theme: Theme::try_from(doc.theme.as_str())?,
            update_check_enabled: doc.update_check_enabled.unwrap_or(false),
        }
        .validate()
    })()
    .map_err(StoreError::InvalidPreferences)?;
    Ok(StoredPreferences {
        preferences,
        first_run_completed: doc.first_run_completed,
        updates: doc.updates.unwrap_or_default(),
        needs_migration: version == 1,
    })
}
fn encode(value: &StoredPreferences) -> Result<Vec<u8>, StoreError> {
    let p = &value.preferences;
    serde_json::to_vec(&Document {
        version: VERSION,
        format: p.format_source().into(),
        monitors: match p.monitors() {
            Monitors::Primary => "primary",
            Monitors::All => "all",
        }
        .into(),
        font_size_pt: i64::from(p.font_size_pt()),
        theme: match p.theme() {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
        .into(),
        first_run_completed: value.first_run_completed,
        update_check_enabled: Some(p.update_check_enabled()),
        updates: Some(value.updates.clone()),
    })
    .map_err(|_| StoreError::DamagedData)
}

/// Production constructor uses Known Folders, never HOME or a path override.
pub struct LocalPreferencesStore {
    path: PathBuf,
    recovery_save_allowed: bool,
}
/// Known Folder LocalAppData (`%LOCALAPPDATA%`), never HOME or a path override.
#[cfg(windows)]
pub fn local_app_data() -> io::Result<PathBuf> {
    use crate::bindings::{
        CoTaskMemFree, FOLDERID_LocalAppData, KF_FLAG_DEFAULT, SHGetKnownFolderPath,
    };
    // SAFETY: API allocates a NUL-terminated path. Copy it before freeing with
    // CoTaskMemFree on every path; no native pointer escapes this function.
    let folder = unsafe { SHGetKnownFolderPath(&FOLDERID_LocalAppData, KF_FLAG_DEFAULT, None) }
        .map_err(io::Error::other)?;
    let wide = unsafe { folder.as_wide() };
    use std::os::windows::ffi::OsStringExt;
    let path = PathBuf::from(std::ffi::OsString::from_wide(wide));
    unsafe { CoTaskMemFree(folder.0.cast()) };
    Ok(path)
}

/// The app-owned per-user data folder below Known Folder LocalAppData. Preferences
/// and the bounded logs both live here; never HOME or a path override.
#[cfg(windows)]
pub fn app_data_dir() -> io::Result<PathBuf> {
    Ok(local_app_data()?.join("ElevenClock-rs"))
}

#[cfg(windows)]
impl LocalPreferencesStore {
    pub fn new() -> Result<Self, StoreError> {
        Ok(Self {
            path: app_data_dir()?.join("preferences.dat"),
            recovery_save_allowed: false,
        })
    }
}
impl PreferencesStore for LocalPreferencesStore {
    fn load(&self) -> Result<StoredPreferences, StoreError> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                return Ok(StoredPreferences::default());
            }
            Err(e) => return Err(e.into()),
        };
        let mut bytes = Vec::new();
        file.take(MAX_BYTES + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 > MAX_BYTES {
            return Err(StoreError::DamagedData);
        }
        decode(&bytes)
    }
    fn save(&mut self, value: &StoredPreferences) -> Result<(), StoreError> {
        // Also guard direct trait callers, rather than relying only on a session.
        let migration_required = match self.load() {
            Err(StoreError::UnsupportedVersion(v)) => {
                return Err(StoreError::UnsupportedVersion(v));
            }
            Err(StoreError::DamagedData | StoreError::InvalidPreferences(_))
                if self.recovery_save_allowed =>
            {
                false
            }
            Err(e) => return Err(e),
            Ok(value) => value.needs_migration,
        };
        if migration_required {
            self.preserve_for_recovery()?;
        }
        self.recovery_save_allowed = false;
        self.write(value, || Ok(()))
    }
    fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
        // Copy first; the original remains in place until a successful atomic save.
        // Recovery permission is private to this store and scoped to the next save.
        if let Err(StoreError::UnsupportedVersion(v)) = self.load() {
            return Err(StoreError::UnsupportedVersion(v));
        }
        let mut source = File::open(&self.path)?;
        let (path, mut backup) = unique_file(&self.path, "preserved")?;
        let result = (|| {
            io::copy(&mut source, &mut backup)?;
            backup.sync_all()
        })();
        drop(backup);
        if let Err(e) = result {
            let _ = fs::remove_file(path);
            return Err(e.into());
        }
        self.recovery_save_allowed = true;
        Ok(())
    }
}

struct PendingFile(PathBuf);
impl Drop for PendingFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
fn unique_file(target: &Path, suffix: &str) -> io::Result<(PathBuf, File)> {
    let parent = target
        .parent()
        .ok_or_else(|| io::Error::other("missing store directory"))?;
    for _ in 0..100 {
        let n = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!("preferences.{}.{n}.{suffix}", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::other(
        "could not allocate temporary preference file",
    ))
}
impl LocalPreferencesStore {
    fn write(
        &self,
        value: &StoredPreferences,
        before_replace: impl FnOnce() -> io::Result<()>,
    ) -> Result<(), StoreError> {
        let bytes = encode(value)?;
        let parent = self
            .path
            .parent()
            .ok_or_else(|| io::Error::other("missing store directory"))?;
        fs::create_dir_all(parent)?;
        let (path, mut file) = unique_file(&self.path, "tmp")?;
        let pending = PendingFile(path);
        let written = (|| {
            file.write_all(&bytes)?;
            file.sync_all()
        })();
        // Close before Windows move/replacement; no handle remains open at commit.
        drop(file);
        written?;
        before_replace()?;
        replace(&pending.0, &self.path)?;
        Ok(())
    }
    #[cfg(test)]
    fn temporary(path: PathBuf) -> Self {
        Self {
            path,
            recovery_save_allowed: false,
        }
    }
}
/// Replace `target` with `source` (same volume) without ever leaving `target` missing
/// or half written: `MoveFileExW(REPLACE_EXISTING | WRITE_THROUGH)` on Windows.
#[cfg(windows)]
pub fn replace(source: &Path, target: &Path) -> io::Result<()> {
    use crate::bindings::{MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW};
    use std::os::windows::ffi::OsStrExt;
    use windows_core::PCWSTR;
    let source: Vec<u16> = source.as_os_str().encode_wide().chain(Some(0)).collect();
    let target: Vec<u16> = target.as_os_str().encode_wide().chain(Some(0)).collect();
    // SAFETY: owned NUL-terminated buffers live for the synchronous API call.
    // Same directory/volume. MoveFileExW supports both first create and replacement;
    // locked/denied destinations fail without truncating the previous file.
    unsafe {
        MoveFileExW(
            PCWSTR(source.as_ptr()),
            PCWSTR(target.as_ptr()),
            (MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH) as u32,
        )
    }
    .ok()
    .map_err(io::Error::other)
}
#[cfg(not(windows))]
pub fn replace(source: &Path, target: &Path) -> io::Result<()> {
    fs::rename(source, target)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn temporary() -> (tempfile::TempDir, LocalPreferencesStore) {
        let dir = tempfile::tempdir().unwrap();
        let store =
            LocalPreferencesStore::temporary(dir.path().join("owned").join("preferences.dat"));
        (dir, store)
    }
    fn changed() -> Preferences {
        Preferences {
            theme: Theme::Dark,
            monitors: Monitors::All,
            font_size_pt: 24,
            ..Preferences::default()
        }
    }
    #[test]
    fn retained_v1_fixtures_decode_and_preserve_the_encoding_contract() {
        for (bytes, expected, completed) in [
            (
                include_bytes!("../tests/fixtures/preferences/v1-defaults.json").as_slice(),
                Preferences::default(),
                false,
            ),
            (
                include_bytes!("../tests/fixtures/preferences/v1-custom.json").as_slice(),
                Preferences {
                    format: "%Y-%m-%d %H:%M:%S".into(),
                    ..changed()
                },
                true,
            ),
        ] {
            let stored = decode(bytes).unwrap();
            assert_eq!(stored.preferences.draft(), expected);
            assert_eq!(stored.first_run_completed, completed);
            let original: serde_json::Value = serde_json::from_slice(bytes).unwrap();
            let encoded: serde_json::Value =
                serde_json::from_slice(&encode(&stored).unwrap()).unwrap();
            let mut expected = original;
            expected["version"] = 2.into();
            expected["update_check_enabled"] = false.into();
            expected["updates"] = serde_json::json!({"last_update_check":null,"last_notified_version":null,"backoff_until":0});
            assert_eq!(encoded, expected);
        }
    }
    #[test]
    fn migration_preserves_original_reopens_v2_and_matches_retained_target() {
        for (source, target) in [
            (
                include_bytes!("../tests/fixtures/preferences/v1-defaults.json").as_slice(),
                include_bytes!("../tests/fixtures/preferences/v2-defaults.json").as_slice(),
            ),
            (
                include_bytes!("../tests/fixtures/preferences/v1-custom.json").as_slice(),
                include_bytes!("../tests/fixtures/preferences/v2-custom.json").as_slice(),
            ),
        ] {
            let (_dir, store) = temporary();
            fs::create_dir_all(store.path.parent().unwrap()).unwrap();
            fs::write(&store.path, source).unwrap();
            let path = store.path.clone();
            let session = PreferenceSession::open(store);
            assert!(session.load_error().is_none());
            assert!(!session.active().update_check_enabled());
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(&fs::read(&path).unwrap()).unwrap(),
                serde_json::from_slice::<serde_json::Value>(target).unwrap()
            );
            let backups: Vec<_> = fs::read_dir(path.parent().unwrap())
                .unwrap()
                .flatten()
                .filter(|e| e.path().extension().is_some_and(|s| s == "preserved"))
                .collect();
            assert_eq!(backups.len(), 1);
            assert_eq!(fs::read(backups[0].path()).unwrap(), source);
            let reopened = PreferenceSession::open(LocalPreferencesStore::temporary(path));
            assert_eq!(reopened.active(), session.active());
            assert_eq!(
                reopened.first_run_completed(),
                session.first_run_completed()
            );
        }
    }
    #[test]
    fn background_commits_preserve_settings_and_failed_writes_do_not_publish() {
        let mut session = PreferenceSession::open(MemoryPreferencesStore::default());
        session.apply(changed()).unwrap();
        let history = UpdateHistory {
            last_update_check: Some(123),
            last_notified_version: Some("1.2.3".into()),
            backoff_until: 456,
        };
        session.save_update_history(history.clone()).unwrap();
        session.apply(Preferences::default()).unwrap();
        assert_eq!(session.update_history(), &history);
        let mut failed = PreferenceSession::open(FailingStore(MemoryPreferencesStore::default()));
        assert!(failed.save_update_history(history).is_err());
        assert_eq!(failed.update_history(), &UpdateHistory::default());
    }

    #[test]
    fn failed_migration_preservation_or_commit_keeps_input_and_checks_off() {
        struct MigrationFailure {
            store: LocalPreferencesStore,
            preserve_fails: bool,
        }
        impl PreferencesStore for MigrationFailure {
            fn load(&self) -> Result<StoredPreferences, StoreError> {
                self.store.load()
            }
            fn save(&mut self, value: &StoredPreferences) -> Result<(), StoreError> {
                if self.preserve_fails {
                    return Err(io::Error::other("injected preservation failure").into());
                }
                self.store.preserve_for_recovery()?;
                self.store
                    .write(value, || Err(io::Error::other("injected commit failure")))
            }
            fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
                self.store.preserve_for_recovery()
            }
        }
        for preserve_fails in [true, false] {
            let (_dir, store) = temporary();
            let source = include_bytes!("../tests/fixtures/preferences/v1-custom.json");
            fs::create_dir_all(store.path.parent().unwrap()).unwrap();
            fs::write(&store.path, source).unwrap();
            let path = store.path.clone();
            let mut session = PreferenceSession::open(MigrationFailure {
                store,
                preserve_fails,
            });
            assert!(session.load_error().is_some());
            assert_eq!(session.active().format_source(), "%Y-%m-%d %H:%M:%S");
            assert!(!session.active().update_check_enabled());
            assert!(
                session
                    .save_update_history(UpdateHistory::default())
                    .is_err()
            );
            assert_eq!(fs::read(path).unwrap(), source);
        }
    }

    #[test]
    fn retained_v2_update_state_roundtrips_and_survives_settings_commit() {
        let bytes = include_bytes!("../tests/fixtures/preferences/v2-updates-enabled.json");
        let stored = decode(bytes).unwrap();
        assert!(!stored.needs_migration);
        assert!(stored.preferences.update_check_enabled());
        assert_eq!(
            stored.updates.last_notified_version.as_deref(),
            Some("1.2.3")
        );
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&encode(&stored).unwrap()).unwrap(),
            serde_json::from_slice::<serde_json::Value>(bytes).unwrap()
        );
        let expected = stored.updates.clone();
        let mut session = PreferenceSession::open(MemoryPreferencesStore {
            value: Some(stored),
        });
        session.apply(changed()).unwrap();
        let reopened = PreferenceSession::open(session.store);
        assert_eq!(reopened.update_history(), &expected);
    }

    #[test]
    fn unavailable_store_runs_on_defaults_and_fails_every_save_visibly() {
        let mut session = PreferenceSession::open(UnavailableStore);
        assert!(session.load_error().is_none());
        assert_eq!(session.active().font_size_pt(), 9);
        let error = session.apply(changed()).unwrap_err();
        assert!(matches!(error, StoreError::Io(_)));
        assert_eq!(session.active().font_size_pt(), 9);
        assert!(session.recover(changed()).is_err());
    }
    #[test]
    fn first_save_replacement_and_first_run() {
        let (_dir, store) = temporary();
        let path = store.path.clone();
        let mut session = PreferenceSession::open(store);
        assert!(session.load_error().is_none());
        assert!(!path.exists());
        assert!(!session.first_run_completed());
        session.apply(Preferences::default()).unwrap();
        assert_eq!(
            decode(&fs::read(&path).unwrap())
                .unwrap()
                .preferences
                .draft(),
            Preferences::default()
        );
        session.apply(changed()).unwrap();
        session.complete_first_run().unwrap();
        let session = PreferenceSession::open(LocalPreferencesStore::temporary(path));
        assert_eq!(session.active().draft(), changed());
        assert!(session.first_run_completed());
        assert_eq!(session.reset_draft(), Preferences::default());
        assert_eq!(session.active().draft(), changed());
    }
    #[test]
    fn invalid_values_have_specific_errors() {
        let bytes = encode(&StoredPreferences::default()).unwrap();
        let original: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for (field, value, expected) in [
            (
                "monitors",
                serde_json::json!("other"),
                PreferencesError::InvalidMonitors("other".into()),
            ),
            (
                "theme",
                serde_json::json!("other"),
                PreferencesError::InvalidTheme("other".into()),
            ),
            (
                "font_size_pt",
                serde_json::json!(-1),
                PreferencesError::FontSizeOutOfRange(-1),
            ),
            (
                "font_size_pt",
                serde_json::json!(256),
                PreferencesError::FontSizeOutOfRange(256),
            ),
            (
                "font_size_pt",
                serde_json::json!(5),
                PreferencesError::FontSizeOutOfRange(5),
            ),
            (
                "font_size_pt",
                serde_json::json!(25),
                PreferencesError::FontSizeOutOfRange(25),
            ),
        ] {
            let mut raw = original.clone();
            raw[field] = value;
            assert!(
                matches!(decode(&serde_json::to_vec(&raw).unwrap()), Err(StoreError::InvalidPreferences(e)) if e == expected)
            );
        }
        let mut raw = original;
        raw["format"] = serde_json::json!("%n");
        assert!(matches!(
            decode(&serde_json::to_vec(&raw).unwrap()),
            Err(StoreError::InvalidPreferences(PreferencesError::Format(_)))
        ));
        for version in [0, 3, u64::MAX] {
            assert!(
                matches!(decode(format!("{{\"version\":{version},\"future\":true}}").as_bytes()), Err(StoreError::UnsupportedVersion(v)) if v == version)
            );
        }
    }
    #[test]
    fn damaged_data_is_preserved_and_recovery_is_explicit() {
        let (_dir, store) = temporary();
        fs::create_dir_all(store.path.parent().unwrap()).unwrap();
        fs::write(&store.path, b"broken original").unwrap();
        let path = store.path.clone();
        let mut session = PreferenceSession::open(store);
        assert!(matches!(
            session.load_error(),
            Some(StoreError::DamagedData)
        ));
        assert_eq!(session.active().draft(), Preferences::default());
        assert!(matches!(
            session.apply(changed()),
            Err(StoreError::RecoveryRequired)
        ));
        assert_eq!(fs::read(&path).unwrap(), b"broken original");
        session.recover(changed()).unwrap();
        assert!(session.load_error().is_none());
        let backups: Vec<_> = fs::read_dir(path.parent().unwrap())
            .unwrap()
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|s| s == "preserved"))
            .collect();
        assert_eq!(backups.len(), 1);
        assert_eq!(fs::read(backups[0].path()).unwrap(), b"broken original");
        assert_eq!(session.active().draft(), changed());
    }
    #[test]
    fn unsupported_version_cannot_be_overwritten() {
        let (_dir, mut store) = temporary();
        fs::create_dir_all(store.path.parent().unwrap()).unwrap();
        let bytes = b"{\"version\":3,\"new_shape\":true}";
        fs::write(&store.path, bytes).unwrap();
        assert!(matches!(
            store.save(&StoredPreferences::default()),
            Err(StoreError::UnsupportedVersion(3))
        ));
        assert!(matches!(
            store.preserve_for_recovery(),
            Err(StoreError::UnsupportedVersion(3))
        ));
        let path = store.path.clone();
        let mut session = PreferenceSession::open(store);
        assert!(matches!(
            session.apply(changed()),
            Err(StoreError::UnsupportedVersion(3))
        ));
        assert!(matches!(
            session.recover(changed()),
            Err(StoreError::UnsupportedVersion(3))
        ));
        assert!(matches!(
            session.complete_first_run(),
            Err(StoreError::UnsupportedVersion(3))
        ));
        assert_eq!(fs::read(path).unwrap(), bytes);
    }
    #[test]
    fn injected_failure_before_replace_preserves_disk_and_cleans_temp() {
        let (_dir, mut store) = temporary();
        store.save(&StoredPreferences::default()).unwrap();
        let before = fs::read(&store.path).unwrap();
        let next = StoredPreferences {
            preferences: changed().validate().unwrap(),
            first_run_completed: true,
            updates: UpdateHistory::default(),
            needs_migration: false,
        };
        assert!(
            store
                .write(&next, || Err(io::Error::other("injected")))
                .is_err()
        );
        assert_eq!(fs::read(&store.path).unwrap(), before);
        assert_eq!(
            fs::read_dir(store.path.parent().unwrap()).unwrap().count(),
            1
        );
    }
    struct FailingStore(MemoryPreferencesStore);
    impl PreferencesStore for FailingStore {
        fn load(&self) -> Result<StoredPreferences, StoreError> {
            self.0.load()
        }
        fn save(&mut self, _: &StoredPreferences) -> Result<(), StoreError> {
            Err(io::Error::other("injected").into())
        }
        fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
            Ok(())
        }
    }
    #[test]
    fn failed_save_keeps_active_and_first_run() {
        let mut session = PreferenceSession::open(FailingStore(MemoryPreferencesStore::default()));
        assert!(session.apply(changed()).is_err());
        assert!(session.complete_first_run().is_err());
        assert_eq!(session.active().draft(), Preferences::default());
        assert!(!session.first_run_completed());
    }
    #[test]
    fn memory_store_and_reset_draft() {
        let mut session = PreferenceSession::open(MemoryPreferencesStore::default());
        session.apply(changed()).unwrap();
        session.complete_first_run().unwrap();
        let draft = session.reset_draft();
        assert_eq!(session.active().draft(), changed());
        session.apply(draft).unwrap();
        assert!(session.first_run_completed());
        let reopened = PreferenceSession::open(session.store);
        assert_eq!(reopened.active().draft(), Preferences::default());
        assert!(reopened.first_run_completed());
    }
    #[test]
    fn oversized_and_missing_fields_are_damaged() {
        assert!(matches!(decode(b"{}"), Err(StoreError::DamagedData)));
        let (_dir, store) = temporary();
        fs::create_dir_all(store.path.parent().unwrap()).unwrap();
        fs::write(&store.path, vec![b' '; MAX_BYTES as usize + 1]).unwrap();
        assert!(matches!(store.load(), Err(StoreError::DamagedData)));
    }
    #[test]
    fn failed_first_create_and_legacy_files_remain_untouched() {
        let (dir, store) = temporary();
        let legacy = dir.path().join(".elevenclock");
        fs::create_dir(&legacy).unwrap();
        fs::write(legacy.join("legacy-setting"), b"keep").unwrap();
        assert!(
            store
                .write(&StoredPreferences::default(), || Err(io::Error::other(
                    "injected"
                )))
                .is_err()
        );
        assert!(!store.path.exists());
        assert_eq!(
            fs::read_dir(store.path.parent().unwrap()).unwrap().count(),
            0
        );
        let mut session = PreferenceSession::open(store);
        session.apply(changed()).unwrap();
        session.complete_first_run().unwrap();
        session.apply(session.reset_draft()).unwrap();
        assert_eq!(fs::read(legacy.join("legacy-setting")).unwrap(), b"keep");
        assert_eq!(fs::read_dir(legacy).unwrap().count(), 1);
    }
    #[cfg(windows)]
    #[test]
    fn known_folder_constructor_is_app_owned() {
        let store = LocalPreferencesStore::new().unwrap();
        assert!(store.path.is_absolute());
        assert!(
            store
                .path
                .ends_with(Path::new("ElevenClock-rs").join("preferences.dat"))
        );
    }
    #[cfg(windows)]
    #[test]
    fn locked_target_preserves_disk_and_active() {
        use std::os::windows::fs::OpenOptionsExt;
        let (_dir, store) = temporary();
        let path = store.path.clone();
        let mut session = PreferenceSession::open(store);
        session.apply(Preferences::default()).unwrap();
        let before = fs::read(&path).unwrap();
        // Allow reads/writes, deny delete sharing required by MoveFileExW.
        let lock = OpenOptions::new()
            .read(true)
            .share_mode(3)
            .open(&path)
            .unwrap();
        assert!(session.apply(changed()).is_err());
        assert_eq!(fs::read(&path).unwrap(), before);
        assert_eq!(session.active().draft(), Preferences::default());
        drop(lock);
        session.apply(changed()).unwrap();
    }
    #[cfg(windows)]
    #[test]
    fn read_only_target_preserves_disk_and_active() {
        let (_dir, store) = temporary();
        let path = store.path.clone();
        let mut session = PreferenceSession::open(store);
        session.apply(Preferences::default()).unwrap();
        let before = fs::read(&path).unwrap();
        let original_permissions = fs::metadata(&path).unwrap().permissions();
        let mut permissions = original_permissions.clone();
        permissions.set_readonly(true);
        fs::set_permissions(&path, permissions.clone()).unwrap();
        let result = session.apply(changed());
        fs::set_permissions(&path, original_permissions).unwrap();
        assert!(result.is_err());
        assert_eq!(fs::read(&path).unwrap(), before);
        assert_eq!(session.active().draft(), Preferences::default());
    }
}
