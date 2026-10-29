//! `Shell_NotifyIconW` registration (Windows, UI thread only).
use super::WM_TRAYICON;
use crate::bindings::{
    HICON, HWND, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY,
    NIM_SETVERSION, NOTIFYICON_VERSION_4, NOTIFYICONDATAW, Shell_NotifyIconW,
};
use crate::interaction::tray_tooltip;

/// Identifies this app's single icon among those owned by the host window.
const ICON_ID: u32 = 1;

/// The notification-area icon. Dropping it removes the icon, so every exit path
/// that drops the platform leaves nothing behind in the tray.
pub struct Tray {
    host: HWND,
    icon: HICON,
    paused: bool,
    registered: bool,
    release_url: Option<String>,
}

impl Tray {
    /// `icon` must stay valid for the life of the tray (a shared resource icon).
    pub fn new(host: HWND, icon: HICON) -> Self {
        Self {
            host,
            icon,
            paused: false,
            registered: false,
            release_url: None,
        }
    }

    pub fn is_registered(&self) -> bool {
        self.registered
    }

    fn data(&self) -> NOTIFYICONDATAW {
        let mut data = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.host,
            uID: ICON_ID,
            uFlags: (NIF_MESSAGE | NIF_ICON | NIF_TIP | NIF_SHOWTIP) as u32,
            uCallbackMessage: WM_TRAYICON,
            hIcon: self.icon,
            ..Default::default()
        };
        // szTip is 128 UTF-16 units including the terminator; the tooltip is short.
        for (slot, unit) in data
            .szTip
            .iter_mut()
            .zip(tray_tooltip(self.paused).encode_utf16())
        {
            *slot = unit;
        }
        data
    }

    /// Add the icon, or re-add it after Explorer recreated the notification area.
    /// Returns whether the icon is now registered; failure is expected while
    /// Explorer is still starting, and the next `TaskbarCreated` tries again.
    pub fn restore(&mut self) -> bool {
        let mut data = self.data();
        // SAFETY: `data` is a fully initialized structure of the advertised size.
        unsafe {
            // After an Explorer restart the old entry is gone and ADD succeeds. If
            // it still exists (a repeated restore request), ADD fails and MODIFY
            // refreshes it instead of stacking a second icon.
            let added = Shell_NotifyIconW(NIM_ADD as u32, &data).as_bool()
                || Shell_NotifyIconW(NIM_MODIFY as u32, &data).as_bool();
            if added {
                // Version 4 delivers keyboard selection and the anchor point.
                data.Anonymous.uVersion = NOTIFYICON_VERSION_4 as u32;
                let _ = Shell_NotifyIconW(NIM_SETVERSION as u32, &data);
            }
            self.registered = added;
        }
        self.registered
    }

    /// Marker has already been committed before this call. Shell owns display
    /// timing (including Do Not Disturb); the app never retries a balloon.
    pub fn notify_update(&mut self, version: &str, url: &str) {
        if crate::managed::detect() || !crate::update_check::valid_release_url(url) {
            return;
        }
        let mut data = self.data();
        data.uFlags = crate::bindings::NIF_INFO as u32;
        let message = format!("ElevenClock-rs {version} is available");
        for (slot, unit) in data.szInfo.iter_mut().take(255).zip(message.encode_utf16()) {
            *slot = unit;
        }
        for (slot, unit) in data
            .szInfoTitle
            .iter_mut()
            .zip("ElevenClock-rs update".encode_utf16())
        {
            *slot = unit;
        }
        data.dwInfoFlags = crate::bindings::NIIF_INFO as u32;
        self.release_url = Some(url.to_owned());
        // SAFETY: initialized structure and bounded NUL-terminated inline buffers;
        // this call runs on the tray's owning UI thread.
        unsafe {
            let _ = Shell_NotifyIconW(NIM_MODIFY as u32, &data);
        }
    }
    pub fn open_update(&self) {
        if crate::managed::detect() {
            return;
        }
        if let Some(url) = &self.release_url {
            let url: Vec<u16> = url.encode_utf16().chain(Some(0)).collect();
            // SAFETY: only a validated HTTPS release URL is retained. The owned
            // NUL-terminated buffer lives through the synchronous shell call.
            unsafe {
                crate::bindings::ShellExecuteW(
                    Some(self.host),
                    windows_core::w!("open"),
                    windows_core::PCWSTR(url.as_ptr()),
                    None,
                    None,
                    crate::bindings::SW_SHOWNORMAL,
                );
            }
        }
    }

    /// Update the tooltip for the replacement mode.
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
        if self.registered {
            let data = self.data();
            // SAFETY: as above.
            self.registered = unsafe { Shell_NotifyIconW(NIM_MODIFY as u32, &data) }.as_bool();
        }
    }
}

impl Drop for Tray {
    fn drop(&mut self) {
        let data = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: self.host,
            uID: ICON_ID,
            ..Default::default()
        };
        // SAFETY: deleting this icon by (window, id); a missing icon is harmless.
        unsafe {
            let _ = Shell_NotifyIconW(NIM_DELETE as u32, &data);
        }
    }
}
