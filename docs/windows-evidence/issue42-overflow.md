# Issue #42: overflow recovery

Investigated on 2026-10-19 against `01143da` plus this change, on native Windows
x64 (OS build 26300). This record separates the reproducible recovery failure
from the still-unexplained initial overflow in [R13](acceptance.md).

## Reproduction

`runtime::tests::transient_overflow_recovers_without_a_text_change_or_another_apply`
starts with working primary and secondary clocks, applies `%g%V.%u`, and scripts
one primary paint overflow. It then removes that overflow condition without
changing preferences or time. A timer tick and recovery poll should retry the
affected clock and clear its diagnostic reason.

Before the fix, the test failed: expected one paint on the recovery poll, got
zero. The reducer records `bar.painted` when it requests Paint, before the native
result is known. The runtime hides an overflowed clock and records the fault,
but unchanged text generates no further Paint effects. Minute ticks and unchanged
discovery results therefore do not recover a date-only clock until its text or
another paint-triggering input changes. This reproduces the stuck state with an
injected initial overflow; it does not reproduce the original overflow's cause.

## Change and verification

The existing recovery poll now retries overflow faults only while the reducer
considers their taskbars visible. Retry batches are limited to once per two
seconds, including during fast transition polling. They use one fresh local time
snapshot and the committed format, font and appearance. A successful paint clears
the fault through the ordinary paint path; a persistent overflow stays hidden.

The new runtime tests cover recovery without another Apply, isolation from a
healthy secondary clock, bounded persistent retries, and suppression while paused,
locked, fullscreen, hidden or unavailable. Generation checks remain owned by the
reducer's `overlay_state` query.

The native test
`iso_week_apply_and_transient_region_overflow_recover_on_the_same_surface`
uses real DirectWrite/Direct2D with hidden windows at 96/120/144/192 DPI. It paints
the default format, applies ISO week text, shrinks the client width to one pixel
to cache an overflow, restores the original size and paints successfully. It also
checks long-format overflow followed by successful ISO week rendering on the same
surface. No cache invalidation change was needed.

Executed checks:

- `cargo test --manifest-path rust/Cargo.toml -p clock-windows --lib --locked`:
  216 passed. The initial sandboxed run had 15 unrelated temporary-file/registry
  access failures; the same suite passed with normal test access.
- `cargo test --manifest-path rust/Cargo.toml -p clock-windows --test render_native --locked`:
  10 passed.
- `cargo clippy --manifest-path rust/Cargo.toml --workspace --all-targets --locked -- -D warnings`:
  passed.
- Workspace library tests: 219 passed; workspace doctests: 2 passed.
- Formatting and `git diff --check`: passed.
- Native x64 `clock-app` release build with `--locked`: passed.

These checks establish recovery behavior and native cache behavior, not a natural
first-Apply failure on Explorer. The original overflow trigger still needs a
desktop reproduction with captured measurements. The additional investigation
below does not pass an interactive R13 gate.

## Initial-overflow investigation

The original local application log is still available. Its relevant sequence is:

```text
2026-10-16T09:21:33Z ElevenClock-rs 0.1.0 started
2026-10-16T09:21:34Z primary taskbar 1: clock replaced
2026-10-16T09:21:37Z primary taskbar 1: clock hidden for now: a full-screen app is in front
2026-10-16T09:21:58Z primary taskbar 1: Windows clock in use: the clock text does not fit this taskbar. Use a shorter format or a smaller font size in Settings
2026-10-16T09:22:46Z status refresh requested
2026-10-16T09:22:51Z diagnostics copied to the clipboard
2026-10-16T09:26:25Z primary taskbar 1: clock replaced
```

This establishes that fullscreen hiding preceded the overflow. It does not identify
the foreground application, prove Apply's exact time, or record the dimensions of
the failed layout. A geometry/DPI transient remains a hypothesis.

Code tracing found:

- Preset selection changes only the format. Apply pulls the actual format and font
  controls, validates and checks fit, saves, then passes the committed preferences
  to the runtime. The runtime updates worst-case candidates before dispatching the
  preference event.
- Settings preflight measures the discovered clock rectangle. Actual painting
  measures `GetClientRect` on the overlay. A discrepancy could explain an accepted
  Apply followed by overflow, but the old log cannot establish whether it occurred.
- The default is `%H:%M\n%g%V.%u`, so the ISO-only format removes a line that already
  fits. The renderer and default-format implementation are unchanged from the
  original `940634a252c1` observation.

The new native test
`iso_week_fit_is_a_subset_of_the_default_fit_for_all_supported_fonts_and_dpis`
checks all 19 supported point sizes (6–24) at 96/120/144/192 DPI. It measures both
formats, verifies ISO needs no more width/height, then paints default → ISO in the
smallest region accepted for the default. All 76 combinations passed, including
the final exact drawing-layout guard. The complete native renderer suite now has
11 passing tests.

Live checks used the same Windows 11 build, **26300.9711**, via Computer Use. The
installed executable was identified separately from the workspace build. The
workspace executable was copied to the ignored local evidence directory with a
distinct name to verify its process identity. Its SHA-256 was
`D08A9A1C91EDE28C33D200E456C2E91BA5F3A5D5FCC616A974731DBC8AE56A3B`.

Two default → ISO Apply attempts completed with "Settings applied." and no
overflow transition. The second started a fresh process with saved defaults and
selected ISO as the first change after launch. These are **inconclusive desktop
reproductions**: both the installed and workspace binaries encountered a separate
`placement: Access is denied. (0x80070005)` while trying to show the primary
overlay, before the Apply attempts. Thus neither provided the original healthy
clock-replacement baseline. No causal link between that placement error and #42
has been established. Display scaling and Explorer were left unchanged; the
original saved preferences were restored and checked byte-for-byte afterward.

The investigation therefore has **no proven cause for the initial overflow**.
The renderer cache hypothesis was not reproduced. The recovery defect is proven
and fixed. The owner has chosen to close #42 with the recovery fix and reopen it
if the symptom returns; this does not establish an initial-overflow cause.

To make another occurrence actionable, overflow transitions now log required and
available pixel dimensions, discovered slot size, DPI and font point size. These
are logged only on the state transition, not on retries; clock text, custom format
and foreground window titles are excluded. A regression test verifies the exact
measurements and quiet retries/recovery. All 61 runtime tests and workspace
Clippy pass after this addition.
