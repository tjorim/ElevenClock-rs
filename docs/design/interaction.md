# Clock actions, tray and accessibility

## Action behavior table

| Source | Result | Notes |
| --- | --- | --- |
| Left press **and** release inside an overlay | One calendar action: a single `SendInput` batch Win down, N down, N up, Win up | Mouse capture pairs press and release; release outside the slot or lost capture does nothing |
| A second activation within 400 ms | Ignored | Win+N toggles the flyout; a double click must not open and close it |
| Activation while Ctrl, Shift, Alt, Win or N is physically held | Nothing sent, overlay unchanged, cooldown not consumed | A held modifier would form a different shortcut; the user's keys are never released by the app |
| `SendInput` inserts fewer than four events | Accepted key-downs are released, then **all overlays are hidden** and `Reason::ActionFailed` is recorded; no retry | The native clock stays reachable. Pause then Resume (or an Explorer restart) tries again |
| Right click / touch press-and-hold on the overlay | Fixed menu at the pointer | `WM_CONTEXTMENU`, one menu per gesture |
| Tray icon: context menu, mouse select, keyboard select (`NIN_SELECT`, `NIN_KEYSELECT`, `WM_CONTEXTMENU`) | The same fixed menu at the icon's anchor | Win+B reaches the notification area from the keyboard; a menu that just closed is not reopened by the second event Windows may send |
| Menu **Settings** | `Platform::open_settings` → loop → `SettingsWindow::show_or_focus` | Never a second window |
| Menu **Pause clock replacement** / **Resume clock replacement** | Core `Event::SetPaused`: every overlay is hidden (native clock shows) / shown again; tooltip reads "ElevenClock (paused)" | Session only: not persisted, a new process starts running |
| Menu **Exit** | `Quit`: overlays, tray icon, hooks and helper are removed; native clock untouched | Same path as the Settings Exit button |
| UIA **Invoke** on the overlay element | Same as a left click (queued to the UI thread) | See below |
| Anything while paused, fullscreen-hidden, faulted, or for a stale generation | Ignored | The runtime re-checks the reducer's `OverlayState` before acting |

Out of scope and absent: arbitrary commands, double/middle-click customization, keyboard
shortcuts beyond standard menu navigation, a duplicated notification counter, shell
injection, synthetic mouse relay. The menu has exactly three commands (`MenuCommand`).

## Calendar strategy

On tested builds, UIA `Invoke` on the native clock returned success without opening the
flyout, while one deliberate Win+N opened it.
The production action is therefore the deliberate keyboard batch only. It runs on the UI
thread, needs no COM and cannot block the message loop; UIA is not used to *discover* or
*invoke* the native clock from the click path. If a later build proves a supported UIA
invoke, only `Platform::open_calendar` changes.

## UI Automation provider

* One element per overlay window (`accessibility.rs`, `IRawElementProviderSimple` +
  `IInvokeProvider`, server-side, host provider from the HWND).
* **Name** = displayed text in reading order (`"12:34, 14/10/2026"`, trimmed, at most 256
  characters). **ControlType** = Button. **AutomationId** = `ElevenClock.Clock`. Help text
  "Opens the calendar and notifications". Not keyboard focusable.
* **Pattern choice: Invoke only.** There is no Value pattern (a second copy of the same
  text) and **no UIA event is ever raised**, so a per-second format cannot produce a
  stream of announcements; a screen reader reads the current text on demand.
* UIA calls the provider on its own thread. The provider holds only a mutex-guarded
  string, an atomic liveness flag and a window id; `Invoke` posts an interaction to the
  UI thread and returns. After `UiaDisconnectProvider` (called before the window is
  destroyed) every method returns `UIA_E_ELEMENTNOTAVAILABLE`.
* A hidden overlay (paused, fullscreen, fault, Explorer loss) is not in the UIA tree, so
  Explorer's own clock element is the only one a screen reader sees.

## Keyboard access

The overlay is non-activating, so it cannot be tabbed to. Keyboard users reach Settings,
Pause/Resume and Exit through the **notification-area icon** (registered with
`NOTIFYICON_VERSION_4`; keyboard activation and the context-menu key open the same menu),
and through Start-menu relaunch for Settings. The Win32 popup menu provides arrow keys,
Enter and Escape and native UIA/MSAA exposure.

## Explorer restart

`Runtime` calls `Platform::restore_tray` once at start and from `check_shell`, the single
existing path run for `TaskbarCreated` and for a changed Explorer PID/creation time. There
is no timer or retry of its own. If Explorer is not ready, `Shell_NotifyIconW(NIM_ADD)`
fails quietly and the `TaskbarCreated` broadcast that follows tries again; a repeated
restore uses `NIM_MODIFY` and never stacks a second icon.

## Hooks for Settings, Support and UI tests

* **Settings:** the menu's Settings item calls `Platform::open_settings`, which sets
  the loop's existing `SHOW_SETTINGS` flag; `pump` then calls
  `SettingsWindow::show_or_focus` after the current input. `WM_SHOW_SETTINGS` (second launch) uses the same flag.
* **Support:** `Runtime::diagnostics()` carries `paused` and, per taskbar,
  `Reason::ActionFailed(reason)` with the exact `SendInput` failure text.
* **UI tests:** `AutomationId == "ElevenClock.Clock"`; overlay class
  `ElevenClock.Overlay`; tray icon id 1 on the host window class `ElevenClock.Runtime.Host`;
  menu item ids 1..3.

## Known limitations

* The menu is modal: while it is open the overlay clock does not repaint (timers queue
  and run when it closes).
* Narrator may see both the overlay element and Explorer's own clock element while the
  overlay is shown; not de-duplicated without modifying Explorer.
* A UIA client that invokes the element cannot learn that the later calendar action
  failed (the call returns once queued); the failure is visible in diagnostics.
* Menu strings are fixed English.
* Pause is deliberately not persisted.
