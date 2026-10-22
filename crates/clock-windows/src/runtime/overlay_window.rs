//! One overlay HWND and its drawing surface (Windows, UI thread only).
//!
//! The window is a borderless, non-activating tool window sized to the verified
//! clock rectangle. It is never parented into Explorer, never changes Explorer's
//! styles and never uses an unconditional global topmost: z-order mirrors the
//! visible taskbar's own band, immediately above it (R03 strategy). It is created
//! in that band and keeps it while hidden; see `overlay::band_action` (#23).
use crate::bindings::*;
use crate::{
    accessibility::{ClockAccessible, is_root_object, return_object},
    interaction::{Interaction, PressTracker, ScreenPoint, post_interaction},
    overlay::{BandAction, band_action},
    render::{ClockSurface, PaintOutcome, PaintRequest, RenderError, Renderer},
};
use clock_core::PhysicalRect;
use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};
use windows_core::{Result, w};

/// Bumped by the window procedure (UI thread) on messages that invalidate pixels;
/// the loop compares it and repaints from stored state. The callback keeps no
/// references to loop state.
pub static REPAINT_EPOCH: AtomicU64 = AtomicU64::new(0);
const CLASS_NAME: windows_core::PCWSTR = w!("ElevenClock-rs.Overlay");
/// How long after exposure the confirming redraw waits; upkeep runs every 2 s.
pub const CONFIRM_DELAY: Duration = Duration::from_millis(500);

thread_local! {
    /// The button press in progress. Mouse capture makes at most one possible.
    static PRESS: Cell<PressTracker> = const { Cell::new(PressTracker::new()) };
    /// UIA providers by window id, for `WM_GETOBJECT`. The window procedure runs
    /// on the UI thread that created every overlay, so no lock is needed.
    static PROVIDERS: RefCell<HashMap<u64, IRawElementProviderSimple>> =
        RefCell::new(HashMap::new());
}

pub fn window_id(hwnd: HWND) -> u64 {
    hwnd.0 as usize as u64
}

/// Registers the window class for the life of the runtime.
pub struct OverlayClass(crate::bindings::HINSTANCE);
impl OverlayClass {
    pub fn register() -> Result<Self> {
        // SAFETY: standard class registration with a static procedure/name.
        unsafe {
            let module = GetModuleHandleW(None).ok()?;
            let class = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                hInstance: module,
                lpszClassName: CLASS_NAME,
                hCursor: LoadCursorW(None, IDC_ARROW).ok()?,
                ..Default::default()
            };
            if RegisterClassW(&class).0 == 0 {
                return Err(windows_core::Error::from_thread());
            }
            Ok(Self(module))
        }
    }
}
impl Drop for OverlayClass {
    fn drop(&mut self) {
        // SAFETY: all overlay windows are destroyed before the class guard drops.
        let _ = unsafe { UnregisterClassW(CLASS_NAME, Some(self.0)) };
    }
}

pub struct Overlay {
    // Declared first: the surface releases its Direct2D target before the HWND dies.
    pub surface: ClockSurface,
    hwnd: HWND,
    taskbar: HWND,
    bounds: PhysicalRect,
    pub last_text: Option<clock_core::ClockText>,
    accessible: ClockAccessible,
    shown: bool,
    /// When the overlay was last exposed, until its confirming redraw.
    exposed_at: Option<Instant>,
}
impl Drop for Overlay {
    fn drop(&mut self) {
        let id = window_id(self.hwnd);
        PROVIDERS.with(|providers| providers.borrow_mut().remove(&id));
        PRESS.with(|press| {
            let mut tracker = press.get();
            tracker.cancel();
            press.set(tracker);
        });
        // Detach from UI Automation first, so a client that still holds the
        // element gets "not available" rather than a dangling window.
        self.accessible.disconnect();
        // SAFETY: destroying a window created and owned by this thread.
        unsafe {
            let _ = DestroyWindow(self.hwnd);
        }
    }
}

/// Far outside any monitor, yet clear of the -32000 minimized-window sentinel.
const OFFSCREEN: i32 = -30_000;

/// Undo a minimize from outside the app (screen-control or window-management
/// tools) while keeping the window hidden. A minimized window has a 0x0 client
/// area, so every paint would report overflow and the clock would stay on the
/// native clock until restart. Restoring off screen, hiding, then moving back
/// means no unpainted pixels are ever visible. Returns whether it was minimized.
fn restore_hidden(hwnd: HWND, bounds: PhysicalRect) -> Result<bool> {
    // SAFETY: state query on our own window.
    if !unsafe { IsIconic(hwnd) }.as_bool() {
        return Ok(false);
    }
    let mut placement = WINDOWPLACEMENT {
        length: std::mem::size_of::<WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    let (width, height) = (bounds.width(), bounds.height());
    // SAFETY: `placement` is a live, correctly sized buffer; only our own window
    // is restored (without activation), hidden and moved.
    unsafe {
        GetWindowPlacement(hwnd, &mut placement).ok()?;
        placement.showCmd = SW_SHOWNOACTIVATE as u32;
        placement.rcNormalPosition = RECT {
            left: OFFSCREEN,
            top: OFFSCREEN,
            right: OFFSCREEN + width,
            bottom: OFFSCREEN + height,
        };
        SetWindowPlacement(hwnd, &placement).ok()?;
        let _ = ShowWindow(hwnd, SW_HIDE);
        SetWindowPos(
            hwnd,
            None,
            bounds.left(),
            bounds.top(),
            width,
            height,
            (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
        )
        .ok()?;
    }
    Ok(true)
}

fn topmost(hwnd: HWND) -> bool {
    // SAFETY: style query on a possibly stale handle; failure reads as zero.
    unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) & WS_EX_TOPMOST as isize != 0 }
}

/// Nearest predecessor of the taskbar in its own z-order band. Windows can list
/// windows of other bands between taskbar entries; anchoring after one of those
/// would demote the overlay.
fn relative_predecessor(root: HWND, root_topmost: bool) -> Result<Option<HWND>> {
    let mut cursor = root;
    for _ in 0..128 {
        // SAFETY: z-order walk over handles that may go stale; errors end the walk.
        let previous = unsafe { GetWindow(cursor, GW_HWNDPREV as u32) };
        if previous.is_invalid() {
            return Ok(None);
        }
        if topmost(previous) == root_topmost {
            return Ok(Some(previous));
        }
        cursor = previous;
    }
    Err(windows_core::Error::new(
        windows_core::HRESULT(0x80004005u32 as i32),
        "Taskbar predecessor inventory exceeded its bound",
    ))
}

impl Overlay {
    pub fn create(renderer: &Renderer, taskbar: HWND, bounds: PhysicalRect) -> Result<Self> {
        // Born in the taskbar's band: Windows ignores a later HWND_TOPMOST from
        // this background process while the desktop is foreground (#23).
        let band = if topmost(taskbar) { WS_EX_TOPMOST } else { 0 };
        // SAFETY: class registered by OverlayClass; creates a hidden popup owned
        // by this thread. WS_EX_NOACTIVATE/TOOLWINDOW keep it out of Alt-Tab and
        // focus. It is never shown here.
        let hwnd = unsafe {
            CreateWindowExW(
                (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | band) as u32,
                CLASS_NAME,
                w!("ElevenClock-rs"),
                WS_POPUP,
                bounds.left(),
                bounds.top(),
                bounds.width(),
                bounds.height(),
                None,
                None,
                Some(GetModuleHandleW(None).ok()?),
                None,
            )
            .ok()?
        };
        let accessible = ClockAccessible::new(hwnd);
        PROVIDERS.with(|providers| {
            providers
                .borrow_mut()
                .insert(window_id(hwnd), accessible.provider())
        });
        Ok(Self {
            surface: renderer.create_surface(hwnd),
            hwnd,
            taskbar,
            bounds,
            last_text: None,
            accessible,
            shown: false,
            exposed_at: None,
        })
    }

    pub fn window_id(&self) -> u64 {
        window_id(self.hwnd)
    }

    /// Remember what is displayed, for repaints and for the accessible name. No
    /// UIA event is raised: see `accessibility` for why.
    pub fn remember_text(&mut self, text: &clock_core::ClockText) {
        self.accessible.set_text(&text.lines);
        self.last_text = Some(text.clone());
    }

    pub fn set_bounds(&mut self, bounds: PhysicalRect) -> Result<()> {
        // SAFETY: moves our own window without activation or z-order change.
        unsafe {
            SetWindowPos(
                self.hwnd,
                None,
                bounds.left(),
                bounds.top(),
                bounds.width(),
                bounds.height(),
                (SWP_NOACTIVATE | SWP_NOZORDER) as u32,
            )
            .ok()?;
        }
        self.bounds = bounds;
        Ok(())
    }

    pub fn paint(
        &mut self,
        renderer: &Renderer,
        request: &PaintRequest<'_>,
    ) -> std::result::Result<PaintOutcome, RenderError> {
        self.surface.paint(renderer, request)
    }

    fn invalidate(&self) {
        // SAFETY: invalidates our own window; no erase is requested.
        unsafe {
            let _ = InvalidateRect(Some(self.hwnd), None, false);
        }
    }

    /// Once, at least [`CONFIRM_DELAY`] after the overlay was exposed, ask for one
    /// more redraw while it is still shown. Returns whether it did. Call it from
    /// periodic upkeep.
    pub fn confirm_exposure(&mut self) -> bool {
        match self.exposed_at {
            Some(at) if at.elapsed() >= CONFIRM_DELAY => {
                self.exposed_at = None;
                if self.shown {
                    self.invalidate();
                }
                self.shown
            }
            _ => false,
        }
    }

    pub fn is_minimized(&self) -> bool {
        // SAFETY: state query on our own window.
        unsafe { IsIconic(self.hwnd) }.as_bool()
    }

    /// Bring an externally minimized overlay back to its bounds, still hidden,
    /// so the next paint measures the real region instead of 0x0.
    pub fn restore_if_minimized(&mut self) -> Result<bool> {
        let restored = restore_hidden(self.hwnd, self.bounds)?;
        if restored {
            self.shown = false;
            // The old pixels did not survive; never skip the next draw as unchanged.
            self.surface.mark_dirty();
        }
        Ok(restored)
    }

    pub fn is_shown(&self) -> bool {
        self.shown
    }

    pub fn hide(&mut self) {
        // SAFETY: hiding our own window. It keeps its band: a hidden window
        // covers nothing, and a demoted one may not be allowed back (#23).
        unsafe {
            let _ = ShowWindow(self.hwnd, SW_HIDE);
        }
        self.shown = false;
    }

    /// True when the overlay already sits immediately above its taskbar.
    pub fn is_anchored(&self) -> bool {
        if !self.shown {
            return false;
        }
        let root_topmost = topmost(self.taskbar);
        topmost(self.hwnd) == root_topmost
            && matches!(relative_predecessor(self.taskbar, root_topmost), Ok(Some(p)) if p == self.hwnd)
    }

    /// Show (or re-anchor) the overlay directly above its taskbar in the
    /// taskbar's band. Call only after a successful paint: nothing is exposed
    /// until its pixels are correct. Failure hides the overlay.
    pub fn show_anchored(&mut self) -> Result<()> {
        let root_topmost = topmost(self.taskbar);
        let band = band_action(true, root_topmost, topmost(self.hwnd));
        // Hidden now, or hidden below for a band change: about to be exposed.
        let exposing = !self.shown || band != BandAction::Preserve;
        if band != BandAction::Preserve {
            // SAFETY: hide first so temporary promotion is never visible above
            // unrelated topmost windows; only our own HWND changes band.
            unsafe {
                let _ = ShowWindow(self.hwnd, SW_HIDE);
                let target = if band == BandAction::PromoteToTaskbar {
                    HWND_TOPMOST
                } else {
                    HWND_NOTOPMOST
                };
                SetWindowPos(
                    self.hwnd,
                    Some(target),
                    0,
                    0,
                    0,
                    0,
                    (SWP_NOMOVE | SWP_NOSIZE | SWP_NOACTIVATE) as u32,
                )
                .ok()?;
            }
            // SetWindowPos can succeed without changing the band; never show a
            // window that would sit below the taskbar.
            if topmost(self.hwnd) != root_topmost {
                self.hide();
                return Err(windows_core::Error::new(
                    windows_core::HRESULT(0x80004005u32 as i32),
                    "Taskbar-band change ignored",
                ));
            }
        }
        let preceding = relative_predecessor(self.taskbar, root_topmost)?.unwrap_or(HWND_TOP);
        let adjacent = preceding == self.hwnd;
        let insert = if preceding.is_invalid() || adjacent {
            HWND_TOP
        } else {
            preceding
        };
        let keep_order = if adjacent { SWP_NOZORDER } else { 0 };
        let b = self.bounds;
        // SAFETY: positions/shows our own window without activation.
        let positioned = unsafe {
            SetWindowPos(
                self.hwnd,
                Some(insert),
                b.left(),
                b.top(),
                b.width(),
                b.height(),
                (SWP_NOACTIVATE | SWP_SHOWWINDOW | keep_order) as u32,
            )
            .ok()
        };
        match positioned {
            Ok(()) => {
                self.shown = true;
                if exposing {
                    // Pixels drawn while hidden, or kept across a hide, may not
                    // survive; a date-only clock might not repaint for hours (#46).
                    // The WM_PAINT this causes redraws the overlay while visible.
                    self.invalidate();
                    // A redraw straight after showing was observed not to reach
                    // the screen on 26300.9711; confirm it once a little later.
                    self.exposed_at = Some(Instant::now());
                }
                Ok(())
            }
            Err(error) => {
                self.hide();
                Err(error)
            }
        }
    }
}

unsafe extern "system" fn window_proc(hwnd: HWND, message: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    // SAFETY: called by the system for our own windows on the UI thread.
    unsafe {
        match message as i32 {
            WM_MOUSEACTIVATE => LRESULT(MA_NOACTIVATE as isize),
            WM_ERASEBKGND => LRESULT(1),
            WM_GETOBJECT if is_root_object(lp) => {
                let provider = PROVIDERS.with(|p| p.borrow().get(&window_id(hwnd)).cloned());
                match provider {
                    Some(provider) => return_object(&provider, hwnd, wp, lp),
                    None => DefWindowProcW(hwnd, message, wp, lp),
                }
            }
            // A click is a press and a release on this window. Capture pairs them
            // even if the pointer leaves; nothing is forwarded to the native clock.
            WM_LBUTTONDOWN => {
                PRESS.with(|press| {
                    let mut tracker = press.get();
                    tracker.down(window_id(hwnd));
                    press.set(tracker);
                });
                let _ = SetCapture(hwnd);
                LRESULT(0)
            }
            WM_LBUTTONUP => {
                let inside = point_in_client(hwnd, lp);
                let clicked = PRESS.with(|press| {
                    let mut tracker = press.get();
                    let clicked = tracker.up(window_id(hwnd), inside);
                    press.set(tracker);
                    clicked
                });
                let _ = ReleaseCapture();
                if clicked {
                    post_interaction(Interaction::Activate {
                        window: window_id(hwnd),
                    });
                }
                LRESULT(0)
            }
            WM_CAPTURECHANGED | WM_CANCELMODE => {
                PRESS.with(|press| {
                    let mut tracker = press.get();
                    tracker.cancel();
                    press.set(tracker);
                });
                if message == WM_CANCELMODE as u32 {
                    let _ = ReleaseCapture();
                }
                LRESULT(0)
            }
            // Generated by DefWindowProc after a right click, and by touch
            // press-and-hold. The overlay cannot take keyboard focus, so (-1,-1)
            // never arrives from the keyboard; the cursor position is a fallback.
            WM_CONTEXTMENU => {
                let x = i32::from(lp.0 as u16 as i16);
                let y = i32::from((lp.0 >> 16) as u16 as i16);
                let anchor = (lp.0 as i32 != -1).then_some(ScreenPoint { x, y });
                post_interaction(Interaction::Menu { anchor });
                LRESULT(0)
            }
            WM_PAINT | WM_DPICHANGED | WM_DISPLAYCHANGE => {
                if message != WM_PAINT as u32 {
                    crate::events::enqueue(crate::events::Signal::TopologyChanged);
                }
                // Drawing happens in the loop, which owns the surface; the
                // callback only records that the pixels were invalidated.
                REPAINT_EPOCH.fetch_add(1, Ordering::Relaxed);
                let _ = ValidateRect(Some(hwnd), None);
                LRESULT(0)
            }
            // Never forward a close request: the host owns this window's lifetime.
            WM_CLOSE => LRESULT(0),
            _ => DefWindowProcW(hwnd, message, wp, lp),
        }
    }
}

/// Whether a mouse message position (client coordinates, signed words) is inside
/// the window. Capture delivers releases from outside, with negative values.
fn point_in_client(hwnd: HWND, lparam: LPARAM) -> bool {
    let x = i32::from(lparam.0 as u16 as i16);
    let y = i32::from((lparam.0 >> 16) as u16 as i16);
    let mut client = RECT::default();
    // SAFETY: live output buffer; a destroyed window fails and reads as outside.
    unsafe { GetClientRect(hwnd, &mut client) }.as_bool()
        && x >= client.left
        && x < client.right
        && y >= client.top
        && y < client.bottom
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::GetUpdateRect;
    use crate::{render::WorstCase, theme::Palette};
    use clock_core::Preferences;

    /// Re-showing a hidden overlay must not trust pixels drawn while hidden or
    /// kept across the hide: it is left invalid, so WM_PAINT redraws it (#46).
    #[test]
    fn re_showing_an_overlay_requests_a_repaint() {
        let _class = OverlayClass::register().unwrap();
        // SAFETY: STATIC is a system class; this hidden stand-in taskbar is
        // destroyed below, after the overlay.
        let taskbar = unsafe {
            CreateWindowExW(
                WS_EX_TOOLWINDOW as u32,
                w!("STATIC"),
                w!(""),
                WS_POPUP,
                -10_000,
                -10_000,
                200,
                40,
                None,
                None,
                None,
                None,
            )
        }
        .ok()
        .unwrap();
        let renderer = Renderer::new().unwrap();
        let bounds = PhysicalRect::new(-10_000, -10_000, 92, 40).unwrap();
        let mut overlay = Overlay::create(&renderer, taskbar, bounds).unwrap();
        let pending = |hwnd| unsafe { GetUpdateRect(hwnd, None, false) }.as_bool();

        overlay.show_anchored().unwrap();
        assert!(pending(overlay.hwnd), "first show");
        // SAFETY: validates the test's own window, as WM_PAINT handling would.
        unsafe {
            let _ = ValidateRect(Some(overlay.hwnd), None);
        }
        overlay.show_anchored().unwrap();
        assert!(
            !pending(overlay.hwnd),
            "re-anchoring a shown overlay is not a repaint"
        );

        overlay.hide();
        overlay.show_anchored().unwrap();
        assert!(pending(overlay.hwnd), "re-shown after a hide");

        // One confirming redraw, no earlier than CONFIRM_DELAY, and only once.
        // SAFETY: validates the test's own window, as WM_PAINT handling would.
        unsafe {
            let _ = ValidateRect(Some(overlay.hwnd), None);
        }
        assert!(!overlay.confirm_exposure(), "too early");
        assert!(!pending(overlay.hwnd));
        std::thread::sleep(CONFIRM_DELAY);
        assert!(overlay.confirm_exposure());
        assert!(pending(overlay.hwnd), "confirming redraw requested");
        unsafe {
            let _ = ValidateRect(Some(overlay.hwnd), None);
        }
        assert!(!overlay.confirm_exposure(), "only once per exposure");
        assert!(!pending(overlay.hwnd));

        drop(overlay);
        // SAFETY: owned by this test thread.
        unsafe {
            let _ = DestroyWindow(taskbar);
        }
    }

    /// A minimized window measures 0x0, so paint latches on overflow (#46). After
    /// `restore_hidden` the same surface paints again and nothing was shown.
    #[test]
    fn a_minimized_window_is_restored_hidden_and_paints_again() {
        let bounds = PhysicalRect::new(100, 100, 92, 60).unwrap();
        // SAFETY: STATIC is a system class; the window is destroyed below.
        let hwnd = unsafe {
            CreateWindowExW(
                (WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE) as u32,
                w!("STATIC"),
                w!(""),
                WS_POPUP,
                bounds.left(),
                bounds.top(),
                bounds.width(),
                bounds.height(),
                None,
                None,
                None,
                None,
            )
        }
        .ok()
        .unwrap();
        let renderer = Renderer::new().unwrap();
        let mut surface = renderer.create_surface(hwnd);
        let prefs = Preferences::default().validate().unwrap();
        let text = clock_core::ClockText {
            lines: vec!["12:34".into()],
        };
        let worst_case = WorstCase::for_format(prefs.format());
        let request = PaintRequest {
            text: &text,
            worst_case: &worst_case,
            dpi: 96,
            font_pt: 9,
            palette: Palette::DARK,
        };
        assert_eq!(
            surface.paint(&renderer, &request).unwrap(),
            PaintOutcome::Painted
        );
        assert!(!restore_hidden(hwnd, bounds).unwrap(), "not minimized yet");

        // SAFETY: minimizes the test's own window without activating it.
        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
        }
        assert!(unsafe { IsIconic(hwnd) }.as_bool());
        assert!(matches!(
            surface.paint(&renderer, &request).unwrap(),
            PaintOutcome::Overflow(overflow) if overflow.available.width == 0
        ));

        assert!(restore_hidden(hwnd, bounds).unwrap());
        let mut rect = RECT::default();
        // SAFETY: live output buffer for the test's own window.
        unsafe { GetWindowRect(hwnd, &mut rect) }.unwrap();
        assert!(!unsafe { IsIconic(hwnd) }.as_bool());
        assert!(!unsafe { IsWindowVisible(hwnd) }.as_bool());
        assert_eq!(
            (rect.left, rect.top, rect.right, rect.bottom),
            (100, 100, 192, 160)
        );
        surface.mark_dirty();
        assert_eq!(
            surface.paint(&renderer, &request).unwrap(),
            PaintOutcome::Painted
        );

        drop(surface);
        // SAFETY: owned by this test thread.
        unsafe {
            let _ = DestroyWindow(hwnd);
        }
    }
}
