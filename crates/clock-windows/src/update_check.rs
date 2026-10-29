//! Optional, bounded GitHub release checks (ADR 0003). No work starts on construction.
use serde::Deserialize;

#[cfg(windows)]
pub(crate) mod native;

pub const RESPONSE_LIMIT: usize = 64 * 1024;
pub const RELEASES_URL: &str = "https://github.com/tjorim/ElevenClock-rs/releases/latest";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Failure {
    Network,
    Timeout,
    BadResponse,
    Oversized,
    RateLimited { until: u64 },
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckResult {
    UpToDate,
    Available { version: String, url: String },
    Failed(Failure),
}

/// Bounded vocabulary: diagnostics never contain release data or network errors.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ResultKind {
    #[default]
    NotChecked,
    UpToDate,
    Available,
    Failed,
}
impl ResultKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotChecked => "not checked",
            Self::UpToDate => "up to date",
            Self::Available => "available",
            Self::Failed => "failed",
        }
    }
}
impl CheckResult {
    pub fn kind(&self) -> ResultKind {
        match self {
            Self::UpToDate => ResultKind::UpToDate,
            Self::Available { .. } => ResultKind::Available,
            Self::Failed(_) => ResultKind::Failed,
        }
    }
    pub fn message(&self) -> String {
        match self {
            Self::UpToDate => "You have the latest version.".into(),
            Self::Available { version, .. } => format!(
                "Version {version} is available. Run the new setup, or for portable use, exit and replace the exe."
            ),
            Self::Failed(_) => "Couldn't check for updates.".into(),
        }
    }
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
    prerelease: bool,
    draft: bool,
}

// Bounded numeric SemVer parser; metadata never participates in precedence.
pub(crate) fn version(input: &str) -> Option<([u64; 3], bool)> {
    let (precedence, build) = input
        .split_once('+')
        .map_or((input, None), |(a, b)| (a, Some(b)));
    let identifiers = |text: &str, numeric_zeros: bool| {
        !text.is_empty()
            && text.split('.').all(|part| {
                !part.is_empty()
                    && part.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
                    && !(numeric_zeros
                        && part.len() > 1
                        && part.starts_with('0')
                        && part.bytes().all(|c| c.is_ascii_digit()))
            })
    };
    if build.is_some_and(|s| !identifiers(s, false)) {
        return None;
    }
    let (core, pre) = precedence
        .split_once('-')
        .map_or((precedence, None), |(a, b)| (a, Some(b)));
    if pre.is_some_and(|s| !identifiers(s, true)) {
        return None;
    }
    let mut numbers = core.split('.');
    let mut result = [0; 3];
    for value in &mut result {
        let s = numbers.next()?;
        if s.is_empty()
            || !s.bytes().all(|c| c.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return None;
        }
        *value = s.parse().ok()?;
    }
    if numbers.next().is_some() {
        return None;
    }
    Some((result, pre.is_some()))
}

/// Only a literal HTTPS GitHub release URL, with an ASCII unescaped tag segment.
/// Reject percent escapes too: neither encoded path traversal nor browser parsing
/// differences can turn an accepted string into a different destination.
pub fn valid_release_url(url: &str) -> bool {
    url.strip_prefix("https://github.com/tjorim/ElevenClock-rs/releases/tag/")
        .is_some_and(|tag| {
            !tag.is_empty()
                && tag.len() <= 256
                && tag
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"-._+".contains(&c))
                && tag != "."
                && tag != ".."
        })
}

pub fn decode(bytes: &[u8], current: &str) -> CheckResult {
    let failed = CheckResult::Failed(Failure::BadResponse);
    if bytes.len() > RESPONSE_LIMIT {
        return CheckResult::Failed(Failure::Oversized);
    }
    let Ok(release) = serde_json::from_slice::<Release>(bytes) else {
        return failed;
    };
    let tag = release
        .tag_name
        .strip_prefix('v')
        .unwrap_or(&release.tag_name);
    if tag.len() > 256 || !valid_release_url(&release.html_url) {
        return failed;
    }
    let Some((latest, pre)) = version(tag) else {
        return failed;
    };
    let Some((installed, _)) = version(current) else {
        return failed;
    };
    if release.draft || release.prerelease || pre || latest <= installed {
        CheckResult::UpToDate
    } else {
        CheckResult::Available {
            version: tag.to_owned(),
            url: release.html_url,
        }
    }
}

/// Retry-After seconds or HTTP date, and X-RateLimit-Reset epoch seconds. Never
/// shorten one server deadline with another; unusable deadlines defer a day.
pub fn backoff(now: u64, retry: Option<&str>, reset: Option<&str>) -> u64 {
    let retry = retry.and_then(|s| {
        s.parse::<u64>()
            .ok()
            .map(|v| now.saturating_add(v))
            .or_else(|| {
                chrono::DateTime::parse_from_rfc2822(s)
                    .ok()
                    .and_then(|v| u64::try_from(v.timestamp()).ok())
            })
    });
    retry
        .into_iter()
        .chain(reset.and_then(|s| s.parse::<u64>().ok()))
        .filter(|&v| v > now)
        .max()
        .unwrap_or_else(|| now.saturating_add(86400))
}

/// One coordinator survives Settings closing. Cancellation invalidates display
/// eligibility immediately; a replacement cannot overlap the old worker.
#[derive(Default)]
pub struct Coordinator {
    active: bool,
    visible: bool,
    backoff_until: u64,
    pub last: ResultKind,
}
impl Coordinator {
    pub fn begin(&mut self, managed: bool, now: u64) -> bool {
        if !crate::settings_model::update_checks_available(managed)
            || self.active
            || now < self.backoff_until
        {
            return false;
        }
        self.active = true;
        self.visible = true;
        true
    }
    pub fn cancel(&mut self) {
        self.visible = false;
    }
    pub fn complete(&mut self, result: &CheckResult, managed: bool) -> bool {
        if let CheckResult::Failed(Failure::RateLimited { until }) = result {
            self.backoff_until = self.backoff_until.max(*until);
        }
        let show = self.active && self.visible && !managed;
        self.active = false;
        self.visible = false;
        if show {
            self.last = result.kind();
        }
        show
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(tag: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({"tag_name":tag,"html_url":format!("https://github.com/tjorim/ElevenClock-rs/releases/tag/{tag}"),"prerelease":false,"draft":false,"ignored":{"normal":"API fields"}})).unwrap()
    }
    #[test]
    fn stable_numeric_order_and_metadata() {
        for tag in ["v1.2.3", "1.2.2", "0.99.99", "1.2.3+build.01"] {
            assert_eq!(decode(&fixture(tag), "1.2.3"), CheckResult::UpToDate);
        }
        for tag in ["v1.2.4", "1.10.0", "2.0.0+build"] {
            assert!(matches!(
                decode(&fixture(tag), "1.2.3"),
                CheckResult::Available { .. }
            ));
        }
    }
    #[test]
    fn excludes_drafts_and_all_prereleases() {
        assert_eq!(
            decode(&fixture("9.0.0-rc.1"), "1.0.0"),
            CheckResult::UpToDate
        );
        for field in ["draft", "prerelease"] {
            let mut value: serde_json::Value = serde_json::from_slice(&fixture("9.0.0")).unwrap();
            value[field] = true.into();
            assert_eq!(
                decode(&serde_json::to_vec(&value).unwrap(), "1.0.0"),
                CheckResult::UpToDate
            );
        }
    }
    #[test]
    fn rejects_bad_versions_types_missing_fields_and_size() {
        for tag in [
            "01.2.3",
            "1.2",
            "1.2.3.4",
            "1.2.3-01",
            "1.2.3+",
            "1.2.3+abc+def",
            "18446744073709551616.0.0",
            "vv1.2.3",
        ] {
            assert_eq!(
                decode(&fixture(tag), "1.0.0"),
                CheckResult::Failed(Failure::BadResponse)
            );
        }
        for bytes in [
            b"{".as_slice(),
            b"{}",
            br#"{"tag_name":123}"#,
            br#"{"tag_name":"1.0.0","html_url":"x","draft":false,"prerelease":"false"}"#,
        ] {
            assert_eq!(
                decode(bytes, "1.0.0"),
                CheckResult::Failed(Failure::BadResponse)
            );
        }
        assert_eq!(
            decode(&vec![b' '; RESPONSE_LIMIT + 1], "1.0.0"),
            CheckResult::Failed(Failure::Oversized)
        );
        let mut exact = fixture("1.0.0");
        exact.resize(RESPONSE_LIMIT, b' ');
        assert_eq!(decode(&exact, "1.0.0"), CheckResult::UpToDate);
    }
    #[test]
    fn release_links_stay_on_github() {
        assert!(valid_release_url(
            "https://github.com/tjorim/ElevenClock-rs/releases/tag/v1.2.3"
        ));
        for url in [
            "http://github.com/tjorim/ElevenClock-rs/releases/tag/v1",
            "https://github.com@evil.test/tjorim/ElevenClock-rs/releases/tag/v1",
            "https://github.com/tjorim/ElevenClock-rs/releases/tag/../bad",
            "https://github.com/tjorim/ElevenClock-rs/releases/tag/%2e%2e",
            "https://github.com/tjorim/ElevenClock-rs/releases/tag/v1?x",
            "https://github.com/tjorim/ElevenClock-rs/releases/tag/v1#x",
        ] {
            assert!(!valid_release_url(url));
        }
    }
    #[test]
    fn manual_only_managed_overlap_cancel_and_backoff() {
        let mut c = Coordinator::default();
        assert!(!c.begin(true, 0));
        assert_eq!(c.last, ResultKind::NotChecked);
        assert!(c.begin(false, 0));
        assert!(!c.begin(false, 0));
        c.cancel();
        assert!(!c.begin(false, 0));
        assert!(!c.complete(&CheckResult::UpToDate, false));
        assert_eq!(c.last, ResultKind::NotChecked);
        assert!(c.begin(false, 0));
        assert!(c.complete(
            &CheckResult::Failed(Failure::RateLimited { until: 100 }),
            false
        ));
        assert!(!c.begin(false, 99));
        assert!(c.begin(false, 100));
        assert!(!c.complete(&CheckResult::UpToDate, true));
        assert!(!c.begin(true, 101));
    }
    #[test]
    fn server_backoff_and_bad_headers() {
        assert_eq!(backoff(100, Some("20"), Some("150")), 150);
        assert_eq!(backoff(100, Some("120"), Some("150")), 220);
        assert_eq!(backoff(100, Some("bad"), Some("99")), 86500);
        assert_eq!(backoff(0, Some("Thu, 01 Jan 1970 00:02:00 GMT"), None), 120);
    }
}

/// Runtime policy for the shared manual/automatic worker. All clocks are inputs;
/// persistence is serialized through the same session as Settings Apply.
#[derive(Default)]
pub struct UpdateRuntime {
    coordinator: Coordinator,
    automatic: bool,
    cancelled: bool,
    active: bool,
    active_epoch: u64,
    due: Option<std::time::Duration>,
    epoch: Option<u64>,
    allowed: bool,
    retry_not_before: std::time::Duration,
}
#[derive(Clone, Copy)]
pub struct UpdateContext {
    pub managed: bool,
    pub allowed: bool,
    pub epoch: u64,
    pub wall: u64,
    pub monotonic: std::time::Duration,
}
impl UpdateRuntime {
    /// Returns the absolute monotonic deadline for a single one-shot timer.
    pub fn schedule<S: crate::preferences_store::PreferencesStore>(
        &mut self,
        session: &crate::preferences_store::PreferenceSession<S>,
        context: UpdateContext,
    ) -> Option<std::time::Duration> {
        let allowed = !context.managed && context.allowed;
        let enabled =
            allowed && session.active().update_check_enabled() && session.load_error().is_none();
        if !allowed
            || (self.active && self.active_epoch != context.epoch)
            || (self.automatic && !enabled)
        {
            self.cancel();
        }
        let resumed = self.epoch != Some(context.epoch) || !self.allowed;
        self.epoch = Some(context.epoch);
        self.allowed = allowed;
        if !enabled || self.active {
            self.due = None;
        } else if self.due.is_none() || resumed {
            let history = session.update_history();
            let next = history
                .last_update_check
                .map(|t| t.saturating_add(86400))
                .unwrap_or(0)
                .max(history.backoff_until)
                .max(self.coordinator.backoff_until);
            self.due = Some(
                context
                    .monotonic
                    .saturating_add(std::time::Duration::from_secs(
                        next.saturating_sub(context.wall).max(180),
                    ))
                    .max(self.retry_not_before),
            );
        }
        self.due
    }
    pub fn is_due(&self, now: std::time::Duration) -> bool {
        self.due.is_some_and(|due| now >= due)
    }
    pub fn defer(&mut self) {
        self.due = None;
    }
    pub fn is_automatic(&self) -> bool {
        self.automatic && self.active
    }
    pub fn needs_cancel(&self) -> bool {
        self.active && self.cancelled
    }
    pub fn cancel(&mut self) {
        self.coordinator.cancel();
        self.cancelled = true;
        self.due = None;
    }
    pub fn begin<S: crate::preferences_store::PreferencesStore>(
        &mut self,
        session: &mut crate::preferences_store::PreferenceSession<S>,
        context: UpdateContext,
        automatic: bool,
    ) -> bool {
        if self.active
            || context.managed
            || !context.allowed
            || context.wall
                < session
                    .update_history()
                    .backoff_until
                    .max(self.coordinator.backoff_until)
            || (automatic
                && (!session.active().update_check_enabled()
                    || context.monotonic < self.retry_not_before
                    || !self.is_due(context.monotonic)
                    || session
                        .update_history()
                        .last_update_check
                        .is_some_and(|last| context.wall < last.saturating_add(86400))))
        {
            return false;
        }
        let mut history = session.update_history().clone();
        history.last_update_check = Some(context.wall.max(history.last_update_check.unwrap_or(0)));
        if session.save_update_history(history).is_err() {
            self.retry_not_before = context
                .monotonic
                .saturating_add(std::time::Duration::from_secs(86400));
            self.due = None;
            return false;
        }
        if !self.coordinator.begin(context.managed, context.wall) {
            return false;
        }
        self.active = true;
        self.active_epoch = context.epoch;
        self.automatic = automatic;
        self.cancelled = false;
        self.due = None;
        true
    }
    pub fn accepts_result<S: crate::preferences_store::PreferencesStore>(
        &self,
        session: &crate::preferences_store::PreferenceSession<S>,
        context: UpdateContext,
    ) -> bool {
        self.active
            && self.active_epoch == context.epoch
            && !self.cancelled
            && !context.managed
            && context.allowed
            && (!self.automatic || session.active().update_check_enabled())
    }
    /// A cancelled worker remains in flight until completion, preventing overlap.
    /// Returns (show manual result, emit automatic notification).
    pub fn complete<S: crate::preferences_store::PreferencesStore>(
        &mut self,
        session: &mut crate::preferences_store::PreferenceSession<S>,
        result: &CheckResult,
        context: UpdateContext,
    ) -> (bool, bool) {
        let mut history = session.update_history().clone();
        if let CheckResult::Failed(Failure::RateLimited { until }) = result {
            history.backoff_until = history.backoff_until.max(*until);
        }
        let eligible = self.accepts_result(session, context);
        let notify = eligible
            && self.automatic
            && match result {
                CheckResult::Available { version, .. } => {
                    let fresh = history.last_notified_version.as_ref().is_none_or(|last| {
                        match (self::version(version), self::version(last)) {
                            (Some((new, _)), Some((old, _))) => new > old,
                            _ => false,
                        }
                    });
                    if fresh {
                        history.last_notified_version = Some(version.clone());
                    }
                    fresh
                }
                _ => false,
            };
        let saved = if history != *session.update_history() {
            session.save_update_history(history).is_ok()
        } else {
            true
        };
        self.coordinator.complete(result, !eligible);
        self.active = false;
        self.due = None;
        (eligible && !self.automatic, notify && saved)
    }
}

#[cfg(test)]
mod automatic_tests {
    use super::*;
    use crate::preferences_store::{MemoryPreferencesStore, PreferenceSession};
    use std::time::Duration;

    fn context(seconds: u64) -> UpdateContext {
        UpdateContext {
            managed: false,
            allowed: true,
            epoch: 0,
            wall: 1000 + seconds,
            monotonic: Duration::from_secs(seconds),
        }
    }
    fn session(enabled: bool) -> PreferenceSession<MemoryPreferencesStore> {
        let mut session = PreferenceSession::open(MemoryPreferencesStore::default());
        let mut draft = session.active().draft();
        draft.update_check_enabled = enabled;
        session.apply(draft).unwrap();
        session
    }
    fn available(version: &str) -> CheckResult {
        CheckResult::Available {
            version: version.into(),
            url: format!("https://github.com/tjorim/ElevenClock-rs/releases/tag/v{version}"),
        }
    }
    #[test]
    fn default_off_and_managed_never_schedule_or_dispatch() {
        for (enabled, managed) in [(false, false), (false, true), (true, true)] {
            let mut session = session(enabled);
            let mut runtime = UpdateRuntime::default();
            let ctx = UpdateContext {
                managed,
                ..context(0)
            };
            assert_eq!(runtime.schedule(&session, ctx), None);
            assert!(!runtime.begin(&mut session, ctx, true));
            if managed {
                assert!(!runtime.begin(&mut session, ctx, false));
            }
            assert_eq!(session.update_history().last_update_check, None);
        }
        let mut session = session(false);
        let mut runtime = UpdateRuntime::default();
        assert!(runtime.begin(&mut session, context(0), false));
        assert_eq!(
            runtime.complete(&mut session, &available("1.0.0"), context(1)),
            (true, false)
        );
    }
    #[test]
    fn delayed_daily_offline_restart_rollback_and_manual_throttle() {
        let mut session = session(true);
        let mut runtime = UpdateRuntime::default();
        assert_eq!(
            runtime.schedule(&session, context(0)),
            Some(Duration::from_secs(180))
        );
        assert!(!runtime.begin(&mut session, context(179), true));
        assert!(runtime.begin(&mut session, context(180), true));
        assert_eq!(session.update_history().last_update_check, Some(1180));
        assert!(!runtime.begin(&mut session, context(181), false));
        assert_eq!(
            runtime.complete(
                &mut session,
                &CheckResult::Failed(Failure::Network),
                context(181)
            ),
            (false, false)
        );
        let mut restarted = UpdateRuntime::default();
        assert_eq!(
            restarted.schedule(&session, context(0)),
            Some(Duration::from_secs(86580))
        );
        assert!(!restarted.begin(&mut session, context(86579), true));
        assert!(restarted.begin(&mut session, context(86580), true));
        restarted.complete(&mut session, &CheckResult::UpToDate, context(86581));
        // A deliberate manual attempt also advances the daily deadline.
        assert!(restarted.begin(&mut session, context(86600), false));
        restarted.complete(&mut session, &CheckResult::UpToDate, context(86601));
        assert_eq!(
            restarted.schedule(&session, context(86601)),
            Some(Duration::from_secs(173000))
        );
    }
    #[test]
    fn notification_marker_committed_once_and_never_regresses() {
        let mut session = session(true);
        for (day, release, notify) in [
            (0, "1.0.0", true),
            (1, "1.0.0", false),
            (2, "2.0.0", true),
            (3, "1.0.0", false),
        ] {
            let start = day * 86400;
            let mut runtime = UpdateRuntime::default();
            runtime.schedule(&session, context(start));
            assert!(runtime.begin(&mut session, context(start + 180), true));
            assert_eq!(
                runtime.complete(&mut session, &available(release), context(start + 181)),
                (false, notify)
            );
        }
        assert_eq!(
            session.update_history().last_notified_version.as_deref(),
            Some("2.0.0")
        );
    }
    #[test]
    fn disable_lock_managed_and_exit_suppress_late_results_without_overlap() {
        for cause in 0..4 {
            let mut session = session(true);
            let mut runtime = UpdateRuntime::default();
            runtime.schedule(&session, context(0));
            assert!(runtime.begin(&mut session, context(180), true));
            let mut ctx = context(181);
            match cause {
                0 => {
                    let mut draft = session.active().draft();
                    draft.update_check_enabled = false;
                    session.apply(draft).unwrap();
                }
                1 => ctx.allowed = false,
                2 => ctx.managed = true,
                _ => runtime.cancel(),
            }
            runtime.schedule(&session, ctx);
            assert!(runtime.needs_cancel());
            assert!(!runtime.begin(&mut session, context(182), false));
            assert_eq!(
                runtime.complete(&mut session, &available("1.0.0"), ctx),
                (false, false)
            );
            assert_eq!(session.update_history().last_notified_version, None);
        }
    }
    #[test]
    fn resume_restarts_grace_and_persisted_rate_limit_blocks_both_paths() {
        let mut session = session(true);
        let mut runtime = UpdateRuntime::default();
        runtime.schedule(&session, context(0));
        let resumed = UpdateContext {
            epoch: 1,
            ..context(100)
        };
        assert_eq!(
            runtime.schedule(&session, resumed),
            Some(Duration::from_secs(280))
        );
        assert!(runtime.begin(&mut session, context(280), true));
        runtime.complete(
            &mut session,
            &CheckResult::Failed(Failure::RateLimited { until: 200000 }),
            context(281),
        );
        let mut restarted = UpdateRuntime::default();
        assert!(!restarted.begin(&mut session, context(282), false));
        assert_eq!(
            restarted.schedule(&session, context(282)),
            Some(Duration::from_secs(199000))
        );
    }
    #[test]
    fn failed_attempt_and_notification_commits_suppress_side_effects() {
        use crate::preferences_store::{PreferencesStore, StoreError, StoredPreferences};
        use std::{cell::Cell, rc::Rc};
        struct Store {
            inner: MemoryPreferencesStore,
            fail: Rc<Cell<bool>>,
        }
        impl PreferencesStore for Store {
            fn load(&self) -> Result<StoredPreferences, StoreError> {
                self.inner.load()
            }
            fn save(&mut self, value: &StoredPreferences) -> Result<(), StoreError> {
                if self.fail.get() {
                    return Err(StoreError::Io(std::io::Error::other("injected")));
                }
                self.inner.save(value)
            }
            fn preserve_for_recovery(&mut self) -> Result<(), StoreError> {
                Ok(())
            }
        }
        let fail = Rc::new(Cell::new(false));
        let mut session = PreferenceSession::open(Store {
            inner: MemoryPreferencesStore::default(),
            fail: fail.clone(),
        });
        let mut draft = session.active().draft();
        draft.update_check_enabled = true;
        session.apply(draft).unwrap();
        let mut runtime = UpdateRuntime::default();
        runtime.schedule(&session, context(0));
        fail.set(true);
        assert!(!runtime.begin(&mut session, context(180), true));
        assert_eq!(session.update_history().last_update_check, None);
        assert_eq!(
            runtime.schedule(&session, context(180)),
            Some(Duration::from_secs(86580))
        );
        fail.set(false);
        assert!(runtime.begin(&mut session, context(86580), true));
        fail.set(true);
        assert_eq!(
            runtime.complete(&mut session, &available("1.0.0"), context(86581)),
            (false, false)
        );
        assert_eq!(session.update_history().last_notified_version, None);
        fail.set(false);
        runtime.schedule(&session, context(86581));
        assert!(runtime.begin(&mut session, context(172980), true));
        assert_eq!(
            runtime.complete(&mut session, &available("1.0.0"), context(172981)),
            (false, true)
        );
    }
}
