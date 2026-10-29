# Runtime, monitors and Explorer recovery

There is no support gate: the discovered clock candidate always places the clock after
geometric validation. A failed, late or stale discovery fails closed to an
authoritative empty batch, which removes the overlays and leaves the native clock
untouched.

## Components

| Item | Where | Contract |
| --- | --- | --- |
| `Runtime<P: Platform>` | `clock-windows/src/runtime.rs` | Single coordinator between platform inputs and the core reducer. No second state machine. Pure of OS access; 30 deterministic tests with a scripted platform (plus shell, protocol, gate, candidate, event and time tests). |
| `NativePlatform`, `run` | `runtime/native.rs`, `runtime/overlay_window.rs` | One UI thread: hidden top-level host window, `SetTimer` one-shots, WinEvent hooks, session notifications, overlay windows, Direct2D, Settings dialog. |
| `events` | `events.rs`, `events/native.rs` | Window message / WinEvent → `Signal`; coalescing `Pending`; bounded `TransitionPoll`. Callbacks only queue. |
| `instance` | `instance.rs`, `instance/native.rs` | `Local\ElevenClock-rs.Instance.<SID>` mutex, `HOST_CLASS`/title lookup, `WM_SHOW_SETTINGS`, `Role`/`LaunchMode`. |
| `taskbar::{protocol,worker,helper,candidate}` | `taskbar/` | Wire contract, supervised worker, same-executable helper, candidate selection. See [ADR 0002](../adr/0002-same-executable-discovery-helper.md). |
| `clock_time` | `clock_time.rs` | One fresh zone/DST-resolved `DateTime<FixedOffset>` per update. |
| App lifecycle | `clock-app/src/app.rs`, `settings.rs`, `settings.rc` | Modeless Settings; close keeps running; **Exit** button; second launch focuses. |

## Event-to-core mapping

| Source | Signal | Runtime action → core event |
| --- | --- | --- |
| `WM_DISPLAYCHANGE`, `WM_DPICHANGED`, `WM_SETTINGCHANGE` ("TraySettings" or `SPI_SETWORKAREA`), WTS console/remote connect | `TopologyChanged` | 250 ms trailing settle, then `TopologyChanged` (one discovery per burst; one follow-up if topology changed while one was in flight) |
| `WM_THEMECHANGED`, `WM_SYSCOLORCHANGE`, `WM_SETTINGCHANGE` (`ImmersiveColorSet`, other) | `AppearanceMaybeChanged` | re-read system appearance + font; `ThemeChanged{revision}` only on a real change |
| `WM_TIMECHANGE` | `TimeChanged` | fresh local snapshot → `TimeChanged` |
| `WM_POWERBROADCAST` suspend / resume | `Suspended` / `Resumed` | resume: `TimeChanged`, shell check, appearance, topology |
| `WM_WTSSESSION_CHANGE` lock / unlock | `SessionLocked` / `SessionUnlocked` | `SessionLocked`; unlock: `SessionUnlocked` (reducer requests discovery) + fresh `TimeChanged` + shell check |
| registered `TaskbarCreated` | `TaskbarCreated` | forced shell re-identification → `ExplorerLost` (generation + 1, old overlays destroyed, new discovery) |
| WinEvent show/hide/location (Explorer PID only, `Shell_*TrayWnd` only) | `Shell(TaskbarChanged)` | start bounded transition polling; observe → `VisibilityChanged` / rediscover |
| WinEvent foreground / minimize-end / move-size-end (system-wide, low rate) | `Shell(ForegroundChanged)` | observe → `FullscreenChanged` per monitor |
| `WM_ENDSESSION`, `WM_CLOSE` on host, Settings **Exit** | `EndSession` / exit flag | `Quit` |
| `WM_TIMER` (clock one-shot) | — | one fresh snapshot → `Tick`; reducer returns `ArmTimer` |
| `WM_TIMER` (poll one-shot) | — | shell identity check, due retries / settle, observation pass |
| worker report | `Discovery(report)` | accept only the current generation *and* matching shell identity; failure → authoritative empty batch (fail closed) + backoff |

## Fallback polling rates and timers

* **Recovery poll: 2 s** (documented in the dissection). One `SetTimer` one-shot; cheap
  checks only: shell identity (PID + creation time), then one observation per taskbar.
* **Transition poll: 100 ms**, begun by a taskbar WinEvent, ended after two unchanged
  observations or **15 polls**, then back to 2 s.
* **Topology settle: 250 ms** trailing edge. **Observation-triggered rediscovery
  throttle: one per 2 s**, and a given odd rectangle triggers at most one.
* **Discovery retry backoff:** 2, 4, 8, 16, 32, then 60 s; any real topology event
  resets it. An empty batch while Explorer runs is retried the same way.
* The clock timer is a single monotonic one-shot aligned to the next local
  second/minute boundary; a re-armed poll timer is never postponed by clock ticks.
  There is no high-frequency loop.

## Shutdown and failure semantics

`Quit` destroys every overlay (reducer effects), cancels both timers, unhooks every
WinEvent hook, unregisters session notifications, terminates and reaps any helper,
joins the worker (bounded), destroys the host window and unregisters classes. A panic
in a window procedure or hook becomes a clean shutdown with an error box. A crash
removes all windows and (via the kill-on-close job) any helper. Explorer is never
modified, injected into or parented to, so the native clock is always usable.

## Lifecycle contract

Interactive launch (Start menu, double-click) opens Settings; login startup
(`--startup`, the marker the startup registration uses) runs quietly. A second launch
posts `WM_SHOW_SETTINGS` to the existing host window (lookup by class and
SID-derived title, bounded retry for simultaneous launches) after
`AllowSetForegroundWindow`, and exits 0 without creating a clock. Closing Settings
(Close, Escape, X, system menu) destroys only the dialog. **Exit ElevenClock** in
Settings or the tray/clock menu, a polite `WM_CLOSE` to the host, or session end quits.

## What the tests prove (and do not)

* `runtime::tests` (scripted platform): start sequence; all/primary monitors with
  negative coordinates; clock/time/DST-style jumps use
  fresh snapshots; topology bursts and in-flight refresh; `TaskbarCreated`, PID reuse
  and missing Explorer; stale-generation and wrong-shell rejection; failure →
  fail-closed → bounded backoff → recovery; fatal teardown disables discovery;
  auto-hide, relocation (no loops), DPI/unplug, primary swap, per-monitor fullscreen,
  lock/unlock, appearance/font, live preference changes, overflow/render/placement
  reasons, quit idempotence, 50 consecutive Explorer replacements without overlay
  accumulation.
* `tests/discovery_worker.rs` (real child processes, every host; Windows adds the job
  object and a parent-loss test): current-generation publish, foreign generation,
  malformed/oversized/crashing helpers, **deadline discards a complete-looking partial
  report and the helper is dead before the result is published**, superseding
  generation kills the running helper and only the newest publishes, request bursts
  coalesce, shutdown kills and joins, and **60 mixed failures neither grow threads,
  handles (±2) or memory (+16 MiB) nor leave helpers or temporary files**.
  The helper is a *fixture process* (this test executable): it exercises supervision,
  not a UI Automation provider.
* `tests/uia_lifetime.rs` adds real UIA/provider lifetime and opt-in Explorer
  correlation checks (see below). Overlay visibility, z-order
  and DPI require the separate desktop harnesses.

## UIA lifetime

`tests/uia_lifetime.rs` (custom harness; CI sets `ELEVENCLOCK_RS_UIA_EXPLORER=1` and
`ELEVENCLOCK_RS_EVIDENCE_DIR`) records raw per-call HRESULTs for a healthy window, a
provider that never pumps (hits the 4 s deadline with one helper spawned and reaped), a
provider killed mid-call, elements retained across provider death
(`UIA_E_ELEMENTNOTAVAILABLE`) and a real Explorer suspended and killed mid-call.

What shaped the design: a UIA call in flight when the shell dies is **not**
guaranteed to fail with a useful HRESULT (an earlier attempt returned `S_OK`), so the
parent never trusts a late or successful-looking helper result — it relies on the
generation/shell-identity check and the helper deadline. Retained elements always fail
`0x80040201`, so no element is cached across a generation. On the hosted Server image
Windows did **not** restart the shell after `taskkill`; the harness started `explorer.exe`
itself (`shell_started_manually: true`, no taskbar for 45 s), so this proves correlation
and stale-object behaviour, not Windows' own shell auto-restart timing.

## Known limitations

* Overlay creation failure is reported (`PlacementFailed`) but the reducer keeps its
  view of that overlay until the next Explorer loss or topology change.
* DPI or monitor changes invalidate visibility immediately, releasing the old
  cover before discovery completes. A fresh batch recreates and repaints it even
  when the physical clock rectangle is unchanged.
* The overlay does not forward clicks; clock actions, the menu, tray icon and UIA
  element are in [interaction](interaction.md).
* Auto-hide classification (partly off-monitor = hidden; fully on-monitor but
  different = relocated) is a heuristic.
* Hook coverage and the 2 s poll are the only defence if an event is missed; hook
  count is exposed for diagnostics (`Notifications::hook_count`).

## Parked and late taskbars

Parked taskbars are classified as hidden even when their rectangle equals the
discovery snapshot. Each taskbar is observed independently; revealing a taskbar
whose clock could not be validated schedules fresh discovery, including a reveal
with unchanged window bounds. Offscreen clock candidates remain rejected.

The observation pass also compares the current shell's top-level taskbar handles
with the last discovery batch. This cheap native inventory uses no UI Automation
and detects a secondary taskbar created after a partial Explorer-startup batch,
even if its event was missed. Requests share the existing two-second throttle,
settle window and worker; failed discovery retains its exponential backoff.
Overlay DPI/display messages now queue topology signals as well as invalidating
pixels. Stale geometry releases the cover before the bounded helper finishes.

Visible, selected, on-monitor clocks that remain unverified after a successful batch
get four bounded settling retries (2, 4, 8, 16 seconds); unsupported layouts do not
create an endless discovery loop.

## Optional update checks

The runtime supplies managed mode, lock/pause/suspend/exit eligibility, a resume
sequence, UTC seconds and monotonic time to the portable `UpdateRuntime` policy.
The UI-thread update service owns the single worker and uses the Settings session
for serialized persistence even when no Settings dialog exists. A dedicated one-shot
host timer wakes at the next opportunity, with at least three minutes of startup or
resume grace and at least 24 hours between attempts. No timer is armed by default
or in managed mode; existing clock and shell timers do not initiate requests.

Both manual and automatic requests use the same bounded worker and coordinator.
Lock, suspend and pause cancel work and remove the automatic timer. Turning consent
off cancels automatic work; Exit cancels either kind. Cancelled workers retain their
in-flight slot until their bounded completion, preventing overlap, and cannot notify.
Shutdown drops the worker without joining network operations. Explorer replacement
does not reset the update schedule or start a check. Rate-limit deadlines and
notification markers survive restart; failed automatic checks are quiet. See
[ADR 0003](../adr/0003-update-checks.md) and [preferences](preferences.md).
