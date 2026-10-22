# ElevenClock-rs

![ElevenClock-rs icon](assets/icon.png)

A customizable clock for the Windows 11 taskbar. It shows your own date and time
format, such as the ISO week date `2642.2` (2026, week 42, Tuesday), in place of the
taskbar clock on one or every display. The Windows clock is never changed or
disabled; it sits underneath, so exiting always brings it straight back.

- **Light.** About 3 MiB of memory and around 0.01% CPU while running
  ([measured](docs/tested-configurations.md#resource-use)). There is no busy
  loop: the clock updates only when its text changes, plus a cheap taskbar check
  every 2 seconds.
- **Self-contained.** One native executable: no Python, Qt or Visual C++ runtime to
  install.
- **Stays out of the way.** Installs for your account without administrator rights,
  starts at sign-in only if you turn that on, and never connects to the internet.

## Install

You need stock Windows 11 on an x64 PC. No administrator rights are needed.

1. Download `ElevenClock-rs-…-x64-setup.exe` from the
   [latest release](https://github.com/tjorim/ElevenClock-rs/releases/latest).
   Windows may warn that the download is not code-signed.
2. Run it. It installs for your account only.
3. Open **ElevenClock-rs** from the Start menu.

If you used the original ElevenClock, quit it and switch it off in
**Windows Settings > Apps > Startup** first. The [migration guide](docs/migration.md)
explains this and what changed.

The release also has a portable ZIP: extract it to a folder you can write to and
double-click `elevenclock-rs.exe`.

## Use

The first launch opens **Settings**. Closing Settings keeps the clock running.

| Settings page | What you can change |
| --- | --- |
| Date and time | Pick a preset or write your own format, with a live preview and a list of every supported token. Presets: *ISO week date* (`2642.2`), *Calendar year + ISO week*, *Time + ISO week date* (two lines) |
| Appearance | Primary display only or all displays; font size 6–24 pt; theme System, Light or Dark |
| Startup | Start ElevenClock-rs when you sign in. Off until you turn it on |
| Support | Version and status, and **Copy diagnostics** |

Click **Apply** or **OK** to use your changes; **Cancel** discards them, and
**Reset to defaults** stages the defaults for you to apply. A format that is invalid
or does not fit the taskbar is explained beside the field and is never applied.

On the taskbar:

- **Left-click** the clock to open the Windows calendar and notifications.
- **Right-click** the clock or the tray icon for **Settings**, **Pause clock
  replacement** (shows the Windows clock until you resume) and **Exit**.
- Starting ElevenClock-rs again from the Start menu opens Settings for the clock that
  is already running.

## Getting help

Open **Settings > Support** and click **Copy diagnostics**, then paste the text into
a [new issue](https://github.com/tjorim/ElevenClock-rs/issues/new). The report
contains no user name, file paths or window titles, and nothing is sent anywhere
unless you paste it yourself.

## Uninstall and rollback

1. Choose **Exit ElevenClock-rs** in Settings or the tray menu. The Windows clock
   shows again.
2. Uninstall from **Windows Settings > Apps > Installed apps**. The uninstaller
   also removes ElevenClock-rs from startup. Your ElevenClock-rs settings are kept,
   so reinstalling restores them.

To go back to an earlier version, uninstall and run the earlier installer; to go back
to the original ElevenClock, see [Going back](docs/migration.md#going-back).

## Not supported

Windows 10, ARM64 devices, Windows Insider builds, taskbar replacements such as
ExplorerPatcher or StartAllBack, and taskbars on the side or top of the screen.
Internet time sync, notification counters, custom gestures, format expressions and
time offsets, extra time zones, translations and settings import from the original
ElevenClock are not available; the [migration guide](docs/migration.md#what-carries-over-and-what-does-not)
lists everything that was removed.

## Contributing

Build, test and packaging instructions are in the
[contributor guide](docs/contributing.md) and the [release guide](docs/releasing.md).
The [docs index](docs/README.md) maps the design records and Windows evidence.

## Name

ElevenClock-rs is a Rust rewrite of [ElevenClock](https://github.com/marticliment/ElevenClock)
for Windows 11, begun as a fork of it; the `-rs` suffix is the usual marker for a Rust
port. It is an independent project maintained by tjorim; affiliation with or
endorsement by the original ElevenClock project is not claimed.

Thanks to Martí Climent and the ElevenClock contributors for the original app, which
inspired this one.

## AI assistance

ElevenClock-rs is developed with help from AI coding agents. They write and revise
much of the code, tests and documentation under the maintainer's direction; the
maintainer reviews and merges every change. The same CI checks and Windows testing
apply whoever wrote the change.

## License

The Rust implementation, its tooling and the [application artwork](assets/README.md)
are licensed under [Apache License 2.0](LICENSE). Copyright 2026 ElevenClock-rs
contributors. Third-party dependencies keep their own licenses; see
[dependencies](docs/dependencies.md).

The original Python ElevenClock is GPLv3. It is no longer in this tree and is not
part of ElevenClock-rs releases; the [migration guide](docs/migration.md#legacy-reference)
links the commits that contain it. [Licensing](docs/licensing.md) documents the Apache 2.0 decision, and
[product name](docs/product-name.md) records the name and distribution channels.
