use clock_core::{Cadence, CompiledFormat, FormatErrorKind, format::*};
use proptest::prelude::*;
use time::{
    Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset,
    format_description::well_known::Rfc3339,
};

fn snapshot(value: &str) -> OffsetDateTime {
    OffsetDateTime::parse(value, &Rfc3339).unwrap()
}
fn render(format: &str, value: &str) -> Vec<String> {
    CompiledFormat::parse(format)
        .unwrap()
        .render(snapshot(value))
        .unwrap()
        .lines
}

#[test]
fn iso_and_calendar_presets_at_year_boundaries() {
    for (date, iso, calendar) in [
        ("2026-10-13", "2642.2", "2642.2"),
        ("2027-01-01", "2653.5", "2753.5"),
        ("2027-01-04", "2701.1", "2701.1"),
        ("2025-12-29", "2601.1", "2501.1"),
        ("2027-01-03", "2653.7", "2753.7"),
    ] {
        let time = format!("{date}T23:59:58+01:00");
        assert_eq!(render(PRESETS[0].format, &time), [iso]);
        assert_eq!(render(PRESETS[1].format, &time), [calendar]);
        assert_eq!(render(PRESETS[2].format, &time), ["23:59", iso]);
    }
}

#[test]
fn every_directive_english_names_leap_day_and_literals() {
    assert_eq!(
        render(
            "%Y %y %G %g %m %d %H %I %M %S %p %V %u %a %A %b %B %% 🕒日",
            "2024-02-29T00:07:09+01:00"
        ),
        ["2024 24 2024 24 02 29 00 12 07 09 AM 09 4 Thu Thursday Feb February % 🕒日"]
    );
    assert_eq!(render("%I %p", "2024-02-29T13:00:00+01:00"), ["01 PM"]);
    assert_eq!(
        render("%H:%M\r\n%g%V.%u", "2027-01-01T00:00:00+01:00"),
        ["00:00", "2653.5"]
    );
    assert_eq!(render("%%S", "2027-01-01T00:00:00+01:00"), ["%S"]);
}

#[test]
fn cadence_and_preview_use_supplied_snapshots() {
    for value in ["%H:%M", "%Y-%m-%d", "%%S", "seconds"] {
        assert_eq!(
            CompiledFormat::parse(value).unwrap().cadence(),
            Cadence::Minute
        );
    }
    assert_eq!(
        CompiledFormat::parse("%H\n%S").unwrap().cadence(),
        Cadence::Second
    );
    // Explicit Amsterdam snapshots around both DST transitions. Fixed offsets
    // describe these instants only; the formatter never derives zone rules.
    let samples = [
        snapshot("2026-03-29T01:59:59+01:00"),
        snapshot("2026-03-29T03:00:00+02:00"),
        snapshot("2026-10-25T02:59:59+02:00"),
        snapshot("2026-10-25T02:00:00+01:00"),
    ];
    let previews = preview("%H:%M:%S\n%Y-%m-%d", &samples).unwrap();
    for (result, time) in previews
        .into_iter()
        .zip(["01:59:59", "03:00:00", "02:59:59", "02:00:00"])
    {
        assert_eq!(result.unwrap().lines[0], time);
    }
    assert!(preview("%H", &[]).unwrap().is_empty());
}

#[test]
fn malformed_formats_have_original_character_spans() {
    for (value, kind, span) in [
        ("", FormatErrorKind::EmptyLine, 0..0),
        ("\nx", FormatErrorKind::EmptyLine, 0..0),
        ("x\n", FormatErrorKind::EmptyLine, 2..2),
        ("x\n\ny", FormatErrorKind::EmptyLine, 2..2),
        ("🕒\r\n%Q", FormatErrorKind::UnsupportedDirective, 3..5),
        ("🕒%", FormatErrorKind::IncompleteDirective, 1..2),
        ("x\r", FormatErrorKind::ControlCharacter, 1..2),
        ("x\t", FormatErrorKind::ControlCharacter, 1..2),
        ("x\0", FormatErrorKind::ControlCharacter, 1..2),
        ("x\u{85}", FormatErrorKind::ControlCharacter, 1..2),
    ] {
        let error = CompiledFormat::parse(value).unwrap_err();
        assert_eq!((error.kind, error.span), (kind, span), "{value:?}");
    }
    for token in [
        "%n", "%t", "%-d", "%_H", "%0H", "%:z", "%3S", "%f", "%z", "%Z", "%W", "%U", "%s", "%c",
        "%D", "%e",
    ] {
        let error = CompiledFormat::parse(token).unwrap_err();
        assert_eq!(error.kind, FormatErrorKind::UnsupportedDirective, "{token}");
        assert_eq!(error.span, 0..2);
    }
}

#[test]
fn exact_input_and_output_bounds() {
    let sample = snapshot("2026-09-30T00:00:00+02:00");
    assert!(
        CompiledFormat::parse(&"日".repeat(256))
            .unwrap()
            .render(sample)
            .is_ok()
    );
    let error = CompiledFormat::parse(&"日".repeat(257)).unwrap_err();
    assert_eq!(
        (error.kind, error.span),
        (FormatErrorKind::InputTooLong, 256..257)
    );
    assert!(CompiledFormat::parse(&format!("{}\r\nx", "日".repeat(253))).is_ok());
    assert!(CompiledFormat::parse(&format!("{}\r\nx", "日".repeat(254))).is_err());
    // September is nine scalars: 56*9 + 8 literals = exactly 512.
    let exact = format!("{}{}", "%B".repeat(56), "日".repeat(8));
    let text = CompiledFormat::parse(&exact)
        .unwrap()
        .render(sample)
        .unwrap();
    assert_eq!(text.lines[0].chars().count(), 512);
    let error = CompiledFormat::parse(&(exact + "日"))
        .unwrap()
        .render(sample)
        .unwrap_err();
    assert_eq!(error.kind, FormatErrorKind::OutputTooLong);
    assert_eq!(error.span, 0..121);
    let two_lines = format!("{}\nx", "%B".repeat(56) + &"日".repeat(6));
    let text = CompiledFormat::parse(&two_lines)
        .unwrap()
        .render(sample)
        .unwrap();
    assert_eq!(
        text.lines
            .iter()
            .map(|line| line.chars().count())
            .sum::<usize>()
            + 1,
        512
    );
    assert_eq!(
        CompiledFormat::parse(&(two_lines + "日"))
            .unwrap()
            .render(sample)
            .unwrap_err()
            .kind,
        FormatErrorKind::OutputTooLong
    );
    let over = CompiledFormat::parse(&"%B".repeat(128)).unwrap();
    assert_eq!(
        over.render(sample).unwrap_err().kind,
        FormatErrorKind::OutputTooLong
    );
}

#[test]
fn extreme_dates_and_offsets_render_every_token() {
    let all_tokens: String = DIRECTIVES.chars().map(|c| format!("%{c}")).collect();
    let compiled = CompiledFormat::parse(&all_tokens).unwrap();
    for local in [PrimitiveDateTime::MIN, PrimitiveDateTime::MAX] {
        for offset in [-86399, 0, 86399] {
            let sample = local.assume_offset(UtcOffset::from_whole_seconds(offset).unwrap());
            assert!(compiled.render(sample).is_ok(), "{sample}");
        }
    }
}

/// Years outside 0..=9999 keep the established strftime form: an explicit sign
/// and at least four digits; two-digit years are the Euclidean remainder.
#[test]
fn years_outside_four_digits_keep_their_sign() {
    let render_date = |year, month, day| {
        let date = Date::from_calendar_date(year, month, day).unwrap();
        CompiledFormat::parse("%Y %y %G %g %V %u %a")
            .unwrap()
            .render(PrimitiveDateTime::new(date, Time::MIDNIGHT).assume_utc())
            .unwrap()
            .lines
    };
    for (year, month, day, expected) in [
        (-9999, Month::January, 1, "-9999 01 -9999 01 01 1 Mon"),
        (-1, Month::December, 31, "-0001 99 -0001 99 52 5 Fri"),
        (0, Month::January, 1, "0000 00 -0001 99 52 6 Sat"),
        (1, Month::January, 1, "0001 01 0001 01 01 1 Mon"),
        (999, Month::December, 31, "0999 99 1000 00 01 2 Tue"),
        (9999, Month::December, 31, "9999 99 9999 99 52 5 Fri"),
    ] {
        assert_eq!(
            render_date(year, month, day),
            [expected],
            "{year}-{month}-{day}"
        );
    }
}

/// An independent rendering of each token by `time`'s own formatter.
fn oracle(token: char) -> &'static str {
    match token {
        'Y' => "[year]",
        'y' => "[year repr:last_two]",
        'G' => "[year base:iso_week]",
        'g' => "[year repr:last_two base:iso_week]",
        'm' => "[month]",
        'd' => "[day]",
        'H' => "[hour]",
        'I' => "[hour repr:12]",
        'M' => "[minute]",
        'S' => "[second]",
        'p' => "[period]",
        'V' => "[week_number repr:iso]",
        'u' => "[weekday repr:monday]",
        'a' => "[weekday repr:short]",
        'A' => "[weekday]",
        'b' => "[month repr:short]",
        'B' => "[month repr:long]",
        '%' => "%",
        other => panic!("no oracle for %{other}"),
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2048))]
    #[test]
    fn arbitrary_input_is_bounded(chars in prop::collection::vec(any::<char>(), 0..600)) {
        let input: String = chars.iter().collect();
        match CompiledFormat::parse(&input) {
            Ok(compiled) => {
                if let Ok(text) = compiled.render(snapshot("2027-01-01T23:59:59+01:00")) {
                    // Multiline formats are bounded by total rendered scalars, not two lines.
                    prop_assert!(!text.lines.is_empty());
                    prop_assert!(text.lines.iter().all(|line| !line.is_empty()));
                    prop_assert!(text.lines.iter().map(|line| line.chars().count()).sum::<usize>() + text.lines.len() - 1 <= MAX_RENDERED_SCALARS);
                }
            },
            Err(error) => {
                prop_assert!(error.span.start <= error.span.end);
                prop_assert!(error.span.end <= chars.len());
            }
        }
    }
    #[test]
    fn generated_allowed_tokens_match_reference(
        tokens in prop::collection::vec(prop::sample::select(DIRECTIVES.chars().collect::<Vec<_>>()), 1..129),
        seconds in -2_000_000_000i64..2_000_000_000,
        offset in -86399i32..86400,
    ) {
        let input: String = tokens.iter().map(|c| format!("%{c}")).collect();
        let sample = OffsetDateTime::from_unix_timestamp(seconds).unwrap().to_offset(UtcOffset::from_whole_seconds(offset).unwrap());
        let description: String = tokens.iter().map(|c| oracle(*c)).collect();
        let expected = sample.format(&time::format_description::parse_borrowed::<2>(&description).unwrap()).unwrap();
        let compiled = CompiledFormat::parse(&input).unwrap();
        match compiled.render(sample) {
            Ok(text) => prop_assert_eq!(&text.lines[0], &expected),
            Err(error) => {
                prop_assert_eq!(error.kind, FormatErrorKind::OutputTooLong);
                prop_assert!(expected.chars().count() > 512);
            }
        }
        prop_assert_eq!(compiled.cadence(), if tokens.contains(&'S') { Cadence::Second } else { Cadence::Minute });
    }
}

#[test]
fn multiple_lines_share_snapshot_and_normalize_crlf() {
    assert_eq!(
        render("%H:%M\r\n%A\n%g%V.%u", "2027-01-01T00:00:00+01:00"),
        ["00:00", "Friday", "2653.5"]
    );
}
