# Contributing to ElevenClock-rs

`main` is the default branch and builds only the Rust app; open feature PRs against
it. The Python/Qt app, its build scripts and the rewrite plan were removed at the
cutover. They stay retrievable from Git; the commits are listed in the
[migration guide](migration.md#legacy-reference).

## Toolchains and commands

Rust 2024 on current **stable** only: CI runs one toolchain, and the repository does
not pin one (no `rust-toolchain.toml`). Workspace `rust-version` (1.99, the stable release
current when it was last set) matches what CI builds with; it is not a separately
tested floor, so raise it when you move to a newer stable. Install
rustfmt and Clippy. Use `fmt --all` from the repository root so cargo-fmt finds
this virtual workspace's targets. The workspace's default member is `clock-app`, so a bare
`cargo run --locked` starts the app; use `--workspace` or `-p` (as below and in CI) to build,
lint or test the other crates. Release builds require a native Windows host so
resources cannot be silently omitted during cross-compilation. Linux cross-target
checks are type checks only.
CI layout ([`rust-ci.yml`](../.github/workflows/rust-ci.yml)): the Linux `core` job
runs fmt, host Clippy, binding regeneration and the portable `clock-core` tests.
The `windows` job lints the Windows target and runs the full test suite including
Windows-only tests, the release build with its native resources, and the desktop
smoke checks. Both jobs use a cargo cache.
Portable packaging-script and overlay-harness regression checks run once on Linux;
the dependency job owns the advisory audit and license checks, with a cached,
version-checked advisory tool. Core tests retain Linux and Windows coverage; the
Linux test command already includes doctests.

Win32 APIs come from generated bindings, not the `windows` crate. To use a new
function, constant, struct or interface, add its name (`Windows.Win32.<Name>`) to
`crates/clock-windows/bindings.txt` and run `cargo run -p clock-bindgen --locked`. COM
interfaces list only the methods called (`Windows.Win32.IFoo::{A, B}`), with inherited
methods on the base interface that declares them; an interface that is only referenced
comes out without methods. Constants are plain integers (`#define` values are `i32`), so
cast them where an API takes `u32`. A `match` on a message must use a constant that
is in scope, otherwise the arm silently binds a new variable. The default metadata has
no UIAutomationCore.dll exports, UIA ids or Known Folder GUIDs; those few are
hand-written in `clock-windows/src/bindings.rs`. The `core` job fails when the
checked-in bindings differ from a fresh generation.

The [desktop workflow](../.github/workflows/runtime-lifecycle.yml) shares one
runner per architecture/target between the taskbar discovery, lifecycle/UIA and
overlay checks. All three combinations (native x64, native ARM64 and x64 on
ARM64) are covered. The overlay checks run last because they change display
scaling; their output is included in the `r08-lifecycle-*` artifacts. Packaging keeps
its separate static-runtime build and installer checks, without repeating the
foundation checks. Run the foundation workflow as well when manually validating
a packaging candidate.

Every workflow cancels superseded runs on the same ref. Documentation-only
changes do not start builds (dependency notices still do); packaging runs for its
source, assets, scripts and packaging inputs, rather than unrelated test harnesses.
Windows development requires the MSVC toolchain and
Windows SDK resource compiler; users do not need these developer tools.

```sh
cargo fmt --all --check
cargo test -p clock-core --locked
cargo test -p clock-windows --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

On Linux, optionally type-check and lint the Windows-only app code after installing
`x86_64-pc-windows-msvc` with rustup:

```sh
cargo clippy -p clock-windows -p clock-app --all-targets --target x86_64-pc-windows-msvc --locked -- -D warnings
```

Do not use `--workspace --all-targets` for this cross-target check: Criterion's
`alloca` dependency needs MSVC. Windows binaries cannot be executed on Linux.

On Windows, additionally run:

```sh
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --doc --locked
cargo build -p clock-app --release --target x86_64-pc-windows-msvc --locked
```

Double-click the resulting `elevenclock-rs.exe` for the native
runtime and Settings shell. It has no command suite. Closing Settings keeps the
clock running in the background; **Exit ElevenClock-rs** quits (see
[lifecycle](design/lifecycle.md)). Exit any legacy ElevenClock first: two overlay
apps cannot share a taskbar. What has been tested is in
[tested configurations](tested-configurations.md).
`rust-ci.yml` checks the GUI subsystem, embedded manifest, native launch/close
and forward/reverse dialog focus. Its development harness captures the actual
dialog and records runner OS/build/DPI. The environment
harness uses actual Windows high contrast and Display Scale selections at
100/125/150/200%, restoring the original settings; it never injects DPI messages
or uses a private DPI setter. It does not cover Explorer or the taskbar overlay.

## Website and documentation

The Markdown under `docs/`, the root `README.md`, `CODE_OF_CONDUCT.md` and
[`FORMAT.md`](../crates/clock-core/FORMAT.md) are the single source of the documentation
site; there are no HTML copies. An Astro build renders them next to the landing page and
rewrites their relative links to the published pages, so write links the way GitHub
resolves them. When you add a page, list it in `site/lib/nav.mjs` (`npm test` fails
otherwise). Preview and check the site with `npm ci`, `npm run dev` and
`npm run build && npm run check:site`; the Node and npm versions, routes and
deployment are in [`site/README.md`](../site/README.md). The `pages.yml` workflow builds
and checks the site for pull requests that touch it and deploys from `main`.

## Boundaries and ownership

For the unsigned x64 installer/ZIP workflow, source/notices/checksum companions and
disposable-account lifecycle harness, see [releasing.md](releasing.md).

Dependency direction: app → windows → core and app → core. Core has only
platform-independent chrono datetime values; its graph contains no Windows crate.
App/core forbid unsafe. `clock-windows` compiles native code only on Windows and
owns FFI. On Linux the app is a no-op build stub, not a supported product.

`clock-core::model` defines physical-pixel rectangles with checked signed edges,
transient generation-scoped taskbar keys, discovery snapshots, rendered text,
cadence and event/effect values.
Discovery owns snapshot validation: a Some clock rect means proven clock-only bounds;
DPI must be nonzero, clock/taskbar must fit the monitor, and adjacent controls
must be excluded. Unknown visibility/geometry must not produce overlays.

The formatter enforces ClockText's documented bounds ([format](../crates/clock-core/FORMAT.md));
the preference store owns the validated preference type ([preferences](design/preferences.md)).
The reducer owns clock state; PreferencesApplied carries committed validated preferences. ThemeChanged
carries a resolved appearance revision;
FullscreenChanged carries a generation-scoped taskbar key and boolean. See
[state and scheduling](design/state-and-scheduling.md) for integration and timer semantics.
Time events use one DateTime<FixedOffset> snapshot per update; it is not a stored
timezone rule. Discovery responses and all overlay effects carry ShellGeneration;
the reducer discards stale responses and destroys old-generation overlays on loss.
MonitorId/TaskbarId are process-local values, never raw handles or persistence IDs.
Changes to shared interfaces require coordinated review before dependent work.

The modal dialog manager owns HWNDs, controls, fonts and DPI layout on the calling
UI thread. The callback stores no borrowed state and never unwinds across FFI.
Shared LoadIcon resources are not destroyed. Native handles/COM pointers never
cross worker channels. Future custom owned handles need RAII, explicit disposal
order and UI-thread affinity; UIA objects belong to the bounded discovery helper
([ADR 0002](adr/0002-same-executable-discovery-helper.md)). No blocking discovery or rendering work in callbacks.

Resource files are linked into the app binary, not an intermediate library. Use
standard-user `asInvoker` and per-monitor DPI-v2 activation. Standard dialog
processing supplies Tab/Shift+Tab, mnemonics, Escape and default-button handling.
Check new controls with a keyboard and a screen reader.

Only `clock-app`'s release executable is an app artifact. Tests, examples, probes,
license inventory scripts and debug binaries are development artifacts and must
never enter an installer or support flow. See [releasing](releasing.md) for packaging and notices.

## Extending settings and actions

Use this checklist for each new setting or interaction; link the relevant design
record and verification evidence in the PR.

- Put validated user preferences and format rules in `clock-core`; add an editable
  field to `Preferences` and a validated accessor only when the running clock needs it.
  Put draft validation, availability, messages and Apply rules in the portable
  `settings_model`, then wire native controls in `settings.rs`. OS-owned settings
  such as login startup stay in their owning module, outside the preference file.
- Model clock policy as state/events/effects in the pure reducer. Route user actions
  through `interaction` and `Runtime`; callbacks queue owned values. Keep native
  handles and COM objects in `clock-windows`, on their owning thread, with explicit
  cleanup and no unwinding across FFI. Keep app → windows → core dependency direction.
- Identify blocking work before adding it to the message loop. Discovery stays in
  the bounded helper process; other blocking work needs an explicit ownership,
  cancellation and shutdown design. Do not add callback I/O or unbounded queues.
- Define behavior during pause, lock, Explorer restart, stale discovery and failed
  rendering. Preserve the native-clock fallback and generation checks. Document
  resource disposal and whether a feature creates timers, hooks or background work.
- Describe status and failure recovery without leaking user names, paths or window
  titles into diagnostics or logs. Extend the diagnostics golden/redaction cases
  when their output changes; avoid per-tick logging.
- For persisted changes, follow the [compatibility and fixture policy](design/preferences.md#compatibility-and-retained-fixtures).
  Save before publishing active preferences; retain draft and original data on failure.
- Add focused portable tests at the policy boundary, including changed and unchanged
  inputs for caches. Run fmt, host Clippy and affected portable suites above; for
  native changes also lint the Windows target and obtain Windows CI evidence.
  Verify keyboard/screen-reader behavior for controls and action entry points.
  Changes to wakeups or resources also require the measured evidence procedure below.
- For heavyweight or network-dependent features, record the explicit
  [dependency, privacy and lifecycle decision](dependencies.md#optional-features)
  before implementation.

The current [action audit](design/settings.md#action-audit) records the shared
entry points; introduce another abstraction only when a feature demonstrates a
policy duplication it will remove.

## Taskbar discovery

Portable discovery validation and the process-supervisor tests also run on Linux:

```sh
cargo test -p clock-windows --all-targets --locked
cargo clippy -p clock-windows --all-targets --locked -- -D warnings
```

The Windows-only FFI is gated; only portable geometry/queue code is exposed on
Linux. Native/UIA probing is blocking work in an isolated helper and must never run
in application callbacks. See [ADR 0001](adr/0001-taskbar-discovery.md) and
[ADR 0002](adr/0002-same-executable-discovery-helper.md). The development probe is
the `taskbar_probe` example in `clock-windows`.

When the local workspace is Linux, use the
[hosted desktop workflow](../.github/workflows/runtime-lifecycle.yml) for actual
Win32/COM verification. It runs the same probe on Windows Server x64 and Windows
11 ARM64 with native ARM64 and emulated x64 binaries, retaining screenshots,
structured output, Explorer-restart observations and helper-lifetime results.
ARM64/x64 emulation is not a native Windows 11 x64 OS, and one display is not a
mixed-DPI setup.

## Overlay development example

`overlay_evidence` is a development-only example in `clock-app`, never an installed
app artifact. Its experimental overlay/interaction FFI is private to the example;
a normal `-p clock-app` release build does not compile it. Build it with
`cargo build -p clock-app --example overlay_evidence --locked`.
The hosted workflow uses real native/UIA hits, screenshots, input and shell/display
transitions; positive window flags alone do not prove opaque coverage or calendar
access.

## Rendering

`clock-windows::render` holds portable layout, DPI and overflow logic that tests on
Linux, plus the Windows-only DirectWrite/Direct2D `Renderer`/`ClockSurface`; the
Windows tests in `crates/clock-windows/tests/render_native.rs` need a Windows
host. `clock-windows::theme` resolves opaque palettes (high contrast first). See
[rendering](design/rendering.md) for the API and resource ownership. `ELEVENCLOCK_RS_R07_THEME` is a development-only switch
for the overlay harness, not a user setting.

## Runtime

`clock-windows::runtime::Runtime` is generic over a `Platform` and fully tested on
Linux with a scripted platform; the Windows implementation is `runtime::native`.
`tests/discovery_worker.rs` (custom `main`, no libtest harness) runs real child
processes on every host and, on Windows, job-object and parent-loss tests. The app's
discovery helper is the same executable started with a private marker
([ADR 0002](adr/0002-same-executable-discovery-helper.md)); it is not a user command.

```sh
cargo test -p clock-windows --locked
```

There is no feature flag or support gate for overlays: the clock candidate is
always used, after geometric validation of its rectangle.
Hosted lifecycle evidence: `scripts/test-rust-lifecycle.ps1` via
[runtime-lifecycle.yml](../.github/workflows/runtime-lifecycle.yml).

## Performance and resource guards

Two kinds of evidence, never substituted for each other:

- **Synthetic guards** run in normal CI on any host, without sleeps or a desktop. They
  count operations (measurements, sampling sweeps, paints, discovery requests) rather than
  time, and each pairs an unchanged input that must repeat no work with a changed one that
  must still recompute. They live in `runtime/tests.rs` ("Performance guards"; the
  Settings fit cache, `WorstCase` resampling, idle polls) and `clock-core/tests/state.rs`
  (effects for repeated events). `WorstCase::builds()` is a test-only per-thread counter of
  sampling sweeps. A new cache or throttle needs a guard that fails when it is removed and
  one that fails when it keeps a stale answer. Never cache a failed or unavailable
  measurement as a success.
- **Measured evidence** comes from a real Windows desktop (below) and is what the figures
  in [tested configurations](tested-configurations.md) report.

`cargo bench -p clock-core --bench state` and `cargo bench -p clock-windows --bench
worst_case` print informative timings (the reducer on four displays; the sampling sweep
per preset). `cargo bench -p clock-core --bench format` covers parsing and rendering. None
of them gates CI: shared runners are too noisy for hard time limits, and the benches are
only compiled there by `--all-targets` linting. Compare before and after on one machine
when a change touches those paths.

A change that affects wakeups, discovery, recovery, working set or native resources also
needs this procedure on a release build, once on `main` and once on the branch:

1. Run one instance and pick the format in Settings. Use the three formats from
   [tested configurations](tested-configurations.md#resource-use) when the change can
   affect cadence (date only, minutes, seconds).
2. `./scripts/measure-rust-resources.ps1 -DurationSeconds 600 -IntervalSeconds 10
   -Label <name> -Workload '<format and what you did>'` reports CPU, private working set,
   handles, threads, GDI/USER objects and windows. Growth over the run is a finding; the
   total-machine CPU figure is a comparison between two runs on the same machine, not a limit.
3. The script cannot see timer wakes or hooks. Discovery shows up as helper processes: the
   `-processes.csv` file and `processesStarted` in the summary list every helper it saw.
   A helper that exits between two samples is missed, so use `-IntervalSeconds 1` when
   the question is how often discovery runs, and compare the counts for the same workload.
   Check that the change adds no timer that fires while nothing has changed (see
   [lifecycle](design/lifecycle.md#fallback-polling-rates-and-timers) for the existing ones).
4. For recovery or native-resource changes also run `./scripts/test-rust-lifecycle.ps1
   -ObserveOverlays` (Explorer restarts, handle and memory limits) and record the overlay
   recovery times.
5. Put the numbers, machine and build in the pull request. Update the table in tested
   configurations only when the reference figures actually change.

## Support and logging

`clock-windows::support` (status wording, the diagnostics report, redaction, the
bounded Refresh wait) and `clock-windows::logging` (the 1 MiB x 3 rotating log) are
portable and tested on any host. Only the Windows build lookup and the clipboard are
Windows-only (`support/native.rs`). The report is plain text for people, with no schema
and no command that prints it; change its wording freely, but keep it free of user names,
paths and window titles (`support/tests.rs` has the golden text and the redaction cases).
Write only what a state transition means into the log, never per tick or repaint.

The real-clipboard tests overwrite the clipboard text while they run (and restore it), so
they are ignored by default; CI runs them on its Windows runner:

```sh
cargo test -p clock-windows --test clipboard_native --locked -- --ignored --test-threads=1
```

See [diagnostics](design/diagnostics.md) for behaviour and limits.

## Login startup

`clock-windows::startup` holds every rule (the quoted command line, shell-style parsing,
reconciling the registry value with the running executable, repair) as plain values tested
on any host; only `startup/native.rs` (the `HKCU` Run value and the `StartupApproved` flag)
is Windows-only. The value name `ElevenClockRust` and the `--startup` marker are shared
contracts: the legacy app owns `elevenClock`, and `instance::Role` owns the marker. Startup
state is never stored in preferences; the registry is read each time. Only an explicit user
action writes, and nothing outside this app's own value and flag may be touched.

The real-registry tests use a throwaway `HKCU\Software\ElevenClock-rs-tests\...` tree that
each test removes, so they run with the normal suite. `scripts/test-rust-settings.ps1`
saves, clears and restores the current user's real `ElevenClockRust` value around its
Startup step.

See [startup](design/startup.md) for the owned value and the uninstall contract.
