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
Replacement hosted results and screenshot review are recorded below.
Native release and behavioral validation passed in Windows CI; Linux
cross-target Clippy is only type/lint validation.

Historical R03 commands, captures and tested revisions remain in
[overlay-spike.md](overlay-spike.md) and its linked evidence directories. They
refer to the former executable intentionally. No release gate changes here.

### First replacement hosted run

[Run 39394979728](https://github.com/tjorim/ElevenClock-rs/actions/runs/39394979728)
on PR head `d7815557333a8e8cff8c54c98121ad33324e813f` (tested merge revision
`537e29590229d2f80a9b9793492bfd0dee89e705`) passed Server x64 and native ARM64.
Emulated x64 passed through the 125% transition but failed its foreground setup
before the first fresh restored-DPI process launch. The artifact's `failure.png`
shows the foreground fixture and a hosted Windows paging-file warning;
`context.json` records `No measurable foreground window after DPI restore`.
This is not evidence of a failed replacement overlay launch or a production fix.

The follow-up keeps the five-second setup deadline and measurable-foreground
requirement, retries fixture activation at 750 ms intervals, clicks its measured
client interior rather than a fixed title-bar coordinate, and records activation
attempts on success and failure. No calendar input is retried. A new complete
three-target run and screenshot review remain required.

### Completed replacement evidence (2026-10-20)

[Run 39395963484](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963484)
tested PR head `2248796cf9e141e023cb37152fa2485439bed6ef`, merge revision
`155875d630fe9b7aa0def637e7362a527c8f2673`. All three targets passed. Each
recorded three 96 → 120 → 96 DPI cycles, fresh restored-DPI launches, native
hit ownership, opaque corner captures, Explorer recovery and no failed criteria.
On emulated x64, cycle one's first activation observed handle zero and the second
observed the fixture, directly exercising the bounded retry.

| Target | Successful attempt | Preserved R03 evidence | Original artifact |
| --- | --- | --- | --- |
| Server x64 | 1 | [Screenshots and reports](overlay-retirement-hosted/windows-2025-x86_64-pc-windows-msvc/) | [12251633963](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963484/artifacts/12251633963) |
| Windows 11 ARM64 native | 1 | [Screenshots and reports](overlay-retirement-hosted/windows-11-arm-aarch64-pc-windows-msvc/) | [12251788846](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963484/artifacts/12251788846) |
| Windows 11 ARM64 emulated x64 | 2 | [Screenshots and reports](overlay-retirement-hosted/windows-11-arm-x86_64-pc-windows-msvc/) | [12252656695](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963484/artifacts/12252656695) |

Emulated x64's first attempt failed earlier in R02 desktop preparation (two
helper deadlines, then supervisor timeout with partial output), followed by an
initial R03 style-check failure. It never reached DPI restore. The same revision
passed on a single job rerun, with no relaxed timeouts/assertions. Failure
contexts and screenshots are retained alongside the successful evidence; the
full failed artifact is [12252246474](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963484/artifacts/12252246474).

Visual review of all 22 screenshots per target confirms visible black clock
coverage, its removal during fullscreen/unsupported states, native clock return
on termination, Explorer recovery, and each 125%/restored-DPI capture. Calendar
review deliberately distinguishes the two strategies: `calendar-after-click`
(UIA invocation) shows **no visible calendar** on all three targets;
`calendar-keyboard` shows the native notification/date panel on all three.
This preserves the existing experimental finding that UIA call success is not
visual calendar acceptance, and the deliberate one-shot Win+N strategy remains
the evidenced path. No UIA failure is relabeled as a pass or dropped.

The native ARM auto-hide hidden capture shows the native taskbar still present
while the example overlay is absent; the asserted contract is overlay removal,
not proof that every shell pixel disappeared. Hosted startup/WSL/paging-file
dialogs are visible in several captures and are retained unedited. These remain
hosted single-display results, not stock-x64 or mixed-DPI release acceptance.

[Foundation run 39395963567](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963567)
passed formatting, Linux/Windows-target Clippy, tests, native release build,
resource checks and PowerShell harness regressions.
[Packaging run 39395963561](https://github.com/tjorim/ElevenClock-rs/actions/runs/39395963561)
also passed. The source changes satisfy the three-crate retirement scope; #40's
restore scenario is preserved and demonstrated against the replacement, without
claiming a new production-defect disposition or changing release gates.

Raw contexts are unchanged (including their pre-review status wording). The
review above supplies the separate visual interpretation. `SHA256SUMS` in the
evidence directory covers every preserved raw file; executable hashes and exact
tested revisions remain in each `context.json`.
