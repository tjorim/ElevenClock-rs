# Tested configurations and known limitations

What ElevenClock-rs has actually been run on, and what has not been checked. Raw
screenshots, logs and measurements from development are in Git history, not in the tree.

## Reference machine

Windows 11 Home 10.0.26300.9711 (26H2) x64 in a virtual machine, one 1536×1106 display at
125%, release build (`x86_64-pc-windows-msvc`). Earlier development runs also used
Windows 11 10.0.26200 and two 1280×720 displays at 100%. The maintainer additionally used
the app for a full workday on a physical laptop and handled the multi-display, mixed-DPI
and Explorer-recovery checks on their own hardware.

## Checked

| Area | Result |
| --- | --- |
| Settings | Opens from the taskbar and Start; presets, preview, Apply, Cancel, Reset and Exit; invalid stored preferences recovered (7/7); overflow rejected in Settings; 20/20 repeated launch/Settings cycles with flat handle/GDI/USER counts |
| Clock replacement | Primary taskbar on 26300.9711; split secondary clock (separate bell, no Show Desktop) located and replaced |
| Taskbar states | Auto-hide reveal 3/3; full-screen window hide/restore; tray re-layout (do-not-disturb bell) followed within 5 s |
| Neighbours | Native calendar opens on click; bell and Show Desktop clicks unaffected |
| Session and time | Lock/unlock; time-zone changes and clock jumps (the clock matched local time to the second afterwards) |
| Recovery | Explorer restart ×3 with the same process and flat handles; a forced kill of the app reveals the native clock; quiet `--startup` launch |
| Startup and install | Per-user install, Start menu launch, startup on/off/repair without UAC on a standard account; update, uninstall and rollback between two development builds |
| Support | Status, Refresh and Copy diagnostics; the report contains no user name or path |
| Accessibility | UIA name, role and Invoke on the clock; keyboard access to Settings and the tray menu; Settings layout at 100/125/150/200% and in high contrast (CI) |

## Resource use

Ten minutes per format on the reference machine (8 logical processors), release build:

| Format | CPU (% of machine) | Private working set |
| --- | --- | --- |
| Date only (`%g%V.%u`) | 0.0116% | 3.01 MiB |
| Minutes (`%H:%M`) | 0.0110% | 3.39 MiB |
| Seconds (`%H:%M:%S`) | 0.0080% | 2.71 MiB |

No growth within any run. To measure again, choose the format in Settings with one app
instance running, then:

```powershell
./scripts/measure-rust-resources.ps1 -DurationSeconds 600 -IntervalSeconds 10 -Label minutes -Workload '%H:%M'
```

The script cannot see timer-wake or discovery rates.

## Not checked, or known limitations

- **Other Windows builds.** Only 26300.9711 (and 26200 during development) were run.
  Other stock Windows 11 x64 builds are assumed to work. If discovery cannot confirm
  the clock, the native clock is left alone.
- **Narrator speech.** The UIA data and events it relies on are verified; nobody has
  listened to it.
- **Actual sign-out/sign-in startup** and a clean first-run machine without developer
  runtimes.
- **Secondary taskbars** have no `TrayNotifyWnd`, so a tray re-layout there waits for the
  next discovery.
- **Tests disturb a live desktop.** `cargo test --workspace --locked` opens UI Automation fixture
  windows that take focus. Do not run it during interactive checks.
- **External minimize.** A tool that minimizes the overlay from another process (some
  screen-control tools do) used to latch an overflow; the overlay now restores itself, but
  only this one tool was tried.
