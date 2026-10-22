# R13 ten-minute resource runs at `246a5a4` (#46)

Raw output of `scripts/measure-rust-resources.ps1` for the three ten-minute runs. Every
run ended with `complete: true` and exit status 0. Paths are rewritten relative to the
checkout; nothing else is changed.

## Candidate and reference configuration

| What | Value |
| --- | --- |
| Commit | `246a5a439c6f27ebc032a69c4988b376fd9c1ef0`, built from a clean detached worktree (Support report: `App commit: 246a5a439c6f`, no `+modified`) |
| Build | `cargo build --release --locked -p clock-app`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `x86_64-pc-windows-msvc` |
| Executable SHA256 | `ADD4B581A32F9AE83BD339548CE8436A1C358F25772F45D5AE36DE0E0F48EEDE` |
| OS | Windows 11 Home (Core) 10.0.26300.9711 (26H2), AMD64 |
| CPU | AMD Ryzen 7 5800H, 8 logical processors |
| Machine | QEMU Standard PC (Q35 + ICH9) virtual machine |
| Display | **One** display, 1536×1106 physical at (0, 0), 120 DPI (125%) |
| Theme | Light (apps and system) |

On 2026-10-21 this single 125% display was **declared the new reference configuration**.
The earlier R13 date-only row came from two 1280×720 displays at 96 DPI with
`940634a252c1`. It is kept as history and does not count for this candidate.

Support at the end of the runs: `Status: Active`, one primary taskbar `clock replaced`,
monitor 1536×1106 at DPI 120, discovery UI Automation, clock located: yes. The app log shows
`primary taskbar 1: clock replaced` at 17:06:58 UTC and no later transition through all three
runs. The taskbar text was checked on screen at the start of each run (`2643.3`, `19:18`,
`19:28:52`). Formats were chosen in native Settings: ISO week date preset, then custom
`%H:%M`, then `%H:%M:%S`, applied with OK. One app instance and no helper process.
The desktop was idle and no tests ran.

## Results (10 s interval, app process only; no helper existed)

| Mode | Files | Wall | CPU, % of machine | Private WS max (start → end) | Handles | Threads | GDI / USER | Top-level windows |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Date-only `%g%V.%u` | `20261021-190741-date*` | 608.7 s | **0.0116%** | **3.01 MiB** (2.97 → 2.96) | 230 → 228 | 5 → 3 (max 6) | 10 / 13 flat | 3 flat |
| Minutes `%H:%M` | `20261021-191822-minutes*` | 602.5 s | **0.0110%** | **3.39 MiB** (3.39 → 2.68) | 271 → 272 (max 273) | 7 → 5 | 25 / 22 flat | 4 flat |
| Seconds `%H:%M:%S` | `20261021-192858-seconds*` | 606.9 s | **0.0080%** | **2.71 MiB** (2.70 → 2.60) | 272 flat | 5 → 3 | 25 / 22 flat | 4 flat |

All three runs are below the **< 0.2% CPU** and **< 40 MiB** targets on this reference, by
more than an order of magnitude. Seconds mode measured slightly lower CPU than date-only.
At these levels (about 0.6–0.9 ms of CPU per second across 8 logical processors), a
10 s-interval sampler cannot meaningfully separate the modes.

The higher handle, GDI/USER and window counts in the minutes and seconds runs started
after the Settings window was first opened (between the date and minutes runs). They stay
flat within each run, so they are the hidden Settings window's resources, not growth.

## Instrumentation gaps

- Timer-wake and discovery rates: the Support report has no rate counters (it shows
  status, monitor, DPI and discovery strategy only). They were **not measured**.
- Hooks: not externally observable. GDI/USER objects and top-level windows are the proxy.
- No helper processes exist in this build, so the helper start/exit accounting was not
  exercised by these runs. It was validated separately with a stand-in process.
