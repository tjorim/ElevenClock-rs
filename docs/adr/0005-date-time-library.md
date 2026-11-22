# ADR 0005: `time` for date/time values, with in-house format tokens

Date: 2026-11-22. Remediates [RUSTSEC-2026-0543](https://rustsec.org/advisories/RUSTSEC-2026-0543)
([#177](https://github.com/tjorim/ElevenClock-rs/issues/177)).

## Context

The clock needs one value type for "this instant, seen at this UTC offset": the
runtime reads it from Windows once per update, the reducer stores it, the format
language renders it and Settings previews sample dates with it. The format language
([FORMAT.md](../../crates/clock-core/FORMAT.md)) accepts a fixed allowlist of 17
strftime-style directives plus `%%`, with English names and no modifiers.

The previous library was flagged unmaintained, with no patched release. The candidates
were `jiff`, `time`, or an in-house implementation of the calendar arithmetic.

* **`jiff`** is maintained and dependency-free, but its offset-carrying value
  (`Zoned`) holds a time zone and is not `Copy`; the closest `Copy` form is a
  timestamp/offset pair that every event, effect and test would have to assemble.
* **`time`** is maintained, MIT/Apache-2.0, and its `OffsetDateTime` is a `Copy`
  instant-plus-offset whose equality and ordering compare the instant, the same
  semantics the reducer relied on. It parses RFC 3339 and RFC 2822 behind the
  `parsing` feature. Its default range (years −9999 to 9999) covers every clock.
* **In-house calendar code** would add date arithmetic (ISO weeks, leap years,
  offset conversion) to maintain without a reason to own it.

## Decision

* `clock-core` and `clock-windows` use `time::OffsetDateTime` for every wall-clock
  snapshot. Default features are off; the workspace enables `std`, `clock-windows`
  adds `parsing` for `Date:` headers in the manual release check, and `clock-core`
  enables `parsing` and `formatting` for tests and benchmarks only.
* The format language does **not** use a library formatter. `format.rs` compiles each
  allowed directive to its own token and renders it from the snapshot's calendar
  fields (year, ISO week date, hour, and so on) with fixed padding and English names.
  The library supplies the calendar arithmetic; the project owns the text.
* Years outside 0 to 9999 render with an explicit sign and at least four digits
  (`-0001`), and `%y`/`%g` use the Euclidean remainder, matching the output the format
  language had before.

## Consequences

* Rendered text did not change. Before the previous library was removed, every
  directive was compared against it for one sample per day across years −9998 to 9999,
  with varying times of day and offsets from −23:59:59 to +23:59:59.
* The tests now check the tokens against `time`'s own, independent formatter
  (`generated_allowed_tokens_match_reference`), plus fixed expectations for the year
  boundaries and the documented ISO week examples.
* An `OffsetDateTime` always has valid local calendar fields, so rendering can no
  longer fail because an offset moves a date out of range; the only render error is
  `OutputTooLong`. `FormatErrorKind::SnapshotOutOfRange` was removed.
* The format language's behaviour no longer depends on a library's strftime
  implementation, so a later change of date library cannot change user-visible text
  without failing the format tests.
