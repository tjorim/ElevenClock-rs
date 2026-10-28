ElevenClock-rs (managed deployment)

For IT administrators. A customizable clock for the Windows 11 taskbar (stock
Windows 11 x64). The executables are not code-signed, so Windows may warn about
the download.

What this package is
  The same app as the installer and the portable ZIP, plus
  elevenclock-rs-launcher.exe. The app is stored as elevenclock-rs.data so that
  nobody starts it from the share by accident; the launcher copies it to the user's
  machine as elevenclock-rs.exe. Put the ElevenClock-rs folder on a central location,
  such as a file share. Users start elevenclock-rs-launcher.exe from there (for
  example through a shortcut). The launcher copies the app to the user's own
  %LOCALAPPDATA%\Programs\ElevenClock-rs, starts that local copy and exits. The app
  never runs from the share. SHA256SUMS.txt lists the hash of the app; the launcher
  refuses an app that does not match it, so replace both files together.

Updating
  Replace the files in the central folder. Each user's copy is refreshed the next
  time they start the launcher or sign in, never while the app is running. The
  central copy always wins, so putting an older build there rolls users back.
  The app has no update settings in this mode and never checks for updates itself.

Startup
  Startup is off by default and stays the user's choice in Settings > Startup.
  When it is on, Windows starts the local launcher at sign-in, so that the copy is
  refreshed first. Turning it off in Settings removes the entry.

Do not mix with the installer
  The installer and the launcher use the same local folder. Setup refuses to run
  while a managed copy is present, and the launcher refuses to run while the
  installer's copy is present. Remove one before using the other.
  To remove a managed copy, run elevenclock-rs-launcher.exe --uninstall from the
  central folder. It deletes only the managed files and the startup entry.

Where data lives
  Preferences and logs are in %LOCALAPPDATA%\ElevenClock-rs. The launcher writes
  launcher.log there too. Use Settings > Support > Copy diagnostics when reporting a
  problem; the report says whether the copy is managed.

Licenses
  LICENSE.txt contains Apache License 2.0 for the Rust implementation and the
  application artwork. THIRD-PARTY-NOTICES.txt and RUST-COPYRIGHT.html retain
  dependency/toolchain notices. The source is the tagged commit at
  https://github.com/tjorim/ElevenClock-rs.
