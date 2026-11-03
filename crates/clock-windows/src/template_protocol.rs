//! Per-user URL association. Explicit Settings action; never claims another
//! program's existing handler, and removal checks the exact owned command.
use std::{io, path::Path};
pub fn command(exe: &Path) -> io::Result<String> {
    let text = exe
        .to_str()
        .ok_or_else(|| io::Error::other("executable unavailable"))?;
    if !exe.is_absolute()
        || text
            .chars()
            .any(|c| matches!(c, '"' | '%') || c.is_control())
        || !text.to_ascii_lowercase().ends_with(".exe")
    {
        return Err(io::Error::other("executable unavailable"));
    }
    Ok(format!("\"{text}\" --template \"%1\""))
}
#[cfg(windows)]
mod native {
    use super::*;
    use crate::bindings::*;
    use windows_core::{HSTRING, PCWSTR};
    const ROOT: &str = r"Software\Classes\elevenclock";
    const DESCRIPTION: &str = "URL:ElevenClock-rs template";
    fn read(root: &str) -> io::Result<Option<String>> {
        read_value(&format!(r"{root}\shell\open\command"), "")
    }
    fn read_value(root: &str, name: &str) -> io::Result<Option<String>> {
        let path = HSTRING::from(root);
        let mut data = vec![0u16; 4096];
        let mut bytes = (data.len() * 2) as u32;
        let mut kind = 0;
        // SAFETY: fixed bounded UTF-16 output and live type/length slots.
        let status = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                &path,
                &HSTRING::from(name),
                RRF_RT_ANY as u32,
                Some(&mut kind),
                Some(data.as_mut_ptr().cast()),
                Some(&mut bytes),
            )
        };
        if status.0 == ERROR_FILE_NOT_FOUND || status.0 == ERROR_PATH_NOT_FOUND {
            return Ok(None);
        }
        if status.0 != 0 {
            return Err(io::Error::from_raw_os_error(status.0));
        }
        if kind != REG_SZ || !bytes.is_multiple_of(2) {
            return Err(io::Error::other("unrecognized handler"));
        }
        let data = data
            .get(..bytes as usize / 2)
            .ok_or_else(|| io::Error::other("invalid handler"))?;
        let data = data.strip_suffix(&[0]).unwrap_or(data);
        Ok(Some(String::from_utf16(data).map_err(io::Error::other)?))
    }
    fn write(root: &str, name: &str, text: &str) -> io::Result<()> {
        let mut key = HKEY::default();
        // SAFETY: per-user fixed-owned/scratch key. Handle is closed on all paths.
        let status = unsafe {
            RegCreateKeyExW(
                HKEY_CURRENT_USER,
                &HSTRING::from(root),
                None,
                PCWSTR::null(),
                0,
                ACCESS_MASK(KEY_SET_VALUE as u32),
                None,
                &mut key,
                None,
            )
        };
        if status.0 != 0 {
            return Err(io::Error::from_raw_os_error(status.0));
        }
        let bytes: Vec<u8> = text
            .encode_utf16()
            .chain(Some(0))
            .flat_map(u16::to_le_bytes)
            .collect();
        let status =
            unsafe { RegSetValueExW(key, &HSTRING::from(name), None, REG_SZ, Some(&bytes)) };
        unsafe {
            let _ = RegCloseKey(key);
        }
        if status.0 == 0 {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(status.0))
        }
    }
    fn set(root: &str, exe: &Path, enabled: bool) -> io::Result<()> {
        let expected = command(exe)?;
        let existing = read(root)?;
        if existing.as_ref().is_some_and(|value| value != &expected)
            || read_value(root, "")?.is_some_and(|value| value != DESCRIPTION)
            || read_value(root, "URL Protocol")?.is_some_and(|value| !value.is_empty())
        {
            return Err(io::Error::other(
                "Another copy or program owns template links. Its registration was preserved.",
            ));
        }
        if enabled {
            write(root, "", DESCRIPTION)?;
            write(root, "URL Protocol", "")?;
            write(&format!(r"{root}\shell\open\command"), "", &expected)
        } else if existing.is_some() {
            delete_value(&format!(r"{root}\shell\open\command"), "")?;
            delete_value(root, "URL Protocol")?;
            delete_value(root, "")?;
            // Empty-key deletion preserves unrelated values, verbs and subkeys.
            for suffix in [r"\shell\open\command", r"\shell\open", r"\shell", ""] {
                // SAFETY: pruning is optional and only an empty key is removed.
                unsafe {
                    let _ = SHDeleteEmptyKeyW(
                        HKEY_CURRENT_USER,
                        &HSTRING::from(format!("{root}{suffix}")),
                    );
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }
    pub fn enabled(exe: &Path) -> io::Result<bool> {
        enabled_at(ROOT, exe)
    }
    fn enabled_at(root: &str, exe: &Path) -> io::Result<bool> {
        Ok(read(root)?.as_deref() == Some(command(exe)?.as_str())
            && read_value(root, "")?.is_none_or(|value| value == DESCRIPTION)
            && read_value(root, "URL Protocol")?.is_some_and(|value| value.is_empty()))
    }
    fn delete_value(root: &str, name: &str) -> io::Result<()> {
        let mut key = HKEY::default();
        // SAFETY: fixed-owned/scratch registry paths; handle closed on all paths.
        let status = unsafe {
            RegOpenKeyExW(
                HKEY_CURRENT_USER,
                &HSTRING::from(root),
                None,
                ACCESS_MASK(KEY_SET_VALUE as u32),
                &mut key,
            )
        };
        if status.0 == ERROR_FILE_NOT_FOUND || status.0 == ERROR_PATH_NOT_FOUND {
            return Ok(());
        }
        if status.0 != 0 {
            return Err(io::Error::from_raw_os_error(status.0));
        }
        let status = unsafe { RegDeleteValueW(key, &HSTRING::from(name)) };
        unsafe {
            let _ = RegCloseKey(key);
        }
        if status.0 == 0 || status.0 == ERROR_FILE_NOT_FOUND {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(status.0))
        }
    }
    pub fn set_enabled(exe: &Path, enabled: bool) -> io::Result<()> {
        set(ROOT, exe, enabled)?;
        // SAFETY: notify the shell of the explicit association change, with no
        // pointers or resources retained by the call.
        unsafe {
            SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST as u32, None, None);
        }
        Ok(())
    }
    fn remove_owned(root: &str, exe: &Path) -> io::Result<bool> {
        if read(root)?.as_deref() != Some(command(exe)?.as_str()) {
            return Ok(false);
        }
        set(root, exe, false)?;
        Ok(true)
    }
    pub fn remove_if_owned(exe: &Path) -> io::Result<bool> {
        let removed = remove_owned(ROOT, exe)?;
        if removed {
            unsafe {
                SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST as u32, None, None);
            }
        }
        Ok(removed)
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn per_user_registration_round_trip_preserves_other_handlers() {
            let root = format!(
                r"Software\ElevenClock-rs-tests\template-protocol-{}",
                std::process::id()
            );
            let exe = Path::new(r"C:\Example\elevenclock-rs.exe");
            set(&root, exe, true).unwrap();
            assert_eq!(read(&root).unwrap(), Some(command(exe).unwrap()));
            assert!(set(&root, Path::new(r"C:\Other\other.exe"), true).is_err());
            assert!(set(&root, Path::new(r"C:\Other\other.exe"), false).is_err());
            assert!(!remove_owned(&root, Path::new(r"C:\Other\other.exe")).unwrap());
            assert_eq!(read(&root).unwrap(), Some(command(exe).unwrap()));
            assert!(remove_owned(&root, exe).unwrap());
            assert_eq!(read(&root).unwrap(), None);
            for (name, value) in [("", "Another program"), ("URL Protocol", "foreign")] {
                write(&root, name, value).unwrap();
                assert!(set(&root, exe, true).is_err());
                assert_eq!(read_value(&root, name).unwrap().as_deref(), Some(value));
                assert_eq!(read(&root).unwrap(), None);
                write(
                    &format!(r"{root}\shell\open\command"),
                    "",
                    &command(exe).unwrap(),
                )
                .unwrap();
                assert!(remove_owned(&root, exe).is_err());
                assert_eq!(read_value(&root, name).unwrap().as_deref(), Some(value));
                // SAFETY: this test owns the isolated scratch key.
                assert_eq!(
                    unsafe { RegDeleteTreeW(HKEY_CURRENT_USER, &HSTRING::from(&root)) }.0,
                    0
                );
            }
            set(&root, exe, true).unwrap();
            assert!(enabled_at(&root, exe).unwrap());
            delete_value(&root, "").unwrap();
            assert!(enabled_at(&root, exe).unwrap()); // description is optional
            delete_value(&root, "URL Protocol").unwrap();
            assert!(!enabled_at(&root, exe).unwrap());
            write(&root, "URL Protocol", "foreign").unwrap();
            assert!(!enabled_at(&root, exe).unwrap());
            write(&root, "URL Protocol", "").unwrap();
            write(&root, "", "foreign").unwrap();
            assert!(!enabled_at(&root, exe).unwrap());
            write(&root, "", DESCRIPTION).unwrap();
            write(&root, "Other value", "keep").unwrap();
            let command_key = format!(r"{root}\shell\open\command");
            write(&command_key, "Other value", "keep command metadata").unwrap();
            let sibling = format!(r"{root}\shell\other\command");
            write(&sibling, "", "keep other verb").unwrap();
            assert!(remove_owned(&root, exe).unwrap());
            assert!(!enabled_at(&root, exe).unwrap());
            assert_eq!(read(&root).unwrap(), None);
            assert_eq!(
                read_value(&root, "Other value").unwrap().as_deref(),
                Some("keep")
            );
            assert_eq!(
                read_value(&command_key, "Other value").unwrap().as_deref(),
                Some("keep command metadata")
            );
            assert_eq!(
                read_value(&sibling, "").unwrap().as_deref(),
                Some("keep other verb")
            );
            // SAFETY: remove only this test's isolated scratch tree.
            assert_eq!(
                unsafe { RegDeleteTreeW(HKEY_CURRENT_USER, &HSTRING::from(&root)) }.0,
                0
            );
        }
    }
}
#[cfg(windows)]
pub use native::{enabled, remove_if_owned, set_enabled};
