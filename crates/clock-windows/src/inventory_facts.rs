//! Managed inventory fact acquisition. Call only after explicit inventory opt-in.
//! Account lookup can block on domain resolution: integration runs it in
//! an owned, bounded helper process, never on an app/launcher callback thread.
use crate::bindings::*;
use std::{
    os::windows::ffi::OsStrExt,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use windows_core::{HSTRING, PCWSTR, PWSTR, w};

/// Lookup failures preserve SID independently of the optional display name.
/// This data is for administrator inventory only, never ordinary diagnostics/logs.
pub fn account() -> (Option<String>, Option<String>) {
    let Ok(sid) = crate::instance::current_user_sid() else {
        return (None, None);
    };
    let name = account_name(&sid);
    (Some(sid), name)
}

fn account_name(text: &str) -> Option<String> {
    // SAFETY: converted SID remains allocated throughout both lookups, all output
    // buffers have the requested size, and the allocation is freed on every path.
    unsafe {
        let mut sid = PSID::default();
        ConvertStringSidToSidW(&HSTRING::from(text), &mut sid)
            .ok()
            .ok()?;
        let result = (|| {
            let (mut names, mut domains, mut kind) = (0, 0, SID_NAME_USE::default());
            let _ = LookupAccountSidW(
                PCWSTR::null(),
                sid,
                None,
                &mut names,
                None,
                &mut domains,
                &mut kind,
            );
            if names == 0 || names > 1024 || domains > 1024 {
                return None;
            }
            let mut name = vec![0u16; names as usize];
            let mut domain = vec![0u16; domains.max(1) as usize];
            LookupAccountSidW(
                PCWSTR::null(),
                sid,
                Some(PWSTR(name.as_mut_ptr())),
                &mut names,
                Some(PWSTR(domain.as_mut_ptr())),
                &mut domains,
                &mut kind,
            )
            .ok()
            .ok()?;
            let name = String::from_utf16(name.get(..names as usize)?).ok()?;
            let domain = String::from_utf16(domain.get(..domains as usize)?).ok()?;
            Some(if domain.is_empty() {
                name
            } else {
                format!("{domain}\\{name}")
            })
        })();
        let _ = LocalFree(HANDLE(sid.0));
        result
    }
}

/// A fresh random per-copy ID; the opted-in local store preserves it.
pub fn new_installation_id() -> Option<String> {
    // SAFETY: CoCreateGuid has no pointer arguments and requires no COM apartment.
    unsafe {
        CoCreateGuid()
            .ok()
            .map(|id| format!("{id:?}").to_ascii_lowercase())
    }
}

pub fn observed_utc() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs())
}

/// Reads the actual local executable's fixed file version; never substitutes the
/// central release or the running launcher's package version. Local paths only.
pub fn file_version(path: &Path) -> Option<String> {
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    if wide[..wide.len() - 1].contains(&0) {
        return None;
    }
    // SAFETY: NUL-terminated path, live aligned output block sized for the API,
    // and live query output slots. The returned pointer is never dereferenced:
    // only a checked offset is used to decode bytes from our owned block.
    unsafe {
        let size = GetFileVersionInfoSizeW(PCWSTR(wide.as_ptr()), None);
        if size == 0 || size > 1024 * 1024 {
            return None;
        }
        let mut data = vec![0u64; (size as usize).div_ceil(8)];
        GetFileVersionInfoW(PCWSTR(wide.as_ptr()), None, size, data.as_mut_ptr().cast())
            .ok()
            .ok()?;
        let mut ptr = std::ptr::null_mut();
        let mut len = 0;
        VerQueryValueW(data.as_ptr().cast(), w!("\\"), &mut ptr, &mut len)
            .ok()
            .ok()?;
        if ptr.is_null() {
            return None;
        }
        const _: () = assert!(
            std::mem::size_of::<VS_FIXEDFILEINFO>()
                == crate::inventory_version::FIXED_FILE_INFO_BYTES
        );
        let offset = ptr.addr().checked_sub(data.as_ptr().addr())?;
        crate::inventory_version::decode_fixed_version(&data, size as usize, offset, len as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_identity_and_unique_ids() {
        let (sid, _) = account();
        assert!(sid.unwrap().starts_with("S-1-"));
        let first = new_installation_id().unwrap();
        assert_eq!(first.len(), 36);
        assert_ne!(Some(first), new_installation_id());
        assert!(observed_utc().is_some());
        assert!(file_version(Path::new("missing-inventory-test.exe")).is_none());
    }
}
