# Clock state and scheduling (pure core)

`State::new(ValidatedPreferences)` starts at shell generation zero, without a
wall snapshot or taskbars. `reduce(state, event) -> (state, Vec<Effect>)` is pure;
it reads no platform time and owns no native resources. `overlay_state(key)`
returns Unavailable for absent or stale keys. Effects have stable transient-ID
ordering within each reconciliation phase.

## Input contract and transitions

| Input | Transition / effects |
| --- | --- |
| TopologyChanged | Request discovery once while pending; preserve unchanged overlays until the authoritative result. |
| DiscoveryResult | Accept only the current generation and matching keys; reject duplicate keys atomically. Batch is authoritative: destroy removed keys, preserve existing identities/fullscreen, add new keys. |
| VisibilityChanged | Update one current-generation taskbar; unknown destroys, hidden hides, visible reconciles eligibility. |
| FullscreenChanged | Update one current-generation taskbar; fullscreen hides, leaving fullscreen reconciles eligibility. |
| SessionLocked | Hide existing visible overlays, create none. |
| SetPaused(bool) | User paused/resumed replacement from the clock/tray menu. Paused hides every overlay (`Hidden(Paused)`), creates none and keeps windows; resume repaints once per visible clock. Session-only, unaffected by Explorer loss. |
| SessionUnlocked | Invalidate old visibility and time; request discovery. Runtime supplies a fresh Tick/TimeChanged as well as discovery before showing. |
| Tick / TimeChanged | Replace wall snapshot for all clocks, render once, paint changed text only; replace the one-shot timer even for repeated snapshots. |
| ThemeChanged { revision } | A distinct resolved appearance revision dirties overlays; repeated revisions do nothing. Hidden overlays defer painting. |
| PreferencesApplied(validated) | Commit the validated value; compare preferences, reconcile monitor selection, repaint changed text/font/theme, rearm changed cadence. |
| ExplorerLost | Destroy overlays, invalidate keys, increment generation, request discovery. Repeated loss while awaiting recovery does nothing. Generation exhaustion terminates safely. |
| Quit | Destroy overlays, terminal state; runtime cancels timer and discovery work. Late inputs do nothing. |

Visibility precedence is Unavailable (missing/invalid bounds, zero DPI, unknown
visibility, missing/unrenderable time), then Hidden(SessionLocked),
Hidden(Paused) for unselected monitors or a user pause, Hidden(TaskbarHidden),
Hidden(Fullscreen), then Visible. Proven clock-only bounds are supplied by discovery;
the reducer also checks taskbar/monitor containment defensively. New hidden
entries have no window. Existing hidden windows are retained until removal,
invalid geometry/unknown visibility, shell loss or quit.

## Runtime effect contract

- Discover(generation): one bounded outstanding discovery per generation. Serialize
  discovery responses; coalesce topology notifications while pending. Generation
  is an Explorer identity, not a request sequence. The runtime must retry failed
  discovery and refresh topology after an in-flight discovery if topology changed
  during that request; same-generation responses cannot be ordered by this API.
- CreateOverlay: allocate a hidden UI-thread window using the committed preferences
  and resolved appearance. Never show on creation. Duplicate keys are a reducer bug.
- DestroyOverlay: release the keyed native overlay; do not reuse its key in another
  generation. SetBounds changes physical geometry for an existing window.
- Paint: draw supplied text with current committed font/theme/resolved appearance
  and show that eligible window. Showing a previously hidden window requires Paint
  even when text is unchanged. Hide retains the window and its last paint.
  The reducer records the requested frame without a native success acknowledgement.
  The runtime therefore retries overflow, render and placement faults for
  reducer-visible clocks on its two-second recovery poll, with fresh local time,
  even when formatted text has not changed. Failed moves hide the old window
  immediately; placement recovery recreates it hidden at the current discovered
  bounds before painting. Native repaint/upkeep failures enter the same diagnostics
  and recovery path. Hidden, unavailable and stale-generation clocks are never
  retried. Calendar-action faults still require Pause/Resume or Explorer recovery.
- ArmTimer: replace the outstanding one-shot monotonic wait. The timer callback
  reads a fresh OS-local snapshot once and dispatches Tick. Time-change/resume
  notifications read a fresh snapshot and dispatch TimeChanged; unlock also needs
  discovery. Never advance wall time by the wait, assume 24-hour days, or infer DST
  rules from a FixedOffset. Quit cancels timers outside core.

`next_delay(snapshot, cadence)` returns a strictly positive duration to the next
local second/minute boundary. Date-only formats recheck every minute. Leap-second
subseconds are normalized to a one-second fraction. Identical time events have
idempotent overlay effects but rearm timers because the prior one-shot may have
fired. Theme revisions are equality tokens; platform high-contrast/palette changes
must produce a distinct token even when the preference remains System.

## Test fixtures

`clock-core/tests/state.rs` uses negative-coordinate dual-taskbar fixtures,
unknown/invalid geometry, removal, fullscreen, lock/unlock, stale and malformed
batches, Explorer replacement, theme/config changes, midnight, forward/backward
jumps and explicit spring/fall fixed-offset DST inputs. Property tests check
boundary waits and effect/window lifetime invariants over generated event sequences.
No test sleeps or reads live time. No Windows UI, hooks, COM, actual timer, or
application event loop is part of the core.
