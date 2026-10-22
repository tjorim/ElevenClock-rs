# Clock format strings

`clock_core::CompiledFormat::parse(&str)` validates and owns chrono tokens.
`render(DateTime<FixedOffset>)` returns `Result<ClockText, FormatError>`;
`cadence()` returns `Second` when `%S` occurs, otherwise `Minute` (including
literal `%%S` and date-only formats). No mutable time, OS queries, shell, HTML
interpretation, arithmetic or offset-expression compatibility is involved.

Settings can use `format::PRESETS`, `CompiledFormat::preview(&[snapshot])`, or
`format::preview(input, &[snapshot])`. Callers supply today's snapshot and a
New Year example; no implicit current time or zone rules are introduced.
Preview results retain each sample's render error. Preferences keep the last valid
format when parsing/rendering fails; Settings highlights the structured
`FormatError.kind` and `span`.

| Tokens | Meaning |
| --- | --- |
| `%Y`, `%y` | Calendar year, full / two digits |
| `%G`, `%g` | ISO week-year, full / two digits |
| `%m`, `%d` | Month / day, zero padded |
| `%H`, `%I` | Hour, 24 / 12 hour, zero padded |
| `%M`, `%S`, `%p` | Minute / second, zero padded; English AM/PM |
| `%V`, `%u` | ISO week 01–53; weekday Monday=1 to Sunday=7 |
| `%a`, `%A` | English abbreviated / full weekday |
| `%b`, `%B` | English abbreviated / full month |
| `%%` | Literal percent |

Literal Unicode is supported. LF or CRLF separates nonempty lines; there is no fixed line-count cap.
CRLF is normalized to LF. Other controls, empty input/lines, modifiers and every
unlisted directive are rejected. Spaces are literal content, not trimmed.
Input is at most 256 Unicode scalars **before** normalization. Output is at
most 512 scalars **including** the normalized separator. A single checked
formatting pass produces all lines from the supplied snapshot. Overflow
returns an error, never truncated text. An offset that moves a chrono endpoint
outside the representable local-date range returns `SnapshotOutOfRange`.

Error spans are half-open **original-input Unicode scalar indices**, not byte
indices or normalized-string indices. CRLF counts as two source scalars.
Unsupported directives cover `%` plus its next scalar (including modifiers);
a dangling `%` covers one scalar. Controls/newlines cover their source scalars.
Empty lines have zero-width spans. Input overflow identifies 256..257. Render
errors cover the whole input. Example: `🕒\r\n%Q` has span 3..5.

| Date | ISO week date `%g%V.%u` | Calendar year + ISO week `%y%V.%u` |
| --- | --- | --- |
| 2026-10-13 | 2642.2 | 2642.2 |
| 2027-01-01 | 2653.5 | 2753.5 |
| 2027-01-04 | 2701.1 | 2701.1 |
| 2025-12-29 | 2601.1 | 2501.1 |

The third preset, **Time + ISO week date**, is `%H:%M\n%g%V.%u` (an actual
newline in its stored value). All month/weekday names are English initially.
Amsterdam tests supply explicit +01:00/+02:00 snapshots on both sides of the
2026 spring/fall transitions. A fixed offset never implies a timezone rule.

## Development dependencies

Proptest (locked 1.12.0) generates arbitrary Unicode inputs and supported-token
formats with timestamp/offset variation; Criterion (0.8.2, default features off)
measures parsing versus reuse/rendering. Both are development-only and kept
out of the app's normal dependency graph. Both are maintained Rust ecosystem
packages with MIT/Apache-2.0 alternatives.
