//! The launcher's command line.
//!
//! Two arguments are the launcher's own: `--source <folder>` and `--uninstall`.
//! Everything else belongs to the app and is passed on unchanged and in order, which
//! is how `--startup` (login start, no Settings window) reaches it.
use std::ffi::OsString;
use std::path::PathBuf;

pub const SOURCE: &str = "--source";
pub const UNINSTALL: &str = "--uninstall";
/// The app's login marker (`clock_windows::instance::STARTUP_MARKER`).
pub const STARTUP: &str = clock_windows::instance::STARTUP_MARKER;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Launch(Launch),
    Uninstall,
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
    /// `--source` was the last argument.
    SourceNeedsFolder,
    /// `--uninstall` deletes files, so it is only accepted on its own.
    UninstallIsExclusive,
}
impl ArgError {
    pub fn message(self) -> &'static str {
        match self {
            Self::SourceNeedsFolder => "--source needs the folder that holds ElevenClock-rs.",
            Self::UninstallIsExclusive => "--uninstall cannot be combined with other arguments.",
        }
    }
}

pub fn parse<I: IntoIterator<Item = OsString>>(args: I) -> Result<Command, ArgError> {
    let args: Vec<OsString> = args.into_iter().collect();
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
            launch.source = Some(PathBuf::from(
                rest.next().ok_or(ArgError::SourceNeedsFolder)?,
            ));
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
            Command::Uninstall => panic!("expected a launch"),
        }
    }

    #[test]
    fn no_arguments_is_an_interactive_launch() {
        assert_eq!(launch(&[]), Launch::default());
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
