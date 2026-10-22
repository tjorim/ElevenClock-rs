# Packaging and releasing

Releases are unsigned GitHub releases: the installer, the portable ZIP, matching
source, debug symbols and `SHA256SUMS.txt`. What has been tested is in
[tested configurations](tested-configurations.md). Signing and winget come later;
the name and channels are in [product name](product-name.md).

## Publishing a release

1. Set `version` in the root `Cargo.toml` and merge that change to `main`.
2. Tag the merge commit `v<version>` and push the tag:

   ```sh
   git tag v0.1.0 && git push origin v0.1.0
   ```

3. `rust-release.yml` checks that the tag matches the workspace version, builds in
   `-Mode Distribution`, runs the installer lifecycle test, then creates the GitHub
   release with the artifacts. A failed build publishes nothing; fix it, delete the
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
Use `-Python <path>` or `-Iscc <path>` if needed. The driver builds `clock-app` with
`--locked`, `x86_64-pc-windows-msvc`, a separate `target/packaging` directory,
and [`-C target-feature=+crt-static`](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes).
No global flags or developer build defaults change. The release profile retains PDBs.

The PR/manual `rust-release.yml` workflow uploads these development artifacts:

| Artifact | Contents |
| --- | --- |
| `*-setup.exe` | Normal per-user installer, Start menu shortcut, uninstaller |
| `*-portable.zip` | Explicit app/notices/instructions/provenance/DLL-report file allowlist |
| `*-source.zip` | Actual repository sources, source-file SHA256 inventory, Cargo.lock, vendored Cargo sources and relocatable offline configuration |
| `SHA256SUMS.txt` | Installer, both ZIPs and raw executable hashes |
| Separate symbols artifact | Matching PDB, provenance and its own checksums |
| Packaging evidence (failed runs only) | Actual installer/lifecycle/Settings logs and environment limitations |

Formatting, linting, unit tests and portable script regressions run in
`rust-ci.yml`; the packaging workflow only builds and validates the candidate.
For manual validation, run both workflows against the same ref. The packaging
build remains separate because it links the static runtime. New runs cancel
superseded packaging runs on the same ref.

Keep matching source and binary artifacts together. Modified development checkouts
are recorded honestly: source includes actual tracked and nonignored untracked
working files, omitting tracked deletions. Keep unrelated nonignored files out of
a packaging checkout. The snapshot is not falsely labelled as a clean HEAD archive.
To rebuild offline, extract the source ZIP, **change to its root**, and run:

```powershell
$env:RUSTFLAGS = '-C target-feature=+crt-static'
cargo build -p clock-app --release --target x86_64-pc-windows-msvc --locked --offline
```

MSVC/SDK and the recorded Rust toolchain remain build prerequisites. This supplies
matching source, not a byte-for-byte reproducibility guarantee. Without `.git`, the
rebuilt Support commit says `unknown`; original provenance stays in the JSON file.

`-Mode Distribution` rejects a dirty checkout and a `PACKAGE-README.txt` that
says "Distribution is blocked". Pull requests and manual runs build in Development
mode with read-only permissions; only a `v*` tag runs the `publish` job, which has
`contents: write` and no signing identity. No product prerequisite installer is run.

## Identity and ownership

New AppId: `{97911B67-52D7-4D9B-B833-29303BC540A7}`; legacy AppId:
`{D62480B8-71F1-48CE-BEEC-9D3E172C87B5}`. Install binaries under
`%LOCALAPPDATA%\Programs\ElevenClock-rs`, data under
`%LOCALAPPDATA%\ElevenClock-rs`, shortcut in the current user's Start menu.
The startup registry value is **`ElevenClockRust`**.

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
Delugo, were removed from the tree at cutover. They are no longer in the source
companion and were never installed; they remain in Git history (see the
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
Two distinct development builds can verify replacement/rollback mechanics even
before a supported release exists; record their hashes and distinguish this from
cross-version preference migration. Hosted results
record administrator membership and elevation; developer images cannot prove the
clean-runtime, non-admin or actual Windows-logon cases.

User rollback: Exit through Settings/tray, uninstall in Windows Settings > Apps,
install the saved earlier supported installer and launch from Start. Check its
Support version; choose startup again in Settings if wanted. Private data remains;
if the older version cannot read it, use its native Reset to defaults, not manual
preference edits. Verify Exit reveals the Windows clock. There is **no earlier
supported Rust installer**; development-build A → B → uninstall → A mechanics now
passed on a standard Windows 11 account, while cross-version
release compatibility remains unverified. Returning
to the native clock immediately requires only Exit/uninstall: the underlying clock
was never disabled. Start legacy ElevenClock only after the Rust app has exited.
