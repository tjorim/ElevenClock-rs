# R03 coverage migration (#49)

The workspace has three application crates. The R03 executable is now the
`clock-app` development example `overlay_evidence`; its overlay windows and
helper-only interaction implementation are private example modules. The
`clock-windows/overlay-spike` feature and public prototype API are removed.
Production `clock-windows::overlay::band_action` and its tests remain intact.

## Scenario inventory

All rows retain their existing assertions and captures in
`scripts/test-rust-overlay.ps1`, running the new example. No scenario is waived.

| Scenario / structured result | Replacement and reason |
| --- | --- |
| Primary coverage, opaque corners, native pointer ownership, nonactivation and Alt-Tab styles | Example; independent pixel and hit sampling is still useful feasibility evidence. |
| Same-monitor fullscreen hides coverage | Example; preserves the controlled foreground fixture and bounded hidden observation. |
| Calendar click and keyboard-only fallback, one action, no stuck keys | Example; retains controlled strategy selection and action reports unavailable through production settings. Calendar screenshots still require visual review. |
| Forced termination, helper cleanup and restored native hit | Example; exercises the helper job and verifies no remaining windows/processes. |
| All attached displays | Example; records available display count without claiming mixed-DPI coverage. |
| Auto-hide hide/reveal | Example; retains physical shell transitions. `test-rust-autohide.ps1` also exercises the shipped app, but is not substituted for these assertions. |
| Unsupported discovery / removal after live coverage | Example; retains development-only rejection injection. |
| Adjacent native hit targets | Example; keeps independent Explorer ownership checks. |
| Explorer restart, immediate hide and recovered coverage | Example; preserves recovery captures and failure criteria. The production lifecycle harness separately exercises the shipped runtime. |
| Three 100% → 125% → 100% cycles, fresh process after restore | Example; retains #40/#53 foreground establishment, native DPI verification, restored overlay DPI, screenshots and per-cycle results. Interior hit-sampling regression tests move with the example. |

Keeping these experimental scenarios in an example avoids adding test controls
or the prototype selector/interaction API to the shipped runtime. Existing R02
shell-discovery and R08 production-runtime scenarios continue unchanged. This
migration does not establish production acceptance from prototype results.

## Running and evidence

On an interactive Windows MSVC/SDK host, from the repository root:

```powershell
cargo build --manifest-path rust/Cargo.toml -p clock-app --example overlay_evidence --target x86_64-pc-windows-msvc --locked
./scripts/test-rust-overlay.ps1 -ProbeTarget x86_64-pc-windows-msvc -Executable rust/target/x86_64-pc-windows-msvc/debug/examples/overlay_evidence.exe
```

The harness requires the R02 desktop preparation and `native-evidence/r02/context.json`.
The `runtime-lifecycle.yml` workflow performs that preparation and runs the
replacement on Server x64, Windows 11 ARM64 native and Windows 11 ARM64 emulated
x64. It uploads screenshots, helper JSON/JSONL reports and structured context,
including the exact source commit and executable SHA-256, even after failure.
The example uses the app's embedded PerMonitorV2/security manifest and GUI
subsystem. Cargo does not include examples in ordinary app builds; installer,
portable and development-app uploads continue to select `elevenclock-rs.exe`.

## Evidence status and #40

Issue #40 was closed following the DPI-restore correction in #53 (commit
`9c8b448`). This migration retains that correction and all three hosted restore
cycles; it does not independently claim a production defect was fixed.
Replacement hosted run links and screenshot review must be recorded before
#49 is considered complete. At implementation time those runs are pending.
Native release builds and behavioral verification require the Windows CI jobs;
Linux cross-target Clippy is only type/lint validation.

Historical R03 commands, captures and tested revisions remain in
[overlay-spike.md](overlay-spike.md) and its linked evidence directories. They
refer to the former executable intentionally. No release gate changes here.
