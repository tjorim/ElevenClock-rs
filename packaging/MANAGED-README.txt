ElevenClock-rs (managed deployment: user guide)

A customizable clock for the Windows 11 taskbar (stock Windows 11 x64).
Your organization manages this copy. For deployment and reporting instructions,
see README-MANAGER.txt. Neither everyday use nor the launcher needs Windows
administrator privileges or UAC elevation.

Getting started
  Use the shortcut or launcher supplied by your organization. If you were directed
  to this folder, open elevenclock-rs-launcher.exe. Leave elevenclock-rs.data alone;
  the launcher copies the app to your account and runs it locally. After the first
  successful launch, you can start offline using the local launcher:
    %LOCALAPPDATA%\Programs\ElevenClock-rs\elevenclock-rs-launcher.exe
  Sign-in startup also uses that local launcher. A shortcut to an unavailable
  network share cannot start until the share is accessible again.
  Do not run legacy ElevenClock at the same time.

  Settings opens when you launch interactively. Choose your clock format and
  appearance, then select Apply. Closing Settings leaves the clock running.
  Use Exit ElevenClock-rs in Settings or the tray menu to quit and reveal the
  Windows clock.

Startup
  Startup is off by default and stays your choice in Settings > Startup.
  Turn it on to start the clock when you sign in; turn it off to stop that behavior.

Templates and local presets
  Settings > Templates lets you share your clock format, font size and theme,
  review templates from the website or other people, and save named local presets.
  Select Enable template links there to open elevenclock:// links for your account.
  Review stages a draft; Apply changes the clock. Startup and organization settings
  stay local. Shared links include any personal text you put in your clock format.

Updates and removal
  Your deployment manager supplies updates. The launcher checks the central copy
  on launch or sign-in; it does not replace a running app. There are no update
  settings in this managed copy. Your clock preferences are kept during updates.
  Ask your deployment manager about removal or switching to another package.
  Do not install the normal installer alongside this managed copy.

Your data and optional organization reporting
  Preferences and logs are stored in %LOCALAPPDATA%\ElevenClock-rs.
  Organization reporting is off unless your deployment manager enables it.
  When enabled, launcher observations can include your Windows account name/SID,
  a generated installation ID, observation time, app/launcher versions and fixed
  support codes. Saved clock-format text, including any personal text you put in
  it, is included only if your manager separately enables format reporting.

  Reports are saved locally and may be copied to your organization's configured
  reporting folder. They are not sent to the app's developers. Reporting describes
  launcher observations, not live activity or time spent using the app. It stops
  collecting when the central configuration is disabled or cannot be read.
  Existing reports can remain after reporting is disabled or the app is removed.
  Ask your deployment manager what is enabled, who can read reports, and about
  retention or deletion. These reporting options are managed by your organization,
  rather than a setting in the clock's Settings window.

Getting help
  If no local copy has been deployed yet, connect to your organization's network
  or VPN and try the supplied shortcut again. For other launch or update problems,
  contact your deployment manager. Windows may warn because the files are unsigned;
  follow your organization's software policy.
  Use Settings > Support > Copy diagnostics when reporting a problem. That report
  contains no user name or paths and says whether the copy is managed.

Licenses
  LICENSE.txt covers the Rust implementation and artwork (Apache License 2.0).
  THIRD-PARTY-NOTICES.txt and RUST-COPYRIGHT.html contain dependency/toolchain notices.
  Source: https://github.com/tjorim/ElevenClock-rs
