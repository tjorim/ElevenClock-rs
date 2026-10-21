# #51 checks on the reference VM, 2026-10-21

These checks ran on the single-display reference VM from
[`../performance-246a5a4/`](../performance-246a5a4/README.md):

- Windows 11 Home (EditionID `Core`), 26H2, 10.0.26300.9711, x64.
- One display, `0,0–1536,1106`, at 120 DPI (125%), light theme.
- Primary display only selected; auto-hide off.
- Interactive administrator account, non-elevated.

This is **not** the two-display 96-DPI QEMU guest where #51's Explorer failures were
recorded, so it cannot reproduce or clear them.

| Build | Value |
| --- | --- |
| Commit | `88d17d6a0b5e` (`rust-rewrite` after #64), unmodified |
| Executable | `rust/target/release/elevenclock-rs.exe` |
| SHA256 | `A0F82E7ECFE6B39434A75D38CC4C8C7F1D9DFD854E73885A7FC5D7A3D1F39879` |

## Results

| #51 item | Result here | Evidence |
| --- | --- | --- |
| Explorer recovery | **Pass on one display, 10/10 restarts.** The two-display failure is not reproduced and stays open. | `lifecycle-first-run.json`, `lifecycle-recovery-timing.json`, `runtime.log` |
| Independent UIA identity (clock, Show Desktop, neighbours) | **Pass, 3 runs** (2 retained) | `uia-1/`, `uia-2/` |
| Separate bell | **Not applicable on this layout.** This build's primary tray uses the merged `SystemTray.OmniButton`, which has no bell button. The split-clock bell is still unverified here. | `uia-*/uia-identity.json` |
| Unknown/ambiguous bounds fail closed | **Not run on the desktop.** The only coverage is the deterministic candidate tests. | — |
| Live DPI 150↔100%, primary reveal variants, sleep, time change, monitor removal, mixed-DPI pair, second stock build, ADR review | **Not run** | Each needs a DPI or display-topology change, hardware, a privilege or a human decision |

### Explorer recovery on one display

The run used `scripts/test-rust-lifecycle.ps1 -ExplorerRestarts 5 -IdleSeconds 5
-ObserveOverlays`, twice. The harness now records, for each restart:

- when Explorer's taskbar returned;
- the moment the visible overlay count got back to the count at start;
- every change in that count.

These times are measured from the forced Explorer kill.

| Restart | Taskbar back | Overlay back |
| --- | --- | --- |
| 1 | 3568 ms | 7935 ms |
| 2 | 3574 ms | 8155 ms |
| 3 | 3555 ms | 8007 ms |
| 4 | 3575 ms | 8327 ms |
| 5 | 3558 ms | 8373 ms |

Results from `lifecycle-recovery-timing.json`:

- **Same process throughout.** The run never had more than one helper at a time.
- **Exit code 0.**
- **First run (`lifecycle-first-run.json`).** It had no latency timer. All five
  samples taken 10 s after the taskbar returned showed the overlay back.

`runtime.log` shows the same sequence on every restart:

1. The taskbar is gone.
2. About 3–4 s later, the first discovery runs and cannot verify the clock yet.
3. The taskbar is gone again, and the clock is replaced 1–3 s after that.

There were no helper deadlines. This is unlike the two-display guest, where repeated
five-second deadlines held the overlays off for about 57–68 s.

`-RecoveryBoundSeconds N` makes recovery a failing check. No bound has been
reviewed yet, so the runs here are informational.

### Independent UIA identity

`scripts/verify-rust-uia-identity.ps1` takes these steps:

1. Starts the app quietly and waits until the overlay positions are stable.
2. Captures the taskbar, then stops the app so the native tray is uncovered, and
   captures it again.
3. Uses managed UI Automation to read every `SystemTrayIcon` button: class, name,
   AutomationId, process and bounds, plus each button's parent chain up to the
   taskbar.
4. Hit-tests a 4-pixel grid over each button, including its right and bottom edges,
   with `IUIAutomation::ElementFromPoint`.

None of this uses the app's own selector or helper.

Clock checks:

- **Exactly one clock.** Exactly one `SystemTray.OmniButton` has a
  `TimeInnerTextBlock` child.
- **Owned by Explorer.** Its process is the Explorer that owns `Shell_TrayWnd`.
- **Parent chain.** Its parents are `InputSite.WindowClass` <
  `DesktopWindowContentBridge` < `Shell_TrayWnd`.
- **Matches the overlay exactly.** Both are at `1429,1046–1516,1106`.

Neighbour checks:

- **No overlap.** No neighbour overlaps the overlay. The neighbours are Show Hidden
  Icons, the input indicator, Network, Volume (`OmniButtonRight`) and Show Desktop.
- **Required neighbour present.** Show Desktop, which a primary taskbar requires.

Point checks (identical in all runs):

| Button | Hits | Edge misses | Interior misses | Hit another button |
| --- | --- | --- | --- | --- |
| Clock (overlay rectangle) | 352/368 | 16 | 0 | **0** |
| Show Hidden Icons | 176/176 | 0 | 0 | 0 |
| Input indicator | 224/240 | 16 | 0 | 16 |
| Network | 128/144 | 16 | 0 | 0 |
| Volume | 128/144 | 16 | 0 | 0 |
| Show Desktop | 80/80 | 0 | 0 | 0 |

All the misses are in each button's left column. UIA rounds bounds from fractional
DIPs at 125%, so that pixel column belongs to the gap or to the adjoining button.

- **Clock.** Its 16 left-edge points hit the empty `InputSite` gap, never a
  neighbour. The overlay therefore covers no other button.
- **Input indicator.** Its left column at x 1294 belongs to Show Hidden Icons, whose
  rounded right edge is also 1294.

The check accepts edge misses but no interior misses, and no overlay point may hit
another button. Screenshots: [with overlay](uia-1/taskbar0-overlay.png) and
[native](uia-1/taskbar0-native.png).

### Two findings for the five-second discovery deadline

**The managed UIA client breaks native hit testing.** After the managed
`System.Windows.Automation` client has been used, `ElementFromPoint` in that process
returns `Shell_TrayWnd` for every tray point. The same call in a process that never
loaded the managed client resolves the XAML buttons. The script therefore hit-tests
in a child process.

**Dense hit testing is slow.** On this display, one `ElementFromPoint` takes about
17–26 ms, so the 1,152 points took 26–30 s. That alone rules out dense point identity
inside the app's five-second discovery helper. The shipped selector walks the tree
instead; the tree-walking `taskbar_probe` took 0.35–0.8 s here.

The proposed support position is to keep dense identity checking as an
out-of-process harness, not a runtime step. **This still needs the maintainer's
review**; it does not by itself close that #51 item.

## Cleanup

- The user's startup copy was stopped for the runs and restarted afterwards with
  `--startup`.
- No display, DPI, auto-hide, format or startup setting was changed.
- The redactions are the user-scoped SID in the host-window name and the absolute
  workspace path.
