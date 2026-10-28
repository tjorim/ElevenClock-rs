# Managed deployment guide

For administrators who deploy ElevenClock-rs to many users from one central location. It
covers the `…-managed.zip` release asset; the reasoning behind its design is in
[ADR 0004](adr/0004-managed-launcher.md).

## When to use it

Use the managed deployment when many users should run one maintained version, and some of
them are sometimes offline or off the VPN. You replace a file in one place; each user's
copy follows at their next launch or sign-in.

If users look after their own machines, point them to the installer or the portable ZIP
instead (see the [README](../README.md#install)). Neither needs a share.

## How it works

The app never runs from the share. `elevenclock-rs-launcher.exe` does four things and
exits:

1. Finds the central copy of the app.
2. Copies it to the user's own `%LOCALAPPDATA%\Programs\ElevenClock-rs\` when it differs
   from the local copy.
3. Starts the local copy.
4. Exits.

The launcher has no window of its own, except one error message when it can neither
update nor start anything. It imports no networking libraries: it only copies files.

## What is in the managed ZIP

An `ElevenClock-rs/` folder with:

| File | Purpose |
| --- | --- |
| `elevenclock-rs.data` | The app, byte-identical to the `elevenclock-rs.exe` in the installer and the portable ZIP, stored under a `.data` name so nobody starts it from the share by accident |
| `elevenclock-rs-launcher.exe` | The launcher |
| `SHA256SUMS.txt` | One line with the hash of `elevenclock-rs.data`; the launcher refuses an app that does not match it |
| `README.txt` | A short readme for administrators ([`MANAGED-README.txt`](../packaging/MANAGED-README.txt)) |
| `LICENSE.txt`, `THIRD-PARTY-NOTICES.txt`, `RUST-COPYRIGHT.html` | Licenses and notices |

There is no configuration file to edit. The release's `SHA256SUMS.txt` (a separate
download) lists the ZIP's hash. The executables are not code-signed.

## Set up the central location

### Layout

Extract the ZIP's files to a folder on the share, for example:

```text
\\fileserver\apps\ElevenClock-rs\
    elevenclock-rs-launcher.exe
    elevenclock-rs.data         (or elevenclock-rs.exe)
    SHA256SUMS.txt
```

- **`elevenclock-rs.data`:** the managed ZIP ships the app under this name, so users who
  open the central folder find only the launcher to run. The launcher accepts
  `elevenclock-rs.exe` too (for example if you extract the app from the portable ZIP). The
  local copy is always called `elevenclock-rs.exe`. If both names exist, the `.exe` is used.
- **`SHA256SUMS.txt`:** shipped in the ZIP and checked by the launcher. It checks the app
  against the line that names the file (`elevenclock-rs.data` as shipped, or
  `elevenclock-rs.exe`) and refuses a copy that does not match. Lines for other files are
  ignored, and without the file (or a line for the app) nothing is checked. Replace the
  list together with the app: a stale list stops the update, and users keep their current
  copy. Extracting the whole ZIP always gives a matching pair.
- **The launcher checks the app before copying it:** it must be an x64 program whose
  version information names ElevenClock-rs. Anything else is refused.

### Permissions

Give users **read-only** access to the folder (read and execute on the share and the NTFS
folder). Users never write to it, and only the people who maintain the app need write
access. Anyone who can write there can change what every user runs, so keep that group
small.

### Trust for unsigned files

The executables are unsigned. Windows may warn when a user opens a program from a network
location that is not in the **Local intranet** zone, or from a download that carries the
internet mark.

- Add the share to the Local intranet zone, for example with the Group Policy *Site to
  Zone Assignment List* (*Windows Components > Internet Explorer > Internet Control Panel
  > Security Page*, zone value `1`). Use the host name your users open: an alias or the IP
  address is a different entry.
- Extract the downloaded ZIP after *Properties > Unblock*, or run `Unblock-File` on the
  extracted files, so the copies on the share do not keep the internet mark.

This is general Windows behavior and depends on your policies. It has not been tried on
your environment (see [Tested configurations](tested-configurations.md)).

## Roll out to users

Pick one way. Both end in the same state.

### A shortcut to the launcher

Give users a Start menu or desktop shortcut that points at
`\\fileserver\apps\ElevenClock-rs\elevenclock-rs-launcher.exe`. The source is then
"the folder the launcher was started from". The first launch copies the app and the
launcher to the user's profile (the README, license and notice files stay on the share)
and starts the clock.

### Push the launcher with a deployment tool

With Intune, SCCM or a script, copy `elevenclock-rs-launcher.exe` to a staging folder on
the machine (for example the tool's own cache; policy must allow it to run from there,
see [AppLocker](#applocker-and-software-restriction-policies)) and run it once with the
source named:

```text
elevenclock-rs-launcher.exe --source \\fileserver\apps\ElevenClock-rs
```

Run it **as the signed-in user**, not as SYSTEM or from an elevated process of another
account: the destination is per user (`%LOCALAPPDATA%`). The launcher copies itself next
to the app and remembers the source, so later starts, including the sign-in start, need
no argument. `--source` is only needed when the launcher is not started from the share.

Any other argument is passed on to the app unchanged.

## How updates reach users

- **When:** every time the launcher runs, which is a start from the shortcut or a sign-in
  start (see [Login startup](#login-startup)). The launcher compares the central app
  with the local one by SHA-256 and copies the central one if they differ.
- **Always the central version:** there are no user options and no "newer than" test. To
  roll back, put the older build on the share; users follow at their next launch.
- **A running app is never replaced or restarted.** If the app is running, the launcher
  skips the update and starts the exe, which brings the running copy's Settings to the
  front. A user who stays signed in gets the new version at their next launch, sign-in or
  reboot. Nobody is interrupted.
- **No update prompts.** In a managed copy the app shows no update interface and never
  checks for updates itself. The status on the Support page shows "(managed by your
  organization)" after the version.
- **Safe copy:** the launcher writes a temporary file next to the destination, checks its
  hash and then replaces the old file in one step. A failed update leaves the working copy
  in place.
- **The launcher updates itself the same way:** the local launcher is refreshed from the
  central folder (or from the file the user started).
- **Moving the share:** a local launcher remembers the folder it last deployed from. After
  you move the files, users who start the launcher from the new location pick up the new
  source; for the others, run `--source <new path>` once.

## Login startup

Users turn this on themselves in **Settings > Startup**, as with the normal app. It is off
until they do, and there is no admin switch or Group Policy.

In a managed copy the app registers the **local launcher** (`elevenclock-rs-launcher.exe
--startup`) in its `ElevenClockRust` Run value, so every sign-in also checks for an update.
The `--startup` start is quiet: no Settings window and no message boxes, only the log.
The details are in [Startup](design/startup.md#managed-mode-the-launcher-is-the-registered-program).

## Offline behavior

The launcher probes the central folder for about 3 seconds, and each file it then reads
from the share gets the same limit.

| Situation | Result |
| --- | --- |
| Central folder reachable | Updated if different, then started |
| Central folder unreachable, local copy exists | The local copy starts quietly |
| Central folder unreachable, no local copy (first run) | One message: the central copy cannot be reached, connect to the company network or VPN and try again |
| Central app invalid or fails its checksum | The local copy starts if there is one; otherwise one message asking the user to contact the administrator |
| Copying fails (full disk, locked file) | Same: the local copy starts if there is one |

## Do not mix installs

A user must not have the installed app and a managed (or portable) copy at the same time.

- Managed copies use the **same folder as the installer**
  (`%LOCALAPPDATA%\Programs\ElevenClock-rs`).
- The launcher refuses to run while the installer's copy exists for the account, and the
  installer refuses to run while a managed copy is there.
- To switch, uninstall the other one first: **Windows Settings > Apps** for the installed
  copy, `elevenclock-rs-launcher.exe --uninstall` for the managed one.

The app does no detection of this, so keep to one kind of copy per user.

## Where things are on a client

| What | Where |
| --- | --- |
| App, launcher and the launcher's state file (`source.txt`) | `%LOCALAPPDATA%\Programs\ElevenClock-rs\` (the folder an installed copy uses) |
| Preferences | `%LOCALAPPDATA%\ElevenClock-rs\` |
| App log | `%LOCALAPPDATA%\ElevenClock-rs\logs\elevenclock-rs.log` |
| Launcher log | `%LOCALAPPDATA%\ElevenClock-rs\logs\launcher.log` |
| Startup entry | `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, value `ElevenClockRust` (the same value as for a normal copy) |

Nothing is written under `HKLM` or `Program Files`, and there is no service or scheduled
task. No elevation is needed.

## Remove it

Run the launcher from the **central folder** (not from the user's copy):

```text
\\fileserver\apps\ElevenClock-rs\elevenclock-rs-launcher.exe --uninstall
```

It removes the app, the launcher, its state file and a leftover `.old` launcher from the
user's folder, and removes the `ElevenClockRust` Run value if it starts that copy. The
folder itself, **preferences and logs are kept**. It shows no window, so use the exit code
and `launcher.log`. `--uninstall` does not combine with other arguments, and it does
nothing while the app is running (exit code 6): the user exits the clock first, through
Settings or the tray menu.

The launcher cannot delete itself while it runs. If the local launcher started the
uninstall, run it from the share to remove that file too.

## AppLocker and Software Restriction Policies

If policy only allows programs from approved folders, these locations need a rule:

- the **central folder**, for `elevenclock-rs-launcher.exe` when users start it from the
  share;
- the **staging folder** the deployment tool runs the pushed launcher from, for its first
  run only;
- the **local folder**, `%LOCALAPPDATA%\Programs\ElevenClock-rs\*`, for the app and the
  launcher the Run value starts. An AppLocker path rule can use the wildcard path
  `%OSDRIVE%\Users\*\AppData\Local\Programs\ElevenClock-rs\*`.

The files are unsigned, so a publisher rule cannot match them. Use path rules, or a file
hash rule for each release (the hashes are in `SHA256SUMS.txt`), which then needs
updating with every update. If a policy blocks the app, the launcher still copies it, but
starting it fails (typically exit code 5 and the "could not be started" message).

None of this has been tried against a real AppLocker or Software Restriction Policies
setup. See [Tested configurations](tested-configurations.md).

## Troubleshooting

### Messages users can see

| Message | Cause and fix |
| --- | --- |
| "ElevenClock-rs is already installed for this account; uninstall it first…" | The installer's copy exists. Uninstall it, then start the launcher again |
| "…the central copy cannot be reached and this PC has no local copy yet…" | First run without access to the share. Connect to the network or VPN |
| "…the central copy is not a valid ElevenClock-rs program…" | The file on the share is not the app: wrong file, truncated copy, or a stale `SHA256SUMS.txt` |
| "…copying it to this PC failed…" | The disk is full, or the file in the user's folder is locked. See `launcher.log` |
| "ElevenClock-rs could not be started." | The local app did not start, for example blocked by policy |

### Exit codes

Useful for scripts and deployment tools.

| Code | Meaning |
| --- | --- |
| 0 | Done: the app was started, or `--uninstall` finished |
| 1 | Unexpected internal error |
| 2 | Wrong arguments (for example `--source` without a folder, or `--uninstall` with other arguments) |
| 3 | Refused: the installer's copy is present |
| 4 | No copy to start: the central copy could not be used and there is no local one |
| 5 | The local app could not be started |
| 6 | `--uninstall` did not finish: the app is running, or a file or the Run value could not be removed |

### Logs

`launcher.log` (and `launcher.1.log`, `launcher.2.log`) is in
`%LOCALAPPDATA%\ElevenClock-rs\logs\`. It has one line per decision and no paths or user
names. Typical lines: `updated`, `unchanged`, `skipped: running`, `source unreachable`,
`source invalid: …`, `using the local copy`, `launcher updated`, `source recorded`,
`refused: installed copy`, `started`, `start failed: …`. Read them in that order for one
launch.

### Diagnostics in the app

**Settings > Support > Copy diagnostics** shows `Managed: yes` and
`Startup: launcher` (or `off` / `stale`) for a managed copy. The report never includes a
path. See [Diagnostics](design/diagnostics.md).

## Related

- [ADR 0004: managed launcher](adr/0004-managed-launcher.md), the decision and its limits
- [Startup](design/startup.md) and [Diagnostics](design/diagnostics.md)
- [Releasing](releasing.md), for what the managed ZIP contains and how it is tested
- [Tested configurations](tested-configurations.md)
