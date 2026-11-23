//! Validated, owned datetime formats. Names are English; offsets are snapshots,
//! not timezone rules. No clock access or expression evaluation is performed.
use crate::{Cadence, ClockText};
use std::{fmt, fmt::Write as _, ops::Range};
use time::OffsetDateTime;

pub const MAX_INPUT_SCALARS: usize = 256;
pub const MAX_RENDERED_SCALARS: usize = 512;
/// The complete directive allowlist, including the literal-percent escape.
pub const DIRECTIVES: &str = "YyGgmdHIMSpVuaAbB%";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormatErrorKind {
    InputTooLong,
    EmptyLine,
    ControlCharacter,
    IncompleteDirective,
    UnsupportedDirective,
    OutputTooLong,
}

/// Half-open Unicode scalar indices in the ORIGINAL input (not UTF-8 bytes).
/// CRLF occupies two indices. Render failures cover the entire original input;
/// an empty input has span 0..0. Length errors point at the first excess scalar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatError {
    pub kind: FormatErrorKind,
    pub span: Range<usize>,
}
impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at characters {}..{}",
            self.kind, self.span.start, self.span.end
        )
    }
}
impl std::error::Error for FormatError {}

/// One supported directive. `%%` compiles to a literal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Directive {
    Year,
    YearMod100,
    IsoYear,
    IsoYearMod100,
    Month,
    Day,
    Hour,
    Hour12,
    Minute,
    Second,
    AmPm,
    IsoWeek,
    WeekdayFromMonday,
    WeekdayShort,
    WeekdayLong,
    MonthShort,
    MonthLong,
}
impl Directive {
    const fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'Y' => Self::Year,
            'y' => Self::YearMod100,
            'G' => Self::IsoYear,
            'g' => Self::IsoYearMod100,
            'm' => Self::Month,
            'd' => Self::Day,
            'H' => Self::Hour,
            'I' => Self::Hour12,
            'M' => Self::Minute,
            'S' => Self::Second,
            'p' => Self::AmPm,
            'V' => Self::IsoWeek,
            'u' => Self::WeekdayFromMonday,
            'a' => Self::WeekdayShort,
            'A' => Self::WeekdayLong,
            'b' => Self::MonthShort,
            'B' => Self::MonthLong,
            _ => return None,
        })
    }

    fn write(self, out: &mut dyn fmt::Write, at: &OffsetDateTime) -> fmt::Result {
        let two = |out: &mut dyn fmt::Write, value: u8| write!(out, "{value:02}");
        match self {
            Self::Year => write_year(out, at.year()),
            Self::YearMod100 => two(out, at.year().rem_euclid(100) as u8),
            Self::IsoYear => write_year(out, at.to_iso_week_date().0),
            Self::IsoYearMod100 => two(out, at.to_iso_week_date().0.rem_euclid(100) as u8),
            Self::Month => two(out, at.month() as u8),
            Self::Day => two(out, at.day()),
            Self::Hour => two(out, at.hour()),
            Self::Hour12 => two(out, (at.hour() + 11) % 12 + 1),
            Self::Minute => two(out, at.minute()),
            Self::Second => two(out, at.second()),
            Self::AmPm => out.write_str(if at.hour() < 12 { "AM" } else { "PM" }),
            Self::IsoWeek => two(out, at.iso_week()),
            Self::WeekdayFromMonday => write!(out, "{}", at.weekday().number_from_monday()),
            Self::WeekdayShort => out.write_str(weekday_abbreviation(at.weekday())),
            Self::WeekdayLong => out.write_str(WEEKDAYS[weekday_index(at.weekday())]),
            Self::MonthShort => out.write_str(month_abbreviation(at.month())),
            Self::MonthLong => out.write_str(MONTHS[at.month() as usize - 1]),
        }
    }
}

/// Four digits, zero padded, for years 0 to 9999; otherwise an explicit sign
/// and at least four digits (`-0001`), the established strftime form.
fn write_year(out: &mut dyn fmt::Write, year: i32) -> fmt::Result {
    if (0..=9999).contains(&year) {
        write!(out, "{year:04}")
    } else {
        write!(out, "{year:+05}")
    }
}

const WEEKDAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];
const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
fn weekday_index(weekday: time::Weekday) -> usize {
    usize::from(weekday.number_days_from_monday())
}
/// English three-letter weekday name, as `%a` renders it.
pub fn weekday_abbreviation(weekday: time::Weekday) -> &'static str {
    &WEEKDAYS[weekday_index(weekday)][..3]
}
/// English three-letter month name, as `%b` renders it.
pub fn month_abbreviation(month: time::Month) -> &'static str {
    &MONTHS[month as usize - 1][..3]
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Piece {
    Literal(String),
    Directive(Directive),
}

/// Append a literal scalar, extending the previous literal run.
fn literal(pieces: &mut Vec<Piece>, c: char) {
    match pieces.last_mut() {
        Some(Piece::Literal(text)) => text.push(c),
        _ => pieces.push(Piece::Literal(c.into())),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledFormat {
    pieces: Vec<Piece>,
    cadence: Cadence,
    input_len: usize,
}
impl CompiledFormat {
    pub fn parse(input: &str) -> Result<Self, FormatError> {
        let fail = |kind, span| FormatError { kind, span };
        // Bound work before collecting arbitrary input.
        let chars: Vec<_> = input.chars().take(MAX_INPUT_SCALARS + 1).collect();
        if chars.len() > MAX_INPUT_SCALARS {
            return Err(fail(FormatErrorKind::InputTooLong, 256..257));
        }
        let mut pieces = Vec::new();
        let mut cadence = Cadence::Minute;
        let mut i = 0;
        let mut line_start = 0;
        let mut has_content = false;
        while i < chars.len() {
            let c = chars[i];
            let crlf = c == '\r' && chars.get(i + 1) == Some(&'\n');
            if c == '\n' || crlf {
                let end = i + if crlf { 2 } else { 1 };
                if !has_content {
                    return Err(fail(FormatErrorKind::EmptyLine, line_start..i));
                }
                literal(&mut pieces, '\n');
                has_content = false;
                line_start = end;
                i = end;
                continue;
            }
            if c.is_control() {
                return Err(fail(FormatErrorKind::ControlCharacter, i..i + 1));
            }
            if c == '%' {
                let Some(&directive) = chars.get(i + 1) else {
                    return Err(fail(FormatErrorKind::IncompleteDirective, i..i + 1));
                };
                match Directive::from_char(directive) {
                    Some(directive) => {
                        if directive == Directive::Second {
                            cadence = Cadence::Second;
                        }
                        pieces.push(Piece::Directive(directive));
                    }
                    None if directive == '%' => literal(&mut pieces, '%'),
                    None => {
                        return Err(fail(FormatErrorKind::UnsupportedDirective, i..i + 2));
                    }
                }
                i += 2;
            } else {
                // Always literals, even if they resemble HTML or expressions.
                literal(&mut pieces, c);
                i += 1;
            }
            has_content = true;
        }
        if !has_content {
            return Err(fail(FormatErrorKind::EmptyLine, line_start..chars.len()));
        }
        Ok(Self {
            pieces,
            cadence,
            input_len: chars.len(),
        })
    }

    pub const fn cadence(&self) -> Cadence {
        self.cadence
    }

    /// All lines use exactly this supplied snapshot. Output bounds include the
    /// normalized newline. Failure returns no partial/truncated clock text.
    pub fn render(&self, snapshot: OffsetDateTime) -> Result<ClockText, FormatError> {
        let mut output = BoundedOutput::default();
        // The snapshot's civil fields are always valid, so the only failure is
        // exceeding the output bound.
        for piece in &self.pieces {
            match piece {
                Piece::Literal(text) => output.write_str(text),
                Piece::Directive(directive) => directive.write(&mut output, &snapshot),
            }
            .map_err(|_| FormatError {
                kind: FormatErrorKind::OutputTooLong,
                span: 0..self.input_len,
            })?;
        }
        Ok(ClockText {
            lines: output.text.split('\n').map(str::to_owned).collect(),
        })
    }

    /// Preview user-supplied samples without reading system time or zone rules.
    pub fn preview(&self, snapshots: &[OffsetDateTime]) -> Vec<Result<ClockText, FormatError>> {
        snapshots
            .iter()
            .map(|snapshot| self.render(*snapshot))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormatPreset {
    pub name: &'static str,
    pub format: &'static str,
}
pub const PRESETS: [FormatPreset; 3] = [
    FormatPreset {
        name: "ISO week date",
        format: "%g%V.%u",
    },
    FormatPreset {
        name: "Calendar year + ISO week",
        format: "%y%V.%u",
    },
    FormatPreset {
        name: "Time + ISO week date",
        format: "%H:%M\n%g%V.%u",
    },
];

/// Parse and preview an editor value using caller-selected timestamps.
/// ```
/// use clock_core::format::preview;
/// use time::{OffsetDateTime, format_description::well_known::Rfc3339};
/// let sample = OffsetDateTime::parse("2027-01-01T12:34:00+01:00", &Rfc3339).unwrap();
/// assert_eq!(preview("%g%V.%u", &[sample]).unwrap()[0].as_ref().unwrap().lines, ["2653.5"]);
/// ```
pub fn preview(
    input: &str,
    snapshots: &[OffsetDateTime],
) -> Result<Vec<Result<ClockText, FormatError>>, FormatError> {
    Ok(CompiledFormat::parse(input)?.preview(snapshots))
}

/// A single formatting pass with a checked sink, including the line separator.
#[derive(Default)]
struct BoundedOutput {
    text: String,
    scalars: usize,
}
impl fmt::Write for BoundedOutput {
    fn write_str(&mut self, value: &str) -> fmt::Result {
        let count = value.chars().count();
        if self.scalars + count > MAX_RENDERED_SCALARS {
            return Err(fmt::Error);
        }
        self.scalars += count;
        self.text.push_str(value);
        Ok(())
    }
}
