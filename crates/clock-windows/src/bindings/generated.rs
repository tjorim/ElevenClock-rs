#[inline]
pub unsafe fn AllowSetForegroundWindow(dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AllowSetForegroundWindow(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { AllowSetForegroundWindow(dwprocessid) }
}
#[inline]
pub unsafe fn AppendMenuW<P3>(
    hmenu: HMENU,
    uflags: u32,
    uidnewitem: usize,
    lpnewitem: P3,
) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn AppendMenuW(hmenu : HMENU, uflags : u32, uidnewitem : usize, lpnewitem : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { AppendMenuW(hmenu, uflags, uidnewitem, lpnewitem.param().abi()) }
}
#[inline]
pub unsafe fn AreDpiAwarenessContextsEqual(
    dpicontexta: DPI_AWARENESS_CONTEXT,
    dpicontextb: DPI_AWARENESS_CONTEXT,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn AreDpiAwarenessContextsEqual(dpicontexta : DPI_AWARENESS_CONTEXT, dpicontextb : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { AreDpiAwarenessContextsEqual(dpicontexta, dpicontextb) }
}
#[inline]
pub unsafe fn AssignProcessToJobObject(hjob: HANDLE, hprocess: HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : HANDLE, hprocess : HANDLE) -> windows_core::BOOL);
    unsafe { AssignProcessToJobObject(hjob, hprocess) }
}
#[inline]
pub unsafe fn CloseClipboard() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn CloseClipboard() -> windows_core::BOOL);
    unsafe { CloseClipboard() }
}
#[inline]
pub unsafe fn CloseHandle(hobject: HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> windows_core::BOOL);
    unsafe { CloseHandle(hobject) }
}
#[inline]
pub unsafe fn CoCreateInstance<P1, T>(
    rclsid: *const windows_core::GUID,
    punkouter: P1,
    dwclscontext: u32,
) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        CoCreateInstance(
            rclsid,
            punkouter.param().abi(),
            dwclscontext,
            &T::IID,
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoInitializeEx(
    pvreserved: Option<*const core::ffi::c_void>,
    dwcoinit: u32,
) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: *mut core::ffi::c_void) {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *mut core::ffi::c_void));
    unsafe { CoTaskMemFree(pv as _) }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
#[inline]
pub unsafe fn ConvertSidToStringSidW(
    sid: PSID,
    stringsid: *mut windows_core::PWSTR,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ConvertSidToStringSidW(sid : PSID, stringsid : *mut windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { ConvertSidToStringSidW(sid, stringsid as _) }
}
#[inline]
pub unsafe fn CreateDialogParamW<P1>(
    hinstance: Option<HINSTANCE>,
    lptemplatename: P1,
    hwndparent: Option<HWND>,
    lpdialogfunc: DLGPROC,
    dwinitparam: LPARAM,
) -> HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateDialogParamW(hinstance : HINSTANCE, lptemplatename : windows_core::PCWSTR, hwndparent : HWND, lpdialogfunc : DLGPROC, dwinitparam : LPARAM) -> HWND);
    unsafe {
        CreateDialogParamW(
            hinstance.unwrap_or(core::mem::zeroed()) as _,
            lptemplatename.param().abi(),
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            lpdialogfunc,
            dwinitparam,
        )
    }
}
#[inline]
pub unsafe fn CreateJobObjectW<P1>(
    lpjobattributes: Option<*const SECURITY_ATTRIBUTES>,
    lpname: P1,
) -> HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : *const SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR) -> HANDLE);
    unsafe {
        CreateJobObjectW(
            lpjobattributes.unwrap_or(core::mem::zeroed()) as _,
            lpname.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn CreateMutexW<P2>(
    lpmutexattributes: Option<*const SECURITY_ATTRIBUTES>,
    binitialowner: bool,
    lpname: P2,
) -> HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMutexW(lpmutexattributes : *const SECURITY_ATTRIBUTES, binitialowner : windows_core::BOOL, lpname : windows_core::PCWSTR) -> HANDLE);
    unsafe {
        CreateMutexW(
            lpmutexattributes.unwrap_or(core::mem::zeroed()) as _,
            binitialowner.into(),
            lpname.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn CreatePopupMenu() -> HMENU {
    windows_core::link!("user32.dll" "system" fn CreatePopupMenu() -> HMENU);
    unsafe { CreatePopupMenu() }
}
#[inline]
pub unsafe fn CreateToolhelp32Snapshot(dwflags: u32, th32processid: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateToolhelp32Snapshot(dwflags : u32, th32processid : u32) -> HANDLE);
    unsafe { CreateToolhelp32Snapshot(dwflags, th32processid) }
}
#[inline]
pub unsafe fn CreateWindowExW<P1, P2>(
    dwexstyle: u32,
    lpclassname: P1,
    lpwindowname: P2,
    dwstyle: u32,
    x: i32,
    y: i32,
    nwidth: i32,
    nheight: i32,
    hwndparent: Option<HWND>,
    hmenu: Option<HMENU>,
    hinstance: Option<HINSTANCE>,
    lpparam: Option<*const core::ffi::c_void>,
) -> HWND
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : u32, lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR, dwstyle : u32, x : i32, y : i32, nwidth : i32, nheight : i32, hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : *const core::ffi::c_void) -> HWND);
    unsafe {
        CreateWindowExW(
            dwexstyle,
            lpclassname.param().abi(),
            lpwindowname.param().abi(),
            dwstyle,
            x,
            y,
            nwidth,
            nheight,
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            hmenu.unwrap_or(core::mem::zeroed()) as _,
            hinstance.unwrap_or(core::mem::zeroed()) as _,
            lpparam.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn D2D1CreateFactory<T>(
    factorytype: D2D1_FACTORY_TYPE,
    pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>,
) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        D2D1CreateFactory(
            factorytype,
            &T::IID,
            pfactoryoptions.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        DWriteCreateFactory(factorytype, &T::IID, &mut result__)
            .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn DefWindowProcW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn DefWindowProcW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn DestroyMenu(hmenu: HMENU) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyMenu(hmenu : HMENU) -> windows_core::BOOL);
    unsafe { DestroyMenu(hmenu) }
}
#[inline]
pub unsafe fn DestroyWindow(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn DestroyWindow(hwnd : HWND) -> windows_core::BOOL);
    unsafe { DestroyWindow(hwnd) }
}
#[inline]
pub unsafe fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);
    unsafe { DispatchMessageW(lpmsg) }
}
#[inline]
pub unsafe fn DwmSetWindowAttribute(
    hwnd: HWND,
    dwattribute: u32,
    pvattribute: *const core::ffi::c_void,
    cbattribute: u32,
) -> windows_core::HRESULT {
    windows_core::link!("dwmapi.dll" "system" fn DwmSetWindowAttribute(hwnd : HWND, dwattribute : u32, pvattribute : *const core::ffi::c_void, cbattribute : u32) -> windows_core::HRESULT);
    unsafe { DwmSetWindowAttribute(hwnd, dwattribute, pvattribute, cbattribute) }
}
#[inline]
pub unsafe fn EmptyClipboard() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EmptyClipboard() -> windows_core::BOOL);
    unsafe { EmptyClipboard() }
}
#[inline]
pub unsafe fn EnableWindow(hwnd: HWND, benable: bool) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnableWindow(hwnd : HWND, benable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EnableWindow(hwnd, benable.into()) }
}
#[inline]
pub unsafe fn EnumChildWindows(
    hwndparent: Option<HWND>,
    lpenumfunc: WNDENUMPROC,
    lparam: LPARAM,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumChildWindows(hwndparent : HWND, lpenumfunc : WNDENUMPROC, lparam : LPARAM) -> windows_core::BOOL);
    unsafe {
        EnumChildWindows(
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            lpenumfunc,
            lparam,
        )
    }
}
#[inline]
pub unsafe fn EnumWindows(lpenumfunc: WNDENUMPROC, lparam: LPARAM) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn EnumWindows(lpenumfunc : WNDENUMPROC, lparam : LPARAM) -> windows_core::BOOL);
    unsafe { EnumWindows(lpenumfunc, lparam) }
}
#[inline]
pub unsafe fn FileTimeToSystemTime(
    lpfiletime: *const FILETIME,
    lpsystemtime: *mut SYSTEMTIME,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToSystemTime(lpfiletime : *const FILETIME, lpsystemtime : *mut SYSTEMTIME) -> windows_core::BOOL);
    unsafe { FileTimeToSystemTime(lpfiletime, lpsystemtime as _) }
}
#[inline]
pub unsafe fn FindWindowExW<P2, P3>(
    hwndparent: Option<HWND>,
    hwndchildafter: Option<HWND>,
    lpszclass: P2,
    lpszwindow: P3,
) -> HWND
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowExW(hwndparent : HWND, hwndchildafter : HWND, lpszclass : windows_core::PCWSTR, lpszwindow : windows_core::PCWSTR) -> HWND);
    unsafe {
        FindWindowExW(
            hwndparent.unwrap_or(core::mem::zeroed()) as _,
            hwndchildafter.unwrap_or(core::mem::zeroed()) as _,
            lpszclass.param().abi(),
            lpszwindow.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn FindWindowW<P0, P1>(lpclassname: P0, lpwindowname: P1) -> HWND
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn FindWindowW(lpclassname : windows_core::PCWSTR, lpwindowname : windows_core::PCWSTR) -> HWND);
    unsafe { FindWindowW(lpclassname.param().abi(), lpwindowname.param().abi()) }
}
#[inline]
pub unsafe fn GetAsyncKeyState(vkey: i32) -> i16 {
    windows_core::link!("user32.dll" "system" fn GetAsyncKeyState(vkey : i32) -> i16);
    unsafe { GetAsyncKeyState(vkey) }
}
#[inline]
pub unsafe fn GetCapture() -> HWND {
    windows_core::link!("user32.dll" "system" fn GetCapture() -> HWND);
    unsafe { GetCapture() }
}
#[inline]
pub unsafe fn GetClassNameW(hwnd: HWND, lpclassname: windows_core::PWSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetClassNameW(hwnd : HWND, lpclassname : windows_core::PWSTR, nmaxcount : i32) -> i32);
    unsafe { GetClassNameW(hwnd, lpclassname, nmaxcount) }
}
#[inline]
pub unsafe fn GetClientRect(hwnd: HWND, lprect: *mut RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetClientRect(hwnd : HWND, lprect : *mut RECT) -> windows_core::BOOL);
    unsafe { GetClientRect(hwnd, lprect as _) }
}
#[inline]
pub unsafe fn GetClipboardData(uformat: u32) -> HANDLE {
    windows_core::link!("user32.dll" "system" fn GetClipboardData(uformat : u32) -> HANDLE);
    unsafe { GetClipboardData(uformat) }
}
#[inline]
pub unsafe fn GetCurrentProcess() -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcess() -> HANDLE);
    unsafe { GetCurrentProcess() }
}
#[inline]
pub unsafe fn GetCurrentProcessId() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
    unsafe { GetCurrentProcessId() }
}
#[inline]
pub unsafe fn GetCursorPos(lppoint: *mut POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetCursorPos(lppoint : *mut POINT) -> windows_core::BOOL);
    unsafe { GetCursorPos(lppoint as _) }
}
#[inline]
pub unsafe fn GetDlgItem(hdlg: Option<HWND>, niddlgitem: i32) -> HWND {
    windows_core::link!("user32.dll" "system" fn GetDlgItem(hdlg : HWND, niddlgitem : i32) -> HWND);
    unsafe { GetDlgItem(hdlg.unwrap_or(core::mem::zeroed()) as _, niddlgitem) }
}
#[inline]
pub unsafe fn GetDpiForWindow(hwnd: HWND) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetDpiForWindow(hwnd : HWND) -> u32);
    unsafe { GetDpiForWindow(hwnd) }
}
#[inline]
pub unsafe fn GetExitCodeProcess(hprocess: HANDLE, lpexitcode: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetExitCodeProcess(hprocess : HANDLE, lpexitcode : *mut u32) -> windows_core::BOOL);
    unsafe { GetExitCodeProcess(hprocess, lpexitcode as _) }
}
#[inline]
pub unsafe fn GetFocus() -> HWND {
    windows_core::link!("user32.dll" "system" fn GetFocus() -> HWND);
    unsafe { GetFocus() }
}
#[inline]
pub unsafe fn GetForegroundWindow() -> HWND {
    windows_core::link!("user32.dll" "system" fn GetForegroundWindow() -> HWND);
    unsafe { GetForegroundWindow() }
}
#[inline]
pub unsafe fn GetGuiResources(hprocess: HANDLE, uiflags: u32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetGuiResources(hprocess : HANDLE, uiflags : u32) -> u32);
    unsafe { GetGuiResources(hprocess, uiflags) }
}
#[inline]
pub unsafe fn GetKeyState(nvirtkey: i32) -> i16 {
    windows_core::link!("user32.dll" "system" fn GetKeyState(nvirtkey : i32) -> i16);
    unsafe { GetKeyState(nvirtkey) }
}
#[inline]
pub unsafe fn GetLastError() -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    unsafe { GetLastError() }
}
#[inline]
pub unsafe fn GetMessageW(
    lpmsg: *mut MSG,
    hwnd: Option<HWND>,
    wmsgfiltermin: u32,
    wmsgfiltermax: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> windows_core::BOOL);
    unsafe {
        GetMessageW(
            lpmsg as _,
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            wmsgfiltermin,
            wmsgfiltermax,
        )
    }
}
#[inline]
pub unsafe fn GetModuleHandleW<P0>(lpmodulename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : windows_core::PCWSTR) -> HMODULE);
    unsafe { GetModuleHandleW(lpmodulename.param().abi()) }
}
#[inline]
pub unsafe fn GetMonitorInfoW(hmonitor: HMONITOR, lpmi: *mut MONITORINFO) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetMonitorInfoW(hmonitor : HMONITOR, lpmi : *mut MONITORINFO) -> windows_core::BOOL);
    unsafe { GetMonitorInfoW(hmonitor, lpmi as _) }
}
#[inline]
pub unsafe fn GetNativeSystemInfo(lpsysteminfo: *mut SYSTEM_INFO) {
    windows_core::link!("kernel32.dll" "system" fn GetNativeSystemInfo(lpsysteminfo : *mut SYSTEM_INFO));
    unsafe { GetNativeSystemInfo(lpsysteminfo as _) }
}
#[inline]
pub unsafe fn GetProcessHandleCount(
    hprocess: HANDLE,
    pdwhandlecount: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHandleCount(hprocess : HANDLE, pdwhandlecount : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessHandleCount(hprocess, pdwhandlecount as _) }
}
#[inline]
pub unsafe fn GetProcessMemoryInfo(
    process: HANDLE,
    ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS,
    cb: u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessMemoryInfo" fn GetProcessMemoryInfo(process : HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_core::BOOL);
    unsafe { GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[inline]
pub unsafe fn GetProcessTimes(
    hprocess: HANDLE,
    lpcreationtime: *mut FILETIME,
    lpexittime: *mut FILETIME,
    lpkerneltime: *mut FILETIME,
    lpusertime: *mut FILETIME,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessTimes(hprocess : HANDLE, lpcreationtime : *mut FILETIME, lpexittime : *mut FILETIME, lpkerneltime : *mut FILETIME, lpusertime : *mut FILETIME) -> windows_core::BOOL);
    unsafe {
        GetProcessTimes(
            hprocess,
            lpcreationtime as _,
            lpexittime as _,
            lpkerneltime as _,
            lpusertime as _,
        )
    }
}
#[inline]
pub unsafe fn GetSysColor(nindex: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetSysColor(nindex : i32) -> u32);
    unsafe { GetSysColor(nindex) }
}
#[inline]
pub unsafe fn GetSystemMetrics(nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetSystemMetrics(nindex : i32) -> i32);
    unsafe { GetSystemMetrics(nindex) }
}
#[inline]
pub unsafe fn GetSystemTimePreciseAsFileTime() -> FILETIME {
    windows_core::link!("kernel32.dll" "system" fn GetSystemTimePreciseAsFileTime(lpsystemtimeasfiletime : *mut FILETIME));
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetSystemTimePreciseAsFileTime(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT {
    windows_core::link!("user32.dll" "system" fn GetThreadDpiAwarenessContext() -> DPI_AWARENESS_CONTEXT);
    unsafe { GetThreadDpiAwarenessContext() }
}
#[inline]
pub unsafe fn GetTokenInformation(
    tokenhandle: HANDLE,
    tokeninformationclass: TOKEN_INFORMATION_CLASS,
    tokeninformation: Option<*mut core::ffi::c_void>,
    tokeninformationlength: u32,
    returnlength: *mut u32,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetTokenInformation(tokenhandle : HANDLE, tokeninformationclass : TOKEN_INFORMATION_CLASS, tokeninformation : *mut core::ffi::c_void, tokeninformationlength : u32, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe {
        GetTokenInformation(
            tokenhandle,
            tokeninformationclass,
            tokeninformation.unwrap_or(core::mem::zeroed()) as _,
            tokeninformationlength,
            returnlength as _,
        )
    }
}
#[inline]
pub unsafe fn GetUpdateRect(
    hwnd: HWND,
    lprect: Option<*mut RECT>,
    berase: bool,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetUpdateRect(hwnd : HWND, lprect : *mut RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe {
        GetUpdateRect(
            hwnd,
            lprect.unwrap_or(core::mem::zeroed()) as _,
            berase.into(),
        )
    }
}
#[inline]
pub unsafe fn GetWindow(hwnd: HWND, ucmd: u32) -> HWND {
    windows_core::link!("user32.dll" "system" fn GetWindow(hwnd : HWND, ucmd : u32) -> HWND);
    unsafe { GetWindow(hwnd, ucmd) }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[inline]
pub unsafe fn GetWindowLongPtrW(hwnd: HWND, nindex: i32) -> isize {
    windows_core::link!("user32.dll" "system" fn GetWindowLongPtrW(hwnd : HWND, nindex : i32) -> isize);
    unsafe { GetWindowLongPtrW(hwnd, nindex) }
}
#[cfg(target_pointer_width = "32")]
pub use GetWindowLongW as GetWindowLongPtrW;
#[inline]
pub unsafe fn GetWindowLongW(hwnd: HWND, nindex: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowLongW(hwnd : HWND, nindex : i32) -> i32);
    unsafe { GetWindowLongW(hwnd, nindex) }
}
#[inline]
pub unsafe fn GetWindowPlacement(hwnd: HWND, lpwndpl: *mut WINDOWPLACEMENT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowPlacement(hwnd : HWND, lpwndpl : *mut WINDOWPLACEMENT) -> windows_core::BOOL);
    unsafe { GetWindowPlacement(hwnd, lpwndpl as _) }
}
#[inline]
pub unsafe fn GetWindowRect(hwnd: HWND, lprect: *mut RECT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn GetWindowRect(hwnd : HWND, lprect : *mut RECT) -> windows_core::BOOL);
    unsafe { GetWindowRect(hwnd, lprect as _) }
}
#[inline]
pub unsafe fn GetWindowTextLengthW(hwnd: HWND) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextLengthW(hwnd : HWND) -> i32);
    unsafe { GetWindowTextLengthW(hwnd) }
}
#[inline]
pub unsafe fn GetWindowTextW(hwnd: HWND, lpstring: windows_core::PWSTR, nmaxcount: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn GetWindowTextW(hwnd : HWND, lpstring : windows_core::PWSTR, nmaxcount : i32) -> i32);
    unsafe { GetWindowTextW(hwnd, lpstring, nmaxcount) }
}
#[inline]
pub unsafe fn GetWindowThreadProcessId(hwnd: HWND, lpdwprocessid: Option<*mut u32>) -> u32 {
    windows_core::link!("user32.dll" "system" fn GetWindowThreadProcessId(hwnd : HWND, lpdwprocessid : *mut u32) -> u32);
    unsafe { GetWindowThreadProcessId(hwnd, lpdwprocessid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GlobalAlloc(uflags: u32, dwbytes: usize) -> HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalAlloc(uflags : u32, dwbytes : usize) -> HGLOBAL);
    unsafe { GlobalAlloc(uflags, dwbytes) }
}
#[inline]
pub unsafe fn GlobalFree(hmem: HGLOBAL) -> HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalFree(hmem : HGLOBAL) -> HGLOBAL);
    unsafe { GlobalFree(hmem) }
}
#[inline]
pub unsafe fn GlobalLock(hmem: HGLOBAL) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn GlobalLock(hmem : HGLOBAL) -> *mut core::ffi::c_void);
    unsafe { GlobalLock(hmem) }
}
#[inline]
pub unsafe fn GlobalUnlock(hmem: HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnlock(hmem : HGLOBAL) -> windows_core::BOOL);
    unsafe { GlobalUnlock(hmem) }
}
#[inline]
pub unsafe fn InitCommonControlsEx(picce: *const INITCOMMONCONTROLSEX) -> windows_core::BOOL {
    windows_core::link!("comctl32.dll" "system" fn InitCommonControlsEx(picce : *const INITCOMMONCONTROLSEX) -> windows_core::BOOL);
    unsafe { InitCommonControlsEx(picce) }
}
#[inline]
pub unsafe fn InvalidateRect(
    hwnd: Option<HWND>,
    lprect: Option<*const RECT>,
    berase: bool,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn InvalidateRect(hwnd : HWND, lprect : *const RECT, berase : windows_core::BOOL) -> windows_core::BOOL);
    unsafe {
        InvalidateRect(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            lprect.unwrap_or(core::mem::zeroed()) as _,
            berase.into(),
        )
    }
}
#[inline]
pub unsafe fn IsChild(hwndparent: HWND, hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsChild(hwndparent : HWND, hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsChild(hwndparent, hwnd) }
}
#[inline]
pub unsafe fn IsDialogMessageW(hdlg: HWND, lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsDialogMessageW(hdlg : HWND, lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { IsDialogMessageW(hdlg, lpmsg) }
}
#[inline]
pub unsafe fn IsIconic(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsIconic(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsIconic(hwnd) }
}
#[inline]
pub unsafe fn IsWindow(hwnd: Option<HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindow(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsWindow(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsWindowVisible(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn IsWindowVisible(hwnd : HWND) -> windows_core::BOOL);
    unsafe { IsWindowVisible(hwnd) }
}
#[inline]
pub unsafe fn KillTimer(hwnd: Option<HWND>, uidevent: usize) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn KillTimer(hwnd : HWND, uidevent : usize) -> windows_core::BOOL);
    unsafe { KillTimer(hwnd.unwrap_or(core::mem::zeroed()) as _, uidevent) }
}
#[inline]
pub unsafe fn LoadCursorW<P1>(hinstance: Option<HINSTANCE>, lpcursorname: P1) -> HCURSOR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadCursorW(hinstance : HINSTANCE, lpcursorname : windows_core::PCWSTR) -> HCURSOR);
    unsafe {
        LoadCursorW(
            hinstance.unwrap_or(core::mem::zeroed()) as _,
            lpcursorname.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn LoadIconW<P1>(hinstance: Option<HINSTANCE>, lpiconname: P1) -> HICON
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadIconW(hinstance : HINSTANCE, lpiconname : windows_core::PCWSTR) -> HICON);
    unsafe {
        LoadIconW(
            hinstance.unwrap_or(core::mem::zeroed()) as _,
            lpiconname.param().abi(),
        )
    }
}
#[inline]
pub unsafe fn LoadImageW<P1>(
    hinst: Option<HINSTANCE>,
    name: P1,
    r#type: u32,
    cx: i32,
    cy: i32,
    fuload: u32,
) -> HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn LoadImageW(hinst : HINSTANCE, name : windows_core::PCWSTR, r#type : u32, cx : i32, cy : i32, fuload : u32) -> HANDLE);
    unsafe {
        LoadImageW(
            hinst.unwrap_or(core::mem::zeroed()) as _,
            name.param().abi(),
            r#type,
            cx,
            cy,
            fuload,
        )
    }
}
#[inline]
pub unsafe fn LocalFree(hmem: HLOCAL) -> HLOCAL {
    windows_core::link!("kernel32.dll" "system" fn LocalFree(hmem : HLOCAL) -> HLOCAL);
    unsafe { LocalFree(hmem) }
}
#[inline]
pub unsafe fn MessageBoxW<P1, P2>(hwnd: Option<HWND>, lptext: P1, lpcaption: P2, utype: u32) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn MessageBoxW(hwnd : HWND, lptext : windows_core::PCWSTR, lpcaption : windows_core::PCWSTR, utype : u32) -> i32);
    unsafe {
        MessageBoxW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            lptext.param().abi(),
            lpcaption.param().abi(),
            utype,
        )
    }
}
#[inline]
pub unsafe fn MonitorFromWindow(hwnd: HWND, dwflags: u32) -> HMONITOR {
    windows_core::link!("user32.dll" "system" fn MonitorFromWindow(hwnd : HWND, dwflags : u32) -> HMONITOR);
    unsafe { MonitorFromWindow(hwnd, dwflags) }
}
#[inline]
pub unsafe fn MoveFileExW<P0, P1>(
    lpexistingfilename: P0,
    lpnewfilename: P1,
    dwflags: u32,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileExW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, dwflags : u32) -> windows_core::BOOL);
    unsafe {
        MoveFileExW(
            lpexistingfilename.param().abi(),
            lpnewfilename.param().abi(),
            dwflags,
        )
    }
}
#[inline]
pub unsafe fn NotifyWinEvent(event: u32, hwnd: HWND, idobject: i32, idchild: i32) {
    windows_core::link!("user32.dll" "system" fn NotifyWinEvent(event : u32, hwnd : HWND, idobject : i32, idchild : i32));
    unsafe { NotifyWinEvent(event, hwnd, idobject, idchild) }
}
#[inline]
pub unsafe fn OpenClipboard(hwndnewowner: Option<HWND>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn OpenClipboard(hwndnewowner : HWND) -> windows_core::BOOL);
    unsafe { OpenClipboard(hwndnewowner.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn OpenProcess(dwdesiredaccess: u32, binherithandle: bool, dwprocessid: u32) -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenProcess(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, dwprocessid : u32) -> HANDLE);
    unsafe { OpenProcess(dwdesiredaccess, binherithandle.into(), dwprocessid) }
}
#[inline]
pub unsafe fn OpenProcessToken(
    processhandle: HANDLE,
    desiredaccess: u32,
    tokenhandle: *mut HANDLE,
) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OpenProcessToken(processhandle : HANDLE, desiredaccess : u32, tokenhandle : *mut HANDLE) -> windows_core::BOOL);
    unsafe { OpenProcessToken(processhandle, desiredaccess, tokenhandle as _) }
}
#[inline]
pub unsafe fn PeekMessageW(
    lpmsg: *mut MSG,
    hwnd: Option<HWND>,
    wmsgfiltermin: u32,
    wmsgfiltermax: u32,
    wremovemsg: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PeekMessageW(lpmsg : *mut MSG, hwnd : HWND, wmsgfiltermin : u32, wmsgfiltermax : u32, wremovemsg : u32) -> windows_core::BOOL);
    unsafe {
        PeekMessageW(
            lpmsg as _,
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            wmsgfiltermin,
            wmsgfiltermax,
            wremovemsg,
        )
    }
}
#[inline]
pub unsafe fn PostMessageW(
    hwnd: Option<HWND>,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn PostMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> windows_core::BOOL);
    unsafe {
        PostMessageW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            msg,
            wparam,
            lparam,
        )
    }
}
#[inline]
pub unsafe fn PostQuitMessage(nexitcode: i32) {
    windows_core::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
    unsafe { PostQuitMessage(nexitcode) }
}
#[inline]
pub unsafe fn RegCloseKey(hkey: HKEY) -> LSTATUS {
    windows_core::link!("advapi32.dll" "system" fn RegCloseKey(hkey : HKEY) -> LSTATUS);
    unsafe { RegCloseKey(hkey) }
}
#[inline]
pub unsafe fn RegCreateKeyExW<P1, P3>(
    hkey: HKEY,
    lpsubkey: P1,
    reserved: Option<u32>,
    lpclass: P3,
    dwoptions: u32,
    samdesired: REGSAM,
    lpsecurityattributes: Option<*const SECURITY_ATTRIBUTES>,
    phkresult: *mut HKEY,
    lpdwdisposition: Option<*mut u32>,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegCreateKeyExW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, reserved : u32, lpclass : windows_core::PCWSTR, dwoptions : u32, samdesired : REGSAM, lpsecurityattributes : *const SECURITY_ATTRIBUTES, phkresult : *mut HKEY, lpdwdisposition : *mut u32) -> LSTATUS);
    unsafe {
        RegCreateKeyExW(
            hkey,
            lpsubkey.param().abi(),
            reserved.unwrap_or(core::mem::zeroed()) as _,
            lpclass.param().abi(),
            dwoptions,
            samdesired,
            lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _,
            phkresult as _,
            lpdwdisposition.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn RegDeleteTreeW<P1>(hkey: HKEY, lpsubkey: P1) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteTreeW(hkey : HKEY, lpsubkey : windows_core::PCWSTR) -> LSTATUS);
    unsafe { RegDeleteTreeW(hkey, lpsubkey.param().abi()) }
}
#[inline]
pub unsafe fn RegDeleteValueW<P1>(hkey: HKEY, lpvaluename: P1) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegDeleteValueW(hkey : HKEY, lpvaluename : windows_core::PCWSTR) -> LSTATUS);
    unsafe { RegDeleteValueW(hkey, lpvaluename.param().abi()) }
}
#[inline]
pub unsafe fn RegGetValueW<P1, P2>(
    hkey: HKEY,
    lpsubkey: P1,
    lpvalue: P2,
    dwflags: u32,
    pdwtype: Option<*mut u32>,
    pvdata: Option<*mut core::ffi::c_void>,
    pcbdata: Option<*mut u32>,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegGetValueW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, dwflags : u32, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> LSTATUS);
    unsafe {
        RegGetValueW(
            hkey,
            lpsubkey.param().abi(),
            lpvalue.param().abi(),
            dwflags,
            pdwtype.unwrap_or(core::mem::zeroed()) as _,
            pvdata.unwrap_or(core::mem::zeroed()) as _,
            pcbdata.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn RegOpenKeyExW<P1>(
    hkey: HKEY,
    lpsubkey: P1,
    uloptions: Option<u32>,
    samdesired: REGSAM,
    phkresult: *mut HKEY,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegOpenKeyExW(hkey : HKEY, lpsubkey : windows_core::PCWSTR, uloptions : u32, samdesired : REGSAM, phkresult : *mut HKEY) -> LSTATUS);
    unsafe {
        RegOpenKeyExW(
            hkey,
            lpsubkey.param().abi(),
            uloptions.unwrap_or(core::mem::zeroed()) as _,
            samdesired,
            phkresult as _,
        )
    }
}
#[inline]
pub unsafe fn RegSetValueExW<P1>(
    hkey: HKEY,
    lpvaluename: P1,
    reserved: Option<u32>,
    dwtype: u32,
    lpdata: Option<&[u8]>,
) -> LSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegSetValueExW(hkey : HKEY, lpvaluename : windows_core::PCWSTR, reserved : u32, dwtype : u32, lpdata : *const u8, cbdata : u32) -> LSTATUS);
    unsafe {
        RegSetValueExW(
            hkey,
            lpvaluename.param().abi(),
            reserved.unwrap_or(core::mem::zeroed()) as _,
            dwtype,
            lpdata.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            lpdata.map_or(0, |slice| slice.len().try_into().unwrap()),
        )
    }
}
#[inline]
pub unsafe fn RegisterClassW(lpwndclass: *const WNDCLASSW) -> ATOM {
    windows_core::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> ATOM);
    unsafe { RegisterClassW(lpwndclass) }
}
#[inline]
pub unsafe fn RegisterWindowMessageW<P0>(lpstring: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn RegisterWindowMessageW(lpstring : windows_core::PCWSTR) -> u32);
    unsafe { RegisterWindowMessageW(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn ReleaseCapture() -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ReleaseCapture() -> windows_core::BOOL);
    unsafe { ReleaseCapture() }
}
#[inline]
pub unsafe fn SHGetKnownFolderPath(
    rfid: *const KNOWNFOLDERID,
    dwflags: u32,
    htoken: Option<HANDLE>,
) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("shell32.dll" "system" fn SHGetKnownFolderPath(rfid : *const KNOWNFOLDERID, dwflags : u32, htoken : HANDLE, ppszpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SHGetKnownFolderPath(
            rfid,
            dwflags,
            htoken.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .map(|| result__)
    }
}
#[inline]
pub unsafe fn ScreenToClient(hwnd: HWND, lppoint: *mut POINT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ScreenToClient(hwnd : HWND, lppoint : *mut POINT) -> windows_core::BOOL);
    unsafe { ScreenToClient(hwnd, lppoint as _) }
}
#[inline]
pub unsafe fn SendInput(pinputs: &[INPUT], cbsize: i32) -> u32 {
    windows_core::link!("user32.dll" "system" fn SendInput(cinputs : u32, pinputs : *const INPUT, cbsize : i32) -> u32);
    unsafe { SendInput(pinputs.len().try_into().unwrap(), pinputs.as_ptr(), cbsize) }
}
#[inline]
pub unsafe fn SendMessageW(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    windows_core::link!("user32.dll" "system" fn SendMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
    unsafe { SendMessageW(hwnd, msg, wparam, lparam) }
}
#[inline]
pub unsafe fn SetCapture(hwnd: HWND) -> HWND {
    windows_core::link!("user32.dll" "system" fn SetCapture(hwnd : HWND) -> HWND);
    unsafe { SetCapture(hwnd) }
}
#[inline]
pub unsafe fn SetClipboardData(uformat: u32, hmem: Option<HANDLE>) -> HANDLE {
    windows_core::link!("user32.dll" "system" fn SetClipboardData(uformat : u32, hmem : HANDLE) -> HANDLE);
    unsafe { SetClipboardData(uformat, hmem.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetFocus(hwnd: Option<HWND>) -> HWND {
    windows_core::link!("user32.dll" "system" fn SetFocus(hwnd : HWND) -> HWND);
    unsafe { SetFocus(hwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetForegroundWindow(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetForegroundWindow(hwnd : HWND) -> windows_core::BOOL);
    unsafe { SetForegroundWindow(hwnd) }
}
#[inline]
pub unsafe fn SetInformationJobObject(
    hjob: HANDLE,
    jobobjectinformationclass: JOBOBJECTINFOCLASS,
    lpjobobjectinformation: *const core::ffi::c_void,
    cbjobobjectinformationlength: u32,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : HANDLE, jobobjectinformationclass : JOBOBJECTINFOCLASS, lpjobobjectinformation : *const core::ffi::c_void, cbjobobjectinformationlength : u32) -> windows_core::BOOL);
    unsafe {
        SetInformationJobObject(
            hjob,
            jobobjectinformationclass,
            lpjobobjectinformation,
            cbjobobjectinformationlength,
        )
    }
}
#[inline]
pub unsafe fn SetMenuDefaultItem(hmenu: HMENU, uitem: u32, fbypos: u32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetMenuDefaultItem(hmenu : HMENU, uitem : u32, fbypos : u32) -> windows_core::BOOL);
    unsafe { SetMenuDefaultItem(hmenu, uitem, fbypos) }
}
#[inline]
pub unsafe fn SetProcessDpiAwarenessContext(value: DPI_AWARENESS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetProcessDpiAwarenessContext(value : DPI_AWARENESS_CONTEXT) -> windows_core::BOOL);
    unsafe { SetProcessDpiAwarenessContext(value) }
}
#[inline]
pub unsafe fn SetTimer(
    hwnd: Option<HWND>,
    nidevent: usize,
    uelapse: u32,
    lptimerfunc: TIMERPROC,
) -> usize {
    windows_core::link!("user32.dll" "system" fn SetTimer(hwnd : HWND, nidevent : usize, uelapse : u32, lptimerfunc : TIMERPROC) -> usize);
    unsafe {
        SetTimer(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            nidevent,
            uelapse,
            lptimerfunc,
        )
    }
}
#[inline]
pub unsafe fn SetWinEventHook(
    eventmin: u32,
    eventmax: u32,
    hmodwineventproc: Option<HMODULE>,
    pfnwineventproc: WINEVENTPROC,
    idprocess: u32,
    idthread: u32,
    dwflags: u32,
) -> HWINEVENTHOOK {
    windows_core::link!("user32.dll" "system" fn SetWinEventHook(eventmin : u32, eventmax : u32, hmodwineventproc : HMODULE, pfnwineventproc : WINEVENTPROC, idprocess : u32, idthread : u32, dwflags : u32) -> HWINEVENTHOOK);
    unsafe {
        SetWinEventHook(
            eventmin,
            eventmax,
            hmodwineventproc.unwrap_or(core::mem::zeroed()) as _,
            pfnwineventproc,
            idprocess,
            idthread,
            dwflags,
        )
    }
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[inline]
pub unsafe fn SetWindowLongPtrW(hwnd: HWND, nindex: i32, dwnewlong: isize) -> isize {
    windows_core::link!("user32.dll" "system" fn SetWindowLongPtrW(hwnd : HWND, nindex : i32, dwnewlong : isize) -> isize);
    unsafe { SetWindowLongPtrW(hwnd, nindex, dwnewlong) }
}
#[cfg(target_pointer_width = "32")]
pub use SetWindowLongW as SetWindowLongPtrW;
#[inline]
pub unsafe fn SetWindowLongW(hwnd: HWND, nindex: i32, dwnewlong: i32) -> i32 {
    windows_core::link!("user32.dll" "system" fn SetWindowLongW(hwnd : HWND, nindex : i32, dwnewlong : i32) -> i32);
    unsafe { SetWindowLongW(hwnd, nindex, dwnewlong) }
}
#[inline]
pub unsafe fn SetWindowPlacement(
    hwnd: HWND,
    lpwndpl: *const WINDOWPLACEMENT,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowPlacement(hwnd : HWND, lpwndpl : *const WINDOWPLACEMENT) -> windows_core::BOOL);
    unsafe { SetWindowPlacement(hwnd, lpwndpl) }
}
#[inline]
pub unsafe fn SetWindowPos(
    hwnd: HWND,
    hwndinsertafter: Option<HWND>,
    x: i32,
    y: i32,
    cx: i32,
    cy: i32,
    uflags: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SetWindowPos(hwnd : HWND, hwndinsertafter : HWND, x : i32, y : i32, cx : i32, cy : i32, uflags : u32) -> windows_core::BOOL);
    unsafe {
        SetWindowPos(
            hwnd,
            hwndinsertafter.unwrap_or(core::mem::zeroed()) as _,
            x,
            y,
            cx,
            cy,
            uflags,
        )
    }
}
#[inline]
pub unsafe fn SetWindowTextW<P1>(hwnd: HWND, lpstring: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn SetWindowTextW(hwnd : HWND, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetWindowTextW(hwnd, lpstring.param().abi()) }
}
#[inline]
pub unsafe fn ShellExecuteW<P1, P2, P3, P4>(
    hwnd: Option<HWND>,
    lpoperation: P1,
    lpfile: P2,
    lpparameters: P3,
    lpdirectory: P4,
    nshowcmd: i32,
) -> HINSTANCE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("shell32.dll" "system" fn ShellExecuteW(hwnd : HWND, lpoperation : windows_core::PCWSTR, lpfile : windows_core::PCWSTR, lpparameters : windows_core::PCWSTR, lpdirectory : windows_core::PCWSTR, nshowcmd : i32) -> HINSTANCE);
    unsafe {
        ShellExecuteW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            lpoperation.param().abi(),
            lpfile.param().abi(),
            lpparameters.param().abi(),
            lpdirectory.param().abi(),
            nshowcmd,
        )
    }
}
#[inline]
pub unsafe fn Shell_NotifyIconW(
    dwmessage: u32,
    lpdata: *const NOTIFYICONDATAW,
) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn Shell_NotifyIconW(dwmessage : u32, lpdata : *const NOTIFYICONDATAW) -> windows_core::BOOL);
    unsafe { Shell_NotifyIconW(dwmessage, lpdata) }
}
#[inline]
pub unsafe fn ShowWindow(hwnd: HWND, ncmdshow: i32) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ShowWindow(hwnd : HWND, ncmdshow : i32) -> windows_core::BOOL);
    unsafe { ShowWindow(hwnd, ncmdshow) }
}
#[inline]
pub unsafe fn SystemParametersInfoW(
    uiaction: u32,
    uiparam: u32,
    pvparam: *mut core::ffi::c_void,
    fwinini: u32,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn SystemParametersInfoW(uiaction : u32, uiparam : u32, pvparam : *mut core::ffi::c_void, fwinini : u32) -> windows_core::BOOL);
    unsafe { SystemParametersInfoW(uiaction, uiparam, pvparam as _, fwinini) }
}
#[inline]
pub unsafe fn SystemTimeToTzSpecificLocalTime(
    lptimezoneinformation: Option<*const TIME_ZONE_INFORMATION>,
    lpuniversaltime: *const SYSTEMTIME,
    lplocaltime: *mut SYSTEMTIME,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SystemTimeToTzSpecificLocalTime(lptimezoneinformation : *const TIME_ZONE_INFORMATION, lpuniversaltime : *const SYSTEMTIME, lplocaltime : *mut SYSTEMTIME) -> windows_core::BOOL);
    unsafe {
        SystemTimeToTzSpecificLocalTime(
            lptimezoneinformation.unwrap_or(core::mem::zeroed()) as _,
            lpuniversaltime,
            lplocaltime as _,
        )
    }
}
#[inline]
pub unsafe fn Thread32First(hsnapshot: HANDLE, lpte: *mut THREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32First(hsnapshot : HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32First(hsnapshot, lpte as _) }
}
#[inline]
pub unsafe fn Thread32Next(hsnapshot: HANDLE, lpte: *mut THREADENTRY32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Thread32Next(hsnapshot : HANDLE, lpte : *mut THREADENTRY32) -> windows_core::BOOL);
    unsafe { Thread32Next(hsnapshot, lpte as _) }
}
#[inline]
pub unsafe fn TrackPopupMenuEx(
    hmenu: HMENU,
    uflags: u32,
    x: i32,
    y: i32,
    hwnd: HWND,
    lptpm: Option<*const TPMPARAMS>,
) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TrackPopupMenuEx(hmenu : HMENU, uflags : u32, x : i32, y : i32, hwnd : HWND, lptpm : *const TPMPARAMS) -> windows_core::BOOL);
    unsafe {
        TrackPopupMenuEx(
            hmenu,
            uflags,
            x,
            y,
            hwnd,
            lptpm.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn TranslateMessage(lpmsg: *const MSG) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> windows_core::BOOL);
    unsafe { TranslateMessage(lpmsg) }
}
#[inline]
pub unsafe fn UnhookWinEvent(hwineventhook: HWINEVENTHOOK) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn UnhookWinEvent(hwineventhook : HWINEVENTHOOK) -> windows_core::BOOL);
    unsafe { UnhookWinEvent(hwineventhook) }
}
#[inline]
pub unsafe fn UnregisterClassW<P0>(
    lpclassname: P0,
    hinstance: Option<HINSTANCE>,
) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("user32.dll" "system" fn UnregisterClassW(lpclassname : windows_core::PCWSTR, hinstance : HINSTANCE) -> windows_core::BOOL);
    unsafe {
        UnregisterClassW(
            lpclassname.param().abi(),
            hinstance.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn ValidateRect(hwnd: Option<HWND>, lprect: Option<*const RECT>) -> windows_core::BOOL {
    windows_core::link!("user32.dll" "system" fn ValidateRect(hwnd : HWND, lprect : *const RECT) -> windows_core::BOOL);
    unsafe {
        ValidateRect(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            lprect.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn WTSRegisterSessionNotification(hwnd: HWND, dwflags: u32) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSRegisterSessionNotification(hwnd : HWND, dwflags : u32) -> windows_core::BOOL);
    unsafe { WTSRegisterSessionNotification(hwnd, dwflags) }
}
#[inline]
pub unsafe fn WTSUnRegisterSessionNotification(hwnd: HWND) -> windows_core::BOOL {
    windows_core::link!("wtsapi32.dll" "system" fn WTSUnRegisterSessionNotification(hwnd : HWND) -> windows_core::BOOL);
    unsafe { WTSUnRegisterSessionNotification(hwnd) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ACCESS_MASK(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ATOM(pub u16);
pub const BM_GETCHECK: i32 = 240;
pub const BM_SETCHECK: i32 = 241;
pub const BST_CHECKED: i32 = 1;
pub const CBN_SELCHANGE: i32 = 1;
pub const CB_ADDSTRING: i32 = 323;
pub const CB_GETCURSEL: i32 = 327;
pub const CB_SETCURSEL: i32 = 334;
pub const CF_UNICODETEXT: i32 = 13;
pub type CLSCTX = u32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub type COINIT = i32;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COLOR_WINDOW: i32 = 5;
pub const COLOR_WINDOWTEXT: i32 = 8;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONTROLTYPEID(pub i32);
pub const CREATE_NO_WINDOW: i32 = 134217728;
pub const CUIAutomation: windows_core::GUID =
    windows_core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
pub type D2D1_ALPHA_MODE = i32;
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: windows_numerics::Matrix3x2,
}
pub type D2D1_DEBUG_LEVEL = i32;
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
pub type D2D1_FACTORY_TYPE = i32;
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0;
pub type D2D1_FEATURE_LEVEL = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: HWND,
    pub pixelSize: D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
pub type D2D1_PRESENT_OPTIONS = u32;
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
}
pub type D2D1_RENDER_TARGET_TYPE = i32;
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = 0;
pub type D2D1_RENDER_TARGET_USAGE = u32;
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D2D1_TAG(pub u64);
pub type D2D1_TEXT_ANTIALIAS_MODE = i32;
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1;
pub const D2DERR_RECREATE_TARGET: windows_core::HRESULT =
    windows_core::HRESULT(0x8899000C_u32 as _);
pub type D2D_COLOR_F = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3DCOLORVALUE {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
pub type DLGPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> isize,
>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DPI_AWARENESS_CONTEXT(pub *mut core::ffi::c_void);
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: DPI_AWARENESS_CONTEXT =
    DPI_AWARENESS_CONTEXT(-4 as _);
pub const DWMSBT_MAINWINDOW: DWM_SYSTEMBACKDROP_TYPE = 2;
pub const DWMWA_SYSTEMBACKDROP_TYPE: DWMWINDOWATTRIBUTE = 38;
pub type DWMWINDOWATTRIBUTE = i32;
pub type DWM_SYSTEMBACKDROP_TYPE = i32;
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub type DWRITE_FONT_WEIGHT = i32;
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = 400;
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2;
pub type DWRITE_TEXT_ALIGNMENT = i32;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
pub type DWRITE_WORD_WRAPPING = i32;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = 1;
pub type DXGI_FORMAT = i32;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = 87;
pub const EM_LIMITTEXT: i32 = 197;
pub const EM_SETSEL: i32 = 177;
pub const ERROR_ALREADY_EXISTS: i32 = 183;
pub const ERROR_FILE_NOT_FOUND: i32 = 2;
pub const ERROR_MORE_DATA: i32 = 234;
pub const ERROR_PATH_NOT_FOUND: i32 = 3;
pub const EVENT_OBJECT_HIDE: i32 = 32771;
pub const EVENT_OBJECT_LOCATIONCHANGE: i32 = 32779;
pub const EVENT_OBJECT_SHOW: i32 = 32770;
pub const EVENT_SYSTEM_ALERT: i32 = 2;
pub const EVENT_SYSTEM_FOREGROUND: i32 = 3;
pub const EVENT_SYSTEM_MINIMIZEEND: i32 = 23;
pub const EVENT_SYSTEM_MOVESIZEEND: i32 = 11;
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const GMEM_MOVEABLE: i32 = 2;
pub const GR_GDIOBJECTS: i32 = 0;
pub const GR_USEROBJECTS: i32 = 1;
pub const GWLP_ID: i32 = -12;
pub const GWLP_USERDATA: i32 = -21;
pub const GWL_EXSTYLE: i32 = -20;
pub const GW_HWNDPREV: i32 = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HBRUSH(pub *mut core::ffi::c_void);
pub const HCF_HIGHCONTRASTON: i32 = 1;
pub type HCURSOR = HICON;
pub type HGLOBAL = HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HICON(pub *mut core::ffi::c_void);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIGHCONTRASTW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpszDefaultScheme: windows_core::PWSTR,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HINSTANCE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HKEY(pub *mut core::ffi::c_void);
pub const HKEY_CURRENT_USER: HKEY = HKEY(-2147483647 as _);
pub const HKEY_LOCAL_MACHINE: HKEY = HKEY(-2147483646 as _);
pub type HLOCAL = HANDLE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMENU(pub *mut core::ffi::c_void);
pub type HMODULE = HINSTANCE;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWINEVENTHOOK(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
pub const HWND_NOTOPMOST: HWND = HWND(-2 as _);
pub const HWND_TOP: HWND = HWND(0 as _);
pub const HWND_TOPMOST: HWND = HWND(-1 as _);
pub const ICC_STANDARD_CLASSES: i32 = 16384;
pub const ICC_TAB_CLASSES: i32 = 8;
pub const ICC_UPDOWN_CLASS: i32 = 16;
pub const ICON_BIG: i32 = 1;
pub const ICON_SMALL: i32 = 0;
windows_core::imp::define_interface!(
    ID2D1Brush,
    ID2D1Brush_Vtbl,
    0x2cd906a8_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Brush {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Brush, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    SetOpacity: usize,
    SetTransform: usize,
    GetOpacity: usize,
    GetTransform: usize,
}
impl windows_core::RuntimeName for ID2D1Brush {}
windows_core::imp::define_interface!(
    ID2D1Factory,
    ID2D1Factory_Vtbl,
    0x06152247_6f50_465a_9245_118bfd3b6007
);
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn CreateHwndRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1HwndRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHwndRenderTarget)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
                hwndrendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    ReloadSystemMetrics: usize,
    GetDesktopDpi: usize,
    CreateRectangleGeometry: usize,
    CreateRoundedRectangleGeometry: usize,
    CreateEllipseGeometry: usize,
    CreateGeometryGroup: usize,
    CreateTransformedGeometry: usize,
    CreatePathGeometry: usize,
    CreateStrokeStyle: usize,
    CreateDrawingStateBlock: usize,
    CreateWicBitmapRenderTarget: usize,
    pub CreateHwndRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateDxgiSurfaceRenderTarget: usize,
    CreateDCRenderTarget: usize,
}
impl windows_core::RuntimeName for ID2D1Factory {}
windows_core::imp::define_interface!(
    ID2D1HwndRenderTarget,
    ID2D1HwndRenderTarget_Vtbl,
    0x2cd90698_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1HwndRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1HwndRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
impl ID2D1HwndRenderTarget {
    pub unsafe fn Resize(&self, pixelsize: *const D2D_SIZE_U) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Resize)(
                windows_core::Interface::as_raw(self),
                pixelsize,
            )
        }
    }
}
#[repr(C)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    CheckWindowState: usize,
    pub Resize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_U,
    ) -> windows_core::HRESULT,
    GetHwnd: usize,
}
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1RenderTarget,
    ID2D1RenderTarget_Vtbl,
    0x2cd90694_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RenderTarget {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
impl ID2D1RenderTarget {
    pub unsafe fn CreateSolidColorBrush(
        &self,
        color: *const D2D_COLOR_F,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1SolidColorBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSolidColorBrush)(
                windows_core::Interface::as_raw(self),
                color,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawTextLayout<P1, P2>(
        &self,
        origin: windows_numerics::Vector2,
        textlayout: P1,
        defaultfillbrush: P2,
        options: D2D1_DRAW_TEXT_OPTIONS,
    ) where
        P1: windows_core::Param<IDWriteTextLayout>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawTextLayout)(
                windows_core::Interface::as_raw(self),
                origin,
                textlayout.param().abi(),
                defaultfillbrush.param().abi(),
                options,
            );
        }
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAntialiasMode)(
                windows_core::Interface::as_raw(self),
                textantialiasmode,
            );
        }
    }
    pub unsafe fn Clear(&self, clearcolor: Option<*const D2D_COLOR_F>) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(
                windows_core::Interface::as_raw(self),
                clearcolor.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn BeginDraw(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(
                self,
            ));
        }
    }
    pub unsafe fn EndDraw(
        &self,
        tag1: Option<*mut D2D1_TAG>,
        tag2: Option<*mut D2D1_TAG>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EndDraw)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetPixelSize(&self) -> D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    CreateBitmap: usize,
    CreateBitmapFromWicBitmap: usize,
    CreateSharedBitmap: usize,
    CreateBitmapBrush: usize,
    pub CreateSolidColorBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_COLOR_F,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateGradientStopCollection: usize,
    CreateLinearGradientBrush: usize,
    CreateRadialGradientBrush: usize,
    CreateCompatibleRenderTarget: usize,
    CreateLayer: usize,
    CreateMesh: usize,
    DrawLine: usize,
    DrawRectangle: usize,
    FillRectangle: usize,
    DrawRoundedRectangle: usize,
    FillRoundedRectangle: usize,
    DrawEllipse: usize,
    FillEllipse: usize,
    DrawGeometry: usize,
    FillGeometry: usize,
    FillMesh: usize,
    FillOpacityMask: usize,
    DrawBitmap: usize,
    DrawText: usize,
    pub DrawTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_DRAW_TEXT_OPTIONS,
    ),
    DrawGlyphRun: usize,
    SetTransform: usize,
    GetTransform: usize,
    SetAntialiasMode: usize,
    GetAntialiasMode: usize,
    pub SetTextAntialiasMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
    GetTextAntialiasMode: usize,
    SetTextRenderingParams: usize,
    GetTextRenderingParams: usize,
    SetTags: usize,
    GetTags: usize,
    PushLayer: usize,
    PopLayer: usize,
    Flush: usize,
    SaveDrawingState: usize,
    RestoreDrawingState: usize,
    PushAxisAlignedClip: usize,
    PopAxisAlignedClip: usize,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_COLOR_F),
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D2D1_TAG,
        *mut D2D1_TAG,
    ) -> windows_core::HRESULT,
    GetPixelFormat: usize,
    SetDpi: usize,
    GetDpi: usize,
    GetSize: usize,
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_U),
    GetMaximumBitmapSize: usize,
    IsSupported: usize,
}
impl windows_core::RuntimeName for ID2D1RenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Resource,
    ID2D1Resource_Vtbl,
    0x2cd90691_12e2_11dc_9fed_001143a055f9
);
windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
impl ID2D1Resource {
    pub unsafe fn GetFactory(&self) -> windows_core::Result<ID2D1Factory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFactory)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
pub struct ID2D1Resource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
pub trait ID2D1Resource_Impl: windows_core::IUnknownImpl {
    fn GetFactory(&self, factory: windows_core::OutRef<ID2D1Factory>);
}
impl ID2D1Resource_Vtbl {
    pub const fn new<Identity: ID2D1Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFactory<Identity: ID2D1Resource_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            factory: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Resource_Impl::GetFactory(this, core::mem::transmute_copy(&factory));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFactory: GetFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Resource {}
windows_core::imp::define_interface!(
    ID2D1SolidColorBrush,
    ID2D1SolidColorBrush_Vtbl,
    0x2cd906a9_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1SolidColorBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1SolidColorBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
impl ID2D1SolidColorBrush {
    pub unsafe fn SetColor(&self, color: *const D2D_COLOR_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetColor)(
                windows_core::Interface::as_raw(self),
                color,
            );
        }
    }
    pub unsafe fn GetColor(&self) -> D2D_COLOR_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
}
#[repr(C)]
pub struct ID2D1SolidColorBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    pub SetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_COLOR_F),
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_COLOR_F),
}
impl windows_core::RuntimeName for ID2D1SolidColorBrush {}
pub const IDCANCEL: i32 = 2;
pub const IDC_ARROW: windows_core::PCWSTR = windows_core::PCWSTR(32512 as _);
pub const IDI_APPLICATION: windows_core::PCWSTR = windows_core::PCWSTR(32512 as _);
pub const IDOK: i32 = 1;
windows_core::imp::define_interface!(
    IDWriteFactory,
    IDWriteFactory_Vtbl,
    0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48
);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub unsafe fn CreateTextFormat<P0, P1, P6>(
        &self,
        fontfamilyname: P0,
        fontcollection: P1,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: f32,
        localename: P6,
    ) -> windows_core::Result<IDWriteTextFormat>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(
                windows_core::Interface::as_raw(self),
                fontfamilyname.param().abi(),
                fontcollection.param().abi(),
                fontweight,
                fontstyle,
                fontstretch,
                fontsize,
                localename.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextLayout<P2>(
        &self,
        string: &[u16],
        textformat: P2,
        maxwidth: f32,
        maxheight: f32,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(
                windows_core::Interface::as_raw(self),
                string.as_ptr(),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                maxwidth,
                maxheight,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSystemFontCollection: usize,
    CreateCustomFontCollection: usize,
    RegisterFontCollectionLoader: usize,
    UnregisterFontCollectionLoader: usize,
    CreateFontFileReference: usize,
    CreateCustomFontFileReference: usize,
    CreateFontFace: usize,
    CreateRenderingParams: usize,
    CreateMonitorRenderingParams: usize,
    CreateCustomRenderingParams: usize,
    RegisterFontFileLoader: usize,
    UnregisterFontFileLoader: usize,
    pub CreateTextFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_FONT_STYLE,
        DWRITE_FONT_STRETCH,
        f32,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateTypography: usize,
    GetGdiInterop: usize,
    pub CreateTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const u16,
        u32,
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateGdiCompatibleTextLayout: usize,
    CreateEllipsisTrimmingSign: usize,
    CreateTextAnalyzer: usize,
    CreateNumberSubstitution: usize,
    CreateGlyphRunAnalysis: usize,
}
impl windows_core::RuntimeName for IDWriteFactory {}
windows_core::imp::define_interface!(
    IDWriteFontCollection,
    IDWriteFontCollection_Vtbl,
    0xa84cee02_3eea_4eee_a827_87c1a02a0fcc
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontFamilyCount: usize,
    GetFontFamily: usize,
    FindFamilyName: usize,
    GetFontFromFontFace: usize,
}
impl windows_core::RuntimeName for IDWriteFontCollection {}
windows_core::imp::define_interface!(
    IDWriteTextFormat,
    IDWriteTextFormat_Vtbl,
    0x9c906818_31d7_4fd3_a151_7c5e225db55a
);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub unsafe fn SetTextAlignment(
        &self,
        textalignment: DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAlignment)(
                windows_core::Interface::as_raw(self),
                textalignment,
            )
        }
    }
    pub unsafe fn SetParagraphAlignment(
        &self,
        paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetParagraphAlignment)(
                windows_core::Interface::as_raw(self),
                paragraphalignment,
            )
        }
    }
    pub unsafe fn SetWordWrapping(
        &self,
        wordwrapping: DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetWordWrapping)(
                windows_core::Interface::as_raw(self),
                wordwrapping,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT,
    SetReadingDirection: usize,
    SetFlowDirection: usize,
    SetIncrementalTabStop: usize,
    SetTrimming: usize,
    SetLineSpacing: usize,
    GetTextAlignment: usize,
    GetParagraphAlignment: usize,
    GetWordWrapping: usize,
    GetReadingDirection: usize,
    GetFlowDirection: usize,
    GetIncrementalTabStop: usize,
    GetTrimming: usize,
    GetLineSpacing: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
}
impl windows_core::RuntimeName for IDWriteTextFormat {}
windows_core::imp::define_interface!(
    IDWriteTextLayout,
    IDWriteTextLayout_Vtbl,
    0x53737037_6d14_410b_9bfe_0b182bb70961
);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout,
    windows_core::IUnknown,
    IDWriteTextFormat
);
impl IDWriteTextLayout {
    pub unsafe fn GetMetrics(
        &self,
        textmetrics: *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(
                windows_core::Interface::as_raw(self),
                textmetrics as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    SetMaxWidth: usize,
    SetMaxHeight: usize,
    SetFontCollection: usize,
    SetFontFamilyName: usize,
    SetFontWeight: usize,
    SetFontStyle: usize,
    SetFontStretch: usize,
    SetFontSize: usize,
    SetUnderline: usize,
    SetStrikethrough: usize,
    SetDrawingEffect: usize,
    SetInlineObject: usize,
    SetTypography: usize,
    SetLocaleName: usize,
    GetMaxWidth: usize,
    GetMaxHeight: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetUnderline: usize,
    GetStrikethrough: usize,
    GetDrawingEffect: usize,
    GetInlineObject: usize,
    GetTypography: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
    Draw: usize,
    GetLineMetrics: usize,
    pub GetMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT,
    GetOverhangMetrics: usize,
    GetClusterMetrics: usize,
    DetermineMinWidth: usize,
    HitTestPoint: usize,
    HitTestTextPosition: usize,
    HitTestTextRange: usize,
}
impl windows_core::RuntimeName for IDWriteTextLayout {}
pub const IDYES: i32 = 6;
windows_core::imp::define_interface!(
    IDispatch,
    IDispatch_Vtbl,
    0x00020400_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetTypeInfoCount: usize,
    GetTypeInfo: usize,
    GetIDsOfNames: usize,
    Invoke: usize,
}
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(
    IInvokeProvider,
    IInvokeProvider_Vtbl,
    0x54fcb24b_e18e_47a2_b4d3_eccbe77599a2
);
windows_core::imp::interface_hierarchy!(IInvokeProvider, windows_core::IUnknown);
impl IInvokeProvider {
    pub unsafe fn Invoke(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IInvokeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInvokeProvider_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IInvokeProvider_Vtbl {
    pub const fn new<Identity: IInvokeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IInvokeProvider_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInvokeProvider_Impl::Invoke(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInvokeProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInvokeProvider {}
pub const IMAGE_ICON: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INITCOMMONCONTROLSEX {
    pub dwSize: u32,
    pub dwICC: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT {
    pub r#type: u32,
    pub Anonymous: INPUT_0,
}
impl Default for INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_0 {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}
impl Default for INPUT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INPUT_KEYBOARD: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
windows_core::imp::define_interface!(
    IRawElementProviderSimple,
    IRawElementProviderSimple_Vtbl,
    0xd6dd68d1_86fd_4332_8666_9abedea2d24c
);
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple, windows_core::IUnknown);
impl IRawElementProviderSimple {
    pub unsafe fn ProviderOptions(&self) -> windows_core::Result<ProviderOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetPatternProvider(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProvider)(
                windows_core::Interface::as_raw(self),
                patternid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValue)(
                windows_core::Interface::as_raw(self),
                propertyid,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostRawElementProvider(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostRawElementProvider)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IRawElementProviderSimple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProviderOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut ProviderOptions,
    ) -> windows_core::HRESULT,
    pub GetPatternProvider: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPropertyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub HostRawElementProvider: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IRawElementProviderSimple_Impl: windows_core::IUnknownImpl {
    fn ProviderOptions(&self) -> windows_core::Result<ProviderOptions>;
    fn GetPatternProvider(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown>;
    fn GetPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<VARIANT>;
    fn HostRawElementProvider(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl IRawElementProviderSimple_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProviderOptions<
            Identity: IRawElementProviderSimple_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut ProviderOptions,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::ProviderOptions(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPatternProvider<
            Identity: IRawElementProviderSimple_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            patternid: PATTERNID,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::GetPatternProvider(
                    this,
                    core::mem::transmute_copy(&patternid),
                ) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValue<
            Identity: IRawElementProviderSimple_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            pretval: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::GetPropertyValue(
                    this,
                    core::mem::transmute_copy(&propertyid),
                ) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostRawElementProvider<
            Identity: IRawElementProviderSimple_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pretval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::HostRawElementProvider(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProviderOptions: ProviderOptions::<Identity, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRawElementProviderSimple {}
windows_core::imp::define_interface!(
    IRecordInfo,
    IRecordInfo_Vtbl,
    0x0000002f_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
#[repr(C)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    RecordInit: usize,
    RecordClear: usize,
    RecordCopy: usize,
    GetGuid: usize,
    GetName: usize,
    GetSize: usize,
    GetTypeInfo: usize,
    GetField: usize,
    GetFieldNoCopy: usize,
    PutField: usize,
    PutFieldNoCopy: usize,
    GetFieldNames: usize,
    IsMatchingType: usize,
    RecordCreate: usize,
    RecordCreateCopy: usize,
    RecordDestroy: usize,
}
impl windows_core::RuntimeName for IRecordInfo {}
windows_core::imp::define_interface!(
    IUIAutomation,
    IUIAutomation_Vtbl,
    0x30cbe57d_d9d0_452a_ab13_7ac5ac4825ee
);
windows_core::imp::interface_hierarchy!(IUIAutomation, windows_core::IUnknown);
impl IUIAutomation {
    pub unsafe fn ElementFromHandle(
        &self,
        hwnd: UIA_HWND,
    ) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandle)(
                windows_core::Interface::as_raw(self),
                hwnd,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromPoint(&self, pt: POINT) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPoint)(
                windows_core::Interface::as_raw(self),
                pt,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawViewWalker)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrueCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CompareElements: usize,
    CompareRuntimeIds: usize,
    GetRootElement: usize,
    pub ElementFromHandle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UIA_HWND,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ElementFromPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        POINT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetFocusedElement: usize,
    GetRootElementBuildCache: usize,
    ElementFromHandleBuildCache: usize,
    ElementFromPointBuildCache: usize,
    GetFocusedElementBuildCache: usize,
    CreateTreeWalker: usize,
    ControlViewWalker: usize,
    ContentViewWalker: usize,
    pub RawViewWalker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    RawViewCondition: usize,
    ControlViewCondition: usize,
    ContentViewCondition: usize,
    CreateCacheRequest: usize,
    pub CreateTrueCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateFalseCondition: usize,
    CreatePropertyCondition: usize,
    CreatePropertyConditionEx: usize,
    CreateAndCondition: usize,
    CreateAndConditionFromArray: usize,
    CreateAndConditionFromNativeArray: usize,
    CreateOrCondition: usize,
    CreateOrConditionFromArray: usize,
    CreateOrConditionFromNativeArray: usize,
    CreateNotCondition: usize,
    AddAutomationEventHandler: usize,
    RemoveAutomationEventHandler: usize,
    AddPropertyChangedEventHandlerNativeArray: usize,
    AddPropertyChangedEventHandler: usize,
    RemovePropertyChangedEventHandler: usize,
    AddStructureChangedEventHandler: usize,
    RemoveStructureChangedEventHandler: usize,
    AddFocusChangedEventHandler: usize,
    RemoveFocusChangedEventHandler: usize,
    RemoveAllEventHandlers: usize,
    IntNativeArrayToSafeArray: usize,
    IntSafeArrayToNativeArray: usize,
    RectToVariant: usize,
    VariantToRect: usize,
    SafeArrayToRectNativeArray: usize,
    CreateProxyFactoryEntry: usize,
    ProxyFactoryMapping: usize,
    GetPropertyProgrammaticName: usize,
    GetPatternProgrammaticName: usize,
    PollForPotentialSupportedPatterns: usize,
    PollForPotentialSupportedProperties: usize,
    CheckNotSupported: usize,
    ReservedNotSupportedValue: usize,
    ReservedMixedAttributeValue: usize,
    ElementFromIAccessible: usize,
    ElementFromIAccessibleBuildCache: usize,
}
impl windows_core::RuntimeName for IUIAutomation {}
windows_core::imp::define_interface!(
    IUIAutomationCondition,
    IUIAutomationCondition_Vtbl,
    0x352ffba8_0973_437c_a61f_f64cafd81df9
);
windows_core::imp::interface_hierarchy!(IUIAutomationCondition, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationCondition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IUIAutomationCondition_Impl: windows_core::IUnknownImpl {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationCondition_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationCondition {}
windows_core::imp::define_interface!(
    IUIAutomationElement,
    IUIAutomationElement_Vtbl,
    0xd22108aa_8ac5_49a5_837b_37bbb3d7591e
);
windows_core::imp::interface_hierarchy!(IUIAutomationElement, windows_core::IUnknown);
impl IUIAutomationElement {
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn FindAll<P1>(
        &self,
        scope: TreeScope,
        condition: P1,
    ) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAll)(
                windows_core::Interface::as_raw(self),
                scope,
                condition.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentPattern(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPattern)(
                windows_core::Interface::as_raw(self),
                patternid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProcessId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentControlType(&self) -> windows_core::Result<CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentControlType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAutomationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentClassName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsOffscreen)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFrameworkId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentBoundingRectangle(&self) -> windows_core::Result<RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentBoundingRectangle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IUIAutomationElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    GetRuntimeId: usize,
    FindFirst: usize,
    pub FindAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FindFirstBuildCache: usize,
    FindAllBuildCache: usize,
    BuildUpdatedCache: usize,
    GetCurrentPropertyValue: usize,
    GetCurrentPropertyValueEx: usize,
    GetCachedPropertyValue: usize,
    GetCachedPropertyValueEx: usize,
    GetCurrentPatternAs: usize,
    GetCachedPatternAs: usize,
    pub GetCurrentPattern: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetCachedPattern: usize,
    GetCachedParent: usize,
    GetCachedChildren: usize,
    pub CurrentProcessId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CONTROLTYPEID,
    ) -> windows_core::HRESULT,
    CurrentLocalizedControlType: usize,
    CurrentName: usize,
    CurrentAcceleratorKey: usize,
    CurrentAccessKey: usize,
    CurrentHasKeyboardFocus: usize,
    CurrentIsKeyboardFocusable: usize,
    CurrentIsEnabled: usize,
    pub CurrentAutomationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentClassName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CurrentHelpText: usize,
    CurrentCulture: usize,
    CurrentIsControlElement: usize,
    CurrentIsContentElement: usize,
    CurrentIsPassword: usize,
    CurrentNativeWindowHandle: usize,
    CurrentItemType: usize,
    pub CurrentIsOffscreen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    CurrentOrientation: usize,
    pub CurrentFrameworkId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CurrentIsRequiredForForm: usize,
    CurrentItemStatus: usize,
    pub CurrentBoundingRectangle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut RECT) -> windows_core::HRESULT,
    CurrentLabeledBy: usize,
    CurrentAriaRole: usize,
    CurrentAriaProperties: usize,
    CurrentIsDataValidForForm: usize,
    CurrentControllerFor: usize,
    CurrentDescribedBy: usize,
    CurrentFlowsTo: usize,
    CurrentProviderDescription: usize,
    CachedProcessId: usize,
    CachedControlType: usize,
    CachedLocalizedControlType: usize,
    CachedName: usize,
    CachedAcceleratorKey: usize,
    CachedAccessKey: usize,
    CachedHasKeyboardFocus: usize,
    CachedIsKeyboardFocusable: usize,
    CachedIsEnabled: usize,
    CachedAutomationId: usize,
    CachedClassName: usize,
    CachedHelpText: usize,
    CachedCulture: usize,
    CachedIsControlElement: usize,
    CachedIsContentElement: usize,
    CachedIsPassword: usize,
    CachedNativeWindowHandle: usize,
    CachedItemType: usize,
    CachedIsOffscreen: usize,
    CachedOrientation: usize,
    CachedFrameworkId: usize,
    CachedIsRequiredForForm: usize,
    CachedItemStatus: usize,
    CachedBoundingRectangle: usize,
    CachedLabeledBy: usize,
    CachedAriaRole: usize,
    CachedAriaProperties: usize,
    CachedIsDataValidForForm: usize,
    CachedControllerFor: usize,
    CachedDescribedBy: usize,
    CachedFlowsTo: usize,
    CachedProviderDescription: usize,
    GetClickablePoint: usize,
}
impl windows_core::RuntimeName for IUIAutomationElement {}
windows_core::imp::define_interface!(
    IUIAutomationElementArray,
    IUIAutomationElementArray_Vtbl,
    0x14314595_b4bc_4055_95f2_58f2e42c9855
);
windows_core::imp::interface_hierarchy!(IUIAutomationElementArray, windows_core::IUnknown);
impl IUIAutomationElementArray {
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IUIAutomationElementArray_Impl: windows_core::IUnknownImpl {
    fn Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Length<
            Identity: IUIAutomationElementArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            length: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::Length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetElement<
            Identity: IUIAutomationElementArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: i32,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::GetElement(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Length: Length::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationElementArray {}
windows_core::imp::define_interface!(
    IUIAutomationInvokePattern,
    IUIAutomationInvokePattern_Vtbl,
    0xfb377fbe_8ea6_46d5_9c73_6499642d3059
);
windows_core::imp::interface_hierarchy!(IUIAutomationInvokePattern, windows_core::IUnknown);
impl IUIAutomationInvokePattern {
    pub unsafe fn Invoke(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationInvokePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationInvokePattern_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IUIAutomationInvokePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationInvokePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: IUIAutomationInvokePattern_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationInvokePattern_Impl::Invoke(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationInvokePattern {}
windows_core::imp::define_interface!(
    IUIAutomationTreeWalker,
    IUIAutomationTreeWalker_Vtbl,
    0x4042c624_389c_4afc_a630_9df854a541fc
);
windows_core::imp::interface_hierarchy!(IUIAutomationTreeWalker, windows_core::IUnknown);
impl IUIAutomationTreeWalker {
    pub unsafe fn GetParentElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentElement)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElement)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElement)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationTreeWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetParentElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFirstChildElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetLastChildElement: usize,
    pub GetNextSiblingElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetPreviousSiblingElement: usize,
    NormalizeElement: usize,
    GetParentElementBuildCache: usize,
    GetFirstChildElementBuildCache: usize,
    GetLastChildElementBuildCache: usize,
    GetNextSiblingElementBuildCache: usize,
    GetPreviousSiblingElementBuildCache: usize,
    NormalizeElementBuildCache: usize,
    Condition: usize,
}
impl windows_core::RuntimeName for IUIAutomationTreeWalker {}
pub type JOBOBJECTINFOCLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub PerProcessUserTimeLimit: i64,
    pub PerJobUserTimeLimit: i64,
    pub LimitFlags: u32,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub ActiveProcessLimit: u32,
    pub Affinity: usize,
    pub PriorityClass: u32,
    pub SchedulingClass: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: i32 = 8;
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: i32 = 8192;
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBDINPUT {
    pub wVk: u16,
    pub wScan: u16,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const KEYEVENTF_KEYUP: i32 = 2;
pub const KEY_QUERY_VALUE: i32 = 1;
pub const KEY_SET_VALUE: i32 = 2;
pub const KF_FLAG_DEFAULT: KNOWN_FOLDER_FLAG = 0;
pub type KNOWNFOLDERID = windows_core::GUID;
pub type KNOWN_FOLDER_FLAG = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOGFONTW {
    pub lfHeight: i32,
    pub lfWidth: i32,
    pub lfEscapement: i32,
    pub lfOrientation: i32,
    pub lfWeight: i32,
    pub lfItalic: u8,
    pub lfUnderline: u8,
    pub lfStrikeOut: u8,
    pub lfCharSet: u8,
    pub lfOutPrecision: u8,
    pub lfClipPrecision: u8,
    pub lfQuality: u8,
    pub lfPitchAndFamily: u8,
    pub lfFaceName: [u16; 32],
}
impl Default for LOGFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPARAM(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LRESULT(pub isize);
pub const LR_SHARED: i32 = 32768;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LSTATUS(pub i32);
pub const MA_NOACTIVATE: i32 = 3;
pub const MB_ICONERROR: i32 = 16;
pub const MB_ICONQUESTION: i32 = 32;
pub const MB_OK: i32 = 0;
pub const MB_YESNO: i32 = 4;
pub const MF_STRING: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}
pub const MONITOR_DEFAULTTONULL: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: u32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
pub const MOVEFILE_REPLACE_EXISTING: i32 = 1;
pub const MOVEFILE_WRITE_THROUGH: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}
pub const NIF_ICON: i32 = 2;
pub const NIF_MESSAGE: i32 = 1;
pub const NIF_SHOWTIP: i32 = 128;
pub const NIF_TIP: i32 = 4;
pub const NIM_ADD: i32 = 0;
pub const NIM_DELETE: i32 = 2;
pub const NIM_MODIFY: i32 = 1;
pub const NIM_SETVERSION: i32 = 4;
pub const NINF_KEY: i32 = 1;
pub const NIN_BALLOONSHOW: i32 = 1026;
pub const NIN_POPUPCLOSE: i32 = 1031;
pub const NIN_POPUPOPEN: i32 = 1030;
pub const NIN_SELECT: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NMHDR {
    pub hwndFrom: HWND,
    pub idFrom: usize,
    pub code: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NONCLIENTMETRICSW {
    pub cbSize: u32,
    pub iBorderWidth: i32,
    pub iScrollWidth: i32,
    pub iScrollHeight: i32,
    pub iCaptionWidth: i32,
    pub iCaptionHeight: i32,
    pub lfCaptionFont: LOGFONTW,
    pub iSmCaptionWidth: i32,
    pub iSmCaptionHeight: i32,
    pub lfSmCaptionFont: LOGFONTW,
    pub iMenuWidth: i32,
    pub iMenuHeight: i32,
    pub lfMenuFont: LOGFONTW,
    pub lfStatusFont: LOGFONTW,
    pub lfMessageFont: LOGFONTW,
    pub iPaddedBorderWidth: i32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(target_arch = "x86")]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub struct NOTIFYICONDATAW {
    pub cbSize: u32,
    pub hWnd: HWND,
    pub uID: u32,
    pub uFlags: u32,
    pub uCallbackMessage: u32,
    pub hIcon: HICON,
    pub szTip: [u16; 128],
    pub dwState: u32,
    pub dwStateMask: u32,
    pub szInfo: [u16; 256],
    pub Anonymous: NOTIFYICONDATAW_0,
    pub szInfoTitle: [u16; 64],
    pub dwInfoFlags: u32,
    pub guidItem: windows_core::GUID,
    pub hBalloonIcon: HICON,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy)]
pub union NOTIFYICONDATAW_0 {
    pub uTimeout: u32,
    pub uVersion: u32,
}
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
impl Default for NOTIFYICONDATAW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NOTIFYICON_VERSION_4: i32 = 4;
pub const NOTIFY_FOR_THIS_SESSION: i32 = 0;
pub const OBJID_CLIENT: i32 = -4;
pub const OBJID_WINDOW: i32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PATTERNID(pub i32);
pub const PBT_APMRESUMEAUTOMATIC: i32 = 18;
pub const PBT_APMRESUMESUSPEND: i32 = 7;
pub const PBT_APMSUSPEND: i32 = 4;
pub const PM_REMOVE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
pub const PROCESSOR_ARCHITECTURE_AMD64: i32 = 9;
pub const PROCESSOR_ARCHITECTURE_ARM64: i32 = 12;
pub const PROCESSOR_ARCHITECTURE_INTEL: i32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
pub const PROCESS_QUERY_LIMITED_INFORMATION: i32 = 4096;
pub const PROCESS_SUSPEND_RESUME: i32 = 2048;
pub const PROCESS_TERMINATE: i32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPERTYID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSID(pub *mut core::ffi::c_void);
pub type ProviderOptions = u32;
pub const ProviderOptions_ServerSideProvider: ProviderOptions = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
pub type REGSAM = ACCESS_MASK;
pub const REG_BINARY: u32 = 3;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_SZ: u32 = 1;
pub const RRF_NOEXPAND: i32 = 268435456;
pub const RRF_RT_ANY: i32 = 65535;
pub const RRF_RT_REG_DWORD: i32 = 16;
pub const RRF_RT_REG_SZ: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: u16,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SCODE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: PSID,
    pub Attributes: u32,
}
pub const SM_CXSMICON: i32 = 49;
pub const SM_CYSMICON: i32 = 50;
pub const SPI_GETHIGHCONTRAST: i32 = 66;
pub const SPI_GETNONCLIENTMETRICS: i32 = 41;
pub const SPI_SETWORKAREA: i32 = 47;
pub const STILL_ACTIVE: i32 = 259;
pub const SWP_NOACTIVATE: i32 = 16;
pub const SWP_NOMOVE: i32 = 2;
pub const SWP_NOSIZE: i32 = 1;
pub const SWP_NOZORDER: i32 = 4;
pub const SWP_SHOWWINDOW: i32 = 64;
pub const SW_HIDE: i32 = 0;
pub const SW_RESTORE: i32 = 9;
pub const SW_SHOW: i32 = 5;
pub const SW_SHOWMINNOACTIVE: i32 = 7;
pub const SW_SHOWNOACTIVATE: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO {
    pub Anonymous: SYSTEM_INFO_0,
    pub dwPageSize: u32,
    pub lpMinimumApplicationAddress: *mut core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: u32,
    pub dwProcessorType: u32,
    pub dwAllocationGranularity: u32,
    pub wProcessorLevel: u16,
    pub wProcessorRevision: u16,
}
impl Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_INFO_0 {
    pub dwOemId: u32,
    pub Anonymous: SYSTEM_INFO_0_0,
}
impl Default for SYSTEM_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_INFO_0_0 {
    pub wProcessorArchitecture: u16,
    pub wReserved: u16,
}
pub const S_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const TCIF_TEXT: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TCITEMW {
    pub mask: u32,
    pub dwState: u32,
    pub dwStateMask: u32,
    pub pszText: windows_core::PWSTR,
    pub cchTextMax: i32,
    pub iImage: i32,
    pub lParam: LPARAM,
}
pub const TCM_GETCURSEL: i32 = 4875;
pub const TCM_INSERTITEMW: i32 = 4926;
pub const TCM_SETCURSEL: i32 = 4876;
pub const TCN_SELCHANGE: u32 = 4294966745;
pub const TH32CS_SNAPTHREAD: i32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREADENTRY32 {
    pub dwSize: u32,
    pub cntUsage: u32,
    pub th32ThreadID: u32,
    pub th32OwnerProcessID: u32,
    pub tpBasePri: i32,
    pub tpDeltaPri: i32,
    pub dwFlags: u32,
}
pub type TIMERPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: u32, param2: usize, param3: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TIME_ZONE_INFORMATION {
    pub Bias: i32,
    pub StandardName: [u16; 32],
    pub StandardDate: SYSTEMTIME,
    pub StandardBias: i32,
    pub DaylightName: [u16; 32],
    pub DaylightDate: SYSTEMTIME,
    pub DaylightBias: i32,
}
impl Default for TIME_ZONE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TOKEN_INFORMATION_CLASS = i32;
pub const TOKEN_QUERY: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TPMPARAMS {
    pub cbSize: u32,
    pub rcExclude: RECT,
}
pub const TPM_BOTTOMALIGN: i32 = 32;
pub const TPM_RETURNCMD: i32 = 256;
pub const TPM_RIGHTBUTTON: i32 = 2;
pub const TokenUser: TOKEN_INFORMATION_CLASS = 1;
pub type TreeScope = i32;
pub const TreeScope_Children: TreeScope = 2;
pub const UDM_SETBUDDY: i32 = 1129;
pub const UDM_SETRANGE32: i32 = 1135;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UIA_HWND(pub *mut core::ffi::c_void);
pub type VARENUM = i32;
#[repr(C)]
pub struct VARIANT {
    pub Anonymous: VARIANT_0,
}
impl Clone for VARIANT {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union VARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0>,
    pub decVal: DECIMAL,
}
impl Clone for VARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct VARIANT_0_0 {
    pub vt: VARTYPE,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: VARIANT_0_0_0,
}
impl Clone for VARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union VARIANT_0_0_0 {
    pub llVal: i64,
    pub lVal: i32,
    pub bVal: u8,
    pub iVal: i16,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: SCODE,
    pub cyVal: CY,
    pub date: f64,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub parray: *mut SAFEARRAY,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub plVal: *mut i32,
    pub pllVal: *mut i64,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_PBOOL: *mut VARIANT_BOOL,
    pub pscode: *mut SCODE,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut VARIANT,
    pub byref: *mut core::ffi::c_void,
    pub cVal: i8,
    pub uiVal: u16,
    pub ulVal: u32,
    pub ullVal: u64,
    pub intVal: i32,
    pub uintVal: u32,
    pub pdecVal: *mut DECIMAL,
    pub pcVal: *mut i8,
    pub puiVal: *mut u16,
    pub pulVal: *mut u32,
    pub pullVal: *mut u64,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub Anonymous: core::mem::ManuallyDrop<VARIANT_0_0_0_0>,
}
impl Clone for VARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for VARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VARIANT_0_0_0_0 {
    pub pvRecord: *mut core::ffi::c_void,
    pub pRecInfo: core::mem::ManuallyDrop<Option<IRecordInfo>>,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
pub const VARIANT_FALSE: VARIANT_BOOL = VARIANT_BOOL(0);
pub const VARIANT_TRUE: VARIANT_BOOL = VARIANT_BOOL(-1);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARTYPE(pub u16);
pub const VK_CONTROL: i32 = 17;
pub const VK_LWIN: i32 = 91;
pub const VK_MENU: i32 = 18;
pub const VK_NEXT: i32 = 34;
pub const VK_PRIOR: i32 = 33;
pub const VK_RWIN: i32 = 92;
pub const VK_SHIFT: i32 = 16;
pub const VK_TAB: i32 = 9;
pub const VT_BOOL: VARENUM = 11;
pub const VT_BSTR: VARENUM = 8;
pub const VT_EMPTY: VARENUM = 0;
pub const VT_I4: VARENUM = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WINDOWPLACEMENT {
    pub length: u32,
    pub flags: u32,
    pub showCmd: u32,
    pub ptMinPosition: POINT,
    pub ptMaxPosition: POINT,
    pub rcNormalPosition: RECT,
}
pub type WINEVENTPROC = Option<
    unsafe extern "system" fn(
        hwineventhook: HWINEVENTHOOK,
        event: u32,
        hwnd: HWND,
        idobject: i32,
        idchild: i32,
        ideventthread: u32,
        dwmseventtime: u32,
    ),
>;
pub const WINEVENT_OUTOFCONTEXT: i32 = 0;
pub const WINEVENT_SKIPOWNPROCESS: i32 = 2;
pub const WM_APP: i32 = 32768;
pub const WM_CANCELMODE: i32 = 31;
pub const WM_CAPTURECHANGED: i32 = 533;
pub const WM_CLOSE: i32 = 16;
pub const WM_COMMAND: i32 = 273;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_DISPLAYCHANGE: i32 = 126;
pub const WM_DPICHANGED: i32 = 736;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: i32 = 800;
pub const WM_ENDSESSION: i32 = 22;
pub const WM_ERASEBKGND: i32 = 20;
pub const WM_GETOBJECT: i32 = 61;
pub const WM_INITDIALOG: i32 = 272;
pub const WM_KEYDOWN: i32 = 256;
pub const WM_LBUTTONDOWN: i32 = 513;
pub const WM_LBUTTONUP: i32 = 514;
pub const WM_MOUSEACTIVATE: i32 = 33;
pub const WM_MOUSEMOVE: i32 = 512;
pub const WM_NCDESTROY: i32 = 130;
pub const WM_NEXTDLGCTL: i32 = 40;
pub const WM_NOTIFY: i32 = 78;
pub const WM_NULL: i32 = 0;
pub const WM_PAINT: i32 = 15;
pub const WM_POWERBROADCAST: i32 = 536;
pub const WM_QUERYENDSESSION: i32 = 17;
pub const WM_QUIT: i32 = 18;
pub const WM_SETICON: i32 = 128;
pub const WM_SETTINGCHANGE: i32 = 26;
pub const WM_SIZE: i32 = 5;
pub const WM_SYSCOLORCHANGE: i32 = 21;
pub const WM_THEMECHANGED: i32 = 794;
pub const WM_TIMECHANGE: i32 = 30;
pub const WM_TIMER: i32 = 275;
pub const WM_WTSSESSION_CHANGE: i32 = 689;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: windows_core::PCWSTR,
    pub lpszClassName: windows_core::PCWSTR,
}
pub type WNDENUMPROC =
    Option<unsafe extern "system" fn(param0: HWND, param1: LPARAM) -> windows_core::BOOL>;
pub type WNDPROC = Option<
    unsafe extern "system" fn(param0: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT,
>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WPARAM(pub usize);
pub const WS_EX_NOACTIVATE: i32 = 134217728;
pub const WS_EX_TOOLWINDOW: i32 = 128;
pub const WS_EX_TOPMOST: i32 = 8;
pub const WS_OVERLAPPED: i32 = 0;
pub const WS_POPUP: u32 = 2147483648;
pub const WS_VISIBLE: i32 = 268435456;
