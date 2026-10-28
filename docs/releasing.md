# Packaging and releasing

Releases are unsigned GitHub releases with four assets: the installer, the portable
ZIP, the managed ZIP for company deployments and `SHA256SUMS.txt`. The source is the tagged commit, which GitHub also attaches
to every release as a source archive. What has been tested is in
[tested configurations](tested-configurations.md). The [code signing policy](../README.md#code-signing-policy) describes the planned
SignPath signing; winget is a later distribution channel. The name and channels
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
| `*-managed.zip` | The portable ZIP's `ElevenClock-rs/` folder plus `elevenclock-rs-launcher.exe` and the admin `README.txt` ([ADR 0004](adr/0004-managed-launcher.md)); the same `elevenclock-rs.exe` |
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
