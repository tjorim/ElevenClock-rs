# R13 matrix checks on the reference VM (#46)

Run on 2026-10-21, 18:45–18:57 UTC, on the reference configuration declared in
[`../performance-246a5a4/`](../performance-246a5a4/README.md): Windows 11 Home
10.0.26300.9711 x64 VM, one display 1536×1106 at 125%, light theme. Primary-only
selection, format `%g%V.%u`.

| Build | Value |
| --- | --- |
| Rust code | identical to `4f586f5` (both overlay fixes) |
| Reported commit | `157fa1bf8fb5+modified`: built before the fixes were committed; `git diff 4f586f5 HEAD -- rust` is empty |
| Executable SHA256 | `5A0CCB500986271708EE4A3CAA6A5F85FA43A6B99D2A926B74DCDDD20F7B1533` |
| Account | interactive administrator account, **non-elevated** (Medium integrity); not a separate non-administrator account |

## Results

| Check | Result | How |
| --- | --- | --- |
| Single display | **Pass** | Replacement, Support and all checks below ran on a single display. Inspection: 368/368 pointer samples over the clock hit the overlay. |
| Auto-hide taskbar | **Pass, 3/3** | `scripts/test-rust-autohide.ps1 -Repeats 3`. Auto-hide was turned on through the appbar API, the taskbar parked, then the pointer revealed it. Taskbar shown after 111–130 ms; overlay hit at the clock centre after 2843, 112 and 112 ms. Auto-hide was restored to off. The first reveal was slow because enabling auto-hide made the runtime see the taskbar as gone. One discovery helper then stopped unexpectedly (`Some(4)`) and discovery recovered 3 s later. `autohide.json`, `primary-*.png`. |
| Full-screen window | **Pass** | A borderless window covered the monitor. The log shows `clock hidden for now: a full-screen app is in front` within 1 s, then `clock replaced` when it closed (18:47:57 → 18:48:05). |
| Native calendar click | **Pass** | Clicking the overlay opened the notification/calendar flyout; clicking it again closed it. |
| Neighbour clicks | **Pass** | The separate bell button opened and closed the notification centre. Show Desktop and restore kept the overlay visible in two clean repeats (one fresh, one right after the bell). See the observation below. |
| Tray re-layout | **Pass** | Windows' automatic do-not-disturb rule turned on during the full-screen test and moved the clock. The overlay followed: overlay x 1394–1481 = clock button x 1394–1481. |
| Invalid stored preferences | **Pass, 7/7** | Garbage bytes, empty file, truncated JSON, unknown format token (`%Q`), font size 999, version 99 and wrong JSON types. Each time: one process, clock replaced with defaults, damaged file left untouched. With Settings open, it says "Saved preferences are damaged. Defaults are in use…". Cancel kept the damaged file. Original preferences restored afterwards. `damaged-preferences-settings.jpg`. |
| Overflow | **Pass** | Applying `%A %d %B %Y %H:%M:%S` in Settings was refused: "does not fit … needs 190 × 14 px, 87 × 60 px available) even at 6 pt", "Settings were not applied", clock unchanged. The runtime overflow path is covered by the minimize finding in `../acceptance.md`. `overflow-rejected-settings.jpg`. |
| Repeated launch/Settings cycles | **Pass, 20/20** | Second launch → Settings shown (median 354 ms) → `WM_CLOSE` hid it. One process throughout, and every second launch exited 0. Handles 274, threads 5, GDI 25, USER 22–23 and private memory 6.03–6.07 MiB were flat from cycle 1 to 20. |
| Accessible name (Narrator proxy) | **Pass (UIA only)** | The overlay is a UIA Button named with the displayed text (`2643.3`), automation id `ElevenClock-rs.Clock`, Invoke pattern, correct bounds. Hit-testing at the clock centre returns it. Narrator speech was not heard or verified. |
| Reset/Cancel/Exit | **Pass** | Reset to defaults filled in the default preset, and Cancel discarded it (preferences file unchanged). Exit removed the process; native clock visible; log `ElevenClock-rs stopped`. |
| Startup choice | **Pass** | The Startup tab correctly reported that the existing login entry starts a *different* copy (the main checkout's build) and offered Repair. Later in the session, with the maintainer's go-ahead and on build `ef94dcfbc05f` (same Rust code): Repair pointed `HKCU\…\Run\ElevenClockRust` at this copy with `--startup`; unticking removed the value ("does not start when you sign in"); ticking recreated it; Cancel left it as is, as the tab states. The original entry was restored afterwards. Actual sign-in startup is owned by #15. |

## Observation: overlay not drawn after Show Desktop (not reproduced)

Once, right after the bell test, Show Desktop left the overlay window visible,
topmost directly above the topmost taskbar, uncloaked and at the right bounds. Yet
screen captures showed the native clock, and it stayed that way after restoring the
windows, until the app was restarted. The log just before had a full-screen hide and
show in the same second (18:48:56). During diagnosis a cross-process
`DwmSetWindowAttribute(DWMWA_EXCLUDED_FROM_PEEK)` was called on the overlay, so it is
unknown whether it would have recovered by itself. Two clean repeats passed. Open
until reproduced or ruled out.

## Not done here, and why

- **Lock/unlock:** done afterwards by the maintainer; see `../acceptance.md`.
- **System time and time-zone changes:** done afterwards by the maintainer; see `../acceptance.md`.
- **Mixed DPI 100/125/150/200%, hotplug, primary swap, negative origins:** need display
  topology or scale changes on a single-display VM. 125% alone is covered.
- **Start-menu walkthrough, actual sign-in startup and a separate non-administrator account:** owned by #15
  (installed build). Not run here.
- **Narrator speech:** UIA exposure verified only.
