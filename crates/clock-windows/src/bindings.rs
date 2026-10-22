//! Win32 bindings. `generated.rs` comes from `clock-bindgen` (run `cargo run -p clock-bindgen`
//! after editing `bindings.txt`); this file only adds what the default 0.100 metadata lacks.

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[rustfmt::skip]
mod generated;

pub use generated::*;

// The default metadata has no `UIAutomationCore.dll` exports or UIA id constants
// (UIAutomationClient.h/UIAutomationCoreApi.h), and no KnownFolders.h GUIDs.

pub const UIA_E_ELEMENTNOTAVAILABLE: u32 = 0x8004_0201;
pub const UiaRootObjectId: i32 = -25;

pub const UIA_InvokePatternId: PATTERNID = PATTERNID(10000);
pub const UIA_ValuePatternId: PATTERNID = PATTERNID(10002);
pub const UIA_TogglePatternId: PATTERNID = PATTERNID(10015);

pub const UIA_ControlTypePropertyId: PROPERTYID = PROPERTYID(30003);
pub const UIA_NamePropertyId: PROPERTYID = PROPERTYID(30005);
pub const UIA_HasKeyboardFocusPropertyId: PROPERTYID = PROPERTYID(30008);
pub const UIA_IsKeyboardFocusablePropertyId: PROPERTYID = PROPERTYID(30009);
pub const UIA_IsEnabledPropertyId: PROPERTYID = PROPERTYID(30010);
pub const UIA_AutomationIdPropertyId: PROPERTYID = PROPERTYID(30011);
pub const UIA_HelpTextPropertyId: PROPERTYID = PROPERTYID(30013);
pub const UIA_IsControlElementPropertyId: PROPERTYID = PROPERTYID(30016);
pub const UIA_IsContentElementPropertyId: PROPERTYID = PROPERTYID(30017);

pub const UIA_ButtonControlTypeId: CONTROLTYPEID = CONTROLTYPEID(50000);

pub const FOLDERID_LocalAppData: windows_core::GUID =
    windows_core::GUID::from_u128(0xf1b32785_6fba_4fcf_9d55_7b8e7f157091);

#[inline]
pub unsafe fn UiaHostProviderFromHwnd(
    hwnd: HWND,
) -> windows_core::Result<IRawElementProviderSimple> {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHostProviderFromHwnd(hwnd: HWND, ppprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result = core::ptr::null_mut();
        UiaHostProviderFromHwnd(hwnd, &mut result)
            .map(|| <IRawElementProviderSimple as windows_core::Interface>::from_raw(result))
    }
}

#[inline]
pub unsafe fn UiaReturnRawElementProvider<P3>(
    hwnd: HWND,
    wparam: WPARAM,
    lparam: LPARAM,
    el: P3,
) -> LRESULT
where
    P3: windows_core::Param<IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaReturnRawElementProvider(hwnd: HWND, wparam: WPARAM, lparam: LPARAM, el: *mut core::ffi::c_void) -> LRESULT);
    unsafe { UiaReturnRawElementProvider(hwnd, wparam, lparam, el.param().abi()) }
}

#[inline]
pub unsafe fn UiaDisconnectProvider<P0>(pprovider: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaDisconnectProvider(pprovider: *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaDisconnectProvider(pprovider.param().abi()).ok() }
}

/// Handle-returning APIs that report failure as null (or `INVALID_HANDLE_VALUE`) and set the
/// thread's last error. The 0.100 bindings return the raw handle; `.ok()` restores the
/// `Result` the call sites used to get from the `windows` crate.
pub trait CheckHandle: Copy {
    fn is_invalid(&self) -> bool;

    fn ok(self) -> windows_core::Result<Self> {
        if self.is_invalid() {
            Err(windows_core::Error::from_thread())
        } else {
            Ok(self)
        }
    }
    /// For callers that only need to know whether a handle came back.
    fn valid(self) -> Option<Self> {
        (!self.is_invalid()).then_some(self)
    }
}

macro_rules! null_handles {
    ($($handle:ty),*) => {$(
        impl CheckHandle for $handle {
            fn is_invalid(&self) -> bool {
                self.0.is_null()
            }
        }
    )*};
}

null_handles!(HWND, HINSTANCE, HMENU, HICON, HWINEVENTHOOK, HMONITOR);

impl CheckHandle for HANDLE {
    fn is_invalid(&self) -> bool {
        self.0.is_null() || self.0 as isize == -1
    }
}

impl LSTATUS {
    /// Registry calls return the Win32 error code directly rather than setting it.
    pub fn ok(self) -> windows_core::Result<()> {
        windows_core::WIN32_ERROR(self.0 as u32).ok()
    }
}
