ElevenClock-rs

A customizable clock for the Windows 11 taskbar (stock Windows 11 x64).
The executable and installer are not code-signed, so Windows may warn about
the download.

The installer installs for the current user, without elevation, to
%LOCALAPPDATA%\Programs\ElevenClock-rs. Open ElevenClock-rs from Start.
Do not run legacy ElevenClock at the same time.

Settings opens on interactive launch. Closing Settings leaves the clock running.
Use Exit ElevenClock-rs in Settings or the tray menu to quit and reveal the
Windows clock. Startup is off by default; enable it in Settings > Startup.
Use Settings > Support > Copy diagnostics when reporting a problem.

Exit the app before updating, uninstalling or restoring an older installer.
Uninstall through Windows Settings > Apps > Installed apps. Private preferences
and logs in %LOCALAPPDATA%\ElevenClock-rs are preserved. The legacy ElevenClock
installation and configuration are never removed. A portable copy's startup
registration is preserved when it points outside this installation.

If your organization manages ElevenClock-rs on this PC, Setup stops and does not
change that copy; the managed copy and this installer cannot share an account.

No Python or Qt runtime is included. The MSVC CRT is statically linked. No
elevated prerequisite installer is run.

LICENSE.txt contains Apache License 2.0 for the Rust implementation and the
application artwork. ElevenClock-rs is inspired by the original ElevenClock by
Marti Climent; its GPLv3 Python app is not part of this package and remains in
the project's Git history.
THIRD-PARTY-NOTICES.txt and RUST-COPYRIGHT.html retain dependency/toolchain
notices. The source is the tagged commit at
https://github.com/tjorim/ElevenClock-rs. SHA256 checksums are published
alongside the release.
