# Managed deployment guide

For deployment managers who provide ElevenClock-rs to many users from one central location. It
covers the `…-managed.zip` release asset; the reasoning behind its design is in
[ADR 0004](adr/0004-managed-launcher.md).

This is an organizational role, not a requirement to use a Windows administrator
account. The launcher, inventory summary tool and per-user app run without elevation.
Provisioning a central share and its permissions may require help from its owner.

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
update nor start anything. It imports no networking libraries. If an administrator
enables [local inventory](#local-inventory-administrator-opt-in), a bounded helper
records a local snapshot after the app start attempt, before the launcher exits.

## What is in the managed ZIP

An `ElevenClock-rs/` folder with:

| File | Purpose |
| --- | --- |
| `elevenclock-rs.data` | The app, byte-identical to the `elevenclock-rs.exe` in the installer and the portable ZIP, stored under a `.data` name so nobody starts it from the share by accident |
| `elevenclock-rs-launcher.exe` | The launcher |
| `SHA256SUMS.txt` | One line with the hash of `elevenclock-rs.data`; the launcher refuses an app that does not match it |
| `README.txt` | User guide: launching, Settings, startup, updates, support and optional organization reporting ([source](../packaging/MANAGED-README.txt)) |
| `README-MANAGER.txt` | Deployment-manager guide: central deployment, inventory configuration and summary tool; this role does not require a Windows administrator account ([source](../packaging/MANAGED-MANAGER-README.txt)) |
| `LICENSE.txt`, `THIRD-PARTY-NOTICES.txt`, `RUST-COPYRIGHT.html` | Licenses and notices |

No configuration file is required for deployment. Administrators can optionally add
[`elevenclock-rs-launcher.json`](#local-inventory-administrator-opt-in) beside the central
app to enable local inventory. The release's `SHA256SUMS.txt` (a separate download)
lists the ZIP's hash. The executables are not code-signed.

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

## Local inventory (administrator opt-in)

Managed launchers can record one latest local inventory snapshot after an eligible
app start attempt. A share or staging launch requires evidence that the local launcher
matches its refresh source. A launch from the fixed local launcher path can record
even when refresh was skipped or failed, while central configuration remains readable.
This applies only to the launcher shipped in the managed release ZIP. The
installer and portable ZIP do not include that launcher, and the app itself never
collects inventory, even when the central configuration enables it.

Inventory is **off by default**. The administrator can create the optional file
`elevenclock-rs-launcher.json` beside the central app and launcher:

```json
{
  "schema_version": 1,
  "local_inventory_enabled": true
}
```

Keep this file administrator-writable and user-readable, with the same read-only
permissions for users as the deployed binaries. No local administrator privileges,
HKLM changes, device-management control or user approval prompts are required.
The managed ZIP does not ship an enabled configuration. This file controls only
inventory: it cannot override executable source/destination paths or startup.
An optional `report_directory` enables central drop-box reporting as described below. Unknown fields and schema versions are rejected; the file must
be valid UTF-8 JSON no larger than 4 KiB with an explicit boolean setting.

The helper resolves the source using the same priority as app deployment: `--source`,
the folder a central/staging launcher was started from, then the source remembered by
the local launcher. It reads the configuration directly from that source only on
launches eligible for the inventory helper. Local launcher starts can invoke their
own helper implementation after skipped or failed refreshes; share/staging starts
still require a verified local launcher. Configuration in the
local executable folder (including aliases) is rejected. The launcher does not copy
or cache the configuration or an enabled flag on clients.

Set `local_inventory_enabled` to `false` or remove the file to revoke collection.
Missing, unreadable, malformed or unreachable configuration disables collection for
that launch. **Offline starts do not collect inventory**: the existing local app still
starts through normal fallback, and the previous snapshot remains unchanged. This
allows the share administrator to control collection without cached enablement.
The helper checks the configuration before looking up identity and again before
saving. A write already in progress can complete during revocation. Replace the
configuration atomically when editing it so clients do not see a partial file.

Enabling this setting collects the account SID, resolved account name when available,
installation ID, first and latest observation time, local app and launcher versions, and launch/update
outcomes described below. Administrators must disclose this collection under their
organization's policy before enabling it. Files stay in the user's application data;
no vendor endpoint is used. Unless `report_directory` is specified, snapshots remain
local and can be collected using existing administrative tools. User-controlled reports are
not authenticated audit records; a user who controls their local executable/state
can tamper with local inventory just as they can tamper with the managed app.

The snapshot is `%LOCALAPPDATA%\ElevenClock-rs\inventory\latest.json`. It is replaced,
not appended, on each successful observation. Revocation and managed uninstall retain
existing inventory, just as they retain preferences/logs; administrators can remove
the `inventory` directory after revocation to erase its identity data and reset the
installation ID. There is no automatic retention cleanup or periodic reporting.
Ordinary logs and app diagnostics remain free of names and paths.

### Optional central report destination

Add `report_directory` to send the same snapshot to another existing folder, for
example a reporting subfolder on the existing share:

```json
{
  "schema_version": 1,
  "local_inventory_enabled": true,
  "report_directory": "\\\\fileserver\\apps\\ElevenClock-reports"
}
```

Omit `report_directory` (or set it to `null`) for local-only inventory. The directory
must be an absolute UNC or drive path, at most 2048 UTF-8 bytes. Relative paths,
parent traversal, environment-variable expansion and Windows device paths are not
supported. `local_inventory_enabled: false` disables both collection and publication.

Central reports are **drop-box files**. Every account may use the app, so nothing is
provisioned per user: the launcher writes one file straight into `report_directory`.

```text
\\fileserver\apps\ElevenClock-reports\S-1-5-21-123-456-789-1001.12345678-1234-4234-8234-123456789abc.json
```

The name is `<SID>.<installation-ID>.json`, so a person with several PCs has one file
per PC and the folder grows by one file per installation, never per launch. The
local snapshot is committed first; the helper then writes the whole report in place,
preserving each installation's version, first and last-seen time. SID, installation
and time lookup failures prevent publication: no unnamed "user" report or
filename-derived identity is substituted. The launcher creates no folders, reads
nothing back from the folder and writes no staging or renamed file, because users in a blind folder
cannot list, read, delete or rename. A per-user local `publication.lock` serializes
overlapping launchers for the same user, and a launcher whose observation has been
superseded by a newer local one writes nothing, so a slow older launcher cannot leave an
older report. A contending launcher gives up without waiting. The launcher opens the
file for writing (without truncating on open), writes the report and sets the length.
A crash mid-write can leave a short or mixed file, and nothing in the file proves it
complete; the summary excludes files that no longer parse and the next launch rewrites
the report. Missing permissions make publication fail without changing launch results.
The local `latest.json` then gains a publication-failure issue and loses its
`publication_pending` issue; its observation fields stay unchanged.

#### Set up the drop-box folder

Use a new folder that is **not** below the read-only deployed executables, because
permissions inherited from there would let every user read the reports. Everything
below changes only that one folder and its share; nothing touches the app files.

1. **Share permissions.** In the folder's *Properties > Sharing > Advanced Sharing >
   Permissions* (or on the file server's existing share), allow **Change** for
   *Authenticated Users* and **Full Control** for *Administrators*. NTFS cannot grant
   more than the share does, so a share that is read-only breaks publishing.
2. **Stop inheritance.** In *Properties > Security > Advanced*, choose **Disable
   inheritance** and then **Convert inherited permissions into explicit permissions**.
   Then remove every entry except *SYSTEM* and *Administrators* (both Full control,
   applies to "This folder, subfolders and files").
3. **Let users create files here only.** Choose **Add**, principal *Authenticated Users*,
   type *Allow*, applies to **This folder only**, and tick only
   **Create files / write data**.
4. **Let a user rewrite their own file only.** Choose **Add**, principal **CREATOR
   OWNER**, type *Allow*, applies to **Files only**, and tick only: *Create files / write
   data*, *Create folders / append data*, *Write attributes*, *Write extended
   attributes*, *Read attributes* and *Read permissions*. Do not tick Read data, Delete,
   Delete subfolders and files, Change permissions or Take ownership. (*Read attributes*
   is required: without it a user can create their file once but cannot rewrite it.)
5. Choose **OK** on each dialog.

The same result with commands, run once from an administrator session:

```text
icacls "\\fileserver\apps\ElevenClock-reports" /inheritance:r
icacls "\\fileserver\apps\ElevenClock-reports" /grant "*S-1-5-32-544:(OI)(CI)F" "*S-1-5-18:(OI)(CI)F"
icacls "\\fileserver\apps\ElevenClock-reports" /grant "*S-1-5-11:(WD)"
icacls "\\fileserver\apps\ElevenClock-reports" /grant "*S-1-3-0:(OI)(IO)(WD,AD,WA,WEA,RA,RC,S)"
```

The resulting list under *Security* should be exactly: SYSTEM, Administrators,
Authenticated Users (this folder only, create files) and CREATOR OWNER (files only).
If *Authenticated Users*, *Users* or *Everyone* appear with Read or List rights, the
folder is not blind. No Rust, script or helper is needed, and nothing here changes the
app's own permissions.

#### Check it before enabling publication

Use two ordinary test accounts, A and B, and a Windows PC that can reach the share. Do
not use an administrator account: administrators can read everything by design.
Replace the path with your own and use any file name.

| As | Action | Expected |
| --- | --- | --- |
| A | `Set-Content "\\fileserver\apps\ElevenClock-reports\test.json" '{"a":1}'` | works |
| A | the same command again with different, shorter text | works |
| A | `dir "\\fileserver\apps\ElevenClock-reports"` | access denied |
| A | `Get-Content "\\fileserver\apps\ElevenClock-reports\test.json"` | access denied |
| A | `Remove-Item` or `Rename-Item` on `test.json` | access denied |
| B | `Set-Content` on the same `test.json` | access denied |
| B | `Get-Content`, `Remove-Item` or `Rename-Item` on `test.json` | access denied |
| B | `Set-Content` to a new name such as `b.json` | works |
| Admin | open the folder in File Explorer | both files visible |

Delete the test files afterwards (as an administrator). If every row matches, set
`report_directory` and launch the managed app as a test user; an administrator then
finds `<SID>.<installation-ID>.json` in the folder. If publishing fails for a real
user, check the share's *Change* permission and the *Read attributes* tick first.

What has been verified: with two local accounts on one Windows 11 machine over
loopback SMB, the real publisher created and rewrote its file, and the other
account could not list, read, overwrite, delete or rename it or publish over the
same name, but could create a new name. The summary excluded the report (owner not
matching its SID) and counted it with `--skip-owner-check`. A domain share and your
file server's own settings were not tested; the table above is the check for those.

Users can rewrite, but not read, the file they created, and file owners can usually
change their own file's permissions. Because any account can create a file with any
name, the summary command believes a report only when the file's **owner is the SID
written inside it**. It excludes the report and counts it in `unverified_owner_reports`
otherwise, for example a copy made by an administrator or a file created by another
account. Owner checks need Windows, so run the summary there. Reports from
user-controlled clients remain operational inventory, not an authenticated audit.
Files created from an elevated administrator session are usually owned by the
Administrators group instead of the account, so reports from users who run the launcher
elevated land in `unverified_owner_reports`. Ordinary users are unaffected.

Any account that can create files here can also create many of them. The summary ignores
names that are not `<SID>.<installation-ID>.json` (it counts them as `unrecognized_files`
and does not open them), but a flood of well-formed names can still exceed the summary's
100,000-report limit and make it fail until an administrator deletes the extra files.
The launcher cannot prevent this. NTFS per-user disk quotas bound the bytes an account
owns, but very small files may not count against them, so also watch the folder's file
count on the file server and delete files whose owner does not match their name.

The configuration is reread before publication and before the write; a changed
setting or destination aborts export of the observation collected under the old
configuration. Publication, including blocked SMB I/O, shares the existing helper's
three-second deadline; there is no extra worker, timer or retry service. On timeout,
the helper is terminated. An in-flight server operation can complete during
revocation/termination; this is not a distributed transaction or remote cancellation
guarantee.

Removing/changing the destination does not delete existing reports. Administrators
own central retention and cleanup, including old installation IDs after a reset.
Collection remains disabled while the central configuration is unreachable, even if
the report destination could otherwise be reached. There is no replay queue: a later
successful launch publishes its new observation, not a backlog.

### Optional clock-format popularity reporting

Set `include_clock_format: true` in the administrator-owned configuration to include
the user's committed clock format in both local and central snapshots:

```json
{
  "schema_version": 1,
  "local_inventory_enabled": true,
  "include_clock_format": true,
  "report_directory": "\\\\fileserver\\apps\\ElevenClock-reports"
}
```

This setting defaults to false and has no effect unless inventory itself is enabled.
The helper reads preferences only when explicitly enabled, within its existing
process deadline. It reads the saved format, including literal text, not a rendered
clock/date, an unsaved Settings draft, or other preferences. Account-format examples
may contain identifying literal text supplied by the user, so administrators should
include this field in their collection disclosure and protect reports accordingly.
Normal diagnostics/logs remain format-free. Reading never changes, migrates, saves
or recovers the preferences file. Missing preferences mean the app's built-in default;
damaged, unreadable or unsupported preferences leave `clock_format` unavailable.

`clock_format` is optional in schema v1: older reports lacking it remain readable,
and older v1 inventory readers ignore this additive field. New writers emit null
when collection is disabled or the value is unavailable. Changing the configuration
to false removes the format from the next successfully saved/published snapshot; it
does not immediately scrub existing files on offline clients or old central reports.
Retention and deletion of those files remain administrator responsibilities.

Aggregation exposes `format_usage`, keyed by normalized format source (CRLF becomes
LF). Whitespace, literals and directives otherwise remain exact; visually similar
but different formats are not guessed equivalent. Each entry counts installations,
distinct SIDs, recent installations and recent SIDs from the latest attributable
snapshot per installation. Repeated launches and duplicate report files add no votes.
A user with several installations using the same format counts as one user for that
format; users using different formats can appear in multiple groups. Recent counts
use the same configurable recent-activity window as the main summary. This measures
observed adoption, not clock viewing time, number of format selections or live use.

A `preset` label identifies an exact match against the aggregator's current built-in
presets; null means a custom format relative to that preset catalog. This avoids
persisting preset indices that may change between releases. Unknown/disabled format
observations are counted separately in `format_unavailable_installations`, never
inferred as a custom format or default. Sort recent distinct-user counts to identify
candidates for future built-in presets, then inspect installation counts and exact
format text. Selection of new built-ins remains a product decision based on the
collected results. No format telemetry is sent to the app's developers/vendor.

### Support errors and warnings

Managed inventory includes bounded `support_issues` whenever inventory is enabled,
without an extra configuration setting. Each entry is a fixed `code` and `severity`
(`info`, `warning` or `error`), for example:

```json
"support_issues": [
  {"code": "source_unreachable", "severity": "warning"},
  {"code": "using_fallback", "severity": "warning"},
  {"code": "start_failed", "severity": "error"}
]
```

| Severity | Codes / meaning |
| --- | --- |
| Error | `no_usable_copy`, `start_failed`: no app process could be started from a usable copy |
| Warning | `source_unreachable`, `source_invalid`, `copy_failed`, `running_check_failed`: source refresh was unavailable, refused or skipped for safety |
| Warning | `launcher_refresh_failed`, `source_not_recorded`: ancillary launcher self-update or source-state persistence failed even if the app started |
| Warning | `using_fallback`: a refresh failed and an existing local app was attempted |
| Warning | `account_sid_unavailable`, `account_name_unavailable`, `observation_time_unavailable`, `app_version_unavailable`: a fact could not be acquired; null values are retained explicitly |
| Warning | `preferences_read_failed`, `preferences_invalid`, `preferences_unsupported`: the opted-in format read failed; only checked when `include_clock_format` is enabled |
| Info | `publication_pending`: local snapshot committed before central publication; completion has not yet been observed locally |
| Warning | `publication_failed`: the helper observed a central publication failure and could update local status |

Codes are the structured support vocabulary, not a guarantee that every failure
can submit a report. A launch through the local launcher can report skipped refreshes
and refresh/fallback failures, provided the central configuration remains readable.
A share/staging launch without a verified local launcher still skips inventory.
An offline configuration disables collection even when the local app starts normally.
Inspect local launcher logs and central last-seen time when inventory cannot run.

Support fields contain no exception messages, raw log lines, profile/source/report
paths, window titles, user-supplied text or serialized preference documents. Existing
human log messages are not parsed to construct codes. At most 20 unique entries are
accepted; severity/code mismatches, duplicate codes and unknown codes are rejected.
Older schema-v1 reports missing the optional field remain readable as unavailable.
An empty array means no known issues were detected in this observation, not proof
that the clock remains healthy. This is launcher support telemetry, not live app
runtime monitoring, a crash dump, or an historical app log upload. Use the app's
existing Support page/local logs for detailed runtime troubleshooting.

A centrally visible report excludes `publication_pending`: it was atomically placed
at its destination. The local snapshot stays pending until the helper observes
success/failure and can update that status. Timeout/parent loss or a failed local
status rewrite can leave it pending; this means incomplete/unknown, not necessarily
failure. A failed transport cannot send its own failure over that same broken channel,
so `publication_failed` is available locally while the central report stays stale.
Check last-seen time when investigating missing central reports. Missing SID or time
also prevents attributed central publication; corresponding warnings remain local.
If the configuration is unavailable, collection remains disabled and the previous
snapshot is retained, rather than generating an unauthorized offline support report.

Only the latest observation is retained. A later successful launch replaces earlier
errors; there is no historical event queue or per-tick reporting. Changing/revoking
configuration follows the same checks and retention rules as other inventory fields.
Support collection/publication cannot change launcher exit codes, show message boxes,
prevent clock startup, or expand the helper's existing process deadline.

Aggregation exposes `support_issue_usage`, keyed by fixed code with severity,
installation and distinct-user counts, plus recent installation/user counts using
the same activity window. It uses the latest attributable snapshots, so duplicate
files/repeated launches do not inflate counts. These counts identify installations
currently reporting a problem, not the number of error occurrences. Older reports
without support data contribute to `support_unavailable_installations`, not to an
assumed healthy population. Unattributable/undated reports remain excluded under the
main aggregation contract.

### Snapshot format and compatibility

A UTF-8 JSON object, at most 16 KiB, has schema version `1`. Writers emit every field;
unavailable facts are `null`. Readers accept additional fields within v1, but reject
unknown schema versions, unknown outcomes/support codes, malformed JSON, duplicate known fields,
invalid identifiers, control characters and oversized data. Optional fact fields may
be absent when read and mean unavailable. Required decision fields must be present.
An incompatible change requires a new schema version; readers never rewrite data they
cannot understand.

| Field | Meaning / bound |
| --- | --- |
| `schema_version` | Integer `1` |
| `sid` | Token-derived Windows account SID, at most 184 bytes; user grouping key |
| `account_name` | Optional resolved `DOMAIN\username` (or account name without a domain), at most 1024 UTF-8 bytes; label only, requires SID |
| `installation_id` | Lowercase GUID in `8-4-4-4-12` form; per-copy key |
| `observed_utc` | UTC Unix seconds, through year 9999; unavailable time is null |
| `app_version` | Version of the actual local executable, at most 128 bytes; never inferred from the central source |
| `launcher_version` | Version of the launcher executing this observation, at most 128 bytes |
| `clock_format` | Optional administrator-enabled saved format source, validated by the clock formatter (256 Unicode scalars); null when unavailable/disabled |
| `support_issues` | Optional array of up to 20 unique fixed code/severity entries; missing/null means unavailable, empty means no known issue detected |
| `refresh` | `not_attempted`, `updated`, `unchanged`, `skipped_running`, `skipped_check_failed`, `source_unreachable`, `source_invalid`, `copy_failed` |
| `launch` | `not_attempted`, `process_created`, `start_failed` |
| `fallback` | True when refresh failed and the launcher attempted the existing local copy |

`process_created` proves only successful process creation, not a healthy clock or live
presence. Refresh describes the app executable, not launcher self-update or source
recording. A skipped refresh makes no claim about the current central version.
Mixed-install refusal records both operations as not attempted. Usage errors and
uninstall do not create launch observations.

Windows fact helpers live in `clock-windows`: token SID, optional account-name lookup,
UTC observation, GUID generation and fixed executable file version. Failed lookups
stay unavailable; no path-derived identity or guessed version is substituted. The
parent launcher supplies its compiled version, so self-update cannot substitute the
new helper version for the launcher that made the observation.

Collection starts only after the app start attempt. A one-shot copy of the fixed local
launcher runs as a private helper; share/staging launchers cannot collect directly.
The parent assigns the helper to a kill-on-close, single-process job before sending
the handshake that allows configuration reads and collection. When already running
from the fixed local launcher path, the launcher invokes itself: its own implementation
supports the helper role even if refresh was skipped or failed. This permits reporting
an already-running app, an invalid central app, a copy failure, or an unavailable app
payload while configuration remains readable. Share/staging launches require a matching
local-launcher hash, successful replacement, or the local file as the refresh source;
otherwise they skip inventory rather than invoke an unknown older local executable.
The freshness flag does not prove that the central file supports the helper marker;
it is not a negotiated protocol/version boundary. Deployment administrators must keep
the app and launcher from a compatible managed release together. On eligible launches
the helper can still run when configuration is missing or disabled; it
performs no identity lookup unless the central configuration explicitly enables it. The helper has a three-second deadline; on
timeout the job is closed and the parent observes exit for up to one more second.
Parent loss also closes the job. Blocking share configuration reads, account resolution, local snapshot I/O and
optional central publication stay in the helper process under the same deadline. There is no timed-out thread left running, retry service, clock
state-machine change or periodic app timer. Reporting errors never change the launch
exit code, display a message box, or prevent the app start attempt. If the local
launcher could not be deployed, inventory is skipped. Optional central publication uses the drop-box contract below.

### Local persistence and identity lifecycle

The `record_local` API uses `%LOCALAPPDATA%\ElevenClock-rs\inventory` after central configuration opt-in. It writes only
`latest.json` and a stable `inventory.lock`, outside the executable folder and
`source.txt`. It creates no history. Callers must not pass a share path.

A nonblocking exclusive file lock serializes simultaneous sessions. A contending run
skips recording with an error; no wait or retry service is introduced. The next
successful observation reuses the persisted ID. A temporary sibling is flushed,
verified and atomically replaced; failed writes preserve the previous valid snapshot.
Unknown or corrupt existing data is preserved and returns an error. For the same
identity (including unavailable current SID), older or missing timestamps do not
replace a later observation. A different known SID bypasses that comparison and
rotates the installation ID even if its observation time is earlier. Equal timestamps may replace; this is
observation precision, not a cross-machine ordering guarantee.

Executable refreshes retain the ID. Removing the inventory directory resets it;
reinstalling after managed uninstall retains the ID unless the administrator removes
that directory. Copying a profile to a different known SID rotates the ID. Cloning a
profile with the same SID and inventory data cannot be detected by this schema:
provisioning must remove inventory data before cloning or deliberately reset its ID.
Identity failure is recorded explicitly and is excluded from attributed aggregation.
Managed uninstall retains inventory; disable central collection and remove the directory for an explicit reset.

### Aggregation and metric meanings

`aggregate` accepts at most 100,000 bounded snapshots and keeps the latest observation
per **(SID, installation ID)** pair. Exact duplicates contribute once. Timestamp ties
use canonical serialized JSON ordering, so input order cannot change the result; a
conflicting same-time report is not evidence of authenticity. Malformed/unknown-schema
records and records missing SID, installation ID or time are excluded and counted.
Future timestamps are excluded rather than claiming recent activity despite clock
skew. Collection software must handle larger datasets explicitly instead of truncating.

`users` counts distinct SIDs; `latest.len()` counts distinct attributed installations,
not report files. `latest` retains each installation's version and last observation,
so one user can have several versions on several PCs. Missing versions remain unknown.
`recent_users` counts SIDs with at least one observation within the caller's configurable
window (inclusive); stale installations remain in `latest`. Last seen means last
launcher observation, not active use or time spent in the app. An offline PC or one
whose user never relaunches can remain stale indefinitely; there is no periodic app
reporting. Reports from user-controlled clients are operational inventory, not an
authenticated audit.

`first_observed_utc` is the first time the launcher recorded that installation with
inventory enabled, kept across refreshes. It is not the install date. Reports written
before the field existed start from their own last observation, and a new account or reset
starts a new installation. `new_installations` counts installations whose first
observation is inside the recent-activity window.

### Update progress

`versions.by_version` lists each local app version found in the latest reports, newest
first, with its installations, recent installations, distinct users and its percentage of
all attributed installations. `versions.newest_observed` is the highest version any
report carries and `versions.on_newest_percent` is the share of installations on it, so
the percentage of PCs that have updated is read straight from what people run. Versions
compare as dotted numbers (`0.2.10.0` is newer than `0.2.3.0`); a version that is not
purely numeric is listed after them and never counts as newest.

The central release's version or hash is deliberately not collected or written to
reports. "Newest" therefore means newest observed, not newest deployed: while no PC has
updated, the percentage reads 100% on the old version. Compare it with the version you
deployed, and remember that a copy appears only when it is launched (see last seen
above), so a PC that was off since the release is still counted at its older version.

### Generate an inventory summary

The managed launcher includes an explicit administrator command. Run it from a
downloaded/extracted managed release; it does not start the app, deploy files, collect
new facts, or change startup. It reads existing reports using the current account's
permissions and writes a JSON file:

```powershell
$job = Start-Process .\elevenclock-rs-launcher.exe -ArgumentList @(
    '--inventory-summary',
    '"\\fileserver\apps\ElevenClock-reports"',
    '"C:\Admin Reports\clock-summary.json"',
    '30'
) -Wait -PassThru -WindowStyle Hidden
if ($job.ExitCode -ne 0) { throw "Inventory summary failed: $($job.ExitCode)" }
$summary = Get-Content 'C:\Admin Reports\clock-summary.json' -Raw | ConvertFrom-Json
$summary.counts
$summary.formats | Format-Table format, preset, users, recent_users
$summary.support_issues | Format-Table code, severity, installations, recent_users
```

The optional last argument is the recent window in days (1–36,500, default 30).
Add `--skip-owner-check` as the last argument to accept reports regardless of file owner.
Use it for local snapshots you copied in yourself, because copies are owned by whoever
made them; never use it on a drop-box folder.

The input is the drop-box folder, or a directory containing local snapshots. `.json`
files at the root are read, and their owner must match their SID (see above). The
command also reads `.json` files in immediate child directories, so reports left by an
earlier layout still count; owner checks do not apply to those. Links and deeper
directories are skipped. The same library rules above
deduplicate records, exclude malformed/future records, and count unavailable values.
The output contains overall counts, a version breakdown, formats sorted by recent
distinct-user adoption, support counts, excluded-report and unverified-owner counts, and
the latest snapshot for each installation.
It therefore contains the same identity/format data as the input; restrict access and
retention accordingly.

The output's parent directory must exist and be outside the report tree. Successful
generation atomically replaces the output. Missing/unreadable inputs, enumeration
failures, or exceeded limits fail without replacing an existing summary. Up to
100,000 report files and 400,000 directory entries (including locks/staging files)
are accepted. Exit codes are 0 for success, 1 for I/O/limit failures, and 2 for invalid
arguments; the GUI launcher emits no console output or message boxes in this mode.
Run it on demand or from your own scheduled administrative job; no scheduler or
dashboard is installed.

## Related

- [ADR 0004: managed launcher](adr/0004-managed-launcher.md), the decision and its limits
- [Startup](design/startup.md) and [Diagnostics](design/diagnostics.md)
- [Releasing](releasing.md), for what the managed ZIP contains and how it is tested
- [Tested configurations](tested-configurations.md)
