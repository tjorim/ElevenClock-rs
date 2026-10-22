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
pub type D2D1_ANTIALIAS_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
pub type D2D1_BITMAP_INTERPOLATION_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: windows_numerics::Matrix3x2,
}
pub type D2D1_CAP_STYLE = i32;
pub type D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = u32;
pub type D2D1_DASH_STYLE = i32;
pub type D2D1_DEBUG_LEVEL = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: D2D1_TAG,
    pub tag2: D2D1_TAG,
    pub transform: windows_numerics::Matrix3x2,
}
pub type D2D1_DRAW_TEXT_OPTIONS = u32;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: windows_numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_EXTEND_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
pub type D2D1_FACTORY_TYPE = i32;
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0;
pub type D2D1_FEATURE_LEVEL = i32;
pub type D2D1_FILL_MODE = i32;
pub type D2D1_GAMMA = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D_COLOR_F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: HWND,
    pub pixelSize: D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
pub type D2D1_LAYER_OPTIONS = u32;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: windows_numerics::Matrix3x2,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: windows_numerics::Vector2,
    pub endPoint: windows_numerics::Vector2,
}
pub type D2D1_LINE_JOIN = i32;
pub type D2D1_OPACITY_MASK_CONTENT = i32;
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
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: windows_numerics::Vector2,
    pub gradientOriginOffset: windows_numerics::Vector2,
    pub radiusX: f32,
    pub radiusY: f32,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct D2D1_TAG(pub u64);
pub type D2D1_TEXT_ANTIALIAS_MODE = i32;
pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE = 1;
pub type D2D1_WINDOW_STATE = u32;
pub const D2DERR_RECREATE_TARGET: windows_core::HRESULT =
    windows_core::HRESULT(0x8899000C_u32 as _);
pub type D2D_COLOR_F = D3DCOLORVALUE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl DWRITE_CLUSTER_METRICS {
    pub fn canWrapLineAfter(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_canWrapLineAfter(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn isWhitespace(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_isWhitespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn isNewline(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_isNewline(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn isSoftHyphen(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_isSoftHyphen(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn isRightToLeft(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_isRightToLeft(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn padding(&self) -> u16 {
        self._bitfield >> 5
    }
    pub fn set_padding(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
}
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FLOW_DIRECTION = i32;
pub type DWRITE_FONT_FACE_TYPE = i32;
pub type DWRITE_FONT_SIMULATIONS = u32;
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub type DWRITE_FONT_WEIGHT = i32;
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = 400;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: core::mem::ManuallyDrop<Option<IDWriteFontFace>>,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *const u16,
    pub glyphAdvances: *const f32,
    pub glyphOffsets: *const DWRITE_GLYPH_OFFSET,
    pub isSideways: windows_core::BOOL,
    pub bidiLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: windows_core::BOOL,
    pub isTrimmed: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: windows_core::BOOL,
}
pub type DWRITE_LINE_SPACING_METHOD = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
pub type DWRITE_MEASURING_MODE = i32;
pub type DWRITE_NUMBER_SUBSTITUTION_METHOD = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2;
pub type DWRITE_PIXEL_GEOMETRY = i32;
pub type DWRITE_READING_DIRECTION = i32;
pub type DWRITE_RENDERING_MODE = i32;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_TRIMMING {
    pub granularity: DWRITE_TRIMMING_GRANULARITY,
    pub delimiter: u32,
    pub delimiterCount: u32,
}
pub type DWRITE_TRIMMING_GRANULARITY = i32;
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EVENTID(pub i32);
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
windows_core::imp::define_interface!(
    IAccessible,
    IAccessible_Vtbl,
    0x618736e0_3c3d_11cf_810c_00aa00389b71
);
impl core::ops::Deref for IAccessible {
    type Target = IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAccessible, windows_core::IUnknown, IDispatch);
#[repr(C)]
pub struct IAccessible_Vtbl {
    pub base__: IDispatch_Vtbl,
    accParent: usize,
    accChildCount: usize,
    accChild: usize,
    accName: usize,
    accValue: usize,
    accDescription: usize,
    accRole: usize,
    accState: usize,
    accHelp: usize,
    accHelpTopic: usize,
    accKeyboardShortcut: usize,
    accFocus: usize,
    accSelection: usize,
    accDefaultAction: usize,
    accSelect: usize,
    accLocation: usize,
    accNavigate: usize,
    accHitTest: usize,
    accDoDefaultAction: usize,
    SetaccName: usize,
    SetaccValue: usize,
}
impl windows_core::RuntimeName for IAccessible {}
pub const ICC_STANDARD_CLASSES: i32 = 16384;
pub const ICC_TAB_CLASSES: i32 = 8;
pub const ICC_UPDOWN_CLASS: i32 = 16;
pub const ICON_BIG: i32 = 1;
pub const ICON_SMALL: i32 = 0;
windows_core::imp::define_interface!(
    ID2D1Bitmap,
    ID2D1Bitmap_Vtbl,
    0xa2296057_ea42_4099_983b_539fb6505426
);
impl core::ops::Deref for ID2D1Bitmap {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1Bitmap,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Image
);
#[repr(C)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    GetSize: usize,
    GetPixelSize: usize,
    GetPixelFormat: usize,
    GetDpi: usize,
    CopyFromBitmap: usize,
    CopyFromRenderTarget: usize,
    CopyFromMemory: usize,
}
impl windows_core::RuntimeName for ID2D1Bitmap {}
windows_core::imp::define_interface!(
    ID2D1BitmapBrush,
    ID2D1BitmapBrush_Vtbl,
    0x2cd906aa_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1BitmapBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1BitmapBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1BitmapBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetExtendModeX: usize,
    SetExtendModeY: usize,
    SetInterpolationMode: usize,
    SetBitmap: usize,
    GetExtendModeX: usize,
    GetExtendModeY: usize,
    GetInterpolationMode: usize,
    GetBitmap: usize,
}
impl windows_core::RuntimeName for ID2D1BitmapBrush {}
windows_core::imp::define_interface!(
    ID2D1BitmapRenderTarget,
    ID2D1BitmapRenderTarget_Vtbl,
    0x2cd90695_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1BitmapRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1BitmapRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
#[repr(C)]
pub struct ID2D1BitmapRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    GetBitmap: usize,
}
impl windows_core::RuntimeName for ID2D1BitmapRenderTarget {}
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
    ID2D1DCRenderTarget,
    ID2D1DCRenderTarget_Vtbl,
    0x1c51bc64_de61_46fd_9899_63a5d8f03950
);
impl core::ops::Deref for ID2D1DCRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1DCRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
#[repr(C)]
pub struct ID2D1DCRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    BindDC: usize,
}
impl windows_core::RuntimeName for ID2D1DCRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1DrawingStateBlock,
    ID2D1DrawingStateBlock_Vtbl,
    0x28506e39_ebf6_46a1_bb47_fd85565ab957
);
impl core::ops::Deref for ID2D1DrawingStateBlock {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1DrawingStateBlock,
    windows_core::IUnknown,
    ID2D1Resource
);
#[repr(C)]
pub struct ID2D1DrawingStateBlock_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetDescription: usize,
    SetDescription: usize,
    SetTextRenderingParams: usize,
    GetTextRenderingParams: usize,
}
impl windows_core::RuntimeName for ID2D1DrawingStateBlock {}
windows_core::imp::define_interface!(
    ID2D1EllipseGeometry,
    ID2D1EllipseGeometry_Vtbl,
    0x2cd906a4_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1EllipseGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1EllipseGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1EllipseGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetEllipse: usize,
}
impl windows_core::RuntimeName for ID2D1EllipseGeometry {}
windows_core::imp::define_interface!(
    ID2D1Factory,
    ID2D1Factory_Vtbl,
    0x06152247_6f50_465a_9245_118bfd3b6007
);
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).ReloadSystemMetrics)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesktopDpi)(
                windows_core::Interface::as_raw(self),
                dpix as _,
                dpiy as _,
            );
        }
    }
    pub unsafe fn CreateRectangleGeometry(
        &self,
        rectangle: *const D2D_RECT_F,
    ) -> windows_core::Result<ID2D1RectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleGeometry)(
                windows_core::Interface::as_raw(self),
                rectangle,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoundedRectangleGeometry(
        &self,
        roundedrectangle: *const D2D1_ROUNDED_RECT,
    ) -> windows_core::Result<ID2D1RoundedRectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoundedRectangleGeometry)(
                windows_core::Interface::as_raw(self),
                roundedrectangle,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipseGeometry(
        &self,
        ellipse: *const D2D1_ELLIPSE,
    ) -> windows_core::Result<ID2D1EllipseGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipseGeometry)(
                windows_core::Interface::as_raw(self),
                ellipse,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGeometryGroup(
        &self,
        fillmode: D2D1_FILL_MODE,
        geometries: &[Option<ID2D1Geometry>],
    ) -> windows_core::Result<ID2D1GeometryGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeometryGroup)(
                windows_core::Interface::as_raw(self),
                fillmode,
                core::mem::transmute(geometries.as_ptr()),
                geometries.len().try_into().unwrap(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransformedGeometry<P0>(
        &self,
        sourcegeometry: P0,
        transform: *const windows_numerics::Matrix3x2,
    ) -> windows_core::Result<ID2D1TransformedGeometry>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformedGeometry)(
                windows_core::Interface::as_raw(self),
                sourcegeometry.param().abi(),
                transform,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokeStyle(
        &self,
        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
        dashes: Option<&[f32]>,
    ) -> windows_core::Result<ID2D1StrokeStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokeStyle)(
                windows_core::Interface::as_raw(self),
                strokestyleproperties,
                dashes.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                dashes.map_or(0, |slice| slice.len().try_into().unwrap()),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDrawingStateBlock<P1>(
        &self,
        drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION>,
        textrenderingparams: P1,
    ) -> windows_core::Result<ID2D1DrawingStateBlock>
    where
        P1: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(
                windows_core::Interface::as_raw(self),
                drawingstatedescription.unwrap_or(core::mem::zeroed()) as _,
                textrenderingparams.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(
        &self,
        target: P0,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<IWICBitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateWicBitmapRenderTarget)(
                windows_core::Interface::as_raw(self),
                target.param().abi(),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
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
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(
        &self,
        dxgisurface: P0,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDxgiSurfaceRenderTarget)(
                windows_core::Interface::as_raw(self),
                dxgisurface.param().abi(),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDCRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1DCRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDCRenderTarget)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReloadSystemMetrics:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub CreateRectangleGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEllipseGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_FILL_MODE,
        *const *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTransformedGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_numerics::Matrix3x2,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_STROKE_STYLE_PROPERTIES,
        *const f32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDrawingStateBlock: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_DRAWING_STATE_DESCRIPTION,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateHwndRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDCRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ID2D1Factory_Impl: windows_core::IUnknownImpl {
    fn ReloadSystemMetrics(&self) -> windows_core::Result<()>;
    fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CreateRectangleGeometry(
        &self,
        rectangle: *const D2D_RECT_F,
    ) -> windows_core::Result<ID2D1RectangleGeometry>;
    fn CreateRoundedRectangleGeometry(
        &self,
        roundedrectangle: *const D2D1_ROUNDED_RECT,
    ) -> windows_core::Result<ID2D1RoundedRectangleGeometry>;
    fn CreateEllipseGeometry(
        &self,
        ellipse: *const D2D1_ELLIPSE,
    ) -> windows_core::Result<ID2D1EllipseGeometry>;
    fn CreateGeometryGroup(
        &self,
        fillmode: D2D1_FILL_MODE,
        geometries: *const Option<ID2D1Geometry>,
        geometriescount: u32,
    ) -> windows_core::Result<ID2D1GeometryGroup>;
    fn CreateTransformedGeometry(
        &self,
        sourcegeometry: windows_core::Ref<ID2D1Geometry>,
        transform: *const windows_numerics::Matrix3x2,
    ) -> windows_core::Result<ID2D1TransformedGeometry>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry>;
    fn CreateStrokeStyle(
        &self,
        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
        dashes: *const f32,
        dashescount: u32,
    ) -> windows_core::Result<ID2D1StrokeStyle>;
    fn CreateDrawingStateBlock(
        &self,
        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
        textrenderingparams: windows_core::Ref<IDWriteRenderingParams>,
    ) -> windows_core::Result<ID2D1DrawingStateBlock>;
    fn CreateWicBitmapRenderTarget(
        &self,
        target: windows_core::Ref<IWICBitmap>,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateHwndRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1HwndRenderTarget>;
    fn CreateDxgiSurfaceRenderTarget(
        &self,
        dxgisurface: windows_core::Ref<IDXGISurface>,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateDCRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1DCRenderTarget>;
}
impl ID2D1Factory_Vtbl {
    pub const fn new<Identity: ID2D1Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReloadSystemMetrics<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::ReloadSystemMetrics(this).into()
            }
        }
        unsafe extern "system" fn GetDesktopDpi<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dpix: *mut f32,
            dpiy: *mut f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::GetDesktopDpi(
                    this,
                    core::mem::transmute_copy(&dpix),
                    core::mem::transmute_copy(&dpiy),
                );
            }
        }
        unsafe extern "system" fn CreateRectangleGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rectangle: *const D2D_RECT_F,
            rectanglegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRectangleGeometry(
                    this,
                    core::mem::transmute_copy(&rectangle),
                ) {
                    Ok(ok__) => {
                        rectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            roundedrectangle: *const D2D1_ROUNDED_RECT,
            roundedrectanglegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRoundedRectangleGeometry(
                    this,
                    core::mem::transmute_copy(&roundedrectangle),
                ) {
                    Ok(ok__) => {
                        roundedrectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ellipse: *const D2D1_ELLIPSE,
            ellipsegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateEllipseGeometry(
                    this,
                    core::mem::transmute_copy(&ellipse),
                ) {
                    Ok(ok__) => {
                        ellipsegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeometryGroup<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fillmode: D2D1_FILL_MODE,
            geometries: *const *mut core::ffi::c_void,
            geometriescount: u32,
            geometrygroup: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateGeometryGroup(
                    this,
                    core::mem::transmute_copy(&fillmode),
                    core::mem::transmute_copy(&geometries),
                    core::mem::transmute_copy(&geometriescount),
                ) {
                    Ok(ok__) => {
                        geometrygroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformedGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sourcegeometry: *mut core::ffi::c_void,
            transform: *const windows_numerics::Matrix3x2,
            transformedgeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateTransformedGeometry(
                    this,
                    core::mem::transmute_copy(&sourcegeometry),
                    core::mem::transmute_copy(&transform),
                ) {
                    Ok(ok__) => {
                        transformedgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePathGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pathgeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreatePathGeometry(this) {
                    Ok(ok__) => {
                        pathgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
            dashes: *const f32,
            dashescount: u32,
            strokestyle: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateStrokeStyle(
                    this,
                    core::mem::transmute_copy(&strokestyleproperties),
                    core::mem::transmute_copy(&dashes),
                    core::mem::transmute_copy(&dashescount),
                ) {
                    Ok(ok__) => {
                        strokestyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
            textrenderingparams: *mut core::ffi::c_void,
            drawingstateblock: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDrawingStateBlock(
                    this,
                    core::mem::transmute_copy(&drawingstatedescription),
                    core::mem::transmute_copy(&textrenderingparams),
                ) {
                    Ok(ok__) => {
                        drawingstateblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            target: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            rendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateWicBitmapRenderTarget(
                    this,
                    core::mem::transmute_copy(&target),
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateHwndRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
            hwndrendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateHwndRenderTarget(
                    this,
                    core::mem::transmute_copy(&rendertargetproperties),
                    core::mem::transmute_copy(&hwndrendertargetproperties),
                ) {
                    Ok(ok__) => {
                        hwndrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dxgisurface: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            rendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDxgiSurfaceRenderTarget(
                    this,
                    core::mem::transmute_copy(&dxgisurface),
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDCRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            dcrendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDCRenderTarget(
                    this,
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        dcrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReloadSystemMetrics: ReloadSystemMetrics::<Identity, OFFSET>,
            GetDesktopDpi: GetDesktopDpi::<Identity, OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Identity, OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Identity, OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Identity, OFFSET>,
            CreateGeometryGroup: CreateGeometryGroup::<Identity, OFFSET>,
            CreateTransformedGeometry: CreateTransformedGeometry::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateWicBitmapRenderTarget: CreateWicBitmapRenderTarget::<Identity, OFFSET>,
            CreateHwndRenderTarget: CreateHwndRenderTarget::<Identity, OFFSET>,
            CreateDxgiSurfaceRenderTarget: CreateDxgiSurfaceRenderTarget::<Identity, OFFSET>,
            CreateDCRenderTarget: CreateDCRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Factory {}
windows_core::imp::define_interface!(
    ID2D1Geometry,
    ID2D1Geometry_Vtbl,
    0x2cd906a1_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Geometry {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetBounds: usize,
    GetWidenedBounds: usize,
    StrokeContainsPoint: usize,
    FillContainsPoint: usize,
    CompareWithGeometry: usize,
    Simplify: usize,
    Tessellate: usize,
    CombineWithGeometry: usize,
    Outline: usize,
    ComputeArea: usize,
    ComputeLength: usize,
    ComputePointAtLength: usize,
    Widen: usize,
}
impl windows_core::RuntimeName for ID2D1Geometry {}
windows_core::imp::define_interface!(
    ID2D1GeometryGroup,
    ID2D1GeometryGroup_Vtbl,
    0x2cd906a6_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1GeometryGroup {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GeometryGroup,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1GeometryGroup_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetFillMode: usize,
    GetSourceGeometryCount: usize,
    GetSourceGeometries: usize,
}
impl windows_core::RuntimeName for ID2D1GeometryGroup {}
windows_core::imp::define_interface!(
    ID2D1GradientStopCollection,
    ID2D1GradientStopCollection_Vtbl,
    0x2cd906a7_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1GradientStopCollection {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GradientStopCollection,
    windows_core::IUnknown,
    ID2D1Resource
);
#[repr(C)]
pub struct ID2D1GradientStopCollection_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetGradientStopCount: usize,
    GetGradientStops: usize,
    GetColorInterpolationGamma: usize,
    GetExtendMode: usize,
}
impl windows_core::RuntimeName for ID2D1GradientStopCollection {}
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
    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
        unsafe {
            (windows_core::Interface::vtable(self).CheckWindowState)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn Resize(&self, pixelsize: *const D2D_SIZE_U) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Resize)(
                windows_core::Interface::as_raw(self),
                pixelsize,
            )
        }
    }
    pub unsafe fn GetHwnd(&self) -> HWND {
        unsafe {
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CheckWindowState: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_WINDOW_STATE,
    pub Resize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_U,
    ) -> windows_core::HRESULT,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> HWND,
}
pub trait ID2D1HwndRenderTarget_Impl: ID2D1RenderTarget_Impl {
    fn CheckWindowState(&self) -> D2D1_WINDOW_STATE;
    fn Resize(&self, pixelsize: *const D2D_SIZE_U) -> windows_core::Result<()>;
    fn GetHwnd(&self) -> HWND;
}
impl ID2D1HwndRenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1HwndRenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckWindowState<
            Identity: ID2D1HwndRenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D2D1_WINDOW_STATE {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::CheckWindowState(this)
            }
        }
        unsafe extern "system" fn Resize<
            Identity: ID2D1HwndRenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pixelsize: *const D2D_SIZE_U,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::Resize(this, core::mem::transmute_copy(&pixelsize))
                    .into()
            }
        }
        unsafe extern "system" fn GetHwnd<
            Identity: ID2D1HwndRenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> HWND {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1HwndRenderTarget_Impl::GetHwnd(this)
            }
        }
        Self {
            base__: ID2D1RenderTarget_Vtbl::new::<Identity, OFFSET>(),
            CheckWindowState: CheckWindowState::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            GetHwnd: GetHwnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1HwndRenderTarget as windows_core::Interface>::IID
            || iid == &<ID2D1Resource as windows_core::Interface>::IID
            || iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Image,
    ID2D1Image_Vtbl,
    0x65019f75_8da2_497c_b32c_dfa34e48ede6
);
impl core::ops::Deref for ID2D1Image {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
pub trait ID2D1Image_Impl: ID2D1Resource_Impl {}
impl ID2D1Image_Vtbl {
    pub const fn new<Identity: ID2D1Image_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Image as windows_core::Interface>::IID
            || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Image {}
windows_core::imp::define_interface!(
    ID2D1Layer,
    ID2D1Layer_Vtbl,
    0x2cd9069b_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Layer {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Layer, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Layer_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetSize: usize,
}
impl windows_core::RuntimeName for ID2D1Layer {}
windows_core::imp::define_interface!(
    ID2D1LinearGradientBrush,
    ID2D1LinearGradientBrush_Vtbl,
    0x2cd906ab_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1LinearGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1LinearGradientBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1LinearGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetStartPoint: usize,
    SetEndPoint: usize,
    GetStartPoint: usize,
    GetEndPoint: usize,
    GetGradientStopCollection: usize,
}
impl windows_core::RuntimeName for ID2D1LinearGradientBrush {}
windows_core::imp::define_interface!(
    ID2D1Mesh,
    ID2D1Mesh_Vtbl,
    0x2cd906c2_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Mesh {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Mesh, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Mesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    Open: usize,
}
impl windows_core::RuntimeName for ID2D1Mesh {}
windows_core::imp::define_interface!(
    ID2D1PathGeometry,
    ID2D1PathGeometry_Vtbl,
    0x2cd906a5_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1PathGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1PathGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    Open: usize,
    Stream: usize,
    GetSegmentCount: usize,
    GetFigureCount: usize,
}
impl windows_core::RuntimeName for ID2D1PathGeometry {}
windows_core::imp::define_interface!(
    ID2D1RadialGradientBrush,
    ID2D1RadialGradientBrush_Vtbl,
    0x2cd906ac_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RadialGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RadialGradientBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1RadialGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetCenter: usize,
    SetGradientOriginOffset: usize,
    SetRadiusX: usize,
    SetRadiusY: usize,
    GetCenter: usize,
    GetGradientOriginOffset: usize,
    GetRadiusX: usize,
    GetRadiusY: usize,
    GetGradientStopCollection: usize,
}
impl windows_core::RuntimeName for ID2D1RadialGradientBrush {}
windows_core::imp::define_interface!(
    ID2D1RectangleGeometry,
    ID2D1RectangleGeometry_Vtbl,
    0x2cd906a2_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RectangleGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1RectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetRect: usize,
}
impl windows_core::RuntimeName for ID2D1RectangleGeometry {}
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
    pub unsafe fn CreateBitmap(
        &self,
        size: D2D_SIZE_U,
        srcdata: Option<*const core::ffi::c_void>,
        pitch: u32,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
    ) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(
                windows_core::Interface::as_raw(self),
                size,
                srcdata.unwrap_or(core::mem::zeroed()) as _,
                pitch,
                bitmapproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(
        &self,
        wicbitmapsource: P0,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>,
    ) -> windows_core::Result<ID2D1Bitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(
                windows_core::Interface::as_raw(self),
                wicbitmapsource.param().abi(),
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSharedBitmap(
        &self,
        riid: *const windows_core::GUID,
        data: *mut core::ffi::c_void,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>,
        bitmap: *mut Option<ID2D1Bitmap>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSharedBitmap)(
                windows_core::Interface::as_raw(self),
                riid,
                data as _,
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(bitmap),
            )
        }
    }
    pub unsafe fn CreateBitmapBrush<P0>(
        &self,
        bitmap: P0,
        bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1BitmapBrush>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapBrush)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                bitmapbrushproperties.unwrap_or(core::mem::zeroed()) as _,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
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
    pub unsafe fn CreateGradientStopCollection(
        &self,
        gradientstops: &[D2D1_GRADIENT_STOP],
        colorinterpolationgamma: D2D1_GAMMA,
        extendmode: D2D1_EXTEND_MODE,
    ) -> windows_core::Result<ID2D1GradientStopCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(
                windows_core::Interface::as_raw(self),
                gradientstops.as_ptr(),
                gradientstops.len().try_into().unwrap(),
                colorinterpolationgamma,
                extendmode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLinearGradientBrush<P2>(
        &self,
        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
        gradientstopcollection: P2,
    ) -> windows_core::Result<ID2D1LinearGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(
                windows_core::Interface::as_raw(self),
                lineargradientbrushproperties,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                gradientstopcollection.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRadialGradientBrush<P2>(
        &self,
        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
        gradientstopcollection: P2,
    ) -> windows_core::Result<ID2D1RadialGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(
                windows_core::Interface::as_raw(self),
                radialgradientbrushproperties,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                gradientstopcollection.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCompatibleRenderTarget(
        &self,
        desiredsize: Option<*const D2D_SIZE_F>,
        desiredpixelsize: Option<*const D2D_SIZE_U>,
        desiredformat: Option<*const D2D1_PIXEL_FORMAT>,
        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
    ) -> windows_core::Result<ID2D1BitmapRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompatibleRenderTarget)(
                windows_core::Interface::as_raw(self),
                desiredsize.unwrap_or(core::mem::zeroed()) as _,
                desiredpixelsize.unwrap_or(core::mem::zeroed()) as _,
                desiredformat.unwrap_or(core::mem::zeroed()) as _,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLayer(
        &self,
        size: Option<*const D2D_SIZE_F>,
    ) -> windows_core::Result<ID2D1Layer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLayer)(
                windows_core::Interface::as_raw(self),
                size.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMesh)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawLine<P2, P4>(
        &self,
        point0: windows_numerics::Vector2,
        point1: windows_numerics::Vector2,
        brush: P2,
        strokewidth: f32,
        strokestyle: P4,
    ) where
        P2: windows_core::Param<ID2D1Brush>,
        P4: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawLine)(
                windows_core::Interface::as_raw(self),
                point0,
                point1,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn DrawRectangle<P1, P3>(
        &self,
        rect: *const D2D_RECT_F,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRectangle)(
                windows_core::Interface::as_raw(self),
                rect,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillRectangle<P1>(&self, rect: *const D2D_RECT_F, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRectangle)(
                windows_core::Interface::as_raw(self),
                rect,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawRoundedRectangle<P1, P3>(
        &self,
        roundedrect: *const D2D1_ROUNDED_RECT,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRoundedRectangle)(
                windows_core::Interface::as_raw(self),
                roundedrect,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillRoundedRectangle<P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRoundedRectangle)(
                windows_core::Interface::as_raw(self),
                roundedrect,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawEllipse<P1, P3>(
        &self,
        ellipse: *const D2D1_ELLIPSE,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawEllipse)(
                windows_core::Interface::as_raw(self),
                ellipse,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillEllipse<P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillEllipse)(
                windows_core::Interface::as_raw(self),
                ellipse,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawGeometry<P0, P1, P3>(
        &self,
        geometry: P0,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGeometry)(
                windows_core::Interface::as_raw(self),
                geometry.param().abi(),
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillGeometry)(
                windows_core::Interface::as_raw(self),
                geometry.param().abi(),
                brush.param().abi(),
                opacitybrush.param().abi(),
            );
        }
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1Mesh>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillMesh)(
                windows_core::Interface::as_raw(self),
                mesh.param().abi(),
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn FillOpacityMask<P0, P1>(
        &self,
        opacitymask: P0,
        brush: P1,
        content: D2D1_OPACITY_MASK_CONTENT,
        destinationrectangle: Option<*const D2D_RECT_F>,
        sourcerectangle: Option<*const D2D_RECT_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(
                windows_core::Interface::as_raw(self),
                opacitymask.param().abi(),
                brush.param().abi(),
                content,
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawBitmap<P0>(
        &self,
        bitmap: P0,
        destinationrectangle: Option<*const D2D_RECT_F>,
        opacity: f32,
        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
        sourcerectangle: Option<*const D2D_RECT_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                opacity,
                interpolationmode,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawText<P2, P4>(
        &self,
        string: &[u16],
        textformat: P2,
        layoutrect: *const D2D_RECT_F,
        defaultfillbrush: P4,
        options: D2D1_DRAW_TEXT_OPTIONS,
        measuringmode: DWRITE_MEASURING_MODE,
    ) where
        P2: windows_core::Param<IDWriteTextFormat>,
        P4: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawText)(
                windows_core::Interface::as_raw(self),
                string.as_ptr(),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                layoutrect,
                defaultfillbrush.param().abi(),
                options,
                measuringmode,
            );
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
    pub unsafe fn DrawGlyphRun<P2>(
        &self,
        baselineorigin: windows_numerics::Vector2,
        glyphrun: *const DWRITE_GLYPH_RUN,
        foregroundbrush: P2,
        measuringmode: DWRITE_MEASURING_MODE,
    ) where
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRun)(
                windows_core::Interface::as_raw(self),
                baselineorigin,
                glyphrun,
                foregroundbrush.param().abi(),
                measuringmode,
            );
        }
    }
    pub unsafe fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(
                windows_core::Interface::as_raw(self),
                transform,
            );
        }
    }
    pub unsafe fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(
                windows_core::Interface::as_raw(self),
                transform as _,
            );
        }
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetAntialiasMode)(
                windows_core::Interface::as_raw(self),
                antialiasmode,
            );
        }
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        unsafe {
            (windows_core::Interface::vtable(self).GetAntialiasMode)(
                windows_core::Interface::as_raw(self),
            )
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
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        unsafe {
            (windows_core::Interface::vtable(self).GetTextAntialiasMode)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextRenderingParams)(
                windows_core::Interface::as_raw(self),
                textrenderingparams.param().abi(),
            );
        }
    }
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextRenderingParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTags(&self, tag1: D2D1_TAG, tag2: D2D1_TAG) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTags)(
                windows_core::Interface::as_raw(self),
                tag1,
                tag2,
            );
        }
    }
    pub unsafe fn GetTags(&self, tag1: Option<*mut D2D1_TAG>, tag2: Option<*mut D2D1_TAG>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTags)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P1)
    where
        P1: windows_core::Param<ID2D1Layer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PushLayer)(
                windows_core::Interface::as_raw(self),
                layerparameters,
                layer.param().abi(),
            );
        }
    }
    pub unsafe fn PopLayer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(
        &self,
        tag1: Option<*mut D2D1_TAG>,
        tag2: Option<*mut D2D1_TAG>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn SaveDrawingState(&self, drawingstateblock: &Option<ID2D1DrawingStateBlock>) {
        unsafe {
            (windows_core::Interface::vtable(self).SaveDrawingState)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(drawingstateblock),
            );
        }
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RestoreDrawingState)(
                windows_core::Interface::as_raw(self),
                drawingstateblock.param().abi(),
            );
        }
    }
    pub unsafe fn PushAxisAlignedClip(
        &self,
        cliprect: *const D2D_RECT_F,
        antialiasmode: D2D1_ANTIALIAS_MODE,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PushAxisAlignedClip)(
                windows_core::Interface::as_raw(self),
                cliprect,
                antialiasmode,
            );
        }
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopAxisAlignedClip)(
                windows_core::Interface::as_raw(self),
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
    pub unsafe fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDpi)(
                windows_core::Interface::as_raw(self),
                dpix,
                dpiy,
            );
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(
                windows_core::Interface::as_raw(self),
                dpix as _,
                dpiy as _,
            );
        }
    }
    pub unsafe fn GetSize(&self) -> D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
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
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaximumBitmapSize)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn IsSupported(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsSupported)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
            )
        }
    }
}
#[repr(C)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_SIZE_U,
        *const core::ffi::c_void,
        u32,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSharedBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_COLOR_F,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGradientStopCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_GRADIENT_STOP,
        u32,
        D2D1_GAMMA,
        D2D1_EXTEND_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_F,
        *const D2D_SIZE_U,
        *const D2D1_PIXEL_FORMAT,
        D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_F,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMesh: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawLine: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        windows_numerics::Vector2,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub DrawRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
    ),
    pub DrawRoundedRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillRoundedRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut core::ffi::c_void,
    ),
    pub DrawEllipse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillEllipse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut core::ffi::c_void,
    ),
    pub DrawGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub FillMesh: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub FillOpacityMask: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_OPACITY_MASK_CONTENT,
        *const D2D_RECT_F,
        *const D2D_RECT_F,
    ),
    pub DrawBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        f32,
        D2D1_BITMAP_INTERPOLATION_MODE,
        *const D2D_RECT_F,
    ),
    pub DrawText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const u16,
        u32,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
        D2D1_DRAW_TEXT_OPTIONS,
        DWRITE_MEASURING_MODE,
    ),
    pub DrawTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_DRAW_TEXT_OPTIONS,
    ),
    pub DrawGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_numerics::Vector2,
        *const DWRITE_GLYPH_RUN,
        *mut core::ffi::c_void,
        DWRITE_MEASURING_MODE,
    ),
    pub SetTransform:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_numerics::Matrix3x2),
    pub GetTransform:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix3x2),
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
    pub SetTextRenderingParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTextRenderingParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TAG, D2D1_TAG),
    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_TAG, *mut D2D1_TAG),
    pub PushLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_LAYER_PARAMETERS,
        *mut core::ffi::c_void,
    ),
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D2D1_TAG,
        *mut D2D1_TAG,
    ) -> windows_core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RestoreDrawingState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PushAxisAlignedClip:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_RECT_F, D2D1_ANTIALIAS_MODE),
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_COLOR_F),
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut D2D1_TAG,
        *mut D2D1_TAG,
    ) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_PIXEL_FORMAT),
    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_F),
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_U),
    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::BOOL,
}
pub trait ID2D1RenderTarget_Impl: ID2D1Resource_Impl {
    fn CreateBitmap(
        &self,
        size: &D2D_SIZE_U,
        srcdata: *const core::ffi::c_void,
        pitch: u32,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
    ) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateBitmapFromWicBitmap(
        &self,
        wicbitmapsource: windows_core::Ref<IWICBitmapSource>,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
    ) -> windows_core::Result<ID2D1Bitmap>;
    fn CreateSharedBitmap(
        &self,
        riid: *const windows_core::GUID,
        data: *mut core::ffi::c_void,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
        bitmap: windows_core::OutRef<ID2D1Bitmap>,
    ) -> windows_core::Result<()>;
    fn CreateBitmapBrush(
        &self,
        bitmap: windows_core::Ref<ID2D1Bitmap>,
        bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
        brushproperties: *const D2D1_BRUSH_PROPERTIES,
    ) -> windows_core::Result<ID2D1BitmapBrush>;
    fn CreateSolidColorBrush(
        &self,
        color: *const D2D_COLOR_F,
        brushproperties: *const D2D1_BRUSH_PROPERTIES,
    ) -> windows_core::Result<ID2D1SolidColorBrush>;
    fn CreateGradientStopCollection(
        &self,
        gradientstops: *const D2D1_GRADIENT_STOP,
        gradientstopscount: u32,
        colorinterpolationgamma: D2D1_GAMMA,
        extendmode: D2D1_EXTEND_MODE,
    ) -> windows_core::Result<ID2D1GradientStopCollection>;
    fn CreateLinearGradientBrush(
        &self,
        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: *const D2D1_BRUSH_PROPERTIES,
        gradientstopcollection: windows_core::Ref<ID2D1GradientStopCollection>,
    ) -> windows_core::Result<ID2D1LinearGradientBrush>;
    fn CreateRadialGradientBrush(
        &self,
        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: *const D2D1_BRUSH_PROPERTIES,
        gradientstopcollection: windows_core::Ref<ID2D1GradientStopCollection>,
    ) -> windows_core::Result<ID2D1RadialGradientBrush>;
    fn CreateCompatibleRenderTarget(
        &self,
        desiredsize: *const D2D_SIZE_F,
        desiredpixelsize: *const D2D_SIZE_U,
        desiredformat: *const D2D1_PIXEL_FORMAT,
        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
    ) -> windows_core::Result<ID2D1BitmapRenderTarget>;
    fn CreateLayer(&self, size: *const D2D_SIZE_F) -> windows_core::Result<ID2D1Layer>;
    fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh>;
    fn DrawLine(
        &self,
        point0: &windows_numerics::Vector2,
        point1: &windows_numerics::Vector2,
        brush: windows_core::Ref<ID2D1Brush>,
        strokewidth: f32,
        strokestyle: windows_core::Ref<ID2D1StrokeStyle>,
    );
    fn DrawRectangle(
        &self,
        rect: *const D2D_RECT_F,
        brush: windows_core::Ref<ID2D1Brush>,
        strokewidth: f32,
        strokestyle: windows_core::Ref<ID2D1StrokeStyle>,
    );
    fn FillRectangle(&self, rect: *const D2D_RECT_F, brush: windows_core::Ref<ID2D1Brush>);
    fn DrawRoundedRectangle(
        &self,
        roundedrect: *const D2D1_ROUNDED_RECT,
        brush: windows_core::Ref<ID2D1Brush>,
        strokewidth: f32,
        strokestyle: windows_core::Ref<ID2D1StrokeStyle>,
    );
    fn FillRoundedRectangle(
        &self,
        roundedrect: *const D2D1_ROUNDED_RECT,
        brush: windows_core::Ref<ID2D1Brush>,
    );
    fn DrawEllipse(
        &self,
        ellipse: *const D2D1_ELLIPSE,
        brush: windows_core::Ref<ID2D1Brush>,
        strokewidth: f32,
        strokestyle: windows_core::Ref<ID2D1StrokeStyle>,
    );
    fn FillEllipse(&self, ellipse: *const D2D1_ELLIPSE, brush: windows_core::Ref<ID2D1Brush>);
    fn DrawGeometry(
        &self,
        geometry: windows_core::Ref<ID2D1Geometry>,
        brush: windows_core::Ref<ID2D1Brush>,
        strokewidth: f32,
        strokestyle: windows_core::Ref<ID2D1StrokeStyle>,
    );
    fn FillGeometry(
        &self,
        geometry: windows_core::Ref<ID2D1Geometry>,
        brush: windows_core::Ref<ID2D1Brush>,
        opacitybrush: windows_core::Ref<ID2D1Brush>,
    );
    fn FillMesh(&self, mesh: windows_core::Ref<ID2D1Mesh>, brush: windows_core::Ref<ID2D1Brush>);
    fn FillOpacityMask(
        &self,
        opacitymask: windows_core::Ref<ID2D1Bitmap>,
        brush: windows_core::Ref<ID2D1Brush>,
        content: D2D1_OPACITY_MASK_CONTENT,
        destinationrectangle: *const D2D_RECT_F,
        sourcerectangle: *const D2D_RECT_F,
    );
    fn DrawBitmap(
        &self,
        bitmap: windows_core::Ref<ID2D1Bitmap>,
        destinationrectangle: *const D2D_RECT_F,
        opacity: f32,
        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
        sourcerectangle: *const D2D_RECT_F,
    );
    fn DrawText(
        &self,
        string: *const u16,
        stringlength: u32,
        textformat: windows_core::Ref<IDWriteTextFormat>,
        layoutrect: *const D2D_RECT_F,
        defaultfillbrush: windows_core::Ref<ID2D1Brush>,
        options: D2D1_DRAW_TEXT_OPTIONS,
        measuringmode: DWRITE_MEASURING_MODE,
    );
    fn DrawTextLayout(
        &self,
        origin: &windows_numerics::Vector2,
        textlayout: windows_core::Ref<IDWriteTextLayout>,
        defaultfillbrush: windows_core::Ref<ID2D1Brush>,
        options: D2D1_DRAW_TEXT_OPTIONS,
    );
    fn DrawGlyphRun(
        &self,
        baselineorigin: &windows_numerics::Vector2,
        glyphrun: *const DWRITE_GLYPH_RUN,
        foregroundbrush: windows_core::Ref<ID2D1Brush>,
        measuringmode: DWRITE_MEASURING_MODE,
    );
    fn SetTransform(&self, transform: *const windows_numerics::Matrix3x2);
    fn GetTransform(&self, transform: *mut windows_numerics::Matrix3x2);
    fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE);
    fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE;
    fn SetTextRenderingParams(
        &self,
        textrenderingparams: windows_core::Ref<IDWriteRenderingParams>,
    );
    fn GetTextRenderingParams(
        &self,
        textrenderingparams: windows_core::OutRef<IDWriteRenderingParams>,
    );
    fn SetTags(&self, tag1: D2D1_TAG, tag2: D2D1_TAG);
    fn GetTags(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG);
    fn PushLayer(
        &self,
        layerparameters: *const D2D1_LAYER_PARAMETERS,
        layer: windows_core::Ref<ID2D1Layer>,
    );
    fn PopLayer(&self);
    fn Flush(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::Result<()>;
    fn SaveDrawingState(&self, drawingstateblock: windows_core::OutRef<ID2D1DrawingStateBlock>);
    fn RestoreDrawingState(&self, drawingstateblock: windows_core::Ref<ID2D1DrawingStateBlock>);
    fn PushAxisAlignedClip(&self, cliprect: *const D2D_RECT_F, antialiasmode: D2D1_ANTIALIAS_MODE);
    fn PopAxisAlignedClip(&self);
    fn Clear(&self, clearcolor: *const D2D_COLOR_F);
    fn BeginDraw(&self);
    fn EndDraw(&self, tag1: *mut D2D1_TAG, tag2: *mut D2D1_TAG) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT;
    fn SetDpi(&self, dpix: f32, dpiy: f32);
    fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn GetSize(&self) -> D2D_SIZE_F;
    fn GetPixelSize(&self) -> D2D_SIZE_U;
    fn GetMaximumBitmapSize(&self) -> u32;
    fn IsSupported(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::BOOL;
}
impl ID2D1RenderTarget_Vtbl {
    pub const fn new<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBitmap<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            size: D2D_SIZE_U,
            srcdata: *const core::ffi::c_void,
            pitch: u32,
            bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
            bitmap: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmap(
                    this,
                    core::mem::transmute(&size),
                    core::mem::transmute_copy(&srcdata),
                    core::mem::transmute_copy(&pitch),
                    core::mem::transmute_copy(&bitmapproperties),
                ) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBitmapFromWicBitmap<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            wicbitmapsource: *mut core::ffi::c_void,
            bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
            bitmap: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmapFromWicBitmap(
                    this,
                    core::mem::transmute_copy(&wicbitmapsource),
                    core::mem::transmute_copy(&bitmapproperties),
                ) {
                    Ok(ok__) => {
                        bitmap.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSharedBitmap<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            data: *mut core::ffi::c_void,
            bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
            bitmap: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::CreateSharedBitmap(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&data),
                    core::mem::transmute_copy(&bitmapproperties),
                    core::mem::transmute_copy(&bitmap),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateBitmapBrush<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bitmap: *mut core::ffi::c_void,
            bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
            brushproperties: *const D2D1_BRUSH_PROPERTIES,
            bitmapbrush: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateBitmapBrush(
                    this,
                    core::mem::transmute_copy(&bitmap),
                    core::mem::transmute_copy(&bitmapbrushproperties),
                    core::mem::transmute_copy(&brushproperties),
                ) {
                    Ok(ok__) => {
                        bitmapbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSolidColorBrush<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            color: *const D2D_COLOR_F,
            brushproperties: *const D2D1_BRUSH_PROPERTIES,
            solidcolorbrush: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateSolidColorBrush(
                    this,
                    core::mem::transmute_copy(&color),
                    core::mem::transmute_copy(&brushproperties),
                ) {
                    Ok(ok__) => {
                        solidcolorbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGradientStopCollection<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            gradientstops: *const D2D1_GRADIENT_STOP,
            gradientstopscount: u32,
            colorinterpolationgamma: D2D1_GAMMA,
            extendmode: D2D1_EXTEND_MODE,
            gradientstopcollection: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateGradientStopCollection(
                    this,
                    core::mem::transmute_copy(&gradientstops),
                    core::mem::transmute_copy(&gradientstopscount),
                    core::mem::transmute_copy(&colorinterpolationgamma),
                    core::mem::transmute_copy(&extendmode),
                ) {
                    Ok(ok__) => {
                        gradientstopcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLinearGradientBrush<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
            brushproperties: *const D2D1_BRUSH_PROPERTIES,
            gradientstopcollection: *mut core::ffi::c_void,
            lineargradientbrush: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateLinearGradientBrush(
                    this,
                    core::mem::transmute_copy(&lineargradientbrushproperties),
                    core::mem::transmute_copy(&brushproperties),
                    core::mem::transmute_copy(&gradientstopcollection),
                ) {
                    Ok(ok__) => {
                        lineargradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRadialGradientBrush<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
            brushproperties: *const D2D1_BRUSH_PROPERTIES,
            gradientstopcollection: *mut core::ffi::c_void,
            radialgradientbrush: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateRadialGradientBrush(
                    this,
                    core::mem::transmute_copy(&radialgradientbrushproperties),
                    core::mem::transmute_copy(&brushproperties),
                    core::mem::transmute_copy(&gradientstopcollection),
                ) {
                    Ok(ok__) => {
                        radialgradientbrush.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCompatibleRenderTarget<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            desiredsize: *const D2D_SIZE_F,
            desiredpixelsize: *const D2D_SIZE_U,
            desiredformat: *const D2D1_PIXEL_FORMAT,
            options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
            bitmaprendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateCompatibleRenderTarget(
                    this,
                    core::mem::transmute_copy(&desiredsize),
                    core::mem::transmute_copy(&desiredpixelsize),
                    core::mem::transmute_copy(&desiredformat),
                    core::mem::transmute_copy(&options),
                ) {
                    Ok(ok__) => {
                        bitmaprendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateLayer<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            size: *const D2D_SIZE_F,
            layer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateLayer(this, core::mem::transmute_copy(&size)) {
                    Ok(ok__) => {
                        layer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMesh<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            mesh: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1RenderTarget_Impl::CreateMesh(this) {
                    Ok(ok__) => {
                        mesh.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DrawLine<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            point0: windows_numerics::Vector2,
            point1: windows_numerics::Vector2,
            brush: *mut core::ffi::c_void,
            strokewidth: f32,
            strokestyle: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawLine(
                    this,
                    core::mem::transmute(&point0),
                    core::mem::transmute(&point1),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&strokewidth),
                    core::mem::transmute_copy(&strokestyle),
                );
            }
        }
        unsafe extern "system" fn DrawRectangle<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rect: *const D2D_RECT_F,
            brush: *mut core::ffi::c_void,
            strokewidth: f32,
            strokestyle: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawRectangle(
                    this,
                    core::mem::transmute_copy(&rect),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&strokewidth),
                    core::mem::transmute_copy(&strokestyle),
                );
            }
        }
        unsafe extern "system" fn FillRectangle<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rect: *const D2D_RECT_F,
            brush: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillRectangle(
                    this,
                    core::mem::transmute_copy(&rect),
                    core::mem::transmute_copy(&brush),
                );
            }
        }
        unsafe extern "system" fn DrawRoundedRectangle<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            roundedrect: *const D2D1_ROUNDED_RECT,
            brush: *mut core::ffi::c_void,
            strokewidth: f32,
            strokestyle: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawRoundedRectangle(
                    this,
                    core::mem::transmute_copy(&roundedrect),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&strokewidth),
                    core::mem::transmute_copy(&strokestyle),
                );
            }
        }
        unsafe extern "system" fn FillRoundedRectangle<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            roundedrect: *const D2D1_ROUNDED_RECT,
            brush: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillRoundedRectangle(
                    this,
                    core::mem::transmute_copy(&roundedrect),
                    core::mem::transmute_copy(&brush),
                );
            }
        }
        unsafe extern "system" fn DrawEllipse<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ellipse: *const D2D1_ELLIPSE,
            brush: *mut core::ffi::c_void,
            strokewidth: f32,
            strokestyle: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawEllipse(
                    this,
                    core::mem::transmute_copy(&ellipse),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&strokewidth),
                    core::mem::transmute_copy(&strokestyle),
                );
            }
        }
        unsafe extern "system" fn FillEllipse<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ellipse: *const D2D1_ELLIPSE,
            brush: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillEllipse(
                    this,
                    core::mem::transmute_copy(&ellipse),
                    core::mem::transmute_copy(&brush),
                );
            }
        }
        unsafe extern "system" fn DrawGeometry<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            geometry: *mut core::ffi::c_void,
            brush: *mut core::ffi::c_void,
            strokewidth: f32,
            strokestyle: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawGeometry(
                    this,
                    core::mem::transmute_copy(&geometry),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&strokewidth),
                    core::mem::transmute_copy(&strokestyle),
                );
            }
        }
        unsafe extern "system" fn FillGeometry<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            geometry: *mut core::ffi::c_void,
            brush: *mut core::ffi::c_void,
            opacitybrush: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillGeometry(
                    this,
                    core::mem::transmute_copy(&geometry),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&opacitybrush),
                );
            }
        }
        unsafe extern "system" fn FillMesh<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            mesh: *mut core::ffi::c_void,
            brush: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillMesh(
                    this,
                    core::mem::transmute_copy(&mesh),
                    core::mem::transmute_copy(&brush),
                );
            }
        }
        unsafe extern "system" fn FillOpacityMask<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            opacitymask: *mut core::ffi::c_void,
            brush: *mut core::ffi::c_void,
            content: D2D1_OPACITY_MASK_CONTENT,
            destinationrectangle: *const D2D_RECT_F,
            sourcerectangle: *const D2D_RECT_F,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::FillOpacityMask(
                    this,
                    core::mem::transmute_copy(&opacitymask),
                    core::mem::transmute_copy(&brush),
                    core::mem::transmute_copy(&content),
                    core::mem::transmute_copy(&destinationrectangle),
                    core::mem::transmute_copy(&sourcerectangle),
                );
            }
        }
        unsafe extern "system" fn DrawBitmap<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bitmap: *mut core::ffi::c_void,
            destinationrectangle: *const D2D_RECT_F,
            opacity: f32,
            interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
            sourcerectangle: *const D2D_RECT_F,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawBitmap(
                    this,
                    core::mem::transmute_copy(&bitmap),
                    core::mem::transmute_copy(&destinationrectangle),
                    core::mem::transmute_copy(&opacity),
                    core::mem::transmute_copy(&interpolationmode),
                    core::mem::transmute_copy(&sourcerectangle),
                );
            }
        }
        unsafe extern "system" fn DrawText<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            string: *const u16,
            stringlength: u32,
            textformat: *mut core::ffi::c_void,
            layoutrect: *const D2D_RECT_F,
            defaultfillbrush: *mut core::ffi::c_void,
            options: D2D1_DRAW_TEXT_OPTIONS,
            measuringmode: DWRITE_MEASURING_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawText(
                    this,
                    core::mem::transmute_copy(&string),
                    core::mem::transmute_copy(&stringlength),
                    core::mem::transmute_copy(&textformat),
                    core::mem::transmute_copy(&layoutrect),
                    core::mem::transmute_copy(&defaultfillbrush),
                    core::mem::transmute_copy(&options),
                    core::mem::transmute_copy(&measuringmode),
                );
            }
        }
        unsafe extern "system" fn DrawTextLayout<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            origin: windows_numerics::Vector2,
            textlayout: *mut core::ffi::c_void,
            defaultfillbrush: *mut core::ffi::c_void,
            options: D2D1_DRAW_TEXT_OPTIONS,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawTextLayout(
                    this,
                    core::mem::transmute(&origin),
                    core::mem::transmute_copy(&textlayout),
                    core::mem::transmute_copy(&defaultfillbrush),
                    core::mem::transmute_copy(&options),
                );
            }
        }
        unsafe extern "system" fn DrawGlyphRun<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            baselineorigin: windows_numerics::Vector2,
            glyphrun: *const DWRITE_GLYPH_RUN,
            foregroundbrush: *mut core::ffi::c_void,
            measuringmode: DWRITE_MEASURING_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::DrawGlyphRun(
                    this,
                    core::mem::transmute(&baselineorigin),
                    core::mem::transmute_copy(&glyphrun),
                    core::mem::transmute_copy(&foregroundbrush),
                    core::mem::transmute_copy(&measuringmode),
                );
            }
        }
        unsafe extern "system" fn SetTransform<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            transform: *const windows_numerics::Matrix3x2,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn GetTransform<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            transform: *mut windows_numerics::Matrix3x2,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn SetAntialiasMode<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            antialiasmode: D2D1_ANTIALIAS_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetAntialiasMode(
                    this,
                    core::mem::transmute_copy(&antialiasmode),
                );
            }
        }
        unsafe extern "system" fn GetAntialiasMode<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D2D1_ANTIALIAS_MODE {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetAntialiasMode(this)
            }
        }
        unsafe extern "system" fn SetTextAntialiasMode<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTextAntialiasMode(
                    this,
                    core::mem::transmute_copy(&textantialiasmode),
                );
            }
        }
        unsafe extern "system" fn GetTextAntialiasMode<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D2D1_TEXT_ANTIALIAS_MODE {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTextAntialiasMode(this)
            }
        }
        unsafe extern "system" fn SetTextRenderingParams<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textrenderingparams: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTextRenderingParams(
                    this,
                    core::mem::transmute_copy(&textrenderingparams),
                );
            }
        }
        unsafe extern "system" fn GetTextRenderingParams<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textrenderingparams: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTextRenderingParams(
                    this,
                    core::mem::transmute_copy(&textrenderingparams),
                );
            }
        }
        unsafe extern "system" fn SetTags<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            tag1: D2D1_TAG,
            tag2: D2D1_TAG,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetTags(
                    this,
                    core::mem::transmute_copy(&tag1),
                    core::mem::transmute_copy(&tag2),
                );
            }
        }
        unsafe extern "system" fn GetTags<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            tag1: *mut D2D1_TAG,
            tag2: *mut D2D1_TAG,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetTags(
                    this,
                    core::mem::transmute_copy(&tag1),
                    core::mem::transmute_copy(&tag2),
                );
            }
        }
        unsafe extern "system" fn PushLayer<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            layerparameters: *const D2D1_LAYER_PARAMETERS,
            layer: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PushLayer(
                    this,
                    core::mem::transmute_copy(&layerparameters),
                    core::mem::transmute_copy(&layer),
                );
            }
        }
        unsafe extern "system" fn PopLayer<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PopLayer(this);
            }
        }
        unsafe extern "system" fn Flush<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            tag1: *mut D2D1_TAG,
            tag2: *mut D2D1_TAG,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::Flush(
                    this,
                    core::mem::transmute_copy(&tag1),
                    core::mem::transmute_copy(&tag2),
                )
                .into()
            }
        }
        unsafe extern "system" fn SaveDrawingState<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            drawingstateblock: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SaveDrawingState(
                    this,
                    core::mem::transmute(&drawingstateblock),
                );
            }
        }
        unsafe extern "system" fn RestoreDrawingState<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            drawingstateblock: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::RestoreDrawingState(
                    this,
                    core::mem::transmute_copy(&drawingstateblock),
                );
            }
        }
        unsafe extern "system" fn PushAxisAlignedClip<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cliprect: *const D2D_RECT_F,
            antialiasmode: D2D1_ANTIALIAS_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PushAxisAlignedClip(
                    this,
                    core::mem::transmute_copy(&cliprect),
                    core::mem::transmute_copy(&antialiasmode),
                );
            }
        }
        unsafe extern "system" fn PopAxisAlignedClip<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::PopAxisAlignedClip(this);
            }
        }
        unsafe extern "system" fn Clear<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            clearcolor: *const D2D_COLOR_F,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::Clear(this, core::mem::transmute_copy(&clearcolor));
            }
        }
        unsafe extern "system" fn BeginDraw<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::BeginDraw(this);
            }
        }
        unsafe extern "system" fn EndDraw<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            tag1: *mut D2D1_TAG,
            tag2: *mut D2D1_TAG,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::EndDraw(
                    this,
                    core::mem::transmute_copy(&tag1),
                    core::mem::transmute_copy(&tag2),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut D2D1_PIXEL_FORMAT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetPixelFormat(this);
            }
        }
        unsafe extern "system" fn SetDpi<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dpix: f32,
            dpiy: f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::SetDpi(
                    this,
                    core::mem::transmute_copy(&dpix),
                    core::mem::transmute_copy(&dpiy),
                );
            }
        }
        unsafe extern "system" fn GetDpi<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dpix: *mut f32,
            dpiy: *mut f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetDpi(
                    this,
                    core::mem::transmute_copy(&dpix),
                    core::mem::transmute_copy(&dpiy),
                );
            }
        }
        unsafe extern "system" fn GetSize<Identity: ID2D1RenderTarget_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut D2D_SIZE_F,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetSize(this);
            }
        }
        unsafe extern "system" fn GetPixelSize<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut D2D_SIZE_U,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = ID2D1RenderTarget_Impl::GetPixelSize(this);
            }
        }
        unsafe extern "system" fn GetMaximumBitmapSize<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::GetMaximumBitmapSize(this)
            }
        }
        unsafe extern "system" fn IsSupported<
            Identity: ID2D1RenderTarget_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1RenderTarget_Impl::IsSupported(
                    this,
                    core::mem::transmute_copy(&rendertargetproperties),
                )
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            CreateBitmap: CreateBitmap::<Identity, OFFSET>,
            CreateBitmapFromWicBitmap: CreateBitmapFromWicBitmap::<Identity, OFFSET>,
            CreateSharedBitmap: CreateSharedBitmap::<Identity, OFFSET>,
            CreateBitmapBrush: CreateBitmapBrush::<Identity, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, OFFSET>,
            CreateGradientStopCollection: CreateGradientStopCollection::<Identity, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, OFFSET>,
            CreateCompatibleRenderTarget: CreateCompatibleRenderTarget::<Identity, OFFSET>,
            CreateLayer: CreateLayer::<Identity, OFFSET>,
            CreateMesh: CreateMesh::<Identity, OFFSET>,
            DrawLine: DrawLine::<Identity, OFFSET>,
            DrawRectangle: DrawRectangle::<Identity, OFFSET>,
            FillRectangle: FillRectangle::<Identity, OFFSET>,
            DrawRoundedRectangle: DrawRoundedRectangle::<Identity, OFFSET>,
            FillRoundedRectangle: FillRoundedRectangle::<Identity, OFFSET>,
            DrawEllipse: DrawEllipse::<Identity, OFFSET>,
            FillEllipse: FillEllipse::<Identity, OFFSET>,
            DrawGeometry: DrawGeometry::<Identity, OFFSET>,
            FillGeometry: FillGeometry::<Identity, OFFSET>,
            FillMesh: FillMesh::<Identity, OFFSET>,
            FillOpacityMask: FillOpacityMask::<Identity, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, OFFSET>,
            DrawText: DrawText::<Identity, OFFSET>,
            DrawTextLayout: DrawTextLayout::<Identity, OFFSET>,
            DrawGlyphRun: DrawGlyphRun::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
            SetAntialiasMode: SetAntialiasMode::<Identity, OFFSET>,
            GetAntialiasMode: GetAntialiasMode::<Identity, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, OFFSET>,
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, OFFSET>,
            SetTextRenderingParams: SetTextRenderingParams::<Identity, OFFSET>,
            GetTextRenderingParams: GetTextRenderingParams::<Identity, OFFSET>,
            SetTags: SetTags::<Identity, OFFSET>,
            GetTags: GetTags::<Identity, OFFSET>,
            PushLayer: PushLayer::<Identity, OFFSET>,
            PopLayer: PopLayer::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            SaveDrawingState: SaveDrawingState::<Identity, OFFSET>,
            RestoreDrawingState: RestoreDrawingState::<Identity, OFFSET>,
            PushAxisAlignedClip: PushAxisAlignedClip::<Identity, OFFSET>,
            PopAxisAlignedClip: PopAxisAlignedClip::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            BeginDraw: BeginDraw::<Identity, OFFSET>,
            EndDraw: EndDraw::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
            SetDpi: SetDpi::<Identity, OFFSET>,
            GetDpi: GetDpi::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetPixelSize: GetPixelSize::<Identity, OFFSET>,
            GetMaximumBitmapSize: GetMaximumBitmapSize::<Identity, OFFSET>,
            IsSupported: IsSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1RenderTarget as windows_core::Interface>::IID
            || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
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
    ID2D1RoundedRectangleGeometry,
    ID2D1RoundedRectangleGeometry_Vtbl,
    0x2cd906a3_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RoundedRectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RoundedRectangleGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1RoundedRectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetRoundedRect: usize,
}
impl windows_core::RuntimeName for ID2D1RoundedRectangleGeometry {}
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
windows_core::imp::define_interface!(
    ID2D1StrokeStyle,
    ID2D1StrokeStyle_Vtbl,
    0x2cd9069d_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1StrokeStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1StrokeStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetStartCap: usize,
    GetEndCap: usize,
    GetDashCap: usize,
    GetMiterLimit: usize,
    GetLineJoin: usize,
    GetDashOffset: usize,
    GetDashStyle: usize,
    GetDashesCount: usize,
    GetDashes: usize,
}
impl windows_core::RuntimeName for ID2D1StrokeStyle {}
windows_core::imp::define_interface!(
    ID2D1TransformedGeometry,
    ID2D1TransformedGeometry_Vtbl,
    0x2cd906bb_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1TransformedGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1TransformedGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1TransformedGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetSourceGeometry: usize,
    GetTransform: usize,
}
impl windows_core::RuntimeName for ID2D1TransformedGeometry {}
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
    pub unsafe fn GetSystemFontCollection(
        &self,
        fontcollection: *mut Option<IDWriteFontCollection>,
        checkforupdates: bool,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetSystemFontCollection)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(fontcollection),
                checkforupdates.into(),
            )
        }
    }
    pub unsafe fn CreateCustomFontCollection<P0>(
        &self,
        collectionloader: P0,
        collectionkey: *const core::ffi::c_void,
        collectionkeysize: u32,
    ) -> windows_core::Result<IDWriteFontCollection>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontCollection)(
                windows_core::Interface::as_raw(self),
                collectionloader.param().abi(),
                collectionkey,
                collectionkeysize,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(
        &self,
        fontcollectionloader: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RegisterFontCollectionLoader)(
                windows_core::Interface::as_raw(self),
                fontcollectionloader.param().abi(),
            )
        }
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(
        &self,
        fontcollectionloader: P0,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterFontCollectionLoader)(
                windows_core::Interface::as_raw(self),
                fontcollectionloader.param().abi(),
            )
        }
    }
    pub unsafe fn CreateFontFileReference<P0>(
        &self,
        filepath: P0,
        lastwritetime: Option<*const FILETIME>,
    ) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFileReference)(
                windows_core::Interface::as_raw(self),
                filepath.param().abi(),
                lastwritetime.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomFontFileReference<P2>(
        &self,
        fontfilereferencekey: *const core::ffi::c_void,
        fontfilereferencekeysize: u32,
        fontfileloader: P2,
    ) -> windows_core::Result<IDWriteFontFile>
    where
        P2: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontFileReference)(
                windows_core::Interface::as_raw(self),
                fontfilereferencekey,
                fontfilereferencekeysize,
                fontfileloader.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontFace(
        &self,
        fontfacetype: DWRITE_FONT_FACE_TYPE,
        fontfiles: &[Option<IDWriteFontFile>],
        faceindex: u32,
        fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
    ) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(
                windows_core::Interface::as_raw(self),
                fontfacetype,
                fontfiles.len().try_into().unwrap(),
                core::mem::transmute(fontfiles.as_ptr()),
                faceindex,
                fontfacesimulationflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRenderingParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMonitorRenderingParams(
        &self,
        monitor: HMONITOR,
    ) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMonitorRenderingParams)(
                windows_core::Interface::as_raw(self),
                monitor,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomRenderingParams(
        &self,
        gamma: f32,
        enhancedcontrast: f32,
        cleartypelevel: f32,
        pixelgeometry: DWRITE_PIXEL_GEOMETRY,
        renderingmode: DWRITE_RENDERING_MODE,
    ) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(
                windows_core::Interface::as_raw(self),
                gamma,
                enhancedcontrast,
                cleartypelevel,
                pixelgeometry,
                renderingmode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RegisterFontFileLoader)(
                windows_core::Interface::as_raw(self),
                fontfileloader.param().abi(),
            )
        }
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterFontFileLoader)(
                windows_core::Interface::as_raw(self),
                fontfileloader.param().abi(),
            )
        }
    }
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
    pub unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypography)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGdiInterop)(
                windows_core::Interface::as_raw(self),
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
    pub unsafe fn CreateGdiCompatibleTextLayout<P2>(
        &self,
        string: &[u16],
        textformat: P2,
        layoutwidth: f32,
        layoutheight: f32,
        pixelsperdip: f32,
        transform: Option<*const DWRITE_MATRIX>,
        usegdinatural: bool,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGdiCompatibleTextLayout)(
                windows_core::Interface::as_raw(self),
                string.as_ptr(),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                layoutwidth,
                layoutheight,
                pixelsperdip,
                transform.unwrap_or(core::mem::zeroed()) as _,
                usegdinatural.into(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(
        &self,
        textformat: P0,
    ) -> windows_core::Result<IDWriteInlineObject>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(
                windows_core::Interface::as_raw(self),
                textformat.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextAnalyzer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNumberSubstitution<P1>(
        &self,
        substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localename: P1,
        ignoreuseroverride: bool,
    ) -> windows_core::Result<IDWriteNumberSubstitution>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNumberSubstitution)(
                windows_core::Interface::as_raw(self),
                substitutionmethod,
                localename.param().abi(),
                ignoreuseroverride.into(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGlyphRunAnalysis(
        &self,
        glyphrun: *const DWRITE_GLYPH_RUN,
        pixelsperdip: f32,
        transform: Option<*const DWRITE_MATRIX>,
        renderingmode: DWRITE_RENDERING_MODE,
        measuringmode: DWRITE_MEASURING_MODE,
        baselineoriginx: f32,
        baselineoriginy: f32,
    ) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(
                windows_core::Interface::as_raw(self),
                glyphrun,
                pixelsperdip,
                transform.unwrap_or(core::mem::zeroed()) as _,
                renderingmode,
                measuringmode,
                baselineoriginx,
                baselineoriginy,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CreateCustomFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFileReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const FILETIME,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_FACE_TYPE,
        u32,
        *const *mut core::ffi::c_void,
        u32,
        DWRITE_FONT_SIMULATIONS,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HMONITOR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        f32,
        DWRITE_PIXEL_GEOMETRY,
        DWRITE_RENDERING_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub CreateTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const u16,
        u32,
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const u16,
        u32,
        *mut core::ffi::c_void,
        f32,
        f32,
        f32,
        *const DWRITE_MATRIX,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateNumberSubstitution: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_NUMBER_SUBSTITUTION_METHOD,
        windows_core::PCWSTR,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_GLYPH_RUN,
        f32,
        *const DWRITE_MATRIX,
        DWRITE_RENDERING_MODE,
        DWRITE_MEASURING_MODE,
        f32,
        f32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IDWriteFactory_Impl: windows_core::IUnknownImpl {
    fn GetSystemFontCollection(
        &self,
        fontcollection: windows_core::OutRef<IDWriteFontCollection>,
        checkforupdates: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn CreateCustomFontCollection(
        &self,
        collectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
        collectionkey: *const core::ffi::c_void,
        collectionkeysize: u32,
    ) -> windows_core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(
        &self,
        fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
    ) -> windows_core::Result<()>;
    fn UnregisterFontCollectionLoader(
        &self,
        fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
    ) -> windows_core::Result<()>;
    fn CreateFontFileReference(
        &self,
        filepath: &windows_core::PCWSTR,
        lastwritetime: *const FILETIME,
    ) -> windows_core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(
        &self,
        fontfilereferencekey: *const core::ffi::c_void,
        fontfilereferencekeysize: u32,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<IDWriteFontFile>;
    fn CreateFontFace(
        &self,
        fontfacetype: DWRITE_FONT_FACE_TYPE,
        numberoffiles: u32,
        fontfiles: *const Option<IDWriteFontFile>,
        faceindex: u32,
        fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
    ) -> windows_core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(
        &self,
        monitor: HMONITOR,
    ) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(
        &self,
        gamma: f32,
        enhancedcontrast: f32,
        cleartypelevel: f32,
        pixelgeometry: DWRITE_PIXEL_GEOMETRY,
        renderingmode: DWRITE_RENDERING_MODE,
    ) -> windows_core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(
        &self,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<()>;
    fn UnregisterFontFileLoader(
        &self,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<()>;
    fn CreateTextFormat(
        &self,
        fontfamilyname: &windows_core::PCWSTR,
        fontcollection: windows_core::Ref<IDWriteFontCollection>,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: f32,
        localename: &windows_core::PCWSTR,
    ) -> windows_core::Result<IDWriteTextFormat>;
    fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography>;
    fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(
        &self,
        string: *const u16,
        stringlength: u32,
        textformat: windows_core::Ref<IDWriteTextFormat>,
        maxwidth: f32,
        maxheight: f32,
    ) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(
        &self,
        string: *const u16,
        stringlength: u32,
        textformat: windows_core::Ref<IDWriteTextFormat>,
        layoutwidth: f32,
        layoutheight: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: windows_core::BOOL,
    ) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(
        &self,
        textformat: windows_core::Ref<IDWriteTextFormat>,
    ) -> windows_core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(
        &self,
        substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localename: &windows_core::PCWSTR,
        ignoreuseroverride: windows_core::BOOL,
    ) -> windows_core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(
        &self,
        glyphrun: *const DWRITE_GLYPH_RUN,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        renderingmode: DWRITE_RENDERING_MODE,
        measuringmode: DWRITE_MEASURING_MODE,
        baselineoriginx: f32,
        baselineoriginy: f32,
    ) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
}
impl IDWriteFactory_Vtbl {
    pub const fn new<Identity: IDWriteFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemFontCollection<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollection: *mut *mut core::ffi::c_void,
            checkforupdates: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::GetSystemFontCollection(
                    this,
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute_copy(&checkforupdates),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateCustomFontCollection<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            collectionloader: *mut core::ffi::c_void,
            collectionkey: *const core::ffi::c_void,
            collectionkeysize: u32,
            fontcollection: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontCollection(
                    this,
                    core::mem::transmute_copy(&collectionloader),
                    core::mem::transmute_copy(&collectionkey),
                    core::mem::transmute_copy(&collectionkeysize),
                ) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollectionloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontCollectionLoader(
                    this,
                    core::mem::transmute_copy(&fontcollectionloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollectionloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontCollectionLoader(
                    this,
                    core::mem::transmute_copy(&fontcollectionloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateFontFileReference<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            filepath: windows_core::PCWSTR,
            lastwritetime: *const FILETIME,
            fontfile: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFileReference(
                    this,
                    core::mem::transmute(&filepath),
                    core::mem::transmute_copy(&lastwritetime),
                ) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfilereferencekey: *const core::ffi::c_void,
            fontfilereferencekeysize: u32,
            fontfileloader: *mut core::ffi::c_void,
            fontfile: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontFileReference(
                    this,
                    core::mem::transmute_copy(&fontfilereferencekey),
                    core::mem::transmute_copy(&fontfilereferencekeysize),
                    core::mem::transmute_copy(&fontfileloader),
                ) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFace<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfacetype: DWRITE_FONT_FACE_TYPE,
            numberoffiles: u32,
            fontfiles: *const *mut core::ffi::c_void,
            faceindex: u32,
            fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
            fontface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFace(
                    this,
                    core::mem::transmute_copy(&fontfacetype),
                    core::mem::transmute_copy(&numberoffiles),
                    core::mem::transmute_copy(&fontfiles),
                    core::mem::transmute_copy(&faceindex),
                    core::mem::transmute_copy(&fontfacesimulationflags),
                ) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateRenderingParams(this) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            monitor: HMONITOR,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateMonitorRenderingParams(
                    this,
                    core::mem::transmute_copy(&monitor),
                ) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            gamma: f32,
            enhancedcontrast: f32,
            cleartypelevel: f32,
            pixelgeometry: DWRITE_PIXEL_GEOMETRY,
            renderingmode: DWRITE_RENDERING_MODE,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomRenderingParams(
                    this,
                    core::mem::transmute_copy(&gamma),
                    core::mem::transmute_copy(&enhancedcontrast),
                    core::mem::transmute_copy(&cleartypelevel),
                    core::mem::transmute_copy(&pixelgeometry),
                    core::mem::transmute_copy(&renderingmode),
                ) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfileloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontFileLoader(
                    this,
                    core::mem::transmute_copy(&fontfileloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnregisterFontFileLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfileloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontFileLoader(
                    this,
                    core::mem::transmute_copy(&fontfileloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTextFormat<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfamilyname: windows_core::PCWSTR,
            fontcollection: *mut core::ffi::c_void,
            fontweight: DWRITE_FONT_WEIGHT,
            fontstyle: DWRITE_FONT_STYLE,
            fontstretch: DWRITE_FONT_STRETCH,
            fontsize: f32,
            localename: windows_core::PCWSTR,
            textformat: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextFormat(
                    this,
                    core::mem::transmute(&fontfamilyname),
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute_copy(&fontweight),
                    core::mem::transmute_copy(&fontstyle),
                    core::mem::transmute_copy(&fontstretch),
                    core::mem::transmute_copy(&fontsize),
                    core::mem::transmute(&localename),
                ) {
                    Ok(ok__) => {
                        textformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTypography<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typography: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTypography(this) {
                    Ok(ok__) => {
                        typography.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGdiInterop<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            gdiinterop: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::GetGdiInterop(this) {
                    Ok(ok__) => {
                        gdiinterop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextLayout<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            string: *const u16,
            stringlength: u32,
            textformat: *mut core::ffi::c_void,
            maxwidth: f32,
            maxheight: f32,
            textlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextLayout(
                    this,
                    core::mem::transmute_copy(&string),
                    core::mem::transmute_copy(&stringlength),
                    core::mem::transmute_copy(&textformat),
                    core::mem::transmute_copy(&maxwidth),
                    core::mem::transmute_copy(&maxheight),
                ) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            string: *const u16,
            stringlength: u32,
            textformat: *mut core::ffi::c_void,
            layoutwidth: f32,
            layoutheight: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: windows_core::BOOL,
            textlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGdiCompatibleTextLayout(
                    this,
                    core::mem::transmute_copy(&string),
                    core::mem::transmute_copy(&stringlength),
                    core::mem::transmute_copy(&textformat),
                    core::mem::transmute_copy(&layoutwidth),
                    core::mem::transmute_copy(&layoutheight),
                    core::mem::transmute_copy(&pixelsperdip),
                    core::mem::transmute_copy(&transform),
                    core::mem::transmute_copy(&usegdinatural),
                ) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textformat: *mut core::ffi::c_void,
            trimmingsign: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateEllipsisTrimmingSign(
                    this,
                    core::mem::transmute_copy(&textformat),
                ) {
                    Ok(ok__) => {
                        trimmingsign.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textanalyzer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextAnalyzer(this) {
                    Ok(ok__) => {
                        textanalyzer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
            localename: windows_core::PCWSTR,
            ignoreuseroverride: windows_core::BOOL,
            numbersubstitution: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateNumberSubstitution(
                    this,
                    core::mem::transmute_copy(&substitutionmethod),
                    core::mem::transmute(&localename),
                    core::mem::transmute_copy(&ignoreuseroverride),
                ) {
                    Ok(ok__) => {
                        numbersubstitution.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            glyphrun: *const DWRITE_GLYPH_RUN,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            renderingmode: DWRITE_RENDERING_MODE,
            measuringmode: DWRITE_MEASURING_MODE,
            baselineoriginx: f32,
            baselineoriginy: f32,
            glyphrunanalysis: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGlyphRunAnalysis(
                    this,
                    core::mem::transmute_copy(&glyphrun),
                    core::mem::transmute_copy(&pixelsperdip),
                    core::mem::transmute_copy(&transform),
                    core::mem::transmute_copy(&renderingmode),
                    core::mem::transmute_copy(&measuringmode),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                ) {
                    Ok(ok__) => {
                        glyphrunanalysis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
            CreateTypography: CreateTypography::<Identity, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory as windows_core::Interface>::IID
    }
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
    IDWriteFontCollectionLoader,
    IDWriteFontCollectionLoader_Vtbl,
    0xcca920e4_52f0_492b_bfa8_29c72ee0a468
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollectionLoader, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontCollectionLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateEnumeratorFromKey: usize,
}
impl windows_core::RuntimeName for IDWriteFontCollectionLoader {}
windows_core::imp::define_interface!(
    IDWriteFontFace,
    IDWriteFontFace_Vtbl,
    0x5f49804d_7024_4d43_bfa9_d25984f53849
);
windows_core::imp::interface_hierarchy!(IDWriteFontFace, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetType: usize,
    GetFiles: usize,
    GetIndex: usize,
    GetSimulations: usize,
    IsSymbolFont: usize,
    GetMetrics: usize,
    GetGlyphCount: usize,
    GetDesignGlyphMetrics: usize,
    GetGlyphIndices: usize,
    TryGetFontTable: usize,
    ReleaseFontTable: usize,
    GetGlyphRunOutline: usize,
    GetRecommendedRenderingMode: usize,
    GetGdiCompatibleMetrics: usize,
    GetGdiCompatibleGlyphMetrics: usize,
}
impl windows_core::RuntimeName for IDWriteFontFace {}
windows_core::imp::define_interface!(
    IDWriteFontFile,
    IDWriteFontFile_Vtbl,
    0x739d886a_cef5_47dc_8769_1a8b41bebbb0
);
windows_core::imp::interface_hierarchy!(IDWriteFontFile, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetReferenceKey: usize,
    GetLoader: usize,
    Analyze: usize,
}
impl windows_core::RuntimeName for IDWriteFontFile {}
windows_core::imp::define_interface!(
    IDWriteFontFileLoader,
    IDWriteFontFileLoader_Vtbl,
    0x727cad4e_d6af_4c9e_8a08_d695b11caa49
);
windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStreamFromKey: usize,
}
impl windows_core::RuntimeName for IDWriteFontFileLoader {}
windows_core::imp::define_interface!(
    IDWriteGdiInterop,
    IDWriteGdiInterop_Vtbl,
    0x1edd9491_9853_4299_898f_6432983b6f3a
);
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteGdiInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateFontFromLOGFONT: usize,
    ConvertFontToLOGFONT: usize,
    ConvertFontFaceToLOGFONT: usize,
    CreateFontFaceFromHdc: usize,
    CreateBitmapRenderTarget: usize,
}
impl windows_core::RuntimeName for IDWriteGdiInterop {}
windows_core::imp::define_interface!(
    IDWriteGlyphRunAnalysis,
    IDWriteGlyphRunAnalysis_Vtbl,
    0x7d97dbf7_e085_42d4_81e3_6a883bded118
);
windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetAlphaTextureBounds: usize,
    CreateAlphaTexture: usize,
    GetAlphaBlendParams: usize,
}
impl windows_core::RuntimeName for IDWriteGlyphRunAnalysis {}
windows_core::imp::define_interface!(
    IDWriteInlineObject,
    IDWriteInlineObject_Vtbl,
    0x8339fde3_106f_47ab_8373_1c6295eb10b3
);
windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Draw: usize,
    GetMetrics: usize,
    GetOverhangMetrics: usize,
    GetBreakConditions: usize,
}
impl windows_core::RuntimeName for IDWriteInlineObject {}
windows_core::imp::define_interface!(
    IDWriteNumberSubstitution,
    IDWriteNumberSubstitution_Vtbl,
    0x14885cc9_bab0_4f90_b6ed_5c366a2cd03d
);
windows_core::imp::interface_hierarchy!(IDWriteNumberSubstitution, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteNumberSubstitution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDWriteNumberSubstitution_Impl: windows_core::IUnknownImpl {}
impl IDWriteNumberSubstitution_Vtbl {
    pub const fn new<Identity: IDWriteNumberSubstitution_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteNumberSubstitution {}
windows_core::imp::define_interface!(
    IDWritePixelSnapping,
    IDWritePixelSnapping_Vtbl,
    0xeaf3a2da_ecf4_4d24_b644_b34f6842024b
);
windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, windows_core::IUnknown);
#[repr(C)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    IsPixelSnappingDisabled: usize,
    GetCurrentTransform: usize,
    GetPixelsPerDip: usize,
}
impl windows_core::RuntimeName for IDWritePixelSnapping {}
windows_core::imp::define_interface!(
    IDWriteRenderingParams,
    IDWriteRenderingParams_Vtbl,
    0x2f0da53a_2add_47cd_82ee_d9ec34688e75
);
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetGamma: usize,
    GetEnhancedContrast: usize,
    GetClearTypeLevel: usize,
    GetPixelGeometry: usize,
    GetRenderingMode: usize,
}
impl windows_core::RuntimeName for IDWriteRenderingParams {}
windows_core::imp::define_interface!(
    IDWriteTextAnalyzer,
    IDWriteTextAnalyzer_Vtbl,
    0xb7e6163e_7f46_43b4_84b3_e4e6249c365d
);
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteTextAnalyzer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AnalyzeScript: usize,
    AnalyzeBidi: usize,
    AnalyzeNumberSubstitution: usize,
    AnalyzeLineBreakpoints: usize,
    GetGlyphs: usize,
    GetGlyphPlacements: usize,
    GetGdiCompatibleGlyphPlacements: usize,
}
impl windows_core::RuntimeName for IDWriteTextAnalyzer {}
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
    pub unsafe fn SetReadingDirection(
        &self,
        readingdirection: DWRITE_READING_DIRECTION,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetReadingDirection)(
                windows_core::Interface::as_raw(self),
                readingdirection,
            )
        }
    }
    pub unsafe fn SetFlowDirection(
        &self,
        flowdirection: DWRITE_FLOW_DIRECTION,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFlowDirection)(
                windows_core::Interface::as_raw(self),
                flowdirection,
            )
        }
    }
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetIncrementalTabStop)(
                windows_core::Interface::as_raw(self),
                incrementaltabstop,
            )
        }
    }
    pub unsafe fn SetTrimming<P1>(
        &self,
        trimmingoptions: *const DWRITE_TRIMMING,
        trimmingsign: P1,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTrimming)(
                windows_core::Interface::as_raw(self),
                trimmingoptions,
                trimmingsign.param().abi(),
            )
        }
    }
    pub unsafe fn SetLineSpacing(
        &self,
        linespacingmethod: DWRITE_LINE_SPACING_METHOD,
        linespacing: f32,
        baseline: f32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetLineSpacing)(
                windows_core::Interface::as_raw(self),
                linespacingmethod,
                linespacing,
                baseline,
            )
        }
    }
    pub unsafe fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT {
        unsafe {
            (windows_core::Interface::vtable(self).GetTextAlignment)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        unsafe {
            (windows_core::Interface::vtable(self).GetParagraphAlignment)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING {
        unsafe {
            (windows_core::Interface::vtable(self).GetWordWrapping)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION {
        unsafe {
            (windows_core::Interface::vtable(self).GetReadingDirection)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION {
        unsafe {
            (windows_core::Interface::vtable(self).GetFlowDirection)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetIncrementalTabStop)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetTrimming(
        &self,
        trimmingoptions: *mut DWRITE_TRIMMING,
        trimmingsign: *mut Option<IDWriteInlineObject>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetTrimming)(
                windows_core::Interface::as_raw(self),
                trimmingoptions as _,
                core::mem::transmute(trimmingsign),
            )
        }
    }
    pub unsafe fn GetLineSpacing(
        &self,
        linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD,
        linespacing: *mut f32,
        baseline: *mut f32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLineSpacing)(
                windows_core::Interface::as_raw(self),
                linespacingmethod as _,
                linespacing as _,
                baseline as _,
            )
        }
    }
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontCollection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetFontFamilyName(
        &self,
        fontfamilyname: *mut u16,
        namesize: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                fontfamilyname as _,
                namesize,
            )
        }
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleNameLength)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetLocaleName(
        &self,
        localename: *mut u16,
        namesize: u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleName)(
                windows_core::Interface::as_raw(self),
                localename as _,
                namesize,
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
    pub SetReadingDirection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_READING_DIRECTION,
    ) -> windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FLOW_DIRECTION,
    ) -> windows_core::HRESULT,
    pub SetIncrementalTabStop:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetTrimming: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_TRIMMING,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_LINE_SPACING_METHOD,
        f32,
        f32,
    ) -> windows_core::HRESULT,
    pub GetTextAlignment:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT,
    pub GetParagraphAlignment:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT,
    pub GetWordWrapping: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_WORD_WRAPPING,
    pub GetReadingDirection:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_READING_DIRECTION,
    pub GetFlowDirection:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FLOW_DIRECTION,
    pub GetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetTrimming: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TRIMMING,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetLineSpacing: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_LINE_SPACING_METHOD,
        *mut f32,
        *mut f32,
    ) -> windows_core::HRESULT,
    pub GetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamilyName:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleName:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, u32) -> windows_core::HRESULT,
}
pub trait IDWriteTextFormat_Impl: windows_core::IUnknownImpl {
    fn SetTextAlignment(&self, textalignment: DWRITE_TEXT_ALIGNMENT) -> windows_core::Result<()>;
    fn SetParagraphAlignment(
        &self,
        paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::Result<()>;
    fn SetWordWrapping(&self, wordwrapping: DWRITE_WORD_WRAPPING) -> windows_core::Result<()>;
    fn SetReadingDirection(
        &self,
        readingdirection: DWRITE_READING_DIRECTION,
    ) -> windows_core::Result<()>;
    fn SetFlowDirection(&self, flowdirection: DWRITE_FLOW_DIRECTION) -> windows_core::Result<()>;
    fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::Result<()>;
    fn SetTrimming(
        &self,
        trimmingoptions: *const DWRITE_TRIMMING,
        trimmingsign: windows_core::Ref<IDWriteInlineObject>,
    ) -> windows_core::Result<()>;
    fn SetLineSpacing(
        &self,
        linespacingmethod: DWRITE_LINE_SPACING_METHOD,
        linespacing: f32,
        baseline: f32,
    ) -> windows_core::Result<()>;
    fn GetTextAlignment(&self) -> DWRITE_TEXT_ALIGNMENT;
    fn GetParagraphAlignment(&self) -> DWRITE_PARAGRAPH_ALIGNMENT;
    fn GetWordWrapping(&self) -> DWRITE_WORD_WRAPPING;
    fn GetReadingDirection(&self) -> DWRITE_READING_DIRECTION;
    fn GetFlowDirection(&self) -> DWRITE_FLOW_DIRECTION;
    fn GetIncrementalTabStop(&self) -> f32;
    fn GetTrimming(
        &self,
        trimmingoptions: *mut DWRITE_TRIMMING,
        trimmingsign: windows_core::OutRef<IDWriteInlineObject>,
    ) -> windows_core::Result<()>;
    fn GetLineSpacing(
        &self,
        linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD,
        linespacing: *mut f32,
        baseline: *mut f32,
    ) -> windows_core::Result<()>;
    fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection>;
    fn GetFontFamilyNameLength(&self) -> u32;
    fn GetFontFamilyName(
        &self,
        fontfamilyname: *mut u16,
        namesize: u32,
    ) -> windows_core::Result<()>;
    fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT;
    fn GetFontStyle(&self) -> DWRITE_FONT_STYLE;
    fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH;
    fn GetFontSize(&self) -> f32;
    fn GetLocaleNameLength(&self) -> u32;
    fn GetLocaleName(&self, localename: *mut u16, namesize: u32) -> windows_core::Result<()>;
}
impl IDWriteTextFormat_Vtbl {
    pub const fn new<Identity: IDWriteTextFormat_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTextAlignment<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textalignment: DWRITE_TEXT_ALIGNMENT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetTextAlignment(
                    this,
                    core::mem::transmute_copy(&textalignment),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetParagraphAlignment<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetParagraphAlignment(
                    this,
                    core::mem::transmute_copy(&paragraphalignment),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetWordWrapping<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            wordwrapping: DWRITE_WORD_WRAPPING,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetWordWrapping(
                    this,
                    core::mem::transmute_copy(&wordwrapping),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetReadingDirection<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            readingdirection: DWRITE_READING_DIRECTION,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetReadingDirection(
                    this,
                    core::mem::transmute_copy(&readingdirection),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFlowDirection<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            flowdirection: DWRITE_FLOW_DIRECTION,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetFlowDirection(
                    this,
                    core::mem::transmute_copy(&flowdirection),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetIncrementalTabStop<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            incrementaltabstop: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetIncrementalTabStop(
                    this,
                    core::mem::transmute_copy(&incrementaltabstop),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetTrimming<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            trimmingoptions: *const DWRITE_TRIMMING,
            trimmingsign: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetTrimming(
                    this,
                    core::mem::transmute_copy(&trimmingoptions),
                    core::mem::transmute_copy(&trimmingsign),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetLineSpacing<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            linespacingmethod: DWRITE_LINE_SPACING_METHOD,
            linespacing: f32,
            baseline: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::SetLineSpacing(
                    this,
                    core::mem::transmute_copy(&linespacingmethod),
                    core::mem::transmute_copy(&linespacing),
                    core::mem::transmute_copy(&baseline),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetTextAlignment<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_TEXT_ALIGNMENT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetTextAlignment(this)
            }
        }
        unsafe extern "system" fn GetParagraphAlignment<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_PARAGRAPH_ALIGNMENT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetParagraphAlignment(this)
            }
        }
        unsafe extern "system" fn GetWordWrapping<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_WORD_WRAPPING {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetWordWrapping(this)
            }
        }
        unsafe extern "system" fn GetReadingDirection<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_READING_DIRECTION {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetReadingDirection(this)
            }
        }
        unsafe extern "system" fn GetFlowDirection<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_FLOW_DIRECTION {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFlowDirection(this)
            }
        }
        unsafe extern "system" fn GetIncrementalTabStop<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetIncrementalTabStop(this)
            }
        }
        unsafe extern "system" fn GetTrimming<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            trimmingoptions: *mut DWRITE_TRIMMING,
            trimmingsign: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetTrimming(
                    this,
                    core::mem::transmute_copy(&trimmingoptions),
                    core::mem::transmute_copy(&trimmingsign),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetLineSpacing<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD,
            linespacing: *mut f32,
            baseline: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLineSpacing(
                    this,
                    core::mem::transmute_copy(&linespacingmethod),
                    core::mem::transmute_copy(&linespacing),
                    core::mem::transmute_copy(&baseline),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontCollection<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollection: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextFormat_Impl::GetFontCollection(this) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontFamilyNameLength(this)
            }
        }
        unsafe extern "system" fn GetFontFamilyName<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfamilyname: *mut u16,
            namesize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontFamilyName(
                    this,
                    core::mem::transmute_copy(&fontfamilyname),
                    core::mem::transmute_copy(&namesize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontWeight<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_FONT_WEIGHT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontWeight(this)
            }
        }
        unsafe extern "system" fn GetFontStyle<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_FONT_STYLE {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontStyle(this)
            }
        }
        unsafe extern "system" fn GetFontStretch<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> DWRITE_FONT_STRETCH {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontStretch(this)
            }
        }
        unsafe extern "system" fn GetFontSize<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetFontSize(this)
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLocaleNameLength(this)
            }
        }
        unsafe extern "system" fn GetLocaleName<
            Identity: IDWriteTextFormat_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            localename: *mut u16,
            namesize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextFormat_Impl::GetLocaleName(
                    this,
                    core::mem::transmute_copy(&localename),
                    core::mem::transmute_copy(&namesize),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTextAlignment: SetTextAlignment::<Identity, OFFSET>,
            SetParagraphAlignment: SetParagraphAlignment::<Identity, OFFSET>,
            SetWordWrapping: SetWordWrapping::<Identity, OFFSET>,
            SetReadingDirection: SetReadingDirection::<Identity, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, OFFSET>,
            SetIncrementalTabStop: SetIncrementalTabStop::<Identity, OFFSET>,
            SetTrimming: SetTrimming::<Identity, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, OFFSET>,
            GetParagraphAlignment: GetParagraphAlignment::<Identity, OFFSET>,
            GetWordWrapping: GetWordWrapping::<Identity, OFFSET>,
            GetReadingDirection: GetReadingDirection::<Identity, OFFSET>,
            GetFlowDirection: GetFlowDirection::<Identity, OFFSET>,
            GetIncrementalTabStop: GetIncrementalTabStop::<Identity, OFFSET>,
            GetTrimming: GetTrimming::<Identity, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
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
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxWidth)(
                windows_core::Interface::as_raw(self),
                maxwidth,
            )
        }
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxHeight)(
                windows_core::Interface::as_raw(self),
                maxheight,
            )
        }
    }
    pub unsafe fn SetFontCollection<P0>(
        &self,
        fontcollection: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontCollection)(
                windows_core::Interface::as_raw(self),
                fontcollection.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn SetFontFamilyName<P0>(
        &self,
        fontfamilyname: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                fontfamilyname.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn SetFontWeight(
        &self,
        fontweight: DWRITE_FONT_WEIGHT,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                fontweight,
                textrange,
            )
        }
    }
    pub unsafe fn SetFontStyle(
        &self,
        fontstyle: DWRITE_FONT_STYLE,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontStyle)(
                windows_core::Interface::as_raw(self),
                fontstyle,
                textrange,
            )
        }
    }
    pub unsafe fn SetFontStretch(
        &self,
        fontstretch: DWRITE_FONT_STRETCH,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontStretch)(
                windows_core::Interface::as_raw(self),
                fontstretch,
                textrange,
            )
        }
    }
    pub unsafe fn SetFontSize(
        &self,
        fontsize: f32,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                fontsize,
                textrange,
            )
        }
    }
    pub unsafe fn SetUnderline(
        &self,
        hasunderline: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnderline)(
                windows_core::Interface::as_raw(self),
                hasunderline.into(),
                textrange,
            )
        }
    }
    pub unsafe fn SetStrikethrough(
        &self,
        hasstrikethrough: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetStrikethrough)(
                windows_core::Interface::as_raw(self),
                hasstrikethrough.into(),
                textrange,
            )
        }
    }
    pub unsafe fn SetDrawingEffect<P0>(
        &self,
        drawingeffect: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDrawingEffect)(
                windows_core::Interface::as_raw(self),
                drawingeffect.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn SetInlineObject<P0>(
        &self,
        inlineobject: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetInlineObject)(
                windows_core::Interface::as_raw(self),
                inlineobject.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn SetTypography<P0>(
        &self,
        typography: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDWriteTypography>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTypography)(
                windows_core::Interface::as_raw(self),
                typography.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn SetLocaleName<P0>(
        &self,
        localename: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetLocaleName)(
                windows_core::Interface::as_raw(self),
                localename.param().abi(),
                textrange,
            )
        }
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetFontCollection(
        &self,
        currentposition: u32,
        fontcollection: *mut Option<IDWriteFontCollection>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontCollection)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(fontcollection),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontFamilyNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(
                windows_core::Interface::as_raw(self),
                currentposition,
                namelength as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontFamilyName(
        &self,
        currentposition: u32,
        fontfamilyname: *mut u16,
        namesize: u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontfamilyname as _,
                namesize,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontWeight(
        &self,
        currentposition: u32,
        fontweight: *mut DWRITE_FONT_WEIGHT,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontWeight)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontweight as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontStyle(
        &self,
        currentposition: u32,
        fontstyle: *mut DWRITE_FONT_STYLE,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStyle)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontstyle as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontStretch(
        &self,
        currentposition: u32,
        fontstretch: *mut DWRITE_FONT_STRETCH,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStretch)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontstretch as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetFontSize(
        &self,
        currentposition: u32,
        fontsize: *mut f32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontSize)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontsize as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetUnderline(
        &self,
        currentposition: u32,
        hasunderline: *mut windows_core::BOOL,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetUnderline)(
                windows_core::Interface::as_raw(self),
                currentposition,
                hasunderline as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetStrikethrough(
        &self,
        currentposition: u32,
        hasstrikethrough: *mut windows_core::BOOL,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetStrikethrough)(
                windows_core::Interface::as_raw(self),
                currentposition,
                hasstrikethrough as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetDrawingEffect(
        &self,
        currentposition: u32,
        drawingeffect: *mut Option<windows_core::IUnknown>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetDrawingEffect)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(drawingeffect),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetInlineObject(
        &self,
        currentposition: u32,
        inlineobject: *mut Option<IDWriteInlineObject>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetInlineObject)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(inlineobject),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetTypography(
        &self,
        currentposition: u32,
        typography: *mut Option<IDWriteTypography>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetTypography)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(typography),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetLocaleNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleNameLength)(
                windows_core::Interface::as_raw(self),
                currentposition,
                namelength as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetLocaleName(
        &self,
        currentposition: u32,
        localename: *mut u16,
        namesize: u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleName)(
                windows_core::Interface::as_raw(self),
                currentposition,
                localename as _,
                namesize,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn Draw<P1>(
        &self,
        clientdrawingcontext: Option<*const core::ffi::c_void>,
        renderer: P1,
        originx: f32,
        originy: f32,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _,
                renderer.param().abi(),
                originx,
                originy,
            )
        }
    }
    pub unsafe fn GetLineMetrics(
        &self,
        linemetrics: Option<*mut DWRITE_LINE_METRICS>,
        maxlinecount: u32,
        actuallinecount: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetLineMetrics)(
                windows_core::Interface::as_raw(self),
                linemetrics.unwrap_or(core::mem::zeroed()) as _,
                maxlinecount,
                actuallinecount as _,
            )
        }
    }
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
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverhangMetrics)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetClusterMetrics(
        &self,
        clustermetrics: Option<*mut DWRITE_CLUSTER_METRICS>,
        maxclustercount: u32,
        actualclustercount: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetClusterMetrics)(
                windows_core::Interface::as_raw(self),
                clustermetrics.unwrap_or(core::mem::zeroed()) as _,
                maxclustercount,
                actualclustercount as _,
            )
        }
    }
    pub unsafe fn DetermineMinWidth(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetermineMinWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn HitTestPoint(
        &self,
        pointx: f32,
        pointy: f32,
        istrailinghit: *mut windows_core::BOOL,
        isinside: *mut windows_core::BOOL,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestPoint)(
                windows_core::Interface::as_raw(self),
                pointx,
                pointy,
                istrailinghit as _,
                isinside as _,
                hittestmetrics as _,
            )
        }
    }
    pub unsafe fn HitTestTextPosition(
        &self,
        textposition: u32,
        istrailinghit: bool,
        pointx: *mut f32,
        pointy: *mut f32,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextPosition)(
                windows_core::Interface::as_raw(self),
                textposition,
                istrailinghit.into(),
                pointx as _,
                pointy as _,
                hittestmetrics as _,
            )
        }
    }
    pub unsafe fn HitTestTextRange(
        &self,
        textposition: u32,
        textlength: u32,
        originx: f32,
        originy: f32,
        hittestmetrics: Option<*mut DWRITE_HIT_TEST_METRICS>,
        maxhittestmetricscount: u32,
        actualhittestmetricscount: *mut u32,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextRange)(
                windows_core::Interface::as_raw(self),
                textposition,
                textlength,
                originx,
                originy,
                hittestmetrics.unwrap_or(core::mem::zeroed()) as _,
                maxhittestmetricscount,
                actualhittestmetricscount as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_STYLE,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_STRETCH,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetDrawingEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u16,
        u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_WEIGHT,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_STYLE,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontStretch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_STRETCH,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut f32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetDrawingEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u16,
        u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
        f32,
        f32,
    ) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_LINE_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_OVERHANG_METRICS,
    ) -> windows_core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_CLUSTER_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub DetermineMinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub HitTestPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut windows_core::BOOL,
        *mut windows_core::BOOL,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        *mut f32,
        *mut f32,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        f32,
        f32,
        *mut DWRITE_HIT_TEST_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
}
pub trait IDWriteTextLayout_Impl: IDWriteTextFormat_Impl {
    fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()>;
    fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()>;
    fn SetFontCollection(
        &self,
        fontcollection: windows_core::Ref<IDWriteFontCollection>,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetFontFamilyName(
        &self,
        fontfamilyname: &windows_core::PCWSTR,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetFontWeight(
        &self,
        fontweight: DWRITE_FONT_WEIGHT,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetFontStyle(
        &self,
        fontstyle: DWRITE_FONT_STYLE,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetFontStretch(
        &self,
        fontstretch: DWRITE_FONT_STRETCH,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetFontSize(&self, fontsize: f32, textrange: &DWRITE_TEXT_RANGE)
    -> windows_core::Result<()>;
    fn SetUnderline(
        &self,
        hasunderline: windows_core::BOOL,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetStrikethrough(
        &self,
        hasstrikethrough: windows_core::BOOL,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetDrawingEffect(
        &self,
        drawingeffect: windows_core::Ref<windows_core::IUnknown>,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetInlineObject(
        &self,
        inlineobject: windows_core::Ref<IDWriteInlineObject>,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetTypography(
        &self,
        typography: windows_core::Ref<IDWriteTypography>,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn SetLocaleName(
        &self,
        localename: &windows_core::PCWSTR,
        textrange: &DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetMaxWidth(&self) -> f32;
    fn GetMaxHeight(&self) -> f32;
    fn GetFontCollection(
        &self,
        currentposition: u32,
        fontcollection: windows_core::OutRef<IDWriteFontCollection>,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontFamilyNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontFamilyName(
        &self,
        currentposition: u32,
        fontfamilyname: *mut u16,
        namesize: u32,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontWeight(
        &self,
        currentposition: u32,
        fontweight: *mut DWRITE_FONT_WEIGHT,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontStyle(
        &self,
        currentposition: u32,
        fontstyle: *mut DWRITE_FONT_STYLE,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontStretch(
        &self,
        currentposition: u32,
        fontstretch: *mut DWRITE_FONT_STRETCH,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetFontSize(
        &self,
        currentposition: u32,
        fontsize: *mut f32,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetUnderline(
        &self,
        currentposition: u32,
        hasunderline: *mut windows_core::BOOL,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetStrikethrough(
        &self,
        currentposition: u32,
        hasstrikethrough: *mut windows_core::BOOL,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetDrawingEffect(
        &self,
        currentposition: u32,
        drawingeffect: windows_core::OutRef<windows_core::IUnknown>,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetInlineObject(
        &self,
        currentposition: u32,
        inlineobject: windows_core::OutRef<IDWriteInlineObject>,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetTypography(
        &self,
        currentposition: u32,
        typography: windows_core::OutRef<IDWriteTypography>,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetLocaleNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn GetLocaleName(
        &self,
        currentposition: u32,
        localename: *mut u16,
        namesize: u32,
        textrange: *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>;
    fn Draw(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        renderer: windows_core::Ref<IDWriteTextRenderer>,
        originx: f32,
        originy: f32,
    ) -> windows_core::Result<()>;
    fn GetLineMetrics(
        &self,
        linemetrics: *mut DWRITE_LINE_METRICS,
        maxlinecount: u32,
        actuallinecount: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()>;
    fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetClusterMetrics(
        &self,
        clustermetrics: *mut DWRITE_CLUSTER_METRICS,
        maxclustercount: u32,
        actualclustercount: *mut u32,
    ) -> windows_core::Result<()>;
    fn DetermineMinWidth(&self) -> windows_core::Result<f32>;
    fn HitTestPoint(
        &self,
        pointx: f32,
        pointy: f32,
        istrailinghit: *mut windows_core::BOOL,
        isinside: *mut windows_core::BOOL,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::Result<()>;
    fn HitTestTextPosition(
        &self,
        textposition: u32,
        istrailinghit: windows_core::BOOL,
        pointx: *mut f32,
        pointy: *mut f32,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::Result<()>;
    fn HitTestTextRange(
        &self,
        textposition: u32,
        textlength: u32,
        originx: f32,
        originy: f32,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
        maxhittestmetricscount: u32,
        actualhittestmetricscount: *mut u32,
    ) -> windows_core::Result<()>;
}
impl IDWriteTextLayout_Vtbl {
    pub const fn new<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaxWidth<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            maxwidth: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetMaxWidth(this, core::mem::transmute_copy(&maxwidth))
                    .into()
            }
        }
        unsafe extern "system" fn SetMaxHeight<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            maxheight: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetMaxHeight(this, core::mem::transmute_copy(&maxheight))
                    .into()
            }
        }
        unsafe extern "system" fn SetFontCollection<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollection: *mut core::ffi::c_void,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontCollection(
                    this,
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFontFamilyName<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfamilyname: windows_core::PCWSTR,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontFamilyName(
                    this,
                    core::mem::transmute(&fontfamilyname),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFontWeight<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontweight: DWRITE_FONT_WEIGHT,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontWeight(
                    this,
                    core::mem::transmute_copy(&fontweight),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFontStyle<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontstyle: DWRITE_FONT_STYLE,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontStyle(
                    this,
                    core::mem::transmute_copy(&fontstyle),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFontStretch<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontstretch: DWRITE_FONT_STRETCH,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontStretch(
                    this,
                    core::mem::transmute_copy(&fontstretch),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFontSize<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontsize: f32,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetFontSize(
                    this,
                    core::mem::transmute_copy(&fontsize),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUnderline<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hasunderline: windows_core::BOOL,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetUnderline(
                    this,
                    core::mem::transmute_copy(&hasunderline),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetStrikethrough<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hasstrikethrough: windows_core::BOOL,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetStrikethrough(
                    this,
                    core::mem::transmute_copy(&hasstrikethrough),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetDrawingEffect<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            drawingeffect: *mut core::ffi::c_void,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetDrawingEffect(
                    this,
                    core::mem::transmute_copy(&drawingeffect),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetInlineObject<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            inlineobject: *mut core::ffi::c_void,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetInlineObject(
                    this,
                    core::mem::transmute_copy(&inlineobject),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetTypography<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typography: *mut core::ffi::c_void,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetTypography(
                    this,
                    core::mem::transmute_copy(&typography),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetLocaleName<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            localename: windows_core::PCWSTR,
            textrange: DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::SetLocaleName(
                    this,
                    core::mem::transmute(&localename),
                    core::mem::transmute(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetMaxWidth<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMaxWidth(this)
            }
        }
        unsafe extern "system" fn GetMaxHeight<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMaxHeight(this)
            }
        }
        unsafe extern "system" fn GetFontCollection<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontcollection: *mut *mut core::ffi::c_void,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontCollection(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontFamilyNameLength<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            namelength: *mut u32,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontFamilyNameLength(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&namelength),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontFamilyName<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontfamilyname: *mut u16,
            namesize: u32,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontFamilyName(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontfamilyname),
                    core::mem::transmute_copy(&namesize),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontWeight<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontweight: *mut DWRITE_FONT_WEIGHT,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontWeight(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontweight),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontStyle<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontstyle: *mut DWRITE_FONT_STYLE,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontStyle(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontstyle),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontStretch<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontstretch: *mut DWRITE_FONT_STRETCH,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontStretch(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontstretch),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFontSize<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            fontsize: *mut f32,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetFontSize(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&fontsize),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUnderline<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            hasunderline: *mut windows_core::BOOL,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetUnderline(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&hasunderline),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStrikethrough<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            hasstrikethrough: *mut windows_core::BOOL,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetStrikethrough(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&hasstrikethrough),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetDrawingEffect<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            drawingeffect: *mut *mut core::ffi::c_void,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetDrawingEffect(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&drawingeffect),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetInlineObject<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            inlineobject: *mut *mut core::ffi::c_void,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetInlineObject(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&inlineobject),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetTypography<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            typography: *mut *mut core::ffi::c_void,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetTypography(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&typography),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetLocaleNameLength<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            namelength: *mut u32,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLocaleNameLength(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&namelength),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetLocaleName<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            currentposition: u32,
            localename: *mut u16,
            namesize: u32,
            textrange: *mut DWRITE_TEXT_RANGE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLocaleName(
                    this,
                    core::mem::transmute_copy(&currentposition),
                    core::mem::transmute_copy(&localename),
                    core::mem::transmute_copy(&namesize),
                    core::mem::transmute_copy(&textrange),
                )
                .into()
            }
        }
        unsafe extern "system" fn Draw<Identity: IDWriteTextLayout_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            clientdrawingcontext: *const core::ffi::c_void,
            renderer: *mut core::ffi::c_void,
            originx: f32,
            originy: f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::Draw(
                    this,
                    core::mem::transmute_copy(&clientdrawingcontext),
                    core::mem::transmute_copy(&renderer),
                    core::mem::transmute_copy(&originx),
                    core::mem::transmute_copy(&originy),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetLineMetrics<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            linemetrics: *mut DWRITE_LINE_METRICS,
            maxlinecount: u32,
            actuallinecount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetLineMetrics(
                    this,
                    core::mem::transmute_copy(&linemetrics),
                    core::mem::transmute_copy(&maxlinecount),
                    core::mem::transmute_copy(&actuallinecount),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetMetrics<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textmetrics: *mut DWRITE_TEXT_METRICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetMetrics(this, core::mem::transmute_copy(&textmetrics))
                    .into()
            }
        }
        unsafe extern "system" fn GetOverhangMetrics<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            overhangs: *mut DWRITE_OVERHANG_METRICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextLayout_Impl::GetOverhangMetrics(this) {
                    Ok(ok__) => {
                        overhangs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClusterMetrics<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clustermetrics: *mut DWRITE_CLUSTER_METRICS,
            maxclustercount: u32,
            actualclustercount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::GetClusterMetrics(
                    this,
                    core::mem::transmute_copy(&clustermetrics),
                    core::mem::transmute_copy(&maxclustercount),
                    core::mem::transmute_copy(&actualclustercount),
                )
                .into()
            }
        }
        unsafe extern "system" fn DetermineMinWidth<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            minwidth: *mut f32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteTextLayout_Impl::DetermineMinWidth(this) {
                    Ok(ok__) => {
                        minwidth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HitTestPoint<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pointx: f32,
            pointy: f32,
            istrailinghit: *mut windows_core::BOOL,
            isinside: *mut windows_core::BOOL,
            hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestPoint(
                    this,
                    core::mem::transmute_copy(&pointx),
                    core::mem::transmute_copy(&pointy),
                    core::mem::transmute_copy(&istrailinghit),
                    core::mem::transmute_copy(&isinside),
                    core::mem::transmute_copy(&hittestmetrics),
                )
                .into()
            }
        }
        unsafe extern "system" fn HitTestTextPosition<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textposition: u32,
            istrailinghit: windows_core::BOOL,
            pointx: *mut f32,
            pointy: *mut f32,
            hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestTextPosition(
                    this,
                    core::mem::transmute_copy(&textposition),
                    core::mem::transmute_copy(&istrailinghit),
                    core::mem::transmute_copy(&pointx),
                    core::mem::transmute_copy(&pointy),
                    core::mem::transmute_copy(&hittestmetrics),
                )
                .into()
            }
        }
        unsafe extern "system" fn HitTestTextRange<
            Identity: IDWriteTextLayout_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textposition: u32,
            textlength: u32,
            originx: f32,
            originy: f32,
            hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
            maxhittestmetricscount: u32,
            actualhittestmetricscount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteTextLayout_Impl::HitTestTextRange(
                    this,
                    core::mem::transmute_copy(&textposition),
                    core::mem::transmute_copy(&textlength),
                    core::mem::transmute_copy(&originx),
                    core::mem::transmute_copy(&originy),
                    core::mem::transmute_copy(&hittestmetrics),
                    core::mem::transmute_copy(&maxhittestmetricscount),
                    core::mem::transmute_copy(&actualhittestmetricscount),
                )
                .into()
            }
        }
        Self {
            base__: IDWriteTextFormat_Vtbl::new::<Identity, OFFSET>(),
            SetMaxWidth: SetMaxWidth::<Identity, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, OFFSET>,
            SetFontCollection: SetFontCollection::<Identity, OFFSET>,
            SetFontFamilyName: SetFontFamilyName::<Identity, OFFSET>,
            SetFontWeight: SetFontWeight::<Identity, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, OFFSET>,
            SetFontSize: SetFontSize::<Identity, OFFSET>,
            SetUnderline: SetUnderline::<Identity, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, OFFSET>,
            SetDrawingEffect: SetDrawingEffect::<Identity, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, OFFSET>,
            SetTypography: SetTypography::<Identity, OFFSET>,
            SetLocaleName: SetLocaleName::<Identity, OFFSET>,
            GetMaxWidth: GetMaxWidth::<Identity, OFFSET>,
            GetMaxHeight: GetMaxHeight::<Identity, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetUnderline: GetUnderline::<Identity, OFFSET>,
            GetStrikethrough: GetStrikethrough::<Identity, OFFSET>,
            GetDrawingEffect: GetDrawingEffect::<Identity, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, OFFSET>,
            GetTypography: GetTypography::<Identity, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, OFFSET>,
            GetMetrics: GetMetrics::<Identity, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, OFFSET>,
            GetClusterMetrics: GetClusterMetrics::<Identity, OFFSET>,
            DetermineMinWidth: DetermineMinWidth::<Identity, OFFSET>,
            HitTestPoint: HitTestPoint::<Identity, OFFSET>,
            HitTestTextPosition: HitTestTextPosition::<Identity, OFFSET>,
            HitTestTextRange: HitTestTextRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteTextLayout as windows_core::Interface>::IID
            || iid == &<IDWriteTextFormat as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteTextLayout {}
windows_core::imp::define_interface!(
    IDWriteTextRenderer,
    IDWriteTextRenderer_Vtbl,
    0xef8a8135_5cc6_45fe_8825_c5a0724eb819
);
impl core::ops::Deref for IDWriteTextRenderer {
    type Target = IDWritePixelSnapping;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextRenderer,
    windows_core::IUnknown,
    IDWritePixelSnapping
);
#[repr(C)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    DrawGlyphRun: usize,
    DrawUnderline: usize,
    DrawStrikethrough: usize,
    DrawInlineObject: usize,
}
impl windows_core::RuntimeName for IDWriteTextRenderer {}
windows_core::imp::define_interface!(
    IDWriteTypography,
    IDWriteTypography_Vtbl,
    0x55f1112b_1dc2_4b3c_9541_f46894ed85b6
);
windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteTypography_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddFontFeature: usize,
    GetFontFeatureCount: usize,
    GetFontFeature: usize,
}
impl windows_core::RuntimeName for IDWriteTypography {}
windows_core::imp::define_interface!(
    IDXGIDeviceSubObject,
    IDXGIDeviceSubObject_Vtbl,
    0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6
);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    GetDevice: usize,
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(
    IDXGIObject,
    IDXGIObject_Vtbl,
    0xaec22fb8_76f3_4639_9be0_28eb43a67a2e
);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
#[repr(C)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetPrivateData: usize,
    SetPrivateDataInterface: usize,
    GetPrivateData: usize,
    GetParent: usize,
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(
    IDXGISurface,
    IDXGISurface_Vtbl,
    0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec
);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISurface,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject
);
#[repr(C)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    GetDesc: usize,
    Map: usize,
    Unmap: usize,
}
impl windows_core::RuntimeName for IDXGISurface {}
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
    pub unsafe fn CompareElements<P0, P1>(
        &self,
        el1: P0,
        el2: P1,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareElements)(
                windows_core::Interface::as_raw(self),
                el1.param().abi(),
                el2.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CompareRuntimeIds(
        &self,
        runtimeid1: *const SAFEARRAY,
        runtimeid2: *const SAFEARRAY,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareRuntimeIds)(
                windows_core::Interface::as_raw(self),
                runtimeid1,
                runtimeid2,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
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
    pub unsafe fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootElementBuildCache<P0>(
        &self,
        cacherequest: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElementBuildCache)(
                windows_core::Interface::as_raw(self),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromHandleBuildCache<P1>(
        &self,
        hwnd: UIA_HWND,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandleBuildCache)(
                windows_core::Interface::as_raw(self),
                hwnd,
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromPointBuildCache<P1>(
        &self,
        pt: POINT,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPointBuildCache)(
                windows_core::Interface::as_raw(self),
                pt,
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(
        &self,
        cacherequest: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElementBuildCache)(
                windows_core::Interface::as_raw(self),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTreeWalker<P0>(
        &self,
        pcondition: P0,
    ) -> windows_core::Result<IUIAutomationTreeWalker>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTreeWalker)(
                windows_core::Interface::as_raw(self),
                pcondition.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlViewWalker)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentViewWalker)(
                windows_core::Interface::as_raw(self),
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
    pub unsafe fn RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawViewCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlViewCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentViewCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCacheRequest)(
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
    pub unsafe fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFalseCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePropertyCondition(
        &self,
        propertyid: PROPERTYID,
        value: &VARIANT,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyCondition)(
                windows_core::Interface::as_raw(self),
                propertyid,
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePropertyConditionEx(
        &self,
        propertyid: PROPERTYID,
        value: &VARIANT,
        flags: PropertyConditionFlags,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyConditionEx)(
                windows_core::Interface::as_raw(self),
                propertyid,
                core::mem::transmute_copy(value),
                flags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndCondition<P0, P1>(
        &self,
        condition1: P0,
        condition2: P1,
    ) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndCondition)(
                windows_core::Interface::as_raw(self),
                condition1.param().abi(),
                condition2.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndConditionFromArray(
        &self,
        conditions: *const SAFEARRAY,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromArray)(
                windows_core::Interface::as_raw(self),
                conditions,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndConditionFromNativeArray(
        &self,
        conditions: *const Option<IUIAutomationCondition>,
        conditioncount: i32,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromNativeArray)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(conditions),
                conditioncount,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrCondition<P0, P1>(
        &self,
        condition1: P0,
        condition2: P1,
    ) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrCondition)(
                windows_core::Interface::as_raw(self),
                condition1.param().abi(),
                condition2.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrConditionFromArray(
        &self,
        conditions: *const SAFEARRAY,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromArray)(
                windows_core::Interface::as_raw(self),
                conditions,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrConditionFromNativeArray(
        &self,
        conditions: *const Option<IUIAutomationCondition>,
        conditioncount: i32,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromNativeArray)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(conditions),
                conditioncount,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNotCondition<P0>(
        &self,
        condition: P0,
    ) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNotCondition)(
                windows_core::Interface::as_raw(self),
                condition.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddAutomationEventHandler<P1, P3, P4>(
        &self,
        eventid: EVENTID,
        element: P1,
        scope: TreeScope,
        cacherequest: P3,
        handler: P4,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddAutomationEventHandler)(
                windows_core::Interface::as_raw(self),
                eventid,
                element.param().abi(),
                scope,
                cacherequest.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn RemoveAutomationEventHandler<P1, P2>(
        &self,
        eventid: EVENTID,
        element: P1,
        handler: P2,
    ) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAutomationEventHandler)(
                windows_core::Interface::as_raw(self),
                eventid,
                element.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P2, P3>(
        &self,
        element: P0,
        scope: TreeScope,
        cacherequest: P2,
        handler: P3,
        propertyarray: *const PROPERTYID,
        propertycount: i32,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddPropertyChangedEventHandlerNativeArray)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                scope,
                cacherequest.param().abi(),
                handler.param().abi(),
                propertyarray,
                propertycount,
            )
        }
    }
    pub unsafe fn AddPropertyChangedEventHandler<P0, P2, P3>(
        &self,
        element: P0,
        scope: TreeScope,
        cacherequest: P2,
        handler: P3,
        propertyarray: *const SAFEARRAY,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                scope,
                cacherequest.param().abi(),
                handler.param().abi(),
                propertyarray,
            )
        }
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(
        &self,
        element: P0,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemovePropertyChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P2, P3>(
        &self,
        element: P0,
        scope: TreeScope,
        cacherequest: P2,
        handler: P3,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                scope,
                cacherequest.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(
        &self,
        element: P0,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveStructureChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(
        &self,
        cacherequest: P0,
        handler: P1,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddFocusChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                cacherequest.param().abi(),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveFocusChangedEventHandler)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
            )
        }
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAllEventHandlers)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn IntNativeArrayToSafeArray(
        &self,
        array: *const i32,
        arraycount: i32,
    ) -> windows_core::Result<*mut SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntNativeArrayToSafeArray)(
                windows_core::Interface::as_raw(self),
                array,
                arraycount,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn IntSafeArrayToNativeArray(
        &self,
        intarray: *const SAFEARRAY,
        array: *mut *mut i32,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntSafeArrayToNativeArray)(
                windows_core::Interface::as_raw(self),
                intarray,
                array as _,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RectToVariant(&self, rc: RECT) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RectToVariant)(
                windows_core::Interface::as_raw(self),
                rc,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VariantToRect(&self, var: &VARIANT) -> windows_core::Result<RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VariantToRect)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(var),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SafeArrayToRectNativeArray(
        &self,
        rects: *const SAFEARRAY,
        rectarray: *mut *mut RECT,
    ) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SafeArrayToRectNativeArray)(
                windows_core::Interface::as_raw(self),
                rects,
                rectarray as _,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(
        &self,
        factory: P0,
    ) -> windows_core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: windows_core::Param<IUIAutomationProxyFactory>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProxyFactoryEntry)(
                windows_core::Interface::as_raw(self),
                factory.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProxyFactoryMapping(
        &self,
    ) -> windows_core::Result<IUIAutomationProxyFactoryMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyFactoryMapping)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyProgrammaticName(
        &self,
        property: PROPERTYID,
    ) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyProgrammaticName)(
                windows_core::Interface::as_raw(self),
                property,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPatternProgrammaticName(
        &self,
        pattern: PATTERNID,
    ) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProgrammaticName)(
                windows_core::Interface::as_raw(self),
                pattern,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(
        &self,
        pelement: P0,
        patternids: *mut *mut SAFEARRAY,
        patternnames: *mut *mut SAFEARRAY,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PollForPotentialSupportedPatterns)(
                windows_core::Interface::as_raw(self),
                pelement.param().abi(),
                patternids as _,
                patternnames as _,
            )
        }
    }
    pub unsafe fn PollForPotentialSupportedProperties<P0>(
        &self,
        pelement: P0,
        propertyids: *mut *mut SAFEARRAY,
        propertynames: *mut *mut SAFEARRAY,
    ) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PollForPotentialSupportedProperties)(
                windows_core::Interface::as_raw(self),
                pelement.param().abi(),
                propertyids as _,
                propertynames as _,
            )
        }
    }
    pub unsafe fn CheckNotSupported(
        &self,
        value: &VARIANT,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckNotSupported)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReservedNotSupportedValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReservedMixedAttributeValue(
        &self,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReservedMixedAttributeValue)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromIAccessible<P0>(
        &self,
        accessible: P0,
        childid: i32,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IAccessible>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessible)(
                windows_core::Interface::as_raw(self),
                accessible.param().abi(),
                childid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P2>(
        &self,
        accessible: P0,
        childid: i32,
        cacherequest: P2,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IAccessible>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessibleBuildCache)(
                windows_core::Interface::as_raw(self),
                accessible.param().abi(),
                childid,
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompareElements: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CompareRuntimeIds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const SAFEARRAY,
        *const SAFEARRAY,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetRootElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
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
    pub GetFocusedElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetRootElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ElementFromHandleBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UIA_HWND,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ElementFromPointBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        POINT,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFocusedElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTreeWalker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ControlViewWalker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContentViewWalker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RawViewWalker: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RawViewCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ControlViewCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ContentViewCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCacheRequest: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTrueCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFalseCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePropertyCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        VARIANT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePropertyConditionEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        VARIANT,
        PropertyConditionFlags,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateAndCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateAndConditionFromArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const SAFEARRAY,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateAndConditionFromNativeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateOrCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateOrConditionFromArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const SAFEARRAY,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateOrConditionFromNativeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateNotCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddAutomationEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        EVENTID,
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveAutomationEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        EVENTID,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddPropertyChangedEventHandlerNativeArray:
        unsafe extern "system" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            TreeScope,
            *mut core::ffi::c_void,
            *mut core::ffi::c_void,
            *const PROPERTYID,
            i32,
        ) -> windows_core::HRESULT,
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const SAFEARRAY,
    ) -> windows_core::HRESULT,
    pub RemovePropertyChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveStructureChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddFocusChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveFocusChangedEventHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RemoveAllEventHandlers:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IntNativeArrayToSafeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const i32,
        i32,
        *mut *mut SAFEARRAY,
    ) -> windows_core::HRESULT,
    pub IntSafeArrayToNativeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const SAFEARRAY,
        *mut *mut i32,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub RectToVariant: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        RECT,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub VariantToRect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VARIANT,
        *mut RECT,
    ) -> windows_core::HRESULT,
    pub SafeArrayToRectNativeArray: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const SAFEARRAY,
        *mut *mut RECT,
        *mut i32,
    ) -> windows_core::HRESULT,
    pub CreateProxyFactoryEntry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ProxyFactoryMapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPropertyProgrammaticName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPatternProgrammaticName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub PollForPotentialSupportedPatterns: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut SAFEARRAY,
        *mut *mut SAFEARRAY,
    ) -> windows_core::HRESULT,
    pub PollForPotentialSupportedProperties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut SAFEARRAY,
        *mut *mut SAFEARRAY,
    )
        -> windows_core::HRESULT,
    pub CheckNotSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        VARIANT,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ReservedNotSupportedValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ReservedMixedAttributeValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ElementFromIAccessible: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ElementFromIAccessibleBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        i32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IUIAutomation_Impl: windows_core::IUnknownImpl {
    fn CompareElements(
        &self,
        el1: windows_core::Ref<IUIAutomationElement>,
        el2: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn CompareRuntimeIds(
        &self,
        runtimeid1: *const SAFEARRAY,
        runtimeid2: *const SAFEARRAY,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&self, hwnd: UIA_HWND) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&self, pt: &POINT) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(
        &self,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(
        &self,
        hwnd: UIA_HWND,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(
        &self,
        pt: &POINT,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(
        &self,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(
        &self,
        pcondition: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(
        &self,
        propertyid: PROPERTYID,
        value: &VARIANT,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(
        &self,
        propertyid: PROPERTYID,
        value: &VARIANT,
        flags: PropertyConditionFlags,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(
        &self,
        condition1: windows_core::Ref<IUIAutomationCondition>,
        condition2: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(
        &self,
        conditions: *const SAFEARRAY,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(
        &self,
        conditions: *const Option<IUIAutomationCondition>,
        conditioncount: i32,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(
        &self,
        condition1: windows_core::Ref<IUIAutomationCondition>,
        condition2: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(
        &self,
        conditions: *const SAFEARRAY,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(
        &self,
        conditions: *const Option<IUIAutomationCondition>,
        conditioncount: i32,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(
        &self,
        condition: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(
        &self,
        eventid: EVENTID,
        element: windows_core::Ref<IUIAutomationElement>,
        scope: TreeScope,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
        handler: windows_core::Ref<IUIAutomationEventHandler>,
    ) -> windows_core::Result<()>;
    fn RemoveAutomationEventHandler(
        &self,
        eventid: EVENTID,
        element: windows_core::Ref<IUIAutomationElement>,
        handler: windows_core::Ref<IUIAutomationEventHandler>,
    ) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        scope: TreeScope,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
        handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>,
        propertyarray: *const PROPERTYID,
        propertycount: i32,
    ) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        scope: TreeScope,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
        handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>,
        propertyarray: *const SAFEARRAY,
    ) -> windows_core::Result<()>;
    fn RemovePropertyChangedEventHandler(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>,
    ) -> windows_core::Result<()>;
    fn AddStructureChangedEventHandler(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        scope: TreeScope,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
        handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>,
    ) -> windows_core::Result<()>;
    fn RemoveStructureChangedEventHandler(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>,
    ) -> windows_core::Result<()>;
    fn AddFocusChangedEventHandler(
        &self,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
        handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>,
    ) -> windows_core::Result<()>;
    fn RemoveFocusChangedEventHandler(
        &self,
        handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>,
    ) -> windows_core::Result<()>;
    fn RemoveAllEventHandlers(&self) -> windows_core::Result<()>;
    fn IntNativeArrayToSafeArray(
        &self,
        array: *const i32,
        arraycount: i32,
    ) -> windows_core::Result<*mut SAFEARRAY>;
    fn IntSafeArrayToNativeArray(
        &self,
        intarray: *const SAFEARRAY,
        array: *mut *mut i32,
    ) -> windows_core::Result<i32>;
    fn RectToVariant(&self, rc: &RECT) -> windows_core::Result<VARIANT>;
    fn VariantToRect(&self, var: &VARIANT) -> windows_core::Result<RECT>;
    fn SafeArrayToRectNativeArray(
        &self,
        rects: *const SAFEARRAY,
        rectarray: *mut *mut RECT,
    ) -> windows_core::Result<i32>;
    fn CreateProxyFactoryEntry(
        &self,
        factory: windows_core::Ref<IUIAutomationProxyFactory>,
    ) -> windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(
        &self,
        property: PROPERTYID,
    ) -> windows_core::Result<windows_core::BSTR>;
    fn GetPatternProgrammaticName(
        &self,
        pattern: PATTERNID,
    ) -> windows_core::Result<windows_core::BSTR>;
    fn PollForPotentialSupportedPatterns(
        &self,
        pelement: windows_core::Ref<IUIAutomationElement>,
        patternids: *mut *mut SAFEARRAY,
        patternnames: *mut *mut SAFEARRAY,
    ) -> windows_core::Result<()>;
    fn PollForPotentialSupportedProperties(
        &self,
        pelement: windows_core::Ref<IUIAutomationElement>,
        propertyids: *mut *mut SAFEARRAY,
        propertynames: *mut *mut SAFEARRAY,
    ) -> windows_core::Result<()>;
    fn CheckNotSupported(&self, value: &VARIANT) -> windows_core::Result<windows_core::BOOL>;
    fn ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ElementFromIAccessible(
        &self,
        accessible: windows_core::Ref<IAccessible>,
        childid: i32,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(
        &self,
        accessible: windows_core::Ref<IAccessible>,
        childid: i32,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomation_Vtbl {
    pub const fn new<Identity: IUIAutomation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompareElements<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            el1: *mut core::ffi::c_void,
            el2: *mut core::ffi::c_void,
            aresame: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CompareElements(
                    this,
                    core::mem::transmute_copy(&el1),
                    core::mem::transmute_copy(&el2),
                ) {
                    Ok(ok__) => {
                        aresame.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            runtimeid1: *const SAFEARRAY,
            runtimeid2: *const SAFEARRAY,
            aresame: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CompareRuntimeIds(
                    this,
                    core::mem::transmute_copy(&runtimeid1),
                    core::mem::transmute_copy(&runtimeid2),
                ) {
                    Ok(ok__) => {
                        aresame.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootElement<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            root: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetRootElement(this) {
                    Ok(ok__) => {
                        root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromHandle<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hwnd: UIA_HWND,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromHandle(this, core::mem::transmute_copy(&hwnd))
                {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromPoint<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pt: POINT,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromPoint(this, core::mem::transmute(&pt)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocusedElement<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetFocusedElement(this) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            root: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetRootElementBuildCache(
                    this,
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hwnd: UIA_HWND,
            cacherequest: *mut core::ffi::c_void,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromHandleBuildCache(
                    this,
                    core::mem::transmute_copy(&hwnd),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pt: POINT,
            cacherequest: *mut core::ffi::c_void,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromPointBuildCache(
                    this,
                    core::mem::transmute(&pt),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetFocusedElementBuildCache(
                    this,
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTreeWalker<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcondition: *mut core::ffi::c_void,
            walker: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateTreeWalker(
                    this,
                    core::mem::transmute_copy(&pcondition),
                ) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ControlViewWalker<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            walker: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ControlViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentViewWalker<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            walker: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ContentViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RawViewWalker<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            walker: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RawViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RawViewCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RawViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ControlViewCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ControlViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentViewCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ContentViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCacheRequest<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cacherequest: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateCacheRequest(this) {
                    Ok(ok__) => {
                        cacherequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTrueCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateTrueCondition(this) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFalseCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateFalseCondition(this) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            value: VARIANT,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreatePropertyCondition(
                    this,
                    core::mem::transmute_copy(&propertyid),
                    core::mem::transmute(&value),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            value: VARIANT,
            flags: PropertyConditionFlags,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreatePropertyConditionEx(
                    this,
                    core::mem::transmute_copy(&propertyid),
                    core::mem::transmute(&value),
                    core::mem::transmute_copy(&flags),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition1: *mut core::ffi::c_void,
            condition2: *mut core::ffi::c_void,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndCondition(
                    this,
                    core::mem::transmute_copy(&condition1),
                    core::mem::transmute_copy(&condition2),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            conditions: *const SAFEARRAY,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndConditionFromArray(
                    this,
                    core::mem::transmute_copy(&conditions),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            conditions: *const *mut core::ffi::c_void,
            conditioncount: i32,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndConditionFromNativeArray(
                    this,
                    core::mem::transmute_copy(&conditions),
                    core::mem::transmute_copy(&conditioncount),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition1: *mut core::ffi::c_void,
            condition2: *mut core::ffi::c_void,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrCondition(
                    this,
                    core::mem::transmute_copy(&condition1),
                    core::mem::transmute_copy(&condition2),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            conditions: *const SAFEARRAY,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrConditionFromArray(
                    this,
                    core::mem::transmute_copy(&conditions),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            conditions: *const *mut core::ffi::c_void,
            conditioncount: i32,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrConditionFromNativeArray(
                    this,
                    core::mem::transmute_copy(&conditions),
                    core::mem::transmute_copy(&conditioncount),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNotCondition<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition: *mut core::ffi::c_void,
            newcondition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateNotCondition(
                    this,
                    core::mem::transmute_copy(&condition),
                ) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            eventid: EVENTID,
            element: *mut core::ffi::c_void,
            scope: TreeScope,
            cacherequest: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddAutomationEventHandler(
                    this,
                    core::mem::transmute_copy(&eventid),
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&cacherequest),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            eventid: EVENTID,
            element: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveAutomationEventHandler(
                    this,
                    core::mem::transmute_copy(&eventid),
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            scope: TreeScope,
            cacherequest: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            propertyarray: *const PROPERTYID,
            propertycount: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddPropertyChangedEventHandlerNativeArray(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&cacherequest),
                    core::mem::transmute_copy(&handler),
                    core::mem::transmute_copy(&propertyarray),
                    core::mem::transmute_copy(&propertycount),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            scope: TreeScope,
            cacherequest: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
            propertyarray: *const SAFEARRAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddPropertyChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&cacherequest),
                    core::mem::transmute_copy(&handler),
                    core::mem::transmute_copy(&propertyarray),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemovePropertyChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            scope: TreeScope,
            cacherequest: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddStructureChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&cacherequest),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveStructureChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddFocusChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&cacherequest),
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveFocusChangedEventHandler(
                    this,
                    core::mem::transmute_copy(&handler),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveAllEventHandlers<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveAllEventHandlers(this).into()
            }
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            array: *const i32,
            arraycount: i32,
            safearray: *mut *mut SAFEARRAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::IntNativeArrayToSafeArray(
                    this,
                    core::mem::transmute_copy(&array),
                    core::mem::transmute_copy(&arraycount),
                ) {
                    Ok(ok__) => {
                        safearray.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            intarray: *const SAFEARRAY,
            array: *mut *mut i32,
            arraycount: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::IntSafeArrayToNativeArray(
                    this,
                    core::mem::transmute_copy(&intarray),
                    core::mem::transmute_copy(&array),
                ) {
                    Ok(ok__) => {
                        arraycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RectToVariant<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rc: RECT,
            var: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RectToVariant(this, core::mem::transmute(&rc)) {
                    Ok(ok__) => {
                        var.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VariantToRect<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            var: VARIANT,
            rc: *mut RECT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::VariantToRect(this, core::mem::transmute(&var)) {
                    Ok(ok__) => {
                        rc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rects: *const SAFEARRAY,
            rectarray: *mut *mut RECT,
            rectarraycount: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::SafeArrayToRectNativeArray(
                    this,
                    core::mem::transmute_copy(&rects),
                    core::mem::transmute_copy(&rectarray),
                ) {
                    Ok(ok__) => {
                        rectarraycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            factory: *mut core::ffi::c_void,
            factoryentry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateProxyFactoryEntry(
                    this,
                    core::mem::transmute_copy(&factory),
                ) {
                    Ok(ok__) => {
                        factoryentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProxyFactoryMapping<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            factorymapping: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ProxyFactoryMapping(this) {
                    Ok(ok__) => {
                        factorymapping.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            property: PROPERTYID,
            name: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetPropertyProgrammaticName(
                    this,
                    core::mem::transmute_copy(&property),
                ) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pattern: PATTERNID,
            name: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetPatternProgrammaticName(
                    this,
                    core::mem::transmute_copy(&pattern),
                ) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pelement: *mut core::ffi::c_void,
            patternids: *mut *mut SAFEARRAY,
            patternnames: *mut *mut SAFEARRAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedPatterns(
                    this,
                    core::mem::transmute_copy(&pelement),
                    core::mem::transmute_copy(&patternids),
                    core::mem::transmute_copy(&patternnames),
                )
                .into()
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pelement: *mut core::ffi::c_void,
            propertyids: *mut *mut SAFEARRAY,
            propertynames: *mut *mut SAFEARRAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedProperties(
                    this,
                    core::mem::transmute_copy(&pelement),
                    core::mem::transmute_copy(&propertyids),
                    core::mem::transmute_copy(&propertynames),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckNotSupported<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            value: VARIANT,
            isnotsupported: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CheckNotSupported(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        isnotsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReservedNotSupportedValue<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            notsupportedvalue: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ReservedNotSupportedValue(this) {
                    Ok(ok__) => {
                        notsupportedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            mixedattributevalue: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ReservedMixedAttributeValue(this) {
                    Ok(ok__) => {
                        mixedattributevalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            accessible: *mut core::ffi::c_void,
            childid: i32,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromIAccessible(
                    this,
                    core::mem::transmute_copy(&accessible),
                    core::mem::transmute_copy(&childid),
                ) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<
            Identity: IUIAutomation_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            accessible: *mut core::ffi::c_void,
            childid: i32,
            cacherequest: *mut core::ffi::c_void,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromIAccessibleBuildCache(
                    this,
                    core::mem::transmute_copy(&accessible),
                    core::mem::transmute_copy(&childid),
                    core::mem::transmute_copy(&cacherequest),
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
            CompareElements: CompareElements::<Identity, OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Identity, OFFSET>,
            GetRootElement: GetRootElement::<Identity, OFFSET>,
            ElementFromHandle: ElementFromHandle::<Identity, OFFSET>,
            ElementFromPoint: ElementFromPoint::<Identity, OFFSET>,
            GetFocusedElement: GetFocusedElement::<Identity, OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Identity, OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Identity, OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Identity, OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Identity, OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Identity, OFFSET>,
            ControlViewWalker: ControlViewWalker::<Identity, OFFSET>,
            ContentViewWalker: ContentViewWalker::<Identity, OFFSET>,
            RawViewWalker: RawViewWalker::<Identity, OFFSET>,
            RawViewCondition: RawViewCondition::<Identity, OFFSET>,
            ControlViewCondition: ControlViewCondition::<Identity, OFFSET>,
            ContentViewCondition: ContentViewCondition::<Identity, OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Identity, OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Identity, OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Identity, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Identity, OFFSET>,
            CreateAndCondition: CreateAndCondition::<Identity, OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Identity, OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Identity, OFFSET>,
            CreateOrCondition: CreateOrCondition::<Identity, OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Identity, OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Identity, OFFSET>,
            CreateNotCondition: CreateNotCondition::<Identity, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Identity, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<
                Identity,
                OFFSET,
            >,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Identity, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<
                Identity,
                OFFSET,
            >,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Identity, OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Identity, OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Identity, OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Identity, OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Identity, OFFSET>,
            RectToVariant: RectToVariant::<Identity, OFFSET>,
            VariantToRect: VariantToRect::<Identity, OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Identity, OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Identity, OFFSET>,
            ProxyFactoryMapping: ProxyFactoryMapping::<Identity, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<
                Identity,
                OFFSET,
            >,
            CheckNotSupported: CheckNotSupported::<Identity, OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Identity, OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Identity, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomation {}
windows_core::imp::define_interface!(
    IUIAutomationCacheRequest,
    IUIAutomationCacheRequest_Vtbl,
    0xb32a92b5_bc25_4078_9c08_d7ee95c48e03
);
windows_core::imp::interface_hierarchy!(IUIAutomationCacheRequest, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationCacheRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddProperty: usize,
    AddPattern: usize,
    Clone: usize,
    TreeScope: usize,
    SetTreeScope: usize,
    TreeFilter: usize,
    SetTreeFilter: usize,
    AutomationElementMode: usize,
    SetAutomationElementMode: usize,
}
impl windows_core::RuntimeName for IUIAutomationCacheRequest {}
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
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuntimeId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn FindFirst<P1>(
        &self,
        scope: TreeScope,
        condition: P1,
    ) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(
                windows_core::Interface::as_raw(self),
                scope,
                condition.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub unsafe fn FindFirstBuildCache<P1, P2>(
        &self,
        scope: TreeScope,
        condition: P1,
        cacherequest: P2,
    ) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstBuildCache)(
                windows_core::Interface::as_raw(self),
                scope,
                condition.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllBuildCache<P1, P2>(
        &self,
        scope: TreeScope,
        condition: P1,
        cacherequest: P2,
    ) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllBuildCache)(
                windows_core::Interface::as_raw(self),
                scope,
                condition.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildUpdatedCache)(
                windows_core::Interface::as_raw(self),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentPropertyValue(
        &self,
        propertyid: PROPERTYID,
    ) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValue)(
                windows_core::Interface::as_raw(self),
                propertyid,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentPropertyValueEx(
        &self,
        propertyid: PROPERTYID,
        ignoredefaultvalue: bool,
    ) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValueEx)(
                windows_core::Interface::as_raw(self),
                propertyid,
                ignoredefaultvalue.into(),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedPropertyValue(
        &self,
        propertyid: PROPERTYID,
    ) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValue)(
                windows_core::Interface::as_raw(self),
                propertyid,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedPropertyValueEx(
        &self,
        propertyid: PROPERTYID,
        ignoredefaultvalue: bool,
    ) -> windows_core::Result<VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValueEx)(
                windows_core::Interface::as_raw(self),
                propertyid,
                ignoredefaultvalue.into(),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetCurrentPatternAs)(
                windows_core::Interface::as_raw(self),
                patternid,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetCachedPatternAs)(
                windows_core::Interface::as_raw(self),
                patternid,
                &T::IID,
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
    pub unsafe fn GetCachedPattern(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPattern)(
                windows_core::Interface::as_raw(self),
                patternid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedParent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedChildren)(
                windows_core::Interface::as_raw(self),
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
    pub unsafe fn CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLocalizedControlType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAcceleratorKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAccessKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHasKeyboardFocus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsKeyboardFocusable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsEnabled)(
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
    pub unsafe fn CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHelpText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCulture)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsControlElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsContentElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsPassword)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentNativeWindowHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentItemType)(
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
    pub unsafe fn CurrentOrientation(&self) -> windows_core::Result<OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentOrientation)(
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
    pub unsafe fn CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsRequiredForForm)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentItemStatus)(
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
    pub unsafe fn CurrentLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLabeledBy)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAriaRole)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAriaProperties)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsDataValidForForm)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentControllerFor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDescribedBy)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFlowsTo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProviderDescription)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedProcessId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedControlType(&self) -> windows_core::Result<CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedControlType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLocalizedControlType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAcceleratorKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAccessKey)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHasKeyboardFocus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsKeyboardFocusable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsEnabled)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAutomationId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedClassName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHelpText)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCulture)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsControlElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsContentElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsPassword)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedNativeWindowHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedItemType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsOffscreen)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedOrientation(&self) -> windows_core::Result<OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedOrientation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFrameworkId)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsRequiredForForm)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedItemStatus)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedBoundingRectangle(&self) -> windows_core::Result<RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedBoundingRectangle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLabeledBy)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAriaRole)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAriaProperties)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsDataValidForForm)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedControllerFor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDescribedBy)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFlowsTo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedProviderDescription)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetClickablePoint(
        &self,
        clickable: *mut POINT,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClickablePoint)(
                windows_core::Interface::as_raw(self),
                clickable as _,
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
    pub GetRuntimeId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut SAFEARRAY,
    ) -> windows_core::HRESULT,
    pub FindFirst: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FindFirstBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub FindAllBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub BuildUpdatedCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCurrentPropertyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub GetCurrentPropertyValueEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        windows_core::BOOL,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub GetCachedPropertyValue: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub GetCachedPropertyValueEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PROPERTYID,
        windows_core::BOOL,
        *mut VARIANT,
    ) -> windows_core::HRESULT,
    pub GetCurrentPatternAs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCachedPatternAs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCurrentPattern: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCachedPattern: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCachedParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCachedChildren: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentProcessId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CONTROLTYPEID,
    ) -> windows_core::HRESULT,
    pub CurrentLocalizedControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentAcceleratorKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentAccessKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentHasKeyboardFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentIsKeyboardFocusable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentIsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentAutomationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentClassName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentHelpText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentCulture:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentIsControlElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentIsContentElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentIsPassword: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentNativeWindowHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub CurrentItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentIsOffscreen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentOrientation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut OrientationType,
    ) -> windows_core::HRESULT,
    pub CurrentFrameworkId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentIsRequiredForForm: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentItemStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentBoundingRectangle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut RECT) -> windows_core::HRESULT,
    pub CurrentLabeledBy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentAriaRole: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentAriaProperties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentIsDataValidForForm: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CurrentControllerFor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentDescribedBy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentFlowsTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CurrentProviderDescription: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedProcessId:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CONTROLTYPEID,
    ) -> windows_core::HRESULT,
    pub CachedLocalizedControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedAcceleratorKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedAccessKey: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedHasKeyboardFocus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedIsKeyboardFocusable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedIsEnabled: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedAutomationId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedClassName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedHelpText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedCulture:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedIsControlElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedIsContentElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedIsPassword: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedNativeWindowHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub CachedItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedIsOffscreen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedOrientation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut OrientationType,
    ) -> windows_core::HRESULT,
    pub CachedFrameworkId: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedIsRequiredForForm: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedItemStatus: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedBoundingRectangle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut RECT) -> windows_core::HRESULT,
    pub CachedLabeledBy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedAriaRole: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedAriaProperties: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedIsDataValidForForm: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CachedControllerFor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedDescribedBy: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedFlowsTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CachedProviderDescription: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetClickablePoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut POINT,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IUIAutomationElement_Impl: windows_core::IUnknownImpl {
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut SAFEARRAY>;
    fn FindFirst(
        &self,
        scope: TreeScope,
        condition: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn FindAll(
        &self,
        scope: TreeScope,
        condition: windows_core::Ref<IUIAutomationCondition>,
    ) -> windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(
        &self,
        scope: TreeScope,
        condition: windows_core::Ref<IUIAutomationCondition>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(
        &self,
        scope: TreeScope,
        condition: windows_core::Ref<IUIAutomationCondition>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(
        &self,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<VARIANT>;
    fn GetCurrentPropertyValueEx(
        &self,
        propertyid: PROPERTYID,
        ignoredefaultvalue: windows_core::BOOL,
    ) -> windows_core::Result<VARIANT>;
    fn GetCachedPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<VARIANT>;
    fn GetCachedPropertyValueEx(
        &self,
        propertyid: PROPERTYID,
        ignoredefaultvalue: windows_core::BOOL,
    ) -> windows_core::Result<VARIANT>;
    fn GetCurrentPatternAs(
        &self,
        patternid: PATTERNID,
        riid: *const windows_core::GUID,
        patternobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetCachedPatternAs(
        &self,
        patternid: PATTERNID,
        riid: *const windows_core::GUID,
        patternobject: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetCurrentPattern(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedPattern(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedParent(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProcessId(&self) -> windows_core::Result<i32>;
    fn CurrentControlType(&self) -> windows_core::Result<CONTROLTYPEID>;
    fn CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentCulture(&self) -> windows_core::Result<i32>;
    fn CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentOrientation(&self) -> windows_core::Result<OrientationType>;
    fn CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentBoundingRectangle(&self) -> windows_core::Result<RECT>;
    fn CurrentLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedProcessId(&self) -> windows_core::Result<i32>;
    fn CachedControlType(&self) -> windows_core::Result<CONTROLTYPEID>;
    fn CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedCulture(&self) -> windows_core::Result<i32>;
    fn CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn CachedItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedOrientation(&self) -> windows_core::Result<OrientationType>;
    fn CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedBoundingRectangle(&self) -> windows_core::Result<RECT>;
    fn CachedLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetClickablePoint(&self, clickable: *mut POINT) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationElement_Vtbl {
    pub const fn new<Identity: IUIAutomationElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFocus<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::SetFocus(this).into()
            }
        }
        unsafe extern "system" fn GetRuntimeId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            runtimeid: *mut *mut SAFEARRAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetRuntimeId(this) {
                    Ok(ok__) => {
                        runtimeid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirst<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scope: TreeScope,
            condition: *mut core::ffi::c_void,
            found: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindFirst(
                    this,
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&condition),
                ) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAll<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scope: TreeScope,
            condition: *mut core::ffi::c_void,
            found: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindAll(
                    this,
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&condition),
                ) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scope: TreeScope,
            condition: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            found: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindFirstBuildCache(
                    this,
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&condition),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAllBuildCache<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            scope: TreeScope,
            condition: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            found: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindAllBuildCache(
                    this,
                    core::mem::transmute_copy(&scope),
                    core::mem::transmute_copy(&condition),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            updatedelement: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::BuildUpdatedCache(
                    this,
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        updatedelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            retval: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPropertyValue(
                    this,
                    core::mem::transmute_copy(&propertyid),
                ) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            ignoredefaultvalue: windows_core::BOOL,
            retval: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPropertyValueEx(
                    this,
                    core::mem::transmute_copy(&propertyid),
                    core::mem::transmute_copy(&ignoredefaultvalue),
                ) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            retval: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPropertyValue(
                    this,
                    core::mem::transmute_copy(&propertyid),
                ) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            propertyid: PROPERTYID,
            ignoredefaultvalue: windows_core::BOOL,
            retval: *mut VARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPropertyValueEx(
                    this,
                    core::mem::transmute_copy(&propertyid),
                    core::mem::transmute_copy(&ignoredefaultvalue),
                ) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            patternid: PATTERNID,
            riid: *const windows_core::GUID,
            patternobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCurrentPatternAs(
                    this,
                    core::mem::transmute_copy(&patternid),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&patternobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetCachedPatternAs<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            patternid: PATTERNID,
            riid: *const windows_core::GUID,
            patternobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCachedPatternAs(
                    this,
                    core::mem::transmute_copy(&patternid),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&patternobject),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetCurrentPattern<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            patternid: PATTERNID,
            patternobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPattern(
                    this,
                    core::mem::transmute_copy(&patternid),
                ) {
                    Ok(ok__) => {
                        patternobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPattern<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            patternid: PATTERNID,
            patternobject: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPattern(
                    this,
                    core::mem::transmute_copy(&patternid),
                ) {
                    Ok(ok__) => {
                        patternobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedParent<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            parent: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedParent(this) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedChildren<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            children: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedChildren(this) {
                    Ok(ok__) => {
                        children.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProcessId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentControlType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut CONTROLTYPEID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentName<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAccessKey<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAutomationId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentClassName<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHelpText<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCulture<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsPassword<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut UIA_HWND,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentItemType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentOrientation<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut OrientationType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentItemStatus<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut RECT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAriaRole<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentControllerFor<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedProcessId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedControlType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut CONTROLTYPEID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedName<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAccessKey<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsEnabled<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAutomationId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedClassName<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHelpText<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCulture<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsControlElement<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsContentElement<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsPassword<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut UIA_HWND,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedItemType<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedOrientation<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut OrientationType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFrameworkId<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedItemStatus<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut RECT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLabeledBy<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAriaRole<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAriaProperties<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedControllerFor<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDescribedBy<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFlowsTo<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedProviderDescription<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            retval: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClickablePoint<
            Identity: IUIAutomationElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            clickable: *mut POINT,
            gotclickable: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetClickablePoint(
                    this,
                    core::mem::transmute_copy(&clickable),
                ) {
                    Ok(ok__) => {
                        gotclickable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFocus: SetFocus::<Identity, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, OFFSET>,
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindAll: FindAll::<Identity, OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Identity, OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Identity, OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Identity, OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Identity, OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Identity, OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Identity, OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Identity, OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Identity, OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Identity, OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Identity, OFFSET>,
            GetCachedPattern: GetCachedPattern::<Identity, OFFSET>,
            GetCachedParent: GetCachedParent::<Identity, OFFSET>,
            GetCachedChildren: GetCachedChildren::<Identity, OFFSET>,
            CurrentProcessId: CurrentProcessId::<Identity, OFFSET>,
            CurrentControlType: CurrentControlType::<Identity, OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Identity, OFFSET>,
            CurrentName: CurrentName::<Identity, OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Identity, OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Identity, OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Identity, OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Identity, OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Identity, OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Identity, OFFSET>,
            CurrentClassName: CurrentClassName::<Identity, OFFSET>,
            CurrentHelpText: CurrentHelpText::<Identity, OFFSET>,
            CurrentCulture: CurrentCulture::<Identity, OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Identity, OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Identity, OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Identity, OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Identity, OFFSET>,
            CurrentItemType: CurrentItemType::<Identity, OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Identity, OFFSET>,
            CurrentOrientation: CurrentOrientation::<Identity, OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Identity, OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Identity, OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Identity, OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Identity, OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Identity, OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Identity, OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Identity, OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Identity, OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Identity, OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Identity, OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Identity, OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Identity, OFFSET>,
            CachedProcessId: CachedProcessId::<Identity, OFFSET>,
            CachedControlType: CachedControlType::<Identity, OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Identity, OFFSET>,
            CachedName: CachedName::<Identity, OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Identity, OFFSET>,
            CachedAccessKey: CachedAccessKey::<Identity, OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Identity, OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Identity, OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Identity, OFFSET>,
            CachedAutomationId: CachedAutomationId::<Identity, OFFSET>,
            CachedClassName: CachedClassName::<Identity, OFFSET>,
            CachedHelpText: CachedHelpText::<Identity, OFFSET>,
            CachedCulture: CachedCulture::<Identity, OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Identity, OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Identity, OFFSET>,
            CachedIsPassword: CachedIsPassword::<Identity, OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Identity, OFFSET>,
            CachedItemType: CachedItemType::<Identity, OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Identity, OFFSET>,
            CachedOrientation: CachedOrientation::<Identity, OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Identity, OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Identity, OFFSET>,
            CachedItemStatus: CachedItemStatus::<Identity, OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Identity, OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Identity, OFFSET>,
            CachedAriaRole: CachedAriaRole::<Identity, OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Identity, OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Identity, OFFSET>,
            CachedControllerFor: CachedControllerFor::<Identity, OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Identity, OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Identity, OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Identity, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement as windows_core::Interface>::IID
    }
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
    IUIAutomationEventHandler,
    IUIAutomationEventHandler_Vtbl,
    0x146c3c17_f12e_4e22_8c27_f894b9b79c69
);
windows_core::imp::interface_hierarchy!(IUIAutomationEventHandler, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    HandleAutomationEvent: usize,
}
impl windows_core::RuntimeName for IUIAutomationEventHandler {}
windows_core::imp::define_interface!(
    IUIAutomationFocusChangedEventHandler,
    IUIAutomationFocusChangedEventHandler_Vtbl,
    0xc270f6b5_5c69_4290_9745_7a7f97169468
);
windows_core::imp::interface_hierarchy!(
    IUIAutomationFocusChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct IUIAutomationFocusChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    HandleFocusChangedEvent: usize,
}
impl windows_core::RuntimeName for IUIAutomationFocusChangedEventHandler {}
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
    IUIAutomationPropertyChangedEventHandler,
    IUIAutomationPropertyChangedEventHandler_Vtbl,
    0x40cd37d4_c756_4b0c_8c6f_bddfeeb13b50
);
windows_core::imp::interface_hierarchy!(
    IUIAutomationPropertyChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    HandlePropertyChangedEvent: usize,
}
impl windows_core::RuntimeName for IUIAutomationPropertyChangedEventHandler {}
windows_core::imp::define_interface!(
    IUIAutomationProxyFactory,
    IUIAutomationProxyFactory_Vtbl,
    0x85b94ecd_849d_42b6_b94d_d6db23fdf5a4
);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactory, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationProxyFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateProvider: usize,
    ProxyFactoryId: usize,
}
impl windows_core::RuntimeName for IUIAutomationProxyFactory {}
windows_core::imp::define_interface!(
    IUIAutomationProxyFactoryEntry,
    IUIAutomationProxyFactoryEntry_Vtbl,
    0xd50e472e_b64b_490c_bca1_d30696f9f289
);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryEntry, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationProxyFactoryEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    ProxyFactory: usize,
    ClassName: usize,
    ImageName: usize,
    AllowSubstringMatch: usize,
    CanCheckBaseClass: usize,
    NeedsAdviseEvents: usize,
    SetClassName: usize,
    SetImageName: usize,
    SetAllowSubstringMatch: usize,
    SetCanCheckBaseClass: usize,
    SetNeedsAdviseEvents: usize,
    SetWinEventsForAutomationEvent: usize,
    GetWinEventsForAutomationEvent: usize,
}
impl windows_core::RuntimeName for IUIAutomationProxyFactoryEntry {}
windows_core::imp::define_interface!(
    IUIAutomationProxyFactoryMapping,
    IUIAutomationProxyFactoryMapping_Vtbl,
    0x09e31e18_872d_4873_93d1_1e541ec133fd
);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryMapping, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationProxyFactoryMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Count: usize,
    GetTable: usize,
    GetEntry: usize,
    SetTable: usize,
    InsertEntries: usize,
    InsertEntry: usize,
    RemoveEntry: usize,
    ClearTable: usize,
    RestoreDefaultTable: usize,
}
impl windows_core::RuntimeName for IUIAutomationProxyFactoryMapping {}
windows_core::imp::define_interface!(
    IUIAutomationStructureChangedEventHandler,
    IUIAutomationStructureChangedEventHandler_Vtbl,
    0xe81d1b4e_11c5_42f8_9754_e7036c79f054
);
windows_core::imp::interface_hierarchy!(
    IUIAutomationStructureChangedEventHandler,
    windows_core::IUnknown
);
#[repr(C)]
pub struct IUIAutomationStructureChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    HandleStructureChangedEvent: usize,
}
impl windows_core::RuntimeName for IUIAutomationStructureChangedEventHandler {}
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
    pub unsafe fn GetLastChildElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElement)(
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
    pub unsafe fn GetPreviousSiblingElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElement)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElement<P0>(
        &self,
        element: P0,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElement)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParentElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastChildElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousSiblingElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElementBuildCache<P0, P1>(
        &self,
        element: P0,
        cacherequest: P1,
    ) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElementBuildCache)(
                windows_core::Interface::as_raw(self),
                element.param().abi(),
                cacherequest.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Condition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Condition)(
                windows_core::Interface::as_raw(self),
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
    pub GetLastChildElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetNextSiblingElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPreviousSiblingElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub NormalizeElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetParentElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFirstChildElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetLastChildElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetNextSiblingElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPreviousSiblingElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub NormalizeElementBuildCache: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Condition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IUIAutomationTreeWalker_Impl: windows_core::IUnknownImpl {
    fn GetParentElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn NormalizeElement(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(
        &self,
        element: windows_core::Ref<IUIAutomationElement>,
        cacherequest: windows_core::Ref<IUIAutomationCacheRequest>,
    ) -> windows_core::Result<IUIAutomationElement>;
    fn Condition(&self) -> windows_core::Result<IUIAutomationCondition>;
}
impl IUIAutomationTreeWalker_Vtbl {
    pub const fn new<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParentElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            parent: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetParentElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstChildElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            first: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetFirstChildElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        first.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastChildElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            last: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetLastChildElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        last.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            next: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetNextSiblingElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        next.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            previous: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetPreviousSiblingElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        previous.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NormalizeElement<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            normalized: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::NormalizeElement(
                    this,
                    core::mem::transmute_copy(&element),
                ) {
                    Ok(ok__) => {
                        normalized.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            parent: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetParentElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            first: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetFirstChildElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        first.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            last: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetLastChildElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        last.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            next: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetNextSiblingElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        next.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            previous: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetPreviousSiblingElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        previous.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            element: *mut core::ffi::c_void,
            cacherequest: *mut core::ffi::c_void,
            normalized: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::NormalizeElementBuildCache(
                    this,
                    core::mem::transmute_copy(&element),
                    core::mem::transmute_copy(&cacherequest),
                ) {
                    Ok(ok__) => {
                        normalized.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Condition<
            Identity: IUIAutomationTreeWalker_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            condition: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::Condition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParentElement: GetParentElement::<Identity, OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Identity, OFFSET>,
            GetLastChildElement: GetLastChildElement::<Identity, OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Identity, OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Identity, OFFSET>,
            NormalizeElement: NormalizeElement::<Identity, OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Identity, OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Identity, OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Identity, OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Identity, OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<
                Identity,
                OFFSET,
            >,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Identity, OFFSET>,
            Condition: Condition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTreeWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTreeWalker {}
windows_core::imp::define_interface!(
    IWICBitmap,
    IWICBitmap_Vtbl,
    0x00000121_a8f2_4877_ba0a_fd2b6645fb94
);
impl core::ops::Deref for IWICBitmap {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
#[repr(C)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    Lock: usize,
    SetPalette: usize,
    SetResolution: usize,
}
impl windows_core::RuntimeName for IWICBitmap {}
windows_core::imp::define_interface!(
    IWICBitmapSource,
    IWICBitmapSource_Vtbl,
    0x00000120_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
#[repr(C)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSize: usize,
    GetPixelFormat: usize,
    GetResolution: usize,
    CopyPalette: usize,
    CopyPixels: usize,
}
impl windows_core::RuntimeName for IWICBitmapSource {}
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
pub type OrientationType = i32;
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
pub type PropertyConditionFlags = i32;
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
