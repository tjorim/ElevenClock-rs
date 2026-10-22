# Rust dependency policy

No command parser, async runtime or web UI runtime is used. What each dependency is
for is listed [below](#what-each-dependency-is-for).

Commit Cargo.lock and use --locked for all checks/builds. New dependencies must
explain product need, maintenance, feature selection and license impact.
Update in a dedicated PR against the default branch. Run the Linux and Windows
jobs and the advisory/license inventory checks. Do not raise the declared
`rust-version` merely to absorb an update; record the justification if it changes.
Transitive updates need the same checks. No audit failure is silently suppressed;
any exception needs a reviewed, dated rationale and remediation issue.

Dependabot follows the default branch for both GitHub Actions and the root Cargo
workspace. GitHub Actions updates are checked daily
and Cargo updates weekly, with separate dependency PRs and the default open-PR
limit.

CI cargo-audit denies warnings; unavailable advisory data is a failure. License
checks fail on terms outside the explicit reviewed set or with missing original
LICENSE/COPYING/NOTICE text; CI does not upload the inventory. Regenerate it on
demand with `cargo metadata --locked --format-version 1 > dependency-metadata.json`
and `python3 scripts/rust-license-check.py dependency-metadata.json`. Review the
inventory on updates. SPDX metadata is not a substitute for shipping attribution.
The inventory script is development-only. Release packaging must retain applicable
notices and satisfy the selected terms for each dependency.

The root LICENSE and workspace metadata declare Apache 2.0 for the original Rust
implementation. [Licensing](licensing.md) explains that scope. The GPLv3 Python code and inherited artwork were removed at
cutover and remain in Git history ([migration guide](migration.md#legacy-reference));
the active icon is [original Apache-licensed artwork](../assets/README.md). Dependency licenses and
original notices are unchanged.
The [packaging workflow](releasing.md) includes the original inventory and
toolchain notices in every package, plus matching vendored
source.
CI uploads one runnable development build (`elevenclock-dev-x64`) for testing, and
test evidence (screenshots, JSON, logs) only when a run fails; neither is a release
package or installer.

The notice checker recognizes only explicit historical dual-license aliases:
`MIT/Apache-2.0`, `Apache-2.0 / MIT` and `Unlicense/MIT`. They correspond to the
already reviewed OR choices; arbitrary slash expressions remain rejected.
For `rustix`/`linux-raw-sys`'s
`Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT` and
`zerocopy`/`zerocopy-derive`'s `BSD-2-Clause OR Apache-2.0 OR MIT`, this project
selects the **MIT alternative**. Acceptance of these exact OR expressions does
not approve standalone BSD-2-Clause, the LLVM exception, new exceptions, or
arbitrary expressions containing MIT. Unrecognized terms still fail review.

The version-specific [supplemental notice manifest](dependency-notices/README.md)
restores anes 0.1.6's omitted original license files and recognizes r-efi 6.0.0's
bundled AUTHORS license/copyright text. Source commits and digests are retained;
missing, changed, unreadable or empty notices fail the check. These are retained
original notices, not missing-notice exemptions. The checker uses no network.
Run `python3 scripts/test-rust-license-check.py` for its positive and rejection
regressions; the dependency CI job runs them before the inventory.

`windows-bindgen`, `windows-core` and `windows-numerics` are on the 0.100 line. The
bindings are generated against `windows-core`, so the three must stay on matching
versions and Dependabot updates them as one group. A version bump regenerates the
bindings (`cargo run -p clock-bindgen --locked`) in the same PR; CI fails when the checked-in
file differs from a fresh generation.

Criterion 0.8 (MSRV 1.86) adds `alloca`, which compiles a small C file, and
`page_size` → `winapi`, whose `winapi-*-pc-windows-gnu` import-library crates
publish no license files; their original notices are supplemented under
[dependency-notices](dependency-notices/README.md). Because `alloca` needs the
MSVC toolchain for `x86_64-pc-windows-msvc`, the Windows-target clippy runs on
the Windows CI runner instead of cross-checking from Linux.

## What each dependency is for

- **`windows-core`** and **`windows-numerics`**: the runtime for the generated Win32
  bindings in `clock-windows/src/bindings/` (COM interfaces, `#[implement]` for the
  overlay's UIA provider, Direct2D's `Vector2`). The bindings cover only the documented
  Win32, COM, UI Automation, registry, process and job APIs listed in
  `clock-windows/bindings.txt`.
- **`windows-bindgen`**, development only (`clock-bindgen`): regenerates those bindings
  from the Windows metadata it bundles. It is not a dependency of the app.
- **`chrono`** 0.4, `std` only: datetime values in `clock-core`.
- **`serde`** 1.0 (derive) and **`serde_json`** 1.0: the private, versioned preference
  encoding and the discovery helper's report, behind `clock-windows`. `clock-core`
  stays free of persistence dependencies.
- **`embed-resource`** 3, build only: the dialog, application icon and DPI-v2 manifest.
- Development only: **`tempfile`** 3 (filesystem failure tests), **proptest** 1.12
  (generated formatter inputs) and **Criterion** 0.8 (parse/render measurements). None
  of them enters the app's normal dependency graph; the license check covers them too.
