# Windows acceptance gates (R13)

Status: **started, not passed.** Partial sessions on one machine. The release gate
stays **open**: most of the matrix, the eight-hour soak and the second build have not
been run. The three ten-minute runs passed for candidate `246a5a4` on 2026-10-21
([Performance](#performance)).
Nothing below is inferred from CI, simulation or another build.

Follow-up for [#41](https://github.com/tjorim/ElevenClock-rs/issues/41): the
secondary-clock discovery failure in finding 3 was fixed in
[PR #50](https://github.com/tjorim/ElevenClock-rs/pull/50) (`fc2434a`). The
[captured UIA inventory and fix record](issue34-26300/README.md) show that this
build's split secondary clock has a separate bell but no Show Desktop button.
Discovery now requires Show Desktop only on primary taskbars; split clocks still
require the bell, shell ownership, an unambiguous candidate and validated geometry.
With **All displays** selected in Appearance, both clocks were located and replaced.
The [independent follow-up](issue34-26300/follow-up/README.md) records settled
visual coverage, native pointer ownership and secondary auto-hide reveal on the
same two-display, 96-DPI guest configuration. It also records failed Explorer
restart recovery checks. This resolves the recorded selector failure, without
passing the broader support or release gates. The original observations and
matrix below remain a historical record of `940634a252c1`.

Follow-up for [#42](https://github.com/tjorim/ElevenClock-rs/issues/42): a
[deterministic reproduction and recovery fix](issue42-overflow.md) demonstrate
that a transient overflow can stay latched while ISO week text remains unchanged.
The runtime now retries eligible overflowed clocks on its bounded recovery poll.
Native renderer tests pass for default → ISO week Apply and recovery after a
temporary small region. The original first-Apply overflow trigger remains unknown;
this does not turn the original observation into a reproduced desktop failure.

## Environment actually used

| What | Value |
| --- | --- |
| App commit | `940634a252c1` (`rust-rewrite`, R12 merged), release build, `x86_64-pc-windows-msvc`, `--locked` |
| OS | Windows 11 Home 10.0.**26300.9711** (26H2), x64 |
| CPU | AMD Ryzen 7 5800H, 8 logical processors |
| Machine | Virtual machine with two Red Hat QXL display adapters |
| Displays | 2 × 1280×720, both 96 DPI (100%); second at (1280, 0) |
| Theme | Default light |

Only **one** Windows build was available. The issue requires two exact stock Windows 11
x64 builds; the second is **not run**.

## Observed

Driven through the real desktop (computer use), no terminal for the user-facing steps:

- Launching the release exe starts one process and the Settings window opens from the
  taskbar button (**ElevenClock-rs Settings**, four tabs: Date and time, Appearance,
  Startup, Support).
- Preset dropdown → *ISO week date* → **Apply** shows "Settings applied."; the preview
  updates (`Now: 2642.5`).
- Support → *Copy diagnostics* copies the report; build, commit and per-taskbar status are
  recorded by the app itself:

```text
App version: 0.1.0 / App commit: 940634a252c1 / Windows: 10.0.26300.9711 (26H2)
Clock replacement: Windows clock in use
1. Primary taskbar: Windows clock in use: the clock text does not fit this taskbar.
   Monitor 1280x720 at (0, 0), DPI 96. Discovery: UI Automation; clock located: yes
2. Secondary taskbar: Windows clock in use: could not verify where the Windows clock is.
   Monitor 1280x720 at (1280, 0), DPI 96. Discovery: UI Automation; clock located: no
   "This Windows version may not be supported yet"
```

## Findings

1. **Primary taskbar: the clock is replaced on 10.0.26300.9711.** The app log shows
   `primary taskbar 1: clock replaced` at launch, and after Apply of *ISO week date*
   (`%g%V.%u`) from the default state the taskbar shows `2642.5` in place of the Windows
   clock, confirmed on screen. *(An earlier version of this file said the clock was not
   replaced; that was wrong. It was read during the transient state in finding 2.)*
2. **Open, possible stuck state:** on first Apply after launch the primary taskbar logged
   `Windows clock in use: the clock text does not fit` (09:21:58 UTC) and stayed that way
   until the user reset the app's preferences (09:26:25 UTC, `clock replaced`). A repeat of
   the same Apply from the clean state did **not** reproduce it. Cause unknown; needs a
   reproduction before it is closed or filed. The Settings Apply message said
   "Settings applied." throughout.
3. **Secondary taskbar: clock not located** (`could not verify where the Windows clock is
   on this taskbar ... This Windows version may not be supported yet`; UI Automation,
   clock located: no). The native clock is correctly left alone. This is a
   support-narrowing finding for 26300.9711 on a second display until fixed. An earlier
   run (08:08 UTC) logged the secondary as "not used: this display is not selected in
   Settings", so selection also changes the outcome; the diagnostics above were taken with
   it selected.
4. **Test runs disturb a live desktop.** `cargo test --workspace` opens UI Automation
   fixture windows (`elevenclock-rs-uia-fixture`) that take focus and intercept input on
   the machine under test. Do not run it during interactive checks.

## Recovery (executed on this VM)

| Check | Result |
| --- | --- |
| Explorer restart ×3 (`Stop-Process explorer`, 12 s apart, app left running) | **Pass.** Same app PID throughout; handles 269 → 270 → 270 → 270, threads 3. After each restart the log shows `taskbar 1/2: gone`, then re-discovery. After the third the primary clock was `replaced` again. |
| Forced app kill (`Stop-Process -Force` on the app) | **Pass.** Process gone, native Windows clock `11:46 16/10/2026` visible again on the primary taskbar (screenshot). No app window left. A leaked-hook check beyond "no window, clock back" was not done. |
| Quiet start (`--startup`) | **Pass.** One process, no Settings window, primary clock replaced within 8 s. |

Observed during the restarts, not yet judged: taskbar numbering swaps between runs (the
log shows `secondary taskbar 1` / `primary taskbar 2` after the second restart), and the
secondary taskbar is never verified (finding 3). Neither left the native clock hidden.

## DPI

**Not run: the VM cannot do it as configured.** The primary display is 1280×720 and
Windows disables **Scale** there (greyed at "100% (Recommended)"). The monitor lists
modes up to 1280×1024 only, which would allow at most 125%, so 150% and 200% are
unobtainable without a larger virtual display set on the VM host. The display mode was not
changed. 125% remains possible at 1280×1024 with a layout change; not attempted.

## Performance

`scripts/measure-rust-resources.ps1` (development-only) samples every
`elevenclock-rs*` process and reports CPU-time delta ÷ (wall time × logical processors),
private working set (`WorkingSetPrivate`), handles and threads. The date-only row below
was taken with its first version.

For [#46](https://github.com/tjorim/ElevenClock-rs/issues/46) the sampler was extended
so the remaining runs and the soak give usable evidence:

- **Helper-safe accounting.** Processes are tracked by PID and start time. One that
  starts between samples counts all of its CPU. One that exits loses only the CPU used
  since its last sample, and that loss is reported as an accounting gap. Previously,
  summed cumulative CPU went wrong whenever the process set changed.
- **Process lifetimes** (`*-processes.csv`: present/started/exited with start time and
  path).
- **Soak counters:** GDI and USER objects and top-level windows per sample, plus a
  start/end/min/max/mean and least-squares slope per hour for every counter.
- **No lost samples.** Rows are appended as they are taken. If the app exits, the run is
  interrupted or sampling fails, the summary is still written with `complete: false`
  and exit status 1. Previously an app exit threw before the CSV was written.
- **Candidate identification** (`*-environment.json`): executable path and SHA256,
  checkout commit, `rustc -V`, Windows edition/build/UBR/architecture, CPU and logical
  processors, machine, theme, system DPI and physical monitor bounds.
- `-Workload` records the format and any transitions with the run, and
  `instrumentationGaps` lists what the script cannot see (timer/discovery rates and
  hooks; take those from Support diagnostics).

Validation on 2026-10-21, at `246a5a4`, on the same VM: a 45 s run against the release
build with a stand-in `elevenclock-rs-fakehelper.exe` started and exited mid-run. The
sampler recorded the helper's start and exit, the process count going 2 → 1 and one
gap interval, with `complete: true`. This validated the script only; it is **not** a
performance measurement.

### Ten-minute runs at `246a5a4` (2026-10-21)

**New reference configuration.** On 2026-10-21 the VM had one display at 1536×1106
physical with system DPI 120 (125%), not the two 1280×720 displays at 96 DPI recorded
above. This single 125% display was declared the reference configuration for #46. All
three modes were measured on it with a clean release build of `246a5a4` (SHA256
`ADD4B581…48EEDE`; Support reports `App commit: 246a5a439c6f`). The clock replaced the
native clock throughout: Support showed `Status: Active` and the log has no transition
after `clock replaced` at 17:06:58 UTC. Formats were chosen in native Settings. Raw
CSV/JSON, process lifetimes, environment and full details are in
[`performance-246a5a4/`](performance-246a5a4/README.md).

| Mode | Wall | CPU, % of machine (8 logical) | Private WS max | Handles | Threads | GDI / USER | Processes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Date-only (`%g%V.%u`, ISO week preset) | 608.7 s | **0.0116%** | **3.01 MiB** | 230 → 228 | 5 → 3 | 10 / 13 flat | 1 |
| Minutes (`%H:%M`) | 602.5 s | **0.0110%** | **3.39 MiB** | 271 → 272 | 7 → 5 | 25 / 22 flat | 1 |
| Seconds (`%H:%M:%S`) | 606.9 s | **0.0080%** | **2.71 MiB** | 272 flat | 5 → 3 | 25 / 22 flat | 1 |

**All three pass** the < 0.2% CPU and < 40 MiB private working set targets on the
reference configuration, with no growth within any run. The step in handles, GDI/USER
objects and windows between date-only and minutes comes from first opening the Settings
window before the minutes run, and stays flat afterwards. The build has no helper
processes.

**Instrumentation gap:** the Support report has no timer-wake or discovery-rate counters,
so those rates were **not measured**. Hooks are not externally observable either.

### Earlier date-only run at `940634a252c1` (history)

Two 1280×720 displays at 96 DPI; superseded for the candidate by the run above.

| Mode | Wall | CPU, % of machine (8 logical) | Private working set | Handles | Threads | Processes |
| --- | --- | --- | --- | --- | --- | --- |
| Date-only (`%g%V.%u`, ISO week preset), primary clock replaced, secondary not | 601.6 s | **0.0088%** | 2.89 → 2.05 MiB, max 2.93 | 224 → 223 | 6 → 5 | 1 |

### Finding: overflow latched after a full-screen hide (2026-10-21)

With a `246a5a4+modified` build (script and doc edits uncommitted; app code identical)
and the ISO week format, the clock was replaced at 17:04:12 UTC. At 17:04:40 it was
correctly hidden because a full-screen window (the maximized Claude desktop app, during
screen control) was in front. When it came back at 17:04:53, it went to Windows clock
in use with this log line:

```text
taskbar 1 overflow: required=10x1px, available=0x0px, discovered slot=87x60px, DPI=120, font=9pt
```

The overflow retry from the #42 fix runs every 2 s, but the clock was still latched about
40 s later. It recovered only when the app was restarted.

**Root cause, reproduced on demand:** the overlay window had been **minimized from
outside the app**. The screen-control tool minimizes windows of processes that are not on
its allowlist, and this build was not on it yet. A minimized window has a 0×0 client
area. The renderer then reports `required = 2 × padding × 1 px` (10×1 at 120 DPI) against
`available=0x0px`, and the overlay is hidden. Nothing ever restores it, so every retry
measures 0×0 again. Calling `ShowWindow(overlay, SW_SHOWMINNOACTIVE)` from another
process reproduces the exact log line. Restoring the window (off screen, hidden, then
back to its bounds) lets the existing retry recover it within 2 s. A second, silent
variant: minimizing also sends `WM_PAINT`, and the invalidation repaint then hid the
overlay **without recording a fault**. With a date-only format, the clock stayed gone
until the next date change while Support still said "replaced".

A fix is in progress: the overlay restores itself hidden before painting or redrawing,
then shows again only after a successful paint. It has a native regression test, and it
was verified on this VM with seconds and date-only formats: three minimize cycles each,
recovered in ≤ 0.1 s, no overflow logged.

### Finding: overlay left behind when the tray re-lays out (2026-10-21)

Parts of the native clock showed beside the overlay: `21/1` on its left, or digits and
the year on its right. Live UI Automation dumps showed the cause. The overlay matches
the clock button (`SystemTray.OmniButton`) exactly, but only as discovery measured it.
Turning do-not-disturb on adds a separate bell button (`SystemTray.OmniButtonRight`)
between the clock and Show Desktop. That moves the clock 35 px left (x 1429–1516 →
1394–1481) inside an unmoved taskbar. The runtime watched only the taskbar window, so
the overlay stayed on the old spot until something else triggered discovery. During
that time it hid the bell and exposed part of the native clock. Turning do-not-disturb
off reverses the move.

A 3-minute recorder while toggling do-not-disturb showed that the legacy
`TrayNotifyWnd` child tracks every move. Its left edge went 1220 → 1255 → 1220 in step
with the clock (1394 → 1429 → 1394). The runtime now reads that rectangle on each
2-second observation pass, relative to the taskbar, with no UI Automation. It records
the rectangle when discovery is requested, and requests one throttled rediscovery
when it differs. The overlay stays up meanwhile, so tray changes that do not move the
clock cause no flash. A moved clock gets new bounds from the result.

Verified on this VM: toggling do-not-disturb off, then on, moved the overlay over the
whole native clock within 5 s each time. One second after each toggle the stale
position was still visible, which is expected with the 2 s poll, settle and helper
run. Final overlay x 1394–1481 = clock button x 1394–1481. Runtime tests cover the
single rediscovery, no loop, and a change during discovery.

Not covered: secondary taskbars (`Shell_SecondaryTrayWnd`) are not expected to have a
`TrayNotifyWnd` (not checked: this VM has one display). Where it is missing, the
observation reports none and a re-layout still waits for the next discovery. The notification
bell is deliberately left uncovered when it is its own button, so its glyph stays visible
to the right of the overlay.

Also from this session: a maximized window that covers the whole monitor, including the
taskbar, counts as full screen. The clock then stays hidden, which is correct.
Measurements need a desktop where no window covers the taskbar.

## Matrix status

Checked boxes mean actually executed here. Everything else is **not run**.

- [x] Release build at an exact commit; Settings opens from the taskbar; preset edit,
      preview, Apply; Support status and Copy diagnostics
- [ ] Second stock Windows 11 build
- [x] Clock replacing the native clock on the primary taskbar (not on the secondary,
      finding 3)
- [ ] Single display; primary swap / hotplug; negative origins (second monitor is at +1280
      only)
- [ ] Mixed DPI 100/125/150/200%
- [ ] Auto-hide taskbar; fullscreen per monitor
- [ ] Native calendar and neighbour clicks
- [ ] Narrator
- [ ] Lock/resume; time and timezone changes
- [x] Explorer restart ×3; forced app kill reveals native clock; quiet `--startup` launch
- [ ] Repeated launch/Settings cycles; invalid stored preferences; overflow
- [x] Ten minutes each: date-only, minutes, seconds, all pass at `246a5a4` on the
      single-display 125% reference (2026-10-21); timer/discovery rates not measured
- [ ] Eight-hour soak
- [ ] Cancel/Reset, startup choice, closing Settings and Exit as one walkthrough from the
      Start menu, and a non-administrator account
- [x] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D
      warnings`, `cargo test --workspace --locked`: all exit 0 at `940634a`

## Handoff

Do not close #14. Supported builds: **none yet**. Finding 3's selector failure is
resolved by the follow-up above; Explorer recovery remains open. Next: reproduce
finding 2, then run the matrix on this VM at mixed DPI and on a second build, then the
three ten-minute runs and the eight-hour soak with the script above.

[#46](https://github.com/tjorim/ElevenClock-rs/issues/46) owns those runs, the soak,
the second build and the final GO/NO-GO. Done so far: the sampler work and the three
ten-minute runs at `246a5a4`. Still open: the eight-hour soak, the second build, the
matrix gaps and the overflow-after-full-screen finding. The release decision is still
**NO-GO / not assessed**. Commands (choose each format through native Settings first,
with one app instance running and no window covering the taskbar):

```powershell
./scripts/measure-rust-resources.ps1 -DurationSeconds 600 -IntervalSeconds 10 -Label date -Workload 'ISO week date (%g%V.%u)'
./scripts/measure-rust-resources.ps1 -DurationSeconds 600 -IntervalSeconds 10 -Label minutes -Workload '%H:%M'
./scripts/measure-rust-resources.ps1 -DurationSeconds 600 -IntervalSeconds 10 -Label seconds -Workload '%H:%M:%S'
./scripts/measure-rust-resources.ps1 -DurationSeconds 28800 -IntervalSeconds 10 -Label soak -Workload '<format and transitions>'
```
