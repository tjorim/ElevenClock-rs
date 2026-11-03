//! Named local presentation presets, stored separately from active preferences.
//! Invalid/newer libraries are preserved; reads and writes are bounded and explicit.
use clock_core::template::ClockTemplate;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const MAX_PRESETS: usize = 50;
const MAX_BYTES: usize = 256 * 1024;
static STAGING_ID: AtomicU64 = AtomicU64::new(0);
thread_local! {
    static INCOMING: RefCell<Option<ClockTemplate>> = const { RefCell::new(None) };
    static LAST_ACCEPTED: std::cell::Cell<Option<std::time::Instant>> = const { std::cell::Cell::new(None) };
}
const MIN_DELIVERY_INTERVAL: std::time::Duration = std::time::Duration::from_secs(1);
/// Accepts a template delivered by another process only when none is already waiting
/// and the previous delivery was not just now, so a flood cannot replace a waiting
/// template or repeatedly pull focus.
pub fn try_receive(template: ClockTemplate) -> bool {
    let now = std::time::Instant::now();
    let recent = LAST_ACCEPTED.with(|last| {
        last.get()
            .is_some_and(|at| now.duration_since(at) < MIN_DELIVERY_INTERVAL)
    });
    if recent || INCOMING.with(|slot| slot.borrow().is_some()) {
        return false;
    }
    LAST_ACCEPTED.with(|last| last.set(Some(now)));
    receive(template);
    true
}
pub fn receive(template: ClockTemplate) {
    INCOMING.with(|slot| *slot.borrow_mut() = Some(template));
}
pub fn take_incoming() -> Option<ClockTemplate> {
    INCOMING.with(|slot| slot.borrow_mut().take())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedTemplate {
    pub name: String,
    pub link: String,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope {
    version: u32,
    presets: Vec<SavedTemplate>,
}
pub struct Library {
    path: PathBuf,
    entries: Vec<SavedTemplate>,
}
fn invalid() -> io::Error {
    io::Error::other(
        "Saved templates could not be read or saved. Existing templates were preserved.",
    )
}
fn valid_name(name: &str) -> bool {
    !name.trim().is_empty()
        && name == name.trim()
        && name.chars().count() <= 48
        && !name.chars().any(char::is_control)
}
impl Library {
    #[cfg(windows)]
    pub fn open() -> io::Result<Self> {
        Self::at(&crate::preferences_store::app_data_dir()?.join("templates.json"))
    }
    pub fn at(path: &Path) -> io::Result<Self> {
        let mut bytes = Vec::new();
        let entries = match File::open(path) {
            Err(e) if e.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(e) => return Err(e),
            Ok(file) => {
                file.take(MAX_BYTES as u64 + 1).read_to_end(&mut bytes)?;
                if bytes.len() > MAX_BYTES {
                    return Err(invalid());
                }
                let envelope: Envelope = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
                if envelope.version != 1 || envelope.presets.len() > MAX_PRESETS {
                    return Err(invalid());
                }
                let mut names = std::collections::BTreeSet::new();
                for entry in &envelope.presets {
                    if !valid_name(&entry.name)
                        || !names.insert(entry.name.to_lowercase())
                        || ClockTemplate::from_link(&entry.link).is_err()
                    {
                        return Err(invalid());
                    }
                }
                envelope.presets
            }
        };
        Ok(Self {
            path: path.into(),
            entries,
        })
    }
    pub fn entries(&self) -> &[SavedTemplate] {
        &self.entries
    }
    pub fn save(&mut self, name: &str, template: &ClockTemplate) -> io::Result<()> {
        let name = name.trim();
        if !valid_name(name) {
            return Err(io::Error::other(
                "Enter a preset name of 1 to 48 characters.",
            ));
        }
        // Re-read the file so another session's changes are not overwritten.
        self.entries = Self::at(&self.path)?.entries;
        let mut entries = self.entries.clone();
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.name.to_lowercase() == name.to_lowercase())
        {
            *entry = SavedTemplate {
                name: name.into(),
                link: template.share_link(),
            };
        } else {
            if entries.len() >= MAX_PRESETS {
                return Err(io::Error::other("You can save up to 50 local presets."));
            }
            entries.push(SavedTemplate {
                name: name.into(),
                link: template.share_link(),
            });
        }
        entries.sort_by_key(|entry| entry.name.to_lowercase());
        self.commit(entries)
    }
    /// Removes the preset with this name from the current on-disk library.
    pub fn remove(&mut self, name: &str) -> io::Result<()> {
        self.entries = Self::at(&self.path)?.entries;
        let mut entries = self.entries.clone();
        let index = entries
            .iter()
            .position(|entry| entry.name == name)
            .ok_or_else(|| io::Error::other("That preset no longer exists."))?;
        entries.remove(index);
        self.commit(entries)
    }
    fn commit(&mut self, entries: Vec<SavedTemplate>) -> io::Result<()> {
        let bytes = serde_json::to_vec(&Envelope {
            version: 1,
            presets: entries.clone(),
        })
        .map_err(|_| invalid())?;
        if bytes.len() > MAX_BYTES {
            return Err(invalid());
        }
        let parent = self.path.parent().ok_or_else(invalid)?;
        fs::create_dir_all(parent)?;
        // Each writer owns its staging file, including concurrent Windows sessions.
        // Atomic replacement is last-writer-wins, never a partially written library.
        let (mut file, temporary) = {
            let mut staged = None;
            for _ in 0..8 {
                let temporary = parent.join(format!(
                    "templates.{}.{}.pending",
                    std::process::id(),
                    STAGING_ID.fetch_add(1, Ordering::Relaxed)
                ));
                match OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&temporary)
                {
                    Ok(file) => {
                        staged = Some((file, temporary));
                        break;
                    }
                    Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(error),
                }
            }
            staged.ok_or_else(invalid)?
        };
        let result = (|| {
            file.write_all(&bytes)?;
            file.sync_all()?;
            drop(file);
            crate::preferences_store::replace(&temporary, &self.path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
        self.entries = entries;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clock_core::Theme;
    #[test]
    fn retained_v1_library_preserves_its_contract() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("templates.json");
        let fixture = include_bytes!("../tests/fixtures/templates/v1-custom.json");
        fs::write(&path, fixture).unwrap();
        let mut library = Library::at(&path).unwrap();
        assert_eq!(library.entries().len(), 2);
        assert_eq!(library.entries()[1].name, "Work");
        let template = ClockTemplate::from_link(&library.entries()[1].link).unwrap();
        assert_eq!(template.format(), "%H:%M");
        assert_eq!(template.font_size_pt(), 12);
        assert_eq!(template.theme(), Theme::Dark);
        library.save("Work", &template).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&fs::read(&path).unwrap()).unwrap(),
            serde_json::from_slice::<serde_json::Value>(fixture).unwrap()
        );
    }
    #[test]
    fn named_presets_round_trip_replace_and_delete_without_active_preferences() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("templates.json");
        let mut library = Library::at(&path).unwrap();
        let a = ClockTemplate::new("%H:%M", 9, Theme::Dark).unwrap();
        let b = ClockTemplate::new("%g%V.%u", 10, Theme::System).unwrap();
        library.save("Work", &a).unwrap();
        library.save("work", &b).unwrap();
        assert_eq!(library.entries().len(), 1);
        let mut reopened = Library::at(&path).unwrap();
        assert_eq!(ClockTemplate::from_link(&reopened.entries()[0].link), Ok(b));
        // A second session's save is kept by a stale handle's later write.
        let mut stale = Library::at(&path).unwrap();
        reopened.save("Home", &a).unwrap();
        stale.save("Other", &a).unwrap();
        assert_eq!(Library::at(&path).unwrap().entries().len(), 3);
        assert!(stale.remove("missing").is_err());
        reopened.remove("Home").unwrap();
        reopened.remove("Other").unwrap();
        reopened.remove("work").unwrap();
        assert!(Library::at(&path).unwrap().entries().is_empty());
        assert!(!dir.path().join("preferences.dat").exists());
    }
    #[test]
    fn newer_or_invalid_libraries_and_failed_saves_are_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("templates.json");
        for bytes in [b"invalid".as_slice(), br#"{"version":2,"presets":[]}"#] {
            fs::write(&path, bytes).unwrap();
            assert!(Library::at(&path).is_err());
            assert_eq!(fs::read(&path).unwrap(), bytes);
        }
        fs::remove_file(&path).unwrap();
        let mut library = Library::at(&path).unwrap();
        fs::create_dir(&path).unwrap();
        assert!(
            library
                .save("Work", &ClockTemplate::new("%H", 9, Theme::System).unwrap())
                .is_err()
        );
        assert!(library.entries().is_empty());
        assert!(path.is_dir());
    }
}
