ElevenClock-rs (managed deployment: manager guide)

For the people managing deployment and reporting for an organization. "Manager"
means that organizational role, not a Windows administrator account. Running the
launcher or summary tool requires no elevation or UAC, only access to the relevant
files. Ask the file-server owner to provision share/NTFS permissions when needed.
For everyday use, customization, startup and support, see README.txt.

A customizable clock for the Windows 11 taskbar (stock Windows 11 x64). The
executables are not code-signed, so Windows may warn about the download.

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
  It also removes a template URL handler if that registration still belongs to
  the local app. Local named presets, preferences and inventory are retained.

Where data lives
  Preferences and logs are in %LOCALAPPDATA%\ElevenClock-rs. The launcher writes
  launcher.log there too. Use Settings > Support > Copy diagnostics when reporting a
  problem; the report says whether the copy is managed.

Licenses
  LICENSE.txt contains Apache License 2.0 for the Rust implementation and the
  application artwork. THIRD-PARTY-NOTICES.txt and RUST-COPYRIGHT.html retain
  dependency/toolchain notices. The source is the tagged commit at
  https://github.com/tjorim/ElevenClock-rs.

Optional local inventory (managed launcher only):
Deployment managers can place elevenclock-rs-launcher.json beside the central app:
  {"schema_version":1,"local_inventory_enabled":true}
Keep it writable only by deployment managers and readable by users. No local
admin/device-management rights are needed. Missing, invalid or unreachable config
means no collection; no enabled flag is cached for offline use. Set false or remove
the file to stop future collection. The helper records identity/version/outcomes in
%LOCALAPPDATA%\ElevenClock-rs\inventory\latest.json.
Optional "report_directory" in the same JSON specifies an absolute UNC/drive path
for central publication, e.g. "\\\\fileserver\\apps\\ElevenClock-reports".
Provision that root and a directory for each account's SID beforehand, with rights
to read/replace the account's own reports and staging/lock files. Users must retain
read-only access to the executables/configuration. Publication needs suitable SMB
and NTFS rights; the launcher creates no reporting directories and changes no ACLs.
Reports use <report_directory>\<SID>\<installation-ID>.json. Local snapshots are
saved first; central failure never prevents startup. A bounded helper owns all I/O.
Omit report_directory for local-only inventory. Managers own central retention/cleanup.
Existing snapshots remain after disabling inventory or uninstalling the managed app.
Explain collection to users before enabling it. README.txt describes the possible
data collection; tell users which options your organization enables, where reports
are stored, who can read them and how long they are retained. See the full guide:
  https://tjorim.github.io/ElevenClock-rs/docs/managed-deployment/

Optional format popularity inventory:
Add "include_clock_format":true to the same central configuration to report saved
clock format source, including user-provided literal text. Default is false. This
reads committed preferences only; it never exports other settings or writes them.
Missing/unavailable format values appear as null. Reports stay in the configured
local/central locations, with no vendor telemetry. Aggregation counts distinct users
and installations per format (including recent counts), rather than launcher runs.

Support reporting is included whenever managed inventory is enabled. Snapshots
contain bounded support_issues with fixed error/warning/info codes for launcher
failures, fallback, fact/preference lookup problems and publication status. No raw
log lines or exception text are exported. A central publication failure can only be
recorded locally while the central report remains stale; inspect last-seen times.
This reports the latest launcher observation, not live clock health or error history.

The executing local launcher can report failed/skipped refreshes, including an
already-running app, while the central configuration remains readable. A launcher
started from a share/staging folder requires a verified local launcher before it
can report. Unreadable configuration still disables collection. Use local launcher
logs and last-seen timestamps for failures that cannot report. Matching
files do not negotiate helper protocol support; deploy compatible app/launcher files
together from the managed release.

Manager summary (no app launch or new collection):
  elevenclock-rs-launcher.exe --inventory-summary <report-root> <output.json> [recent-days]
Use Start-Process -Wait -PassThru in PowerShell to wait and inspect ExitCode. Default
recent window is 30 days. Output's existing parent must be outside the report tree.
The JSON includes distinct users/installations, recent counts, format adoption,
support issues and latest snapshots. Protect it like the source identity-bearing
reports. Invalid/future reports are excluded and counted; unreadable inputs fail
without replacing a previous summary. Exit codes: 0 success, 1 I/O/limit failure,
2 invalid arguments. No console output, dashboard or scheduled job is installed.
