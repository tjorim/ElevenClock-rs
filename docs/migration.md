# Moving from ElevenClock (Python) to ElevenClock-rs

ElevenClock-rs is a new app, not an update of the original ElevenClock. It installs
separately, keeps its own settings and never reads, changes or removes anything the
original app left behind. Nothing here needs a terminal or a hand-edited file.

## Before you start

The two apps must not run at the same time: both draw over the same taskbar clock.

1. **Quit the original ElevenClock.** Right-click its clock or tray icon and choose
   **Quit ElevenClock**. The Windows clock shows again.
2. **Stop it starting at sign-in.** Open Windows **Settings > Apps > Startup** and
   switch **ElevenClock** off. This is the Windows switch, so turning it back on later
   restores your old choice exactly.
3. Leave the original app installed if you might want to go back. Uninstalling it is
   your decision; ElevenClock-rs does not need it removed.

Your original settings stay in place, untouched. ElevenClock-rs does not import them,
so choose your format again in its Settings window.

## What carries over and what does not

| Original ElevenClock | ElevenClock-rs |
| --- | --- |
| Custom date/time format | Re-enter it in **Settings > Date and time**, or pick a preset. Plain date/time tokens are supported; the token list beside the editor shows which |
| Clock on every display | **Settings > Appearance > All displays** (or primary display only) |
| Font size, light/dark clock | **Settings > Appearance**: font size 6–24 pt, theme System / Light / Dark |
| Start at sign-in | Off by default. Turn it on in **Settings > Startup** |
| Click the clock for the calendar | Same: left-click opens the Windows calendar and notifications |
| Right-click menu | Settings, Pause / Resume clock replacement, Exit |

Removed, with no replacement planned:

- Internet time synchronisation and time-server checks. Windows remains the only time
  source; the app never changes the system time.
- Notification counters, focus-assist indicators and other copies of Windows tray
  status.
- Custom gestures, double/middle-click actions and running commands or scripts.
- Format expressions: arithmetic, time offsets, extra time zones and HTML styling.
- Clocks placed anywhere other than over the taskbar clock, or more than one per display.
- Choosing a font, custom colours, background tints and opacity. The clock uses the
  Windows UI font and follows the taskbar's colours.
- Translations: the app is in English only.
- In-app installation and telemetry. Self-managed copies can **Check now** or
  opt into **Check for updates automatically** in Settings (off by default,
  including upgrades). Automatic checks offer one tray notification per newer
  version; clicking opens its release page. Installation remains manual.
  Managed copies use their launcher and have no GitHub update checks.
- Importing settings from the original app.

## Supported Windows

Stock Windows 11 on x64, with the taskbar at the bottom of the screen. The exact
builds that were tested are listed in
[tested configurations](tested-configurations.md).

Not supported: Windows 10, ARM64 devices (no ARM64 package is built), Windows Insider
builds, taskbar replacements such as ExplorerPatcher or StartAllBack, and taskbars on
the side or top of the screen. When the app cannot confirm where the Windows clock
is, it draws nothing there and the Windows clock stays visible.

## Going back

- **To the Windows clock:** choose **Exit ElevenClock-rs** in Settings or the tray
  menu. The Windows clock was never hidden or changed, so it shows immediately.
- **To the original ElevenClock:** exit ElevenClock-rs (or uninstall it in
  **Settings > Apps > Installed apps**), then switch **ElevenClock** back on in
  **Settings > Apps > Startup** and start it from the Start menu. Its settings are
  where you left them.
- **To an earlier ElevenClock-rs:** uninstall, install the earlier installer you kept,
  and start it from the Start menu. Your ElevenClock-rs settings are kept across
  uninstall. If the earlier version cannot read them, use its **Reset to defaults**.

## Legacy reference

The Python/Qt app, its build scripts and workflows, its translations and artwork, and
the rewrite planning documents were removed from the tree at cutover; the development
evidence (Windows run records, screenshots, measurements) followed later. Nothing was
deleted from Git history.

| What | Commit |
| --- | --- |
| Upstream ElevenClock the rewrite started from | [`f967218e95b6`](https://github.com/marticliment/ElevenClock/tree/f967218e95b6f3c15e26f840e6090a6720528e39) |
| Last `main` with the Python app at the repository root | [`b4da4b549f3c`](https://github.com/tjorim/ElevenClock-rs/tree/b4da4b549f3cf3b246eb81efdbb49a4e827fcf41) |
| Last commit with the Python app under `legacy/` and the rewrite plan under `docs/rewrite/` | [`0ac31152e265`](https://github.com/tjorim/ElevenClock-rs/tree/0ac31152e265bb5d6e02fcac18404362453670d2) |
| Last commit with the development evidence under `docs/windows-evidence/` | [`1a08a7a776b0`](https://github.com/tjorim/ElevenClock-rs/tree/1a08a7a776b0e8474beb604affc243da40d501e2/docs/windows-evidence) |

That code keeps its GPLv3 license and original notices (`legacy/LICENSE` at
`0ac31152e265`). It is not part of ElevenClock-rs releases.
