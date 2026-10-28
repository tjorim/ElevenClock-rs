//! What the launcher does when it is started (ADR 0004).
//!
//! The flow is plain code over a [`Host`], the few things only the operating system
//! can answer: is the app running, does the installer's entry exist, start a program,
//! show a message. Files are real files, so the whole flow runs against a temporary
//! folder on any host, with a scripted `Host`, and on Windows against the real one.
//!
//! In order: refuse a mixed install; leave an app that is running alone; find, probe
//! and validate the central copy; copy it when its hash differs; refresh the local
//! launcher and remember the source; start the local app. Anything that goes wrong on
//! the way to a fresh copy falls back to the local copy that is already there.
use crate::{
    args::Launch,
    fsops::{self, install, install_running},
    pe, sha256,
    source::{
        self, APP_EXE, LAUNCHER_EXE, LAUNCHER_OLD, Listed, Probe, STATE_FILE, SUMS, check_sums,
    },
};
use clock_windows::startup::RunKey;
use std::{
    any::Any,
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    time::Duration,
};

/// How long an unreachable central copy may hold the launch up.
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(3);

pub const INSTALLED_COPY: &str = "ElevenClock-rs is already installed for this account; \
    uninstall it first. The managed copy and the installed copy cannot share a folder.";
const NO_SOURCE: &str = "ElevenClock-rs could not be started: the central copy cannot be \
    reached and this PC has no local copy yet. Connect to the company network or VPN and \
    start ElevenClock-rs again.";
const BAD_SOURCE: &str = "ElevenClock-rs could not be started: the central copy is not a \
    valid ElevenClock-rs program. Ask your administrator.";
const NOT_COPIED: &str = "ElevenClock-rs could not be started: copying it to this PC failed. \
    Try again, or ask your administrator.";
const NOT_STARTED: &str = "ElevenClock-rs could not be started.";
const NOT_RECORDED: &str = "ElevenClock-rs was started, but this PC could not remember where \
    the central copy is. It will not update at sign-in until you start ElevenClock-rs from the \
    central folder again.";

/// The process exit code: every outcome has one and a log line.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exit {
    Done,
    Internal,
    Usage,
    /// The installer's copy owns the folder.
    Refused,
    /// No usable central copy and no local copy.
    NoCopy,
    StartFailed,
    UninstallIncomplete,
}
impl Exit {
    pub fn code(self) -> u8 {
        match self {
            Self::Done => 0,
            Self::Internal => 1,
            Self::Usage => 2,
            Self::Refused => 3,
            Self::NoCopy => 4,
            Self::StartFailed => 5,
            Self::UninstallIncomplete => 6,
        }
    }
}

/// Whether the app is running for this user and session.
pub enum Claim {
    /// Not running. The value keeps the per-user instance claimed, so the app cannot
    /// start while its file is being replaced; drop it before starting the app.
    Free(Box<dyn Any>),
    Running,
    /// The check itself failed; treated like running, because replacing a file that
    /// might be in use is the one thing that must not happen.
    Unknown,
}

pub trait Host {
    /// The installer's uninstall entry exists for this account.
    fn installer_entry_present(&mut self) -> bool;
    fn claim_instance(&mut self) -> Claim;
    /// Start the app, detached, and let it take the foreground.
    fn start_app(&mut self, exe: &Path, args: &[OsString]) -> io::Result<()>;
    fn show_error(&mut self, message: &str);
    /// One decision per line, no paths or user names.
    fn log(&mut self, line: &str);
    /// Called just before `target` is swapped for a verified copy. Tests fail it to
    /// simulate an interrupted copy.
    fn before_replace(&mut self, _target: &Path) -> io::Result<()> {
        Ok(())
    }
    /// The Run value and its Windows flag.
    fn run_key(&mut self) -> &mut dyn RunKey;
}

/// The facts about this run that do not depend on the arguments.
pub struct Context {
    /// `%LOCALAPPDATA%\Programs\ElevenClock-rs`.
    pub destination: PathBuf,
    /// This launcher's own file.
    pub own_exe: PathBuf,
    pub probe_timeout: Duration,
}

/// The files the launcher manages in the destination folder, and no others.
pub struct Layout {
    destination: PathBuf,
}
impl Layout {
    pub fn new(destination: &Path) -> Self {
        Self {
            destination: destination.to_path_buf(),
        }
    }
    pub fn app(&self) -> PathBuf {
        self.destination.join(APP_EXE)
    }
    pub fn launcher(&self) -> PathBuf {
        self.destination.join(LAUNCHER_EXE)
    }
    pub fn launcher_old(&self) -> PathBuf {
        self.destination.join(LAUNCHER_OLD)
    }
    pub fn state(&self) -> PathBuf {
        self.destination.join(STATE_FILE)
    }
}

/// A short, path-free description of an I/O failure for the log.
pub fn describe(error: &io::Error) -> String {
    match error.raw_os_error() {
        Some(code) => format!("{:?}, os error {code}", error.kind()),
        None => format!("{:?}", error.kind()),
    }
}

/// Why there is no fresh copy.
enum Problem {
    Unreachable,
    Invalid,
    NotCopied,
}
impl Problem {
    fn message(&self) -> &'static str {
        match self {
            Self::Unreachable => NO_SOURCE,
            Self::Invalid => BAD_SOURCE,
            Self::NotCopied => NOT_COPIED,
        }
    }
}

pub fn launch(host: &mut dyn Host, ctx: &Context, request: &Launch) -> Exit {
    let layout = Layout::new(&ctx.destination);
    if host.installer_entry_present() {
        host.log("refused: installed copy");
        // A sign-in start shows no message boxes, only the log.
        if !request.quiet {
            host.show_error(INSTALLED_COPY);
        }
        return Exit::Refused;
    }
    // Checked before the central copy is touched: a running app means no update, so
    // there is no reason to wait for a share that may be down.
    let claim = host.claim_instance();
    let local_copy = layout.app().is_file();
    if local_copy && !matches!(claim, Claim::Free(_)) {
        host.log(match claim {
            Claim::Running => "skipped: running",
            _ => "skipped: running check failed",
        });
        drop(claim);
        return start(host, request, &layout);
    }
    match refresh(host, ctx, request, &layout) {
        Ok(()) => {}
        Err(_) if local_copy => host.log("using the local copy"),
        Err(problem) => {
            drop(claim);
            host.log("no copy to start");
            if !request.quiet {
                host.show_error(problem.message());
            }
            return Exit::NoCopy;
        }
    }
    drop(claim);
    start(host, request, &layout)
}

fn start(host: &mut dyn Host, request: &Launch, layout: &Layout) -> Exit {
    match host.start_app(&layout.app(), &request.app_args) {
        Ok(()) => {
            host.log("started");
            Exit::Done
        }
        Err(error) => {
            host.log(&format!("start failed: {}", describe(&error)));
            if !request.quiet {
                host.show_error(NOT_STARTED);
            }
            Exit::StartFailed
        }
    }
}

/// Brings the local app up to the central copy, then the launcher and the record of
/// where the source is. Logs what it decides.
fn refresh(
    host: &mut dyn Host,
    ctx: &Context,
    request: &Launch,
    layout: &Layout,
) -> Result<(), Problem> {
    let own_folder = ctx.own_exe.parent().unwrap_or(Path::new(""));
    let state = fsops::read_limited(&layout.state())
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok());
    let Some(folder) = source::resolve(
        request.source.as_deref(),
        own_folder,
        &ctx.destination,
        state.as_deref(),
    ) else {
        host.log("source unreachable: none known");
        return Err(Problem::Unreachable);
    };
    // The local folder is never a source: copying it onto itself would only ever
    // overwrite the record of where the real one is. The text comparison is the fast
    // path; the probe also catches junctions and short names.
    if source::same_folder(&folder, &ctx.destination) {
        host.log("source unreachable: it is the local folder");
        return Err(Problem::Unreachable);
    }
    let found = match source::probe(&folder, &ctx.destination, ctx.probe_timeout) {
        Probe::Found(found) => found,
        Probe::IsDestination => {
            host.log("source unreachable: it is the local folder");
            return Err(Problem::Unreachable);
        }
        Probe::Unreachable => {
            host.log("source unreachable");
            return Err(Problem::Unreachable);
        }
    };
    let bytes = match source::read_bounded(&found.path, ctx.probe_timeout) {
        Ok(bytes) => bytes,
        Err(error) => {
            host.log(&format!("source unreachable: {}", describe(&error)));
            return Err(Problem::Unreachable);
        }
    };
    let digest = sha256::digest(&bytes);
    if let Err(rejection) = pe::validate(&bytes, APP_EXE) {
        host.log(&format!("source invalid: {}", rejection.describe()));
        return Err(Problem::Invalid);
    }
    let sums = source::read_bounded(&folder.join(SUMS), ctx.probe_timeout)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok());
    if sums.is_some_and(|list| check_sums(&list, found.name, &digest) == Listed::Differs) {
        host.log("source invalid: does not match its checksum list");
        return Err(Problem::Invalid);
    }

    let app = layout.app();
    if fsops::hash_of(&app) == Some(digest) {
        host.log("unchanged");
    } else {
        match install(&app, &bytes, || host.before_replace(&app)) {
            Ok(()) => host.log("updated"),
            Err(error) => {
                host.log(&format!("update failed: {}", describe(&error)));
                return Err(Problem::NotCopied);
            }
        }
    }
    refresh_launcher(host, ctx, layout, &folder);
    if !remember_source(host, layout, &folder) && !request.quiet {
        host.show_error(NOT_RECORDED);
    }
    Ok(())
}

/// Best effort: a launcher that cannot be refreshed never keeps the app from starting.
fn refresh_launcher(host: &mut dyn Host, ctx: &Context, layout: &Layout, folder: &Path) {
    let timeout = ctx.probe_timeout;
    let central = folder.join(LAUNCHER_EXE);
    let started_from_share = !ctx
        .own_exe
        .parent()
        .is_some_and(|own| source::same_folder(own, &ctx.destination));
    let (from, read) = match source::read_bounded(&central, timeout) {
        Ok(bytes) => (central, Ok(bytes)),
        // Started from the share, which holds the launcher under another name.
        Err(_) if started_from_share => (
            ctx.own_exe.clone(),
            source::read_bounded(&ctx.own_exe, timeout),
        ),
        // The central folder has no launcher: nothing to refresh from.
        Err(error) if error.kind() == io::ErrorKind::NotFound => return,
        Err(error) => (central, Err(error)),
    };
    let target = layout.launcher();
    if source::same_folder(&from, &target) {
        return;
    }
    let bytes = match read {
        Ok(bytes) => bytes,
        Err(error) => {
            host.log(&format!("launcher refresh skipped: {}", describe(&error)));
            return;
        }
    };
    if let Err(rejection) = pe::validate(&bytes, LAUNCHER_EXE) {
        host.log(&format!(
            "launcher refresh skipped: {}",
            rejection.describe()
        ));
        return;
    }
    if fsops::hash_of(&target) == Some(sha256::digest(&bytes)) {
        return;
    }
    let result = install_running(&target, &layout.launcher_old(), &bytes, || {
        host.before_replace(&target)
    });
    match result {
        Ok(()) => host.log("launcher updated"),
        Err(error) => host.log(&format!("launcher refresh failed: {}", describe(&error))),
    }
}

/// The local launcher needs to know where to look at the next login.
/// Whether the record is in place afterwards.
fn remember_source(host: &mut dyn Host, layout: &Layout, folder: &Path) -> bool {
    let Some(text) = source::state_text(folder) else {
        host.log("source not recorded: not storable as text");
        return false;
    };
    let state = layout.state();
    if fsops::read_limited(&state).is_ok_and(|current| current == text.as_bytes()) {
        return true;
    }
    match install(&state, text.as_bytes(), || host.before_replace(&state)) {
        Ok(()) => {
            host.log("source recorded");
            true
        }
        Err(error) => {
            host.log(&format!("source not recorded: {}", describe(&error)));
            false
        }
    }
}

#[cfg(test)]
mod tests;
