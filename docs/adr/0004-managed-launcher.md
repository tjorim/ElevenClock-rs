# ADR 0004: managed launcher that deploys a local copy from a central source

Date: 2026-10-28. Builds on [ADR 0002](0002-same-executable-discovery-helper.md), which
requires the app executable to be a local file.

## Context

Companies that deploy the app centrally want one source of truth, such as a file share,
without every user running the GUI app straight from it. Running from the share is
undesirable:

* it starts slower;
* the share may be unavailable, including when users are off the intranet or VPN;
* the exe is hard to replace while users run it;
* Windows trust and execution rules differ for network locations;
* login startup needs a stable local path.

The taskbar discovery helper restarts the same exe file (ADR 0002), so the exe must be
local. People installing the app themselves use the normal installer or the portable
ZIP and never see the launcher.

## Decision

A small, separate launcher executable, shipped in a managed ZIP only, gives admins
**one central source** while users always run a **local executable**. It resolves the
source, refreshes the local copy when that is safe, starts the app and exits.

### Responsibilities

* **Launcher:** resolve the source, update the local copy when it is safe, start the
  app and exit. It runs no GUI of its own, apart from an error message box when it can
  neither update nor start anything.
* **App:** the **same exe** as the portable ZIP. It already enforces single instance
  per user and session, and a second launch brings Settings to the front
  (`instance.rs`, `WM_SHOW_SETTINGS`), so the launcher does **not** duplicate that
  logic. In managed mode the app shows no update UI and never checks or updates itself.

### When updates happen: only at launch and login

* The local copy is refreshed **only when the launcher runs**: a manual launch
  (shortcut), or a login start after a sign-in or reboot through the app's Startup
  option (below).
* **The app is never restarted to apply an update** while it runs. A machine that stays
  signed in keeps its version until the next launch or login. This was chosen over
  in-session restarts so users are not interrupted.
* If the app is already running when the launcher runs, the launcher skips the update
  and just starts the exe, which brings the running copy's Settings to the front.
* **Always the central version, no options:** if the central copy differs from the
  local one, it is copied. There is no user-facing switch. The comparison is by
  SHA-256, not "newer", so admins can roll back by deploying an older build.

### Configuration: no admin config file

* **Source:** the folder the launcher was started from, which is automatic when users
  open it from the share. `--source <path>` exists only for when IT pushes the launcher
  directly to the machine instead.
* **Destination, fixed and per user:** `%LOCALAPPDATA%\Programs\ElevenClock-rs\`, the
  **same folder the installer uses**, with no override and no "managed" in the name. A
  shared, user-writable machine folder (for example `C:\Localdata`) is not supported:
  one user could tamper with the exe another user runs, and one user's running copy
  would block everyone's updates.
* The launcher **copies itself next to the app** and writes **one small state file of
  its own** there (for example `source.txt`) recording the source, so the local launcher
  knows where to look at login. Admins never create or edit it.
* The source file name defaults to `elevenclock-rs.exe`. A `.data` copy
  (`elevenclock-rs.data`) is also accepted, so admins can stop users running the exe
  from the share directly. The local copy is always `elevenclock-rs.exe`.

### Mixing installs is unsupported, and enforced both ways

The installer and the launcher use the same folder, so they must never both manage it:

* **Launcher:** refuses to deploy, with a clear message, when the installer's uninstall
  entry `HKCU\…\Uninstall\{97911B67-52D7-4D9B-B833-29303BC540A7}_is1` exists.
* **Installer:** refuses to install, with a clear message, when
  `{app}\elevenclock-rs-launcher.exe` exists (a managed copy is present). Implemented in
  `ElevenClock-rs.iss`.
* The readmes and the admin guide say so. The app itself does no detection.

### Update rules

* **Never replace the exe while the app is running**, and do not use the "rename the
  running exe" trick either. A running app restarts its own exe path for discovery, so a
  swapped file would pair an old app with a new helper. Detect a running app through the
  per-user instance mutex (`InstanceNames::for_user`) or the host window class.
* **Copy safely:** create a temporary file next to the destination, verify its hash
  against the source, then replace atomically with
  `MoveFileExW(REPLACE_EXISTING | WRITE_THROUGH)` (the same pattern as the preferences
  store). Clean up temporary files on failure.
* **Validate the source** before copying: an x64 PE file whose version info names
  ElevenClock-rs. Refuse anything else. Optionally check a `SHA256SUMS.txt` next to the
  source.
* **Reachability:** probe the source on a worker thread with a timeout of about 3 s. An
  unreachable UNC path can hang for 20 s or more. If it is unreachable, start the
  existing local copy quietly. If there is no local copy, show one clear error.
* **Self-update:** the launcher refreshes its own local copy too. It never starts itself
  again, so rename-and-replace is acceptable here.

### Login startup: the user's choice in the app's Settings

* Startup stays exactly as today: the **Startup option in Settings**, turned on and off
  by the user, stored in the app's own `ElevenClockRust` Run value, with the existing
  reconcile and repair flow (`startup.rs`). There is no launcher-owned Run value and no
  admin flag.
* **Managed mode:** when enabling startup, the app checks for
  `elevenclock-rs-launcher.exe` **in its own folder**. If it is there, the Run command
  becomes `"<folder>\elevenclock-rs-launcher.exe" --startup`, so every login start goes
  through the launcher and picks up updates. Otherwise the app registers itself as
  today.
* Only a fixed file name in the app's own folder is considered, never a path from config
  or the environment.
* The launcher passes `--startup` through so the app starts quietly.

## Rejected alternatives

* **Running the exe directly from the share:** the reasons in Context.
* **A silent per-user installer driven by a script, or a separate managed installer:**
  installer prompts, a check that blocks while the app runs, execution-policy issues,
  and two installers sharing one AppId.
* **A PowerShell launcher:** execution policy, a console window flash, and it cannot be
  tested in CI.
* **Reusing the app exe in a "launcher" role:** the whole app would run from the share
  again.
* **A launcher-owned Run value registered by admins:** it splits startup between two
  places.
* **A separate managed build without update code:** one exe for portable and managed is
  simpler, and in managed mode the update code is never used.
* **Restarting the running app when a new central version appears:** it interrupts
  users. Launch and login are enough.
* **An admin-authored `launcher.json`:** the source is implied by where the launcher
  runs, and the destination is fixed.
* **A separate `ElevenClock-rs-managed` folder:** not wanted in the name. The mutual
  refusal rule protects the shared folder instead.

## Rules this changes

These take effect with the implementation, not with this record, because the docs
describe the app as it is now. Each lands in the pull request that makes it true:

* **Launcher crate (#94):** `AGENTS.md` and `docs/contributing.md` change "Only
  `clock-app`'s release executable ships" to "the app, plus the launcher in the managed
  ZIP only".
* **Packaging and managed mode (#96):**
  * `docs/design/startup.md`: in managed mode the registered command may point at the
    sibling launcher. The rule that only the app's own `ElevenClockRust` value is
    written, and only on explicit user action, is **unchanged**.
  * `docs/releasing.md` ("Identity and ownership"): the install folder is shared by the
    installer and managed copies, with the mutual refusal rule above.
  * `ElevenClock-rs.iss`: the installer-side refusal.
* **Diagnostics (#94, #96):** the launcher logs to a bounded file under
  `%LOCALAPPDATA%\ElevenClock-rs\` using the same redaction rules. The app's diagnostics
  say "managed: yes/no" and whether startup targets the app or the launcher, without
  paths.
* **Admin guide (#99).**

Part of #88. Implemented in #94 and #96.
