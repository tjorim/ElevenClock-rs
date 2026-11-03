use super::{HOST_CLASS, InstanceNames, WM_SHOW_SETTINGS};
use crate::bindings::{
    AllowSetForegroundWindow, CheckHandle, CloseHandle, ConvertSidToStringSidW, CreateMutexW,
    ERROR_ALREADY_EXISTS, FindWindowW, GetCurrentProcess, GetLastError, GetTokenInformation,
    GetWindowThreadProcessId, HANDLE, LPARAM, LocalFree, OpenProcessToken, PostMessageW,
    TOKEN_QUERY, TOKEN_USER, TokenUser, WPARAM,
};
use std::time::{Duration, Instant};
use windows_core::{HSTRING, PWSTR, Result};

/// Owns the per-user mutex for the life of the primary process. The mutex is
/// never "owned" for synchronization; only its existence matters, and the kernel
/// releases it when the process ends on any path, so no stale lock can remain.
pub struct InstanceGuard {
    mutex: HANDLE,
    names: InstanceNames,
}
impl InstanceGuard {
    pub fn names(&self) -> &InstanceNames {
        &self.names
    }
}
impl Drop for InstanceGuard {
    fn drop(&mut self) {
        // SAFETY: the handle was created by CreateMutexW and is closed once.
        let _ = unsafe { CloseHandle(self.mutex) };
    }
}

pub enum Acquired {
    Primary(InstanceGuard),
    Existing(InstanceNames),
}

pub(crate) fn current_user_sid() -> Result<String> {
    // SAFETY: standard two-call token query. The token handle is closed on every
    // path, the buffer is sized by the first call, and the converted SID string
    // is LocalFree'd after copying.
    unsafe {
        let mut token = HANDLE::default();
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY as u32, &mut token).ok()?;
        let result = (|| {
            let mut needed = 0u32;
            let _ = GetTokenInformation(token, TokenUser, None, 0, &mut needed);
            if needed == 0 {
                return Err(windows_core::Error::from_thread());
            }
            // u64 elements keep TOKEN_USER (pointer-aligned) correctly aligned.
            let mut buffer = vec![0u64; (needed as usize).div_ceil(8)];
            GetTokenInformation(
                token,
                TokenUser,
                Some(buffer.as_mut_ptr().cast()),
                needed,
                &mut needed,
            )
            .ok()?;
            let user = &*(buffer.as_ptr().cast::<TOKEN_USER>());
            let mut text = PWSTR::null();
            ConvertSidToStringSidW(user.User.Sid, &mut text).ok()?;
            let sid = text
                .to_string()
                .map_err(|_| windows_core::Error::from_thread());
            let _ = LocalFree(HANDLE(text.0.cast()));
            sid
        })();
        let _ = CloseHandle(token);
        result
    }
}

/// Decide ownership. The first process for this user and session becomes
/// primary; later ones are `Existing` and should call [`signal_existing`].
pub fn acquire() -> Result<Acquired> {
    let names = InstanceNames::for_user(&current_user_sid()?);
    // SAFETY: the name is NUL terminated by HSTRING; default security makes the
    // object accessible to this user only. No initial ownership is requested.
    let (mutex, existed) = unsafe {
        let mutex = CreateMutexW(None, false, &HSTRING::from(names.mutex.as_str())).ok()?;
        (mutex, GetLastError() == ERROR_ALREADY_EXISTS as u32)
    };
    if existed {
        // SAFETY: closing our duplicate reference to the existing object.
        let _ = unsafe { CloseHandle(mutex) };
        Ok(Acquired::Existing(names))
    } else {
        Ok(Acquired::Primary(InstanceGuard { mutex, names }))
    }
}

/// Ask the running instance to open or focus Settings. The primary may still be
/// creating its host window (simultaneous launches) or shutting down, so the
/// lookup is bounded rather than assumed. Returns whether the signal was posted.
pub fn signal_existing(names: &InstanceNames, wait: Duration) -> bool {
    let Some(host) = find_existing(names, wait) else {
        return false;
    };
    // SAFETY: fixed pointer-free request to the located user/session host.
    unsafe { PostMessageW(Some(host), WM_SHOW_SETTINGS, WPARAM(0), LPARAM(0)).as_bool() }
}

fn find_existing(names: &InstanceNames, wait: Duration) -> Option<crate::bindings::HWND> {
    let title = HSTRING::from(names.window_title.as_str());
    let class = HSTRING::from(HOST_CLASS);
    let started = Instant::now();
    loop {
        // SAFETY: bounded lookup of the current user's host; a stale handle
        // fails safely, and no pointers are passed to the target process.
        unsafe {
            let host = FindWindowW(&class, &title);
            if !host.is_invalid() {
                let mut pid = 0;
                GetWindowThreadProcessId(host, Some(&mut pid));
                // Let the existing process take the foreground for its window.
                let _ = AllowSetForegroundWindow(pid);
                return Some(host);
            }
        }
        if started.elapsed() >= wait {
            return None;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// WM_COPYDATA marshals these bounded bytes while this synchronous call is live.
/// The receiver copies validated values; no sender pointer is retained.
pub fn signal_template(
    names: &InstanceNames,
    template: &clock_core::template::ClockTemplate,
    wait: Duration,
) -> bool {
    let Some(host) = find_existing(names, wait) else {
        return false;
    };
    let link = template.app_link();
    let data = crate::bindings::COPYDATASTRUCT {
        dwData: super::TEMPLATE_MESSAGE_TAG,
        cbData: link.len() as u32,
        lpData: link.as_ptr().cast_mut().cast(),
    };
    let mut result = 0;
    // SAFETY: system-marshalled WM_COPYDATA, live UTF-8 payload and structure for
    // the entire bounded send, and a live output slot. No pointer is posted.
    unsafe {
        crate::bindings::SendMessageTimeoutW(
            host,
            crate::bindings::WM_COPYDATA as u32,
            WPARAM(0),
            LPARAM((&data as *const crate::bindings::COPYDATASTRUCT) as isize),
            crate::bindings::SMTO_ABORTIFHUNG as u32,
            1000,
            Some(&mut result),
        )
        .0 != 0
            && result == 1
    }
}
