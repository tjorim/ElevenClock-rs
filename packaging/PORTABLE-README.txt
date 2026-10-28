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
  ElevenClock-rs does not check for updates. To update, exit the app, then
  replace elevenclock-rs.exe with the one from the new portable ZIP. Your
  preferences are kept.

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
