ElevenClock-rs (portable)

A customizable clock for the Windows 11 taskbar (stock Windows 11 x64).
The executable is not code-signed, so Windows may warn about the download.

Getting started
  Extract the ElevenClock-rs folder to a folder you can write to, then
  double-click elevenclock-rs.exe. Do not run legacy ElevenClock at the same
  time.

  Settings opens on interactive launch. Closing Settings leaves the clock
  running. Use Exit ElevenClock-rs in Settings or the tray menu to quit and
  reveal the Windows clock.

Startup
  Startup is off by default; enable it in Settings > Startup. It registers this
  exact copy of the executable. If you move or rename the folder afterwards,
  Settings shows startup as missing and offers to repair it.

  Deleting the folder does not remove the startup entry. Turn startup off in
  Settings first, then delete the folder.

Updating
  Updates are never installed automatically. Use Check now in Settings, or opt in
  to automatic update checks. To update, exit the app, then replace
  elevenclock-rs.exe with the one from the new portable ZIP. Your preferences
  are kept.

Templates
  Settings > Templates can share format and appearance links and save named local
  presets. Select Enable template links there to open elevenclock:// links with
  this copy, without administrator rights. Disable them before moving/deleting
  the executable. Review an imported template, then Apply to change the clock.
  Sharing includes format literals, font size and theme; startup stays local.

Where your data lives
  Preferences and logs are in %LOCALAPPDATA%\ElevenClock-rs, not in the
  portable folder. Deleting the folder does not remove them. The legacy
  ElevenClock installation and configuration are never touched.
  Use Settings > Support > Copy diagnostics when reporting a problem.

Licenses
  LICENSE.txt contains Apache License 2.0 for the Rust implementation and the
  application artwork. THIRD-PARTY-NOTICES.txt and RUST-COPYRIGHT.html retain
  dependency/toolchain notices. The source is the tagged commit at
  https://github.com/tjorim/ElevenClock-rs.
