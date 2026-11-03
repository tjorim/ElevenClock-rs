//! The launcher's command line.
//!
//! Launcher actions: `--source <folder>`, `--uninstall`, and the exclusive
//! `--inventory-summary <report-root> <output.json> [recent-days]` admin command.
//! Everything else belongs to the app and is passed on unchanged and in order, which
//! is how `--startup` (login start, no Settings window) reaches it.
use std::ffi::OsString;
use std::path::PathBuf;

pub const SOURCE: &str = "--source";
pub const UNINSTALL: &str = "--uninstall";
pub const INVENTORY_SUMMARY: &str = "--inventory-summary";
/// Accept reports regardless of file owner, for local snapshots an administrator copied in.
pub const SKIP_OWNER_CHECK: &str = "--skip-owner-check";
/// The app's login marker (`clock_windows::instance::STARTUP_MARKER`).
pub const STARTUP: &str = clock_windows::instance::STARTUP_MARKER;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Launch(Launch),
    Uninstall,
    InventorySummary {
        directory: PathBuf,
        output: PathBuf,
        recent_days: u32,
        verify_owners: bool,
    },
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Launch {
    pub source: Option<PathBuf>,
    /// A login start: no message boxes, only the log.
    pub quiet: bool,
    /// What the app receives.
    pub app_args: Vec<OsString>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArgError {
    /// `--source` was the last argument or was followed by another option.
    SourceNeedsFolder,
    /// `--uninstall` deletes files, so it is only accepted on its own.
    UninstallIsExclusive,
    SummaryUsage,
}
impl ArgError {
    pub fn message(self) -> &'static str {
        match self {
            Self::SourceNeedsFolder => "--source needs the folder that holds ElevenClock-rs.",
            Self::UninstallIsExclusive => "--uninstall cannot be combined with other arguments.",
            Self::SummaryUsage => {
                "Use --inventory-summary <report folder> <output.json> [recent days, default 30] [--skip-owner-check]. Output must be outside the report folder."
            }
        }
    }
}

pub fn parse<I: IntoIterator<Item = OsString>>(args: I) -> Result<Command, ArgError> {
    let mut args: Vec<OsString> = args.into_iter().collect();
    if args.iter().any(|arg| arg == INVENTORY_SUMMARY) {
        let verify_owners = args.last().is_none_or(|arg| arg != SKIP_OWNER_CHECK);
        if !verify_owners {
            args.pop();
        }
        let valid_path =
            |arg: &OsString| !arg.is_empty() && !arg.to_string_lossy().starts_with("--");
        if !(args.len() == 3 || args.len() == 4)
            || args[0] != INVENTORY_SUMMARY
            || !valid_path(&args[1])
            || !valid_path(&args[2])
        {
            return Err(ArgError::SummaryUsage);
        }
        let recent_days = if args.len() == 4 {
            args[3]
                .to_str()
                .and_then(|s| s.parse::<u32>().ok())
                .filter(|days| (1..=36500).contains(days))
                .ok_or(ArgError::SummaryUsage)?
        } else {
            30
        };
        return Ok(Command::InventorySummary {
            directory: args[1].clone().into(),
            output: args[2].clone().into(),
            recent_days,
            verify_owners,
        });
    }
    if args.iter().any(|arg| arg == UNINSTALL) {
        return if args.len() == 1 {
            Ok(Command::Uninstall)
        } else {
            Err(ArgError::UninstallIsExclusive)
        };
    }
    let mut launch = Launch::default();
    let mut rest = args.into_iter();
    while let Some(arg) = rest.next() {
        if arg == SOURCE {
            // A following option is not a folder: `--source --startup` is a mistake,
            // not a folder called `--startup`.
            let folder = rest
                .next()
                .filter(|folder| !folder.to_string_lossy().starts_with("--"))
                .ok_or(ArgError::SourceNeedsFolder)?;
            launch.source = Some(PathBuf::from(folder));
        } else {
            launch.quiet |= arg == STARTUP;
            launch.app_args.push(arg);
        }
    }
    Ok(Command::Launch(launch))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_text(args: &[&str]) -> Result<Command, ArgError> {
        parse(args.iter().map(OsString::from))
    }
    fn launch(args: &[&str]) -> Launch {
        match parse_text(args).unwrap() {
            Command::Launch(launch) => launch,
            _ => panic!("expected a launch"),
        }
    }

    #[test]
    fn no_arguments_is_an_interactive_launch() {
        assert_eq!(launch(&[]), Launch::default());
    }

    #[test]
    fn inventory_summary_is_exclusive_with_bounded_recent_days() {
        assert_eq!(
            parse_text(&[INVENTORY_SUMMARY, "reports", "summary.json"]),
            Ok(Command::InventorySummary {
                directory: "reports".into(),
                output: "summary.json".into(),
                recent_days: 30,
                verify_owners: true
            })
        );
        assert!(matches!(
            parse_text(&[
                INVENTORY_SUMMARY,
                "reports",
                "summary.json",
                "7",
                SKIP_OWNER_CHECK
            ]),
            Ok(Command::InventorySummary {
                recent_days: 7,
                verify_owners: false,
                ..
            })
        ));
        assert!(matches!(
            parse_text(&[INVENTORY_SUMMARY, "reports", "summary.json", "7"]),
            Ok(Command::InventorySummary { recent_days: 7, .. })
        ));
        for args in [
            vec![INVENTORY_SUMMARY],
            vec![INVENTORY_SUMMARY, "reports"],
            vec![INVENTORY_SUMMARY, "reports", "summary.json", "0"],
            vec![INVENTORY_SUMMARY, "reports", "summary.json", "36501"],
            vec![INVENTORY_SUMMARY, "reports", "summary.json", "--startup"],
            vec!["--startup", INVENTORY_SUMMARY, "reports", "summary.json"],
            vec![INVENTORY_SUMMARY, SKIP_OWNER_CHECK, "summary.json"],
        ] {
            assert_eq!(parse_text(&args), Err(ArgError::SummaryUsage));
        }
    }

    #[test]
    fn the_startup_marker_is_quiet_and_reaches_the_app() {
        let launch = launch(&["--startup"]);
        assert!(launch.quiet);
        assert_eq!(launch.app_args, [OsString::from("--startup")]);
        // It is the app's marker, so the two can never drift apart.
        assert_eq!(STARTUP, "--startup");
    }

    #[test]
    fn source_is_taken_and_not_passed_on() {
        let launch = launch(&["--source", r"\\server\share\ElevenClock", "--startup"]);
        assert_eq!(
            launch.source,
            Some(PathBuf::from(r"\\server\share\ElevenClock"))
        );
        assert_eq!(launch.app_args, [OsString::from("--startup")]);
    }

    #[test]
    fn other_arguments_pass_through_in_order() {
        let launch = launch(&[
            "a",
            "--source",
            "folder",
            "--startup",
            "b c",
            "--elevenclock-rs-x",
        ]);
        assert_eq!(
            launch.app_args,
            ["a", "--startup", "b c", "--elevenclock-rs-x"].map(OsString::from)
        );
        assert!(launch.quiet);
    }

    #[test]
    fn a_source_without_a_folder_is_an_error() {
        assert_eq!(parse_text(&["--source"]), Err(ArgError::SourceNeedsFolder));
        assert_eq!(
            parse_text(&["--startup", "--source"]),
            Err(ArgError::SourceNeedsFolder)
        );
        // A following option is not the folder.
        assert_eq!(
            parse_text(&["--source", "--startup"]),
            Err(ArgError::SourceNeedsFolder)
        );
    }

    #[test]
    fn uninstall_is_only_accepted_alone() {
        assert_eq!(parse_text(&["--uninstall"]), Ok(Command::Uninstall));
        for other in [
            &["--uninstall", "--startup"][..],
            &["--startup", "--uninstall"],
            &["--uninstall", "--source", "x"],
            &["--uninstall", "--uninstall"],
        ] {
            assert_eq!(
                parse_text(other),
                Err(ArgError::UninstallIsExclusive),
                "{other:?}"
            );
        }
    }
}
