# Packaging and releasing

Releases are unsigned GitHub releases with four assets: the installer, the portable
ZIP, the managed ZIP for company deployments and `SHA256SUMS.txt`. The source is the tagged commit, which GitHub also attaches
to every release as a source archive. What has been tested is in
[tested configurations](tested-configurations.md). The [code signing policy](../README.md#code-signing-policy) describes the planned
SignPath signing; the winget submission procedure is below. The name and channels
are in [product name](product-name.md).

## Publishing a release

1. Set `version` in the root `Cargo.toml` and merge that change to `main`.
2. Tag the merge commit `v<version>` and push the tag:

   ```sh
   git tag v0.1.0 && git push origin v0.1.0
   ```

3. `rust-release.yml` checks that the tag matches the workspace version, builds in
   `-Mode Distribution`, runs the installer lifecycle test and the managed ZIP test, then
   creates the GitHub release with the artifacts. A failed build publishes nothing; fix it, delete the
   tag and push it again.

Before tagging, install and use the exact build on a clean Windows 11 x64 account.
Release artifacts are named `ElevenClock-rs-<version>-x64-*`; development builds add
the commit and `-dev`.

## Build and artifacts

On native Windows with MSVC/SDK, stable Rust, Python 3.11+ (development only) and
[Inno Setup 6.7.3](https://jrsoftware.org/isdl.php), run from the repository root:

```powershell
cargo fetch --locked
./scripts/package-rust-release.ps1
```

The output directory must be new; choose another with `-OutputDirectory dist/name`.
Use `-Python <path>` or `-Iscc <path>` if needed. The driver builds `clock-app` and `clock-launcher` with
`--locked`, `x86_64-pc-windows-msvc`, a separate `target/packaging` directory,
and [`-C target-feature=+crt-static`](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes).
No global flags or developer build defaults change. The release profile retains PDBs.

The PR/manual `rust-release.yml` workflow uploads these development artifacts:

| Artifact | Contents |
| --- | --- |
| `*-setup.exe` | Normal per-user installer, Start menu shortcut, uninstaller |
| `*-portable.zip` | One `ElevenClock-rs/` folder with the executable, `README.txt`, `LICENSE.txt`, `THIRD-PARTY-NOTICES.txt` and `RUST-COPYRIGHT.html` (an explicit allowlist) |
| `*-managed.zip` | The portable ZIP's `ElevenClock-rs/` folder plus `elevenclock-rs-launcher.exe` and the admin `README.txt` ([ADR 0004](adr/0004-managed-launcher.md)); the app is the same executable, stored as `elevenclock-rs.data` so nobody runs it from the share, with a `SHA256SUMS.txt` that lists it |
| `SHA256SUMS.txt` | Installer, portable ZIP, managed ZIP and raw executable (app and launcher) hashes |
| Separate symbols artifact | Matching PDB and its own checksums, for debugging a recent build |
| Packaging evidence (failed runs only) | Actual installer/lifecycle/Settings logs and environment limitations |

Only the installer, the portable ZIP, the managed ZIP and `SHA256SUMS.txt` are published
on a release. There is no managed installer: the launcher deploys the app without one.
`verify` checks that the app executable is byte-identical in the setup payload, the
portable ZIP and the managed ZIP, and that the launcher passes the same system-DLL and
dynamic-CRT gate with no network imports (`winhttp.dll`, `wininet.dll`, `ws2_32.dll`).
The symbols stay a workflow artifact with the default retention, so crash dumps from
older releases cannot be symbolised. Each package gets its own
readme from `packaging/` (`INSTALLER-README.txt` is the installer's `InfoBeforeFile`;
`PORTABLE-README.txt` and `MANAGED-README.txt` become the ZIPs' `README.txt`). The PE import
inspection, the dynamic CRT ban and the system-DLL allowlist run at build time
(`prepare`, and again on the shipped executable in `verify`); their results are not
published.

Formatting, linting, unit tests and portable script regressions run in
`rust-ci.yml`; the packaging workflow only builds and validates the candidate.
For manual validation, run both workflows against the same ref. The packaging
build remains separate because it links the static runtime. New runs cancel
superseded packaging runs on the same ref.

The source is the tagged commit on GitHub, and `Cargo.lock` pins the exact dependency
versions. To rebuild a release, check out the tag and run:

```powershell
$env:RUSTFLAGS = '-C target-feature=+crt-static'
cargo build -p clock-app --release --target x86_64-pc-windows-msvc --locked
```

MSVC/SDK and a current stable Rust toolchain remain build prerequisites. This is not a
byte-for-byte reproducibility guarantee. Keep unrelated nonignored files out of a
packaging checkout.

`-Mode Distribution` rejects a dirty checkout and a packaging readme that
says "Distribution is blocked". Pull requests and manual runs build in Development
mode with read-only permissions; only a `v*` tag runs the `publish` job, which has
`contents: write` and no signing identity. No product prerequisite installer is run.

## Identity and ownership

New AppId: `{97911B67-52D7-4D9B-B833-29303BC540A7}`; legacy AppId:
`{D62480B8-71F1-48CE-BEEC-9D3E172C87B5}`. Install binaries under
`%LOCALAPPDATA%\Programs\ElevenClock-rs`, data under
`%LOCALAPPDATA%\ElevenClock-rs`, shortcut in the current user's Start menu.
The startup registry value is **`ElevenClockRust`**.

The install folder is shared with managed copies (ADR 0004): the launcher deploys the
app, and itself, into the same `%LOCALAPPDATA%\Programs\ElevenClock-rs`. The two never
manage it together. Setup refuses (`InitializeSetup`, and `PrepareToInstall` for a
`/DIR` folder) when `elevenclock-rs-launcher.exe` is in the folder: "A copy managed by
your organization is installed for this account." The uninstaller's file list is an
explicit allowlist that never includes the launcher or its state file. The launcher
refuses in turn when the installer's uninstall entry exists.

Inno uses `PrivilegesRequired=lowest` with no elevation override; packaging verifies
the executable's `asInvoker` / `uiAccess=false` manifest. Startup stays off unless
chosen in the app. Uninstall removes installed files and the shortcut, preserving
private data and unowned files; no recursive/wildcard deletion, reset tasks, HKLM
actions or legacy process termination. It removes Run and StartupApproved values
only when the Run command matches the quoted installed executable plus `--startup`
(case-insensitive, outer whitespace ignored). Other portable commands, malformed
values and legacy `elevenClock` are left alone. Updates preserve opt-in at the same
path. Exit the app before install/update/uninstall; the runtime-window check asks
the user to exit and does not terminate an overlay app.

## Runtime and licensing

The PE gate rejects non-x64/console binaries, dynamic CRT imports including CRT API
sets, unknown DLLs and delay imports requiring review. Static CRT avoids adopting
an elevated VC redistributable installer. Import inspection does not prove clean
Windows launch: COM/DirectWrite can load further system components. Test on stock
Windows 11 x64 without VC developer runtimes, Python or Qt. A missing runtime or admin
requirement found later is a release blocker. No elevated prerequisite is silently run.

[Licensing](licensing.md) documents Apache 2.0 for the independently authored Rust implementation and original
replacement icon. The packaging script copies the root Apache LICENSE into
`LICENSE.txt` for both installer and portable ZIP. The inherited Python application
and imagery, with GPLv3 and the original credits to Martí Climent and Xavier
Delugo, were removed from the tree at cutover. They are no longer in the tree
and were never installed; they remain in Git history (see the
[migration guide](migration.md#legacy-reference)). The stale upstream Apache
README sentence does not relicense that legacy material. The full dependency
inventory supplies original notices (including build/test dependencies); Rust's
COPYRIGHT.html supplies toolchain notices. Hash-checked supplemental notices retain
LF bytes on Windows.

## Lifecycle and rollback

Only in a disposable account, with neither overlay app running:

```powershell
./scripts/test-rust-installer.ps1 -DisposableAccount
# On a fresh disposable account, with a distinct earlier development/release build:
./scripts/test-rust-installer.ps1 -DisposableAccount -PreviousInstaller C:/artifacts/earlier-setup.exe
```

The harness refuses existing app/data/shortcut/startup state. It checks hashes,
default install, shortcut launch, real Settings/Support journey, reinstall, owned
startup removal, private/unowned/legacy preservation, quoted Unicode paths, portable
startup preservation, ZIP launch and quiet marker launch. It preserves sentinel
files intentionally, so use a fresh account per run. With an earlier installer it
checks update hashes and restoring/launching that earlier executable. Same-version
reinstall of the identical executable is never counted as update/rollback evidence.
The managed ZIP has its own harness, also for a disposable account and run by
`rust-release.yml` on a separate runner (it shares the install folder with the
installer test): `./scripts/test-rust-managed.ps1 -DisposableAccount`. It runs the real
launcher from a temporary source folder, checks the deployed copy, the managed note and
`Managed: yes` in the diagnostics, that enabling Startup registers the local launcher,
that a changed source replaces the local copy on the next launch, that setup refuses
while the managed copy is present, and that `--uninstall` removes it.
Two distinct development builds can verify replacement/rollback mechanics even
before a supported release exists; record their hashes and distinguish this from
cross-version preference migration. Hosted results
record administrator membership and elevation; developer images cannot prove the
clean-runtime, non-admin or actual Windows-logon cases.

User rollback: Exit through Settings/tray, uninstall in Windows Settings > Apps,
install the saved earlier supported installer and launch from Start. Check its
Support version; choose startup again in Settings if wanted. Private data remains;
if the older version cannot read it, use its native Reset to defaults, not manual
preference edits. Verify Exit reveals the Windows clock. Development-build
A → B → uninstall → A mechanics passed on a standard Windows 11 account; this
does not establish cross-version release compatibility. See
[tested configurations](tested-configurations.md) for the validation scope. Returning
to the native clock immediately requires only Exit/uninstall: the underlying clock
was never disabled. Start legacy ElevenClock only after the Rust app has exited.

## WinGet submission

The candidate manifests in `packaging/winget/manifests/t/tjorim/ElevenClock-rs/0.2.0`
refer to the published 0.2.0 standard installer. Its downloaded SHA-256 was checked
against the release's `SHA256SUMS.txt`. These files are submission inputs, not evidence
that the package is available in the community catalog. Do not publish the managed
or portable ZIP under this identifier.

For each release, copy the version directory, update all package/display versions,
release and license links, the versioned installer URL and its SHA-256 (download the
setup and use `winget hash`). Never use a latest-release URL or replace a published
installer. Keep the Inno uninstall key `{97911B67-52D7-4D9B-B833-29303BC540A7}_is1`
as both ProductCode and AppsAndFeaturesEntries ProductCode, and the publisher/name
identical to the installer. This is how downloaded-setup installs are recognised.

On Windows, validate before submitting:

```powershell
$manifest = 'packaging/winget/manifests/t/tjorim/ElevenClock-rs/0.2.0'
winget validate --manifest $manifest
# Enabling local manifests may require an elevated terminal; installation must not.
winget settings --enable LocalManifestFiles
winget install --manifest $manifest --scope user
```

Use a disposable standard-user Windows 11 x64 account and retain the terminal output
and installer logs. Verify no UAC, then exit the app before each upgrade/uninstall.
Install an older downloaded setup, enable startup and save custom preferences, then
use the candidate local manifest to upgrade; check the executable version, startup
and preferences. Uninstall with `winget uninstall --id tjorim.ElevenClock-rs --exact`;
verify preferences remain and the owned Run value is removed, as with normal setup.
In a separate account deploy the managed ZIP: check that `winget upgrade --all`
does not select it and local-manifest installation refuses without changing its files.
After catalog acceptance repeat install and upgrade through the community source,
including detection of a downloaded older setup.

Setup's `/SUPPRESSMSGBOXES` hides the explanatory refusal dialogs. Do not rely on
winget displaying the reason: capture the client output on Windows. Inno reports a
failed initialization (code 1), or a Preparing to Install refusal (code 7); neither
uniquely identifies a managed copy or a running app. For diagnosis rerun setup with
`/VERYSILENT /SUPPRESSMSGBOXES /NORESTART /LOG="setup.log"`. Managed refusal logs
`refused: managed copy present`; builds after 0.2.0 also log
`refused: app still running`. Exit via Settings or the tray and retry for the latter;
for a managed copy, use the organization's launcher instead. Do not map these shared
Inno codes to a specific WinGet ExpectedReturnCodes response.
See [Inno exit codes](https://jrsoftware.org/ishelp/topic_setupexitcodes.htm).

Copy the tested version directory into the same `manifests/t/tjorim/ElevenClock-rs/`
path in a fork of [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs),
and submit a PR following its contributing guide. Retain its validation results and
resolve failures before calling the package available. The unsigned EXE still needs
URL/SmartScreen reputation, malware scanning and unattended installation validation;
a hash match does not establish acceptance. See the upstream
[validation requirements](https://github.com/microsoft/winget-pkgs/blob/master/doc/Validation.md).
Do not bypass SmartScreen or promise acceptance of unsigned installers.

After acceptance, add `winget install --id tjorim.ElevenClock-rs --exact --scope user`
to the README install options, with `winget upgrade --id tjorim.ElevenClock-rs --exact`
and the instruction to exit the app first. Until then the downloaded setup is the
supported install route. Manifest updates are manual; no repository token or release
workflow permissions are added for this channel.
