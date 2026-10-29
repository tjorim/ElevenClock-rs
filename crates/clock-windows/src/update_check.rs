//! Manual, bounded GitHub release checks (ADR 0003). No work starts on construction.
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
fn version(input: &str) -> Option<([u64; 3], bool)> {
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
