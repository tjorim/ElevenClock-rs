//! Bounded support vocabulary. No exception messages, paths, user-supplied text,
//! log parsing or claims about live app health enter this field.
use crate::launch::{LaunchOutcome, Observation, RefreshOutcome};
use serde::{Deserialize, Serialize};

pub const MAX_ISSUES: usize = 20;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Code {
    SourceUnreachable,
    SourceInvalid,
    CopyFailed,
    RunningCheckFailed,
    LauncherRefreshFailed,
    SourceNotRecorded,
    UsingFallback,
    NoUsableCopy,
    StartFailed,
    AccountSidUnavailable,
    AccountNameUnavailable,
    ObservationTimeUnavailable,
    AppVersionUnavailable,
    PreferencesReadFailed,
    PreferencesInvalid,
    PreferencesUnsupported,
    PublicationPending,
    PublicationFailed,
}
impl Code {
    pub fn severity(self) -> Severity {
        match self {
            Self::NoUsableCopy | Self::StartFailed => Severity::Error,
            Self::PublicationPending => Severity::Info,
            _ => Severity::Warning,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    pub code: Code,
    pub severity: Severity,
}
impl From<Code> for Issue {
    fn from(code: Code) -> Self {
        Self {
            code,
            severity: code.severity(),
        }
    }
}

pub fn for_observation(observation: Observation) -> Vec<Issue> {
    let mut codes = Vec::new();
    match observation.refresh {
        RefreshOutcome::SourceUnreachable => codes.push(Code::SourceUnreachable),
        RefreshOutcome::SourceInvalid => codes.push(Code::SourceInvalid),
        RefreshOutcome::CopyFailed => codes.push(Code::CopyFailed),
        RefreshOutcome::SkippedCheckFailed => codes.push(Code::RunningCheckFailed),
        _ => {}
    }
    if observation.fallback {
        codes.push(Code::UsingFallback);
    }
    if observation.launch == LaunchOutcome::StartFailed {
        codes.push(Code::StartFailed);
    }
    if observation.launch == LaunchOutcome::NotAttempted
        && matches!(
            observation.refresh,
            RefreshOutcome::SourceUnreachable
                | RefreshOutcome::SourceInvalid
                | RefreshOutcome::CopyFailed
        )
    {
        codes.push(Code::NoUsableCopy);
    }
    codes.into_iter().map(Issue::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::launch::Exit;
    #[test]
    fn recoverable_update_failure_is_a_warning_but_no_copy_and_start_failure_are_errors() {
        let observation = Observation {
            exit: Exit::Done,
            refresh: RefreshOutcome::SourceUnreachable,
            launch: LaunchOutcome::ProcessCreated,
            fallback: true,
        };
        let recovered = for_observation(observation);
        assert!(recovered.contains(&Code::SourceUnreachable.into()));
        assert!(recovered.contains(&Code::UsingFallback.into()));
        assert!(
            recovered
                .iter()
                .all(|issue| issue.severity == Severity::Warning)
        );
        let no_copy = for_observation(Observation {
            exit: Exit::NoCopy,
            launch: LaunchOutcome::NotAttempted,
            fallback: false,
            ..observation
        });
        assert!(no_copy.contains(&Issue {
            code: Code::NoUsableCopy,
            severity: Severity::Error
        }));
        let started = for_observation(Observation {
            refresh: RefreshOutcome::Unchanged,
            fallback: false,
            ..observation
        });
        assert!(started.is_empty()); // process creation is not health telemetry
    }
}
