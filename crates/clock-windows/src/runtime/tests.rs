//! Deterministic coordinator tests. No test sleeps, reads live time or touches
//! the OS: every platform answer is scripted, so each rule is checked on any host.
use super::*;
use crate::{
    events::{ShellEvent, Signal},
    interaction::{ACTIVATION_COOLDOWN, CalendarOutcome, MenuCommand, ScreenPoint},
    render::PixelSize,
};
use chrono::DateTime;
use clock_core::{DiscoveryStrategy, MonitorId, Monitors, Preferences, TaskbarId, Theme};
use std::collections::{BTreeMap, HashSet};

fn wall(value: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(value).unwrap()
}
fn rect(l: i32, t: i32, w: i32, h: i32) -> PhysicalRect {
    PhysicalRect::new(l, t, w, h).unwrap()
}
fn prefs(monitors: Monitors) -> ValidatedPreferences {
    Preferences {
        monitors,
        ..Preferences::default()
    }
    .validate()
    .unwrap()
}
fn key(generation: u64, id: u64) -> TaskbarKey {
    TaskbarKey {
        generation: ShellGeneration(generation),
        id: TaskbarId(id),
    }
}
/// id 1: primary at the origin; id 2: secondary left of it (negative coordinates).
fn bar(generation: u64, id: u64, clock: bool) -> TaskbarSnapshot {
    let (left, primary) = if id == 1 { (0, true) } else { (-1920, false) };
    TaskbarSnapshot {
        key: key(generation, id),
        taskbar_rect: rect(left, 1032, 1920, 48),
        clock_rect: clock.then(|| rect(left + 1800, 1032, 100, 48)),
        monitor: MonitorId(id),
        monitor_rect: rect(left, 0, 1920, 1080),
        dpi: if id == 1 { 96 } else { 144 },
        primary,
        visibility: TaskbarVisibility::Visible,
        strategy: DiscoveryStrategy::NativeChildren,
    }
}
fn shell(pid: u32, started: u64) -> Option<ShellIdentity> {
    Some(ShellIdentity { pid, started })
}

#[derive(Clone, Debug, PartialEq)]
enum Call {
    RequestDiscovery(u64),
    Create(TaskbarKey),
    SetBounds(TaskbarKey),
    Destroy(TaskbarKey),
    Paint(TaskbarKey, Vec<String>),
    Hide(TaskbarKey),
    Timer(Duration),
    Poll(Duration),
    ShellChanged(Option<ShellIdentity>),
    OpenCalendar(TaskbarKey),
    ShowMenu(Option<ScreenPoint>, bool),
    OpenSettings,
    RestoreTray,
    PausedChanged(bool),
    Stop,
}

struct Fake {
    calls: Vec<Call>,
    mono: Duration,
    now: DateTime<FixedOffset>,
    appearance: SystemAppearance,
    font_changed: bool,
    inventory_changed: bool,
    shell: Option<ShellIdentity>,
    observations: BTreeMap<u64, Observation>,
    overlays: HashSet<TaskbarKey>,
    paint_result: BTreeMap<u64, PaintResult>,
    created_bounds: Vec<PhysicalRect>,
    create_error: bool,
    bounds_error: bool,
    overlay_failures: Vec<(TaskbarKey, Reason)>,
    calendar: CalendarOutcome,
    menu_choice: Option<MenuCommand>,
    /// Slot width the fake renderer needs; `None` means it cannot measure.
    fit_needs_width: Option<i32>,
    /// Measuring fails (a transient native error) instead of answering.
    fit_error: bool,
    fit_calls: Vec<(PixelSize, u32, u8)>,
}
impl Fake {
    fn new() -> Self {
        Self {
            calls: Vec::new(),
            mono: Duration::from_secs(100),
            now: wall("2026-10-14T12:00:30+00:00"),
            appearance: SystemAppearance::Light,
            font_changed: false,
            inventory_changed: false,
            shell: shell(10, 1),
            observations: BTreeMap::new(),
            overlays: HashSet::new(),
            paint_result: BTreeMap::new(),
            created_bounds: Vec::new(),
            create_error: false,
            bounds_error: false,
            overlay_failures: Vec::new(),
            calendar: CalendarOutcome::Sent,
            menu_choice: None,
            fit_needs_width: None,
            fit_error: false,
            fit_calls: Vec::new(),
        }
    }
}
impl Platform for Fake {
    fn monotonic(&mut self) -> Duration {
        self.mono
    }
    fn local_now(&mut self) -> DateTime<FixedOffset> {
        self.now
    }
    fn system_appearance(&mut self) -> SystemAppearance {
        self.appearance
    }
    fn refresh_fonts(&mut self) -> bool {
        std::mem::take(&mut self.font_changed)
    }
    fn shell_identity(&mut self) -> Option<ShellIdentity> {
        self.shell
    }
    fn shell_changed(&mut self, shell: Option<ShellIdentity>) {
        self.calls.push(Call::ShellChanged(shell));
    }
    fn request_discovery(&mut self, generation: ShellGeneration) {
        self.calls.push(Call::RequestDiscovery(generation.0));
    }
    fn note_taskbars(&mut self, _: &[TaskbarSnapshot]) {}
    fn taskbar_inventory_changed(&mut self) -> bool {
        self.inventory_changed
    }
    fn observe(&mut self, taskbar: &TaskbarSnapshot) -> Observation {
        // Unscripted taskbars look exactly as discovered.
        self.observations
            .get(&taskbar.key.id.0)
            .copied()
            .unwrap_or(Observation {
                exists: true,
                visible: true,
                rect: Some(taskbar.taskbar_rect),
                dpi: taskbar.dpi,
                monitor_rect: Some(taskbar.monitor_rect),
                fullscreen: false,
                tray: None,
            })
    }
    fn create_overlay(&mut self, key: TaskbarKey, bounds: PhysicalRect) -> Result<(), String> {
        self.calls.push(Call::Create(key));
        self.created_bounds.push(bounds);
        if self.create_error {
            return Err("CreateWindowExW failed".into());
        }
        self.overlays.insert(key);
        Ok(())
    }
    fn set_bounds(&mut self, key: TaskbarKey, _: PhysicalRect) -> Result<(), String> {
        self.calls.push(Call::SetBounds(key));
        if self.bounds_error {
            return Err("SetWindowPos failed".into());
        }
        Ok(())
    }
    fn destroy_overlay(&mut self, key: TaskbarKey) {
        self.calls.push(Call::Destroy(key));
        self.overlays.remove(&key);
    }
    fn paint(&mut self, key: TaskbarKey, context: &PaintContext<'_>) -> PaintResult {
        self.calls
            .push(Call::Paint(key, context.text.lines.clone()));
        self.paint_result
            .get(&key.id.0)
            .cloned()
            .unwrap_or(PaintResult::Painted)
    }
    fn hide(&mut self, key: TaskbarKey) {
        self.calls.push(Call::Hide(key));
    }
    fn plan_fit(
        &mut self,
        _: &ClockText,
        _: &WorstCase,
        region: PixelSize,
        dpi: u32,
        font_pt: u8,
    ) -> Option<Result<Fit, RenderError>> {
        let needs = self.fit_needs_width?;
        self.fit_calls.push((region, dpi, font_pt));
        if self.fit_error {
            return Some(Err(RenderError::InvalidMeasurement));
        }
        let required = PixelSize {
            width: needs,
            height: region.height,
        };
        Some(Ok(if needs > region.width {
            Fit::Overflow(Overflow {
                required,
                available: region,
            })
        } else {
            Fit::Fits(crate::render::Layout {
                font_px: 12.0,
                padding_px: 4,
                region,
                required,
            })
        }))
    }
    fn open_calendar(&mut self, key: TaskbarKey) -> CalendarOutcome {
        self.calls.push(Call::OpenCalendar(key));
        self.calendar.clone()
    }
    fn show_menu(&mut self, anchor: Option<ScreenPoint>, paused: bool) -> Option<MenuCommand> {
        self.calls.push(Call::ShowMenu(anchor, paused));
        self.menu_choice.take()
    }
    fn open_settings(&mut self) {
        self.calls.push(Call::OpenSettings);
    }
    fn restore_tray(&mut self) {
        self.calls.push(Call::RestoreTray);
    }
    fn paused_changed(&mut self, paused: bool) {
        self.calls.push(Call::PausedChanged(paused));
    }
    fn take_overlay_failures(&mut self) -> Vec<(TaskbarKey, Reason)> {
        std::mem::take(&mut self.overlay_failures)
    }
    fn arm_timer(&mut self, delay: Duration) {
        self.calls.push(Call::Timer(delay));
    }
    fn arm_poll(&mut self, delay: Duration) {
        self.calls.push(Call::Poll(delay));
    }
    fn stop(&mut self) {
        self.calls.push(Call::Stop);
    }
}

fn runtime(monitors: Monitors) -> Runtime<Fake> {
    let mut runtime = Runtime::new(Fake::new(), prefs(monitors));
    runtime.start();
    runtime
}
fn report(runtime: &Runtime<Fake>, generation: u64, snapshots: Vec<TaskbarSnapshot>) -> Input {
    Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(generation),
        shell: runtime.shell,
        outcome: Ok(snapshots),
    })
}
fn discover(runtime: &mut Runtime<Fake>, generation: u64, snapshots: Vec<TaskbarSnapshot>) {
    let input = report(runtime, generation, snapshots);
    runtime.handle(input);
}
fn take(runtime: &mut Runtime<Fake>) -> Vec<Call> {
    std::mem::take(&mut runtime.platform.calls)
}
fn advance(runtime: &mut Runtime<Fake>, by: Duration) {
    runtime.platform.mono += by;
    runtime.handle(Input::Poll);
}
fn requests(calls: &[Call]) -> Vec<u64> {
    calls
        .iter()
        .filter_map(|c| match c {
            Call::RequestDiscovery(g) => Some(*g),
            _ => None,
        })
        .collect()
}
fn paints(calls: &[Call]) -> usize {
    calls
        .iter()
        .filter(|c| matches!(c, Call::Paint(..)))
        .count()
}

#[test]
fn start_reads_time_once_requests_discovery_and_arms_both_timers() {
    let mut runtime = Runtime::new(Fake::new(), prefs(Monitors::All));
    runtime.start();
    let calls = take(&mut runtime);
    assert_eq!(
        calls,
        vec![
            Call::ShellChanged(shell(10, 1)),
            Call::RestoreTray,
            // 12:00:30 with a minute cadence: next boundary in 30 s.
            Call::Timer(Duration::from_secs(30)),
            Call::RequestDiscovery(0),
            Call::Poll(RECOVERY_POLL),
        ]
    );
}

#[test]
fn discovery_creates_and_paints_every_selected_monitor_without_restart() {
    let mut runtime = runtime(Monitors::All);
    take(&mut runtime);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Create(key(0, 1))));
    assert!(calls.contains(&Call::Create(key(0, 2))));
    assert_eq!(paints(&calls), 2);
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
    let diagnostics = runtime.diagnostics();
    assert!(diagnostics.taskbars.iter().all(|t| t.reason.is_none()));
}

#[test]
fn primary_mode_paints_only_the_primary_and_explains_the_other() {
    let mut runtime = runtime(Monitors::Primary);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    assert!(runtime.platform.overlays.contains(&key(0, 1)));
    assert!(!runtime.platform.overlays.contains(&key(0, 2)));
    let secondary = &runtime.diagnostics().taskbars[1];
    assert_eq!(
        secondary.state,
        OverlayState::Hidden(clock_core::HiddenReason::Paused)
    );
    assert_eq!(secondary.reason, Some(Reason::NotShown));
}

#[test]
fn timer_takes_one_fresh_snapshot_and_repaints_only_changed_text() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    // Same minute: nothing changes visually but the one-shot is re-armed.
    runtime.platform.now = wall("2026-10-14T12:00:59+00:00");
    runtime.handle(Input::Timer);
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls), 0);
    assert_eq!(calls, vec![Call::Timer(Duration::from_secs(1))]);
    runtime.platform.now = wall("2026-10-14T12:01:00+00:00");
    runtime.handle(Input::Timer);
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls), 2);
    assert!(calls.contains(&Call::Timer(Duration::from_secs(60))));
}

#[test]
fn clock_changes_and_resume_take_fresh_time_instead_of_extrapolating() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    // Manual change backwards by a day (also covers time zone / DST edits).
    runtime.platform.now = wall("2026-10-13T08:15:00+01:00");
    runtime.handle(Input::Signal(Signal::TimeChanged));
    let calls = take(&mut runtime);
    assert!(
        calls
            .iter()
            .any(|c| matches!(c, Call::Paint(_, lines) if lines[0] == "08:15"))
    );
    assert!(calls.contains(&Call::Timer(Duration::from_secs(60))));
    // Resume: time, then topology (after the settle window), appearance and shell.
    runtime.platform.now = wall("2026-10-14T09:00:00+02:00");
    runtime.handle(Input::Signal(Signal::Resumed));
    let calls = take(&mut runtime);
    assert!(
        calls
            .iter()
            .any(|c| matches!(c, Call::Paint(_, lines) if lines[0] == "09:00"))
    );
    assert!(
        requests(&calls).is_empty(),
        "topology waits for the settle window"
    );
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
}

#[test]
fn topology_bursts_settle_into_one_discovery_and_follow_up_after_an_in_flight_one() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    for _ in 0..5 {
        runtime.handle(Input::Signal(Signal::TopologyChanged));
    }
    assert!(requests(&take(&mut runtime)).is_empty());
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    // Another change while that discovery is running requires a second refresh,
    // because the in-flight answer may predate it.
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    assert!(runtime.platform.overlays.contains(&key(0, 2)));
}

#[test]
fn taskbar_created_replaces_the_generation_and_rejects_old_discovery_results() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    runtime.platform.shell = shell(77, 5);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Destroy(key(0, 1))));
    assert!(calls.contains(&Call::Destroy(key(0, 2))));
    assert_eq!(requests(&calls), vec![1]);
    assert!(calls.contains(&Call::ShellChanged(shell(77, 5))));
    assert!(runtime.platform.overlays.is_empty());
    // A late result from the old Explorer must not resurrect anything.
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    assert!(
        take(&mut runtime)
            .iter()
            .all(|c| !matches!(c, Call::Create(_)))
    );
    assert!(runtime.platform.overlays.is_empty());
    runtime.platform.shell = shell(77, 5);
    discover(&mut runtime, 1, vec![bar(1, 1, true), bar(1, 2, true)]);
    assert!(runtime.platform.overlays.contains(&key(1, 1)));
    assert!(runtime.platform.overlays.contains(&key(1, 2)));
}

#[test]
fn a_reused_pid_with_a_new_creation_time_is_a_new_shell() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.platform.shell = shell(10, 999);
    advance(&mut runtime, RECOVERY_POLL);
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Destroy(key(0, 1))));
    assert_eq!(requests(&calls), vec![1]);
}

#[test]
fn a_missing_explorer_removes_overlays_and_recovers_when_it_returns() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.platform.shell = None;
    advance(&mut runtime, RECOVERY_POLL);
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Destroy(key(0, 1))));
    assert_eq!(requests(&calls), vec![1]);
    // No Explorer: an empty batch is legitimate and not retried as a failure.
    discover(&mut runtime, 1, vec![]);
    assert_eq!(runtime.diagnostics().consecutive_failures, 0);
    runtime.platform.shell = shell(88, 9);
    advance(&mut runtime, RECOVERY_POLL);
    assert_eq!(requests(&take(&mut runtime)), vec![2]);
    discover(&mut runtime, 2, vec![bar(2, 1, true)]);
    assert!(runtime.platform.overlays.contains(&key(2, 1)));
}

#[test]
fn a_result_for_a_replaced_shell_is_rejected_even_if_the_generation_matches() {
    let mut runtime = runtime(Monitors::All);
    take(&mut runtime);
    runtime.platform.shell = shell(10, 1);
    let input = Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(0),
        shell: shell(4, 4), // helper saw some other Explorer
        outcome: Ok(vec![bar(0, 1, true)]),
    });
    runtime.handle(input);
    assert!(runtime.platform.overlays.is_empty());
    assert!(matches!(
        runtime.diagnostics().last_discovery_failure,
        Some(DiscoveryFailure::Rejected(_))
    ));
}

#[test]
fn failed_discovery_fails_closed_then_retries_with_bounded_backoff() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    // Topology refresh whose helper hits the deadline.
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    take(&mut runtime);
    let fail = |runtime: &mut Runtime<Fake>, failure| {
        let generation = runtime.state().generation().0;
        runtime.handle(Input::Discovery(DiscoveryReport {
            generation: ShellGeneration(generation),
            shell: runtime.shell,
            outcome: Err(failure),
        }));
    };
    fail(&mut runtime, DiscoveryFailure::Deadline);
    let calls = take(&mut runtime);
    assert!(
        calls.contains(&Call::Destroy(key(0, 1))),
        "partial output is discarded"
    );
    assert!(runtime.platform.overlays.is_empty());
    // First retry after 2 s, then 4 s, 8 s ... capped at 60 s.
    let mut waits = Vec::new();
    for _ in 0..8 {
        let before = runtime.platform.mono;
        loop {
            advance(&mut runtime, Duration::from_millis(500));
            if !requests(&take(&mut runtime)).is_empty() {
                break;
            }
        }
        waits.push((runtime.platform.mono - before).as_secs_f32());
        fail(&mut runtime, DiscoveryFailure::HelperExit(Some(1)));
        take(&mut runtime);
    }
    assert_eq!(runtime.diagnostics().consecutive_failures, 9);
    assert!(waits[0] >= 2.0 && waits[0] < 2.6, "{waits:?}");
    assert!(waits[1] >= 4.0 && waits[1] < 4.6, "{waits:?}");
    assert!(waits[2] >= 8.0 && waits[2] < 8.6, "{waits:?}");
    assert!(waits.iter().all(|w| *w <= 60.6), "{waits:?}");
    assert!(waits[7] >= 60.0, "backoff reaches its cap: {waits:?}");
    // Recovery clears the failure state and restores the overlay.
    let generation = runtime.state().generation().0;
    loop {
        advance(&mut runtime, Duration::from_millis(500));
        if !requests(&take(&mut runtime)).is_empty() {
            break;
        }
    }
    discover(&mut runtime, generation, vec![bar(generation, 1, true)]);
    assert_eq!(runtime.diagnostics().consecutive_failures, 0);
    assert!(runtime.platform.overlays.contains(&key(generation, 1)));
}

#[test]
fn a_failed_helper_teardown_disables_discovery_without_further_requests() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    take(&mut runtime);
    runtime.handle(Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(0),
        shell: runtime.shell,
        outcome: Err(DiscoveryFailure::TeardownFailed),
    }));
    assert!(runtime.diagnostics().discovery_disabled);
    assert!(
        runtime.platform.overlays.is_empty(),
        "native clock restored"
    );
    take(&mut runtime);
    // No signal or poll can start another helper, but Explorer loss still
    // resolves the reducer without one.
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    for _ in 0..10 {
        advance(&mut runtime, Duration::from_secs(30));
    }
    runtime.platform.shell = shell(5, 5);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    assert!(requests(&take(&mut runtime)).is_empty());
}

#[test]
fn teardown_failure_for_a_superseded_generation_still_releases_the_reducer() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    runtime.platform.shell = shell(5, 5);
    runtime.handle(Input::Signal(Signal::TaskbarCreated)); // generation 1 requested
    take(&mut runtime);
    // The worker died while reaping generation 0's helper.
    runtime.handle(Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(0),
        shell: None,
        outcome: Err(DiscoveryFailure::TeardownFailed),
    }));
    assert!(runtime.diagnostics().discovery_disabled);
    // The outstanding generation-1 request was answered locally: a later
    // Explorer loss therefore starts no helper and still completes cleanly.
    runtime.platform.shell = shell(6, 6);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    assert!(requests(&take(&mut runtime)).is_empty());
    assert!(runtime.platform.overlays.is_empty());
}

#[test]
fn an_empty_batch_with_a_running_shell_is_retried_not_trusted() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![]);
    assert_eq!(
        runtime.diagnostics().last_discovery_failure,
        Some(DiscoveryFailure::NoTaskbar)
    );
    take(&mut runtime);
    advance(&mut runtime, RECOVERY_POLL);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
}

#[test]
fn auto_hide_hides_and_restores_each_monitor_independently() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    let sliding = Observation {
        rect: Some(rect(0, 1070, 1920, 48)),
        ..runtime.platform.observe(&bar(0, 1, true))
    };
    runtime.platform.observations.insert(1, sliding);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::TaskbarChanged)));
    let calls = take(&mut runtime);
    assert_eq!(
        calls,
        vec![
            Call::Hide(key(0, 1)),
            Call::Poll(crate::events::TRANSITION_POLL)
        ]
    );
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
    // The slide finishes: shown again, which needs a repaint of the hidden window.
    runtime.platform.observations.remove(&1);
    advance(&mut runtime, Duration::from_millis(100));
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls), 1);
    assert!(!calls.iter().any(|c| matches!(c, Call::Create(_))));
}

#[test]
fn transition_polling_is_short_and_bounded() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::TaskbarChanged)));
    assert!(take(&mut runtime).contains(&Call::Poll(crate::events::TRANSITION_POLL)));
    // Nothing changes: after two stable polls the cadence relaxes to two seconds.
    let mut delays = Vec::new();
    for _ in 0..4 {
        advance(&mut runtime, Duration::from_millis(100));
        delays.extend(take(&mut runtime).into_iter().filter_map(|c| match c {
            Call::Poll(d) => Some(d),
            _ => None,
        }));
    }
    assert_eq!(*delays.last().unwrap(), RECOVERY_POLL, "{delays:?}");
    assert!(delays.contains(&crate::events::TRANSITION_POLL));
}

#[test]
fn clock_ticks_cannot_postpone_the_recovery_poll() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    for second in 1..30 {
        runtime.platform.now = wall(&format!("2026-10-14T12:00:{second:02}+00:00"));
        runtime.handle(Input::Timer);
    }
    assert!(
        take(&mut runtime)
            .iter()
            .all(|c| !matches!(c, Call::Poll(_))),
        "the armed poll timer is reused"
    );
}

#[test]
fn parked_discovery_recovers_both_displays_as_they_reveal_independently() {
    let mut runtime = runtime(Monitors::All);
    let parked = |id| {
        let mut snapshot = bar(0, id, false);
        snapshot.taskbar_rect = rect(snapshot.monitor_rect.left(), 1078, 1920, 48);
        snapshot
    };
    discover(&mut runtime, 0, vec![parked(1), parked(2)]);
    assert!(runtime.platform.overlays.is_empty());
    assert!(
        runtime
            .reported
            .values()
            .all(|r| r.0 == TaskbarVisibility::Hidden)
    );
    take(&mut runtime);

    // Primary reveals while secondary remains parked; only primary is measured.
    let primary = runtime.platform.observe(&bar(0, 1, true));
    runtime.platform.observations.insert(1, primary);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::TaskbarChanged)));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    discover(&mut runtime, 0, vec![bar(0, 1, true), parked(2)]);
    assert!(runtime.platform.overlays.contains(&key(0, 1)));
    assert!(!runtime.platform.overlays.contains(&key(0, 2)));
    take(&mut runtime);

    // No event is required for the secondary: recovery polling sees its reveal.
    let secondary = runtime.platform.observe(&bar(0, 2, true));
    runtime.platform.observations.insert(2, secondary);
    advance(&mut runtime, RECOVERY_POLL);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    assert_eq!(runtime.platform.overlays.len(), 2);
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
}

#[test]
fn an_unverified_clock_is_remeasured_on_reveal_even_with_unchanged_bounds() {
    let mut runtime = runtime(Monitors::All);
    let mut hidden = bar(0, 2, false);
    hidden.visibility = TaskbarVisibility::Hidden;
    let observation = runtime.platform.observe(&hidden);
    runtime.platform.observations.insert(
        2,
        Observation {
            visible: false,
            ..observation
        },
    );
    discover(&mut runtime, 0, vec![hidden]);
    take(&mut runtime);
    runtime.platform.observations.remove(&2);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::TaskbarChanged)));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    discover(&mut runtime, 0, vec![bar(0, 2, true)]);
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
}

#[test]
fn late_secondary_taskbar_after_explorer_restart_is_discovered_without_an_event() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    discover(&mut runtime, 1, vec![bar(1, 1, true)]);
    take(&mut runtime);
    runtime.platform.inventory_changed = true;
    advance(&mut runtime, RECOVERY_POLL);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![1]);
    runtime.platform.inventory_changed = false;
    discover(&mut runtime, 1, vec![bar(1, 1, true), bar(1, 2, true)]);
    assert_eq!(runtime.platform.overlays.len(), 2);
    take(&mut runtime);
    for _ in 0..4 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    assert!(requests(&take(&mut runtime)).is_empty());
}

#[test]
fn visible_unverified_clock_after_explorer_startup_recovers_without_an_event() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    discover(&mut runtime, 1, vec![bar(1, 1, true), bar(1, 2, false)]);
    take(&mut runtime);
    advance(&mut runtime, RECOVERY_POLL);
    assert_eq!(requests(&take(&mut runtime)), vec![1]);
    discover(&mut runtime, 1, vec![bar(1, 1, true), bar(1, 2, true)]);
    assert_eq!(
        runtime.state().overlay_state(key(1, 2)),
        OverlayState::Visible
    );
    assert!(runtime.unverified_retry.due.is_none());
    take(&mut runtime);
    for _ in 0..5 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    assert!(requests(&take(&mut runtime)).is_empty());
}

#[test]
fn fatal_worker_shutdown_cancels_a_pending_unverified_retry() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, false)]);
    assert!(runtime.unverified_retry.due.is_some());
    runtime.topology_changed();
    advance(&mut runtime, TOPOLOGY_SETTLE);
    runtime.handle(Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(0),
        shell: runtime.shell,
        outcome: Err(DiscoveryFailure::TeardownFailed),
    }));
    assert!(runtime.unverified_retry.due.is_none());
    take(&mut runtime);
    advance(&mut runtime, RETRY_CAP);
    let calls = take(&mut runtime);
    assert!(requests(&calls).is_empty());
    assert!(calls.contains(&Call::Poll(RECOVERY_POLL)));
}

#[test]
fn unsupported_visible_clocks_have_only_four_settling_retries() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, false)]);
    take(&mut runtime);
    for attempt in 0..UNVERIFIED_RETRIES {
        let delay = RECOVERY_POLL * (1 << attempt);
        let due = runtime.unverified_retry.due.unwrap();
        assert_eq!(due - runtime.platform.mono, delay);
        advance(&mut runtime, delay);
        assert_eq!(requests(&take(&mut runtime)), vec![0]);
        discover(&mut runtime, 0, vec![bar(0, 1, false)]);
        assert!(runtime.platform.overlays.is_empty());
        take(&mut runtime);
    }
    assert!(runtime.unverified_retry.due.is_none());
    for _ in 0..5 {
        advance(&mut runtime, RETRY_CAP);
    }
    assert!(requests(&take(&mut runtime)).is_empty());
}

#[test]
fn parked_or_unselected_unverified_clocks_do_not_start_settling_retries() {
    let mut runtime = runtime(Monitors::Primary);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, false)]);
    assert!(runtime.unverified_retry.due.is_none());
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    let mut parked = bar(0, 1, false);
    parked.taskbar_rect = rect(0, 1078, 1920, 48);
    discover(&mut runtime, 0, vec![parked]);
    assert!(runtime.unverified_retry.due.is_none());
}

#[test]
fn inventory_mismatch_after_failed_discovery_respects_backoff() {
    let mut runtime = runtime(Monitors::All);
    runtime.platform.inventory_changed = true;
    for attempt in 0..3 {
        runtime.handle(Input::Discovery(DiscoveryReport {
            generation: ShellGeneration(0),
            shell: runtime.shell,
            outcome: Err(DiscoveryFailure::Deadline),
        }));
        take(&mut runtime);
        let due = runtime.retry.due.unwrap();
        advance(&mut runtime, TOPOLOGY_SETTLE);
        assert!(requests(&take(&mut runtime)).is_empty());
        assert_eq!(runtime.retry.due, Some(due));
        runtime.platform.mono = due;
        runtime.handle(Input::Poll);
        assert_eq!(requests(&take(&mut runtime)), vec![0], "attempt {attempt}");
        assert!(runtime.settle_due.is_none());
    }
}

#[test]
fn dpi_change_hides_old_cover_before_discovery_finishes() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    let old = runtime.platform.observe(&bar(0, 2, true));
    runtime
        .platform
        .observations
        .insert(2, Observation { dpi: 96, ..old });
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    assert!(take(&mut runtime).contains(&Call::Destroy(key(0, 2))));
    assert_eq!(
        runtime.state().overlay_state(key(0, 1)),
        OverlayState::Visible
    );
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    let mut scaled = bar(0, 2, true);
    scaled.dpi = 96;
    discover(&mut runtime, 0, vec![bar(0, 1, true), scaled]);
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
}

#[test]
fn a_moved_or_resized_taskbar_triggers_one_rediscovery_not_a_loop() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    let moved = Observation {
        rect: Some(rect(0, 1000, 1920, 80)),
        ..runtime.platform.observe(&bar(0, 1, true))
    };
    runtime.platform.observations.insert(1, moved);
    advance(&mut runtime, RECOVERY_POLL);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    // Discovery returns the same snapshot (the shell still reports the odd rect):
    // the same rectangle must not cause another request.
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    for _ in 0..5 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    assert!(requests(&take(&mut runtime)).is_empty());
    // A *different* odd rectangle is a new fact.
    let again = Observation {
        rect: Some(rect(0, 990, 1920, 90)),
        ..moved
    };
    runtime.platform.observations.insert(1, again);
    advance(&mut runtime, RECOVERY_POLL);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
}

#[test]
fn a_tray_relayout_rediscovers_once_and_keeps_the_overlay_meanwhile() {
    // 26300.9711: enabling do-not-disturb adds a bell button right of the clock
    // and moves the clock left inside an unmoved taskbar (#46).
    let mut runtime = runtime(Monitors::All);
    let base = runtime.platform.observe(&bar(0, 1, true));
    let tray = |left| Observation {
        tray: Some(rect(left, 0, 1920 - left, 80)),
        ..base
    };
    runtime.platform.observations.insert(1, tray(1255));
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);

    runtime.platform.observations.insert(1, tray(1220));
    advance(&mut runtime, RECOVERY_POLL);
    let calls = take(&mut runtime);
    assert!(
        !calls
            .iter()
            .any(|c| matches!(c, Call::Hide(_) | Call::Destroy(_)))
    );
    assert_eq!(
        runtime.state().overlay_state(key(0, 1)),
        OverlayState::Visible
    );
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);

    // The request recorded the new layout: no loop while it stays that way.
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    for _ in 0..5 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    assert!(requests(&take(&mut runtime)).is_empty());

    // Turning do-not-disturb off again is another move.
    runtime.platform.observations.insert(1, tray(1255));
    advance(&mut runtime, RECOVERY_POLL);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
}

#[test]
fn a_tray_change_during_discovery_asks_again_afterwards() {
    let mut runtime = runtime(Monitors::All);
    let base = runtime.platform.observe(&bar(0, 1, true));
    let tray = |left| Observation {
        tray: Some(rect(left, 0, 1920 - left, 80)),
        ..base
    };
    runtime.platform.observations.insert(1, tray(1255));
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    // The layout changes while the helper runs; its answer may predate that.
    runtime.platform.observations.insert(1, tray(1220));
    advance(&mut runtime, RECOVERY_POLL);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
}

#[test]
fn a_dpi_change_or_unplugged_monitor_rediscovers() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    let unplugged = Observation::GONE;
    runtime.platform.observations.insert(2, unplugged);
    advance(&mut runtime, RECOVERY_POLL);
    assert!(take(&mut runtime).contains(&Call::Destroy(key(0, 2))));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    // The new batch drops the removed monitor; its overlay was already released.
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    assert!(!runtime.platform.overlays.contains(&key(0, 2)));
    // Primary swap: the other taskbar becomes primary under "primary only".
    runtime.handle(Input::Preferences(prefs(Monitors::Primary)));
    let mut swapped_secondary = bar(0, 2, true);
    swapped_secondary.primary = true;
    let mut former_primary = bar(0, 1, true);
    former_primary.primary = false;
    runtime.platform.observations.remove(&2); // monitor is attached again
    runtime.handle(Input::Signal(Signal::TopologyChanged));
    advance(&mut runtime, TOPOLOGY_SETTLE);
    take(&mut runtime);
    discover(&mut runtime, 0, vec![former_primary, swapped_secondary]);
    assert_eq!(
        runtime.state().overlay_state(key(0, 2)),
        OverlayState::Visible
    );
    assert_eq!(
        runtime.state().overlay_state(key(0, 1)),
        OverlayState::Hidden(clock_core::HiddenReason::Paused)
    );
}

#[test]
fn fullscreen_hides_only_the_covered_monitor_and_restores_it() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    let covered = Observation {
        fullscreen: true,
        ..runtime.platform.observe(&bar(0, 2, true))
    };
    runtime.platform.observations.insert(2, covered);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::ForegroundChanged)));
    assert_eq!(take(&mut runtime), vec![Call::Hide(key(0, 2))]);
    assert_eq!(
        runtime.state().overlay_state(key(0, 1)),
        OverlayState::Visible
    );
    runtime.platform.observations.remove(&2);
    runtime.handle(Input::Signal(Signal::Shell(ShellEvent::ForegroundChanged)));
    assert_eq!(paints(&take(&mut runtime)), 1);
}

#[test]
fn a_new_taskbar_that_starts_under_a_fullscreen_window_is_hidden_immediately() {
    let mut runtime = runtime(Monitors::All);
    runtime.platform.observations.insert(
        1,
        Observation {
            fullscreen: true,
            exists: true,
            visible: true,
            rect: Some(bar(0, 1, true).taskbar_rect),
            dpi: 96,
            monitor_rect: Some(bar(0, 1, true).monitor_rect),
            tray: None,
        },
    );
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    assert_eq!(
        runtime.state().overlay_state(key(0, 1)),
        OverlayState::Hidden(clock_core::HiddenReason::Fullscreen)
    );
}

#[test]
fn lock_hides_and_unlock_refreshes_time_and_discovery_before_showing() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::SessionLocked));
    assert_eq!(take(&mut runtime), vec![Call::Hide(key(0, 1))]);
    runtime.platform.now = wall("2026-10-14T18:30:00+00:00");
    runtime.handle(Input::Signal(Signal::SessionUnlocked));
    let calls = take(&mut runtime);
    assert_eq!(requests(&calls), vec![0]);
    assert_eq!(paints(&calls), 0, "no pre-lock geometry is shown");
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let calls = take(&mut runtime);
    assert!(
        calls
            .iter()
            .any(|c| matches!(c, Call::Paint(_, lines) if lines[0] == "18:30"))
    );
}

#[test]
fn appearance_changes_repaint_live_and_unrelated_setting_changes_do_not() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
    assert_eq!(paints(&take(&mut runtime)), 0, "same appearance");
    runtime.platform.appearance = SystemAppearance::Dark;
    runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
    assert_eq!(paints(&take(&mut runtime)), 1);
    for background in [
        crate::theme::Rgb::new(12, 34, 56),
        crate::theme::Rgb::new(78, 90, 12),
    ] {
        runtime.platform.appearance = SystemAppearance::Accent { background };
        runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
        assert_eq!(paints(&take(&mut runtime)), 1, "accent change");
        runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
        assert_eq!(paints(&take(&mut runtime)), 0, "unchanged accent");
    }
    runtime.platform.font_changed = true;
    runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
    assert_eq!(paints(&take(&mut runtime)), 1, "system font change");
}

#[test]
fn preference_changes_apply_to_the_running_clock() {
    let mut runtime = runtime(Monitors::Primary);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    let format = Preferences {
        format: "%Y".into(),
        monitors: Monitors::All,
        theme: Theme::Dark,
        ..Preferences::default()
    }
    .validate()
    .unwrap();
    runtime.handle(Input::Preferences(format));
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Create(key(0, 2))), "all monitors");
    assert!(
        calls
            .iter()
            .any(|c| matches!(c, Call::Paint(_, lines) if lines == &vec!["2026".to_string()]))
    );
    assert_eq!(runtime.worst_case.fingerprint(), {
        WorstCase::for_format(runtime.state().preferences().format()).fingerprint()
    });
}

#[test]
fn overflow_and_render_failure_hide_only_the_affected_overlay_with_a_reason() {
    let mut runtime = runtime(Monitors::All);
    let overflow = Overflow {
        required: PixelSize {
            width: 140,
            height: 40,
        },
        available: PixelSize {
            width: 100,
            height: 48,
        },
    };
    runtime
        .platform
        .paint_result
        .insert(2, PaintResult::Overflow(overflow));
    runtime
        .platform
        .paint_result
        .insert(3, PaintResult::Failed("device lost".into()));
    let mut third = bar(0, 1, true);
    third.key.id = TaskbarId(3);
    third.primary = false;
    discover(
        &mut runtime,
        0,
        vec![bar(0, 1, true), bar(0, 2, true), third],
    );
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Hide(key(0, 2))));
    assert!(calls.contains(&Call::Hide(key(0, 3))));
    assert!(!calls.contains(&Call::Hide(key(0, 1))));
    let diagnostics = runtime.diagnostics();
    assert_eq!(diagnostics.taskbars[0].reason, None);
    assert_eq!(
        diagnostics.taskbars[1].reason,
        Some(Reason::Overflow(overflow))
    );
    assert_eq!(
        diagnostics.taskbars[2].reason,
        Some(Reason::RenderFailed("device lost".into()))
    );
    // Fixing the cause (shorter text) clears the reason on the next paint.
    runtime.platform.paint_result.clear();
    runtime.handle(Input::Preferences(
        Preferences {
            format: "%H".into(),
            monitors: Monitors::All,
            ..Preferences::default()
        }
        .validate()
        .unwrap(),
    ));
    assert!(
        runtime
            .diagnostics()
            .taskbars
            .iter()
            .all(|t| t.reason.is_none())
    );
}

#[test]
fn transient_overflow_recovers_without_a_text_change_or_another_apply() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    let overflow = Overflow {
        required: PixelSize {
            width: 70,
            height: 16,
        },
        available: PixelSize {
            width: 1,
            height: 48,
        },
    };
    runtime
        .platform
        .paint_result
        .insert(1, PaintResult::Overflow(overflow));
    runtime.handle(Input::Preferences(
        Preferences {
            format: "%g%V.%u".into(),
            monitors: Monitors::All,
            ..Preferences::default()
        }
        .validate()
        .unwrap(),
    ));
    assert_eq!(
        runtime.diagnostics().taskbars[0].reason,
        Some(Reason::Overflow(overflow))
    );
    take(&mut runtime);
    runtime.platform.paint_result.clear();
    // ISO week text stays identical across ticks and rediscovery. Neither
    // causes the reducer to repaint a frame it already considers painted.
    runtime.handle(Input::Timer);
    assert_eq!(paints(&take(&mut runtime)), 0);
    advance(&mut runtime, RECOVERY_POLL);
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls), 1, "retry only the affected taskbar");
    assert!(
        calls
            .iter()
            .any(|call| matches!(call, Call::Paint(k, _) if *k == key(0, 1)))
    );
    assert!(
        runtime
            .diagnostics()
            .taskbars
            .iter()
            .all(|bar| bar.reason.is_none())
    );
}

#[test]
fn persistent_overflow_retries_are_bounded_and_respect_visibility() {
    for hidden in [
        Event::SetPaused(true),
        Event::SessionLocked,
        Event::FullscreenChanged {
            key: key(0, 1),
            fullscreen: true,
        },
        Event::VisibilityChanged {
            key: key(0, 1),
            visibility: TaskbarVisibility::Hidden,
        },
        Event::VisibilityChanged {
            key: key(0, 1),
            visibility: TaskbarVisibility::Unknown,
        },
    ] {
        let mut runtime = runtime(Monitors::All);
        runtime.platform.paint_result.insert(
            1,
            PaintResult::Overflow(Overflow {
                required: PixelSize {
                    width: 140,
                    height: 16,
                },
                available: PixelSize {
                    width: 100,
                    height: 48,
                },
            }),
        );
        discover(&mut runtime, 0, vec![bar(0, 1, true)]);
        take(&mut runtime);
        advance(&mut runtime, RECOVERY_POLL);
        assert_eq!(paints(&take(&mut runtime)), 1);
        advance(&mut runtime, Duration::from_millis(100));
        assert_eq!(paints(&take(&mut runtime)), 0, "fast transition poll");
        advance(&mut runtime, RECOVERY_POLL);
        assert_eq!(
            paints(&take(&mut runtime)),
            1,
            "persistent overflow is retried"
        );
        assert!(matches!(
            runtime.diagnostics().taskbars[0].reason,
            Some(Reason::Overflow(_))
        ));
        runtime.dispatch(hidden);
        take(&mut runtime);
        // Exercise the retry independently of observation, which would replace
        // a directly scripted visibility event with the fake's live values.
        runtime.retry_overlays(runtime.platform.mono + RECOVERY_POLL);
        assert_eq!(
            paints(&take(&mut runtime)),
            0,
            "ineligible clock stays hidden"
        );
    }
}

#[test]
fn overlay_creation_failure_is_reported_and_hides_the_overlay() {
    let mut runtime = runtime(Monitors::All);
    runtime.platform.create_error = true;
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    assert_eq!(
        runtime.diagnostics().taskbars[0].reason,
        Some(Reason::PlacementFailed("CreateWindowExW failed".into()))
    );
}

#[test]
fn quit_destroys_overlays_stops_the_platform_once_and_ignores_late_input() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    runtime.handle(Input::Quit);
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Destroy(key(0, 1))));
    assert!(calls.contains(&Call::Destroy(key(0, 2))));
    assert_eq!(calls.iter().filter(|c| **c == Call::Stop).count(), 1);
    assert!(runtime.is_finished());
    // Late timers, polls, signals and results are inert: no resurrected overlay.
    runtime.handle(Input::Timer);
    runtime.handle(Input::Poll);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    runtime.handle(Input::Quit);
    assert!(take(&mut runtime).is_empty());
    assert!(runtime.platform.overlays.is_empty());
}

#[test]
fn end_session_quits_like_exit() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.handle(Input::Signal(Signal::EndSession));
    assert!(runtime.is_finished());
    assert!(runtime.platform.overlays.is_empty());
}

#[test]
fn repeated_explorer_failures_never_accumulate_overlays_or_requests() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    let mut live = HashSet::new();
    for round in 1..=50u64 {
        runtime.platform.shell = shell(100 + round as u32, round);
        runtime.handle(Input::Signal(Signal::TaskbarCreated));
        // The previous generation's helper reports late, then the new one answers.
        discover(&mut runtime, round - 1, vec![bar(round - 1, 1, true)]);
        discover(
            &mut runtime,
            round,
            vec![bar(round, 1, true), bar(round, 2, true)],
        );
        live = runtime.platform.overlays.clone();
        assert_eq!(live.len(), 2, "round {round}");
        assert!(live.iter().all(|k| k.generation.0 == round));
    }
    assert_eq!(live.len(), 2);
}

// ---- R09: clock actions, menu, pause and tray ------------------------------

fn shown(monitors: Monitors) -> Runtime<Fake> {
    let mut runtime = runtime(monitors);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    take(&mut runtime);
    runtime
}
fn click(runtime: &mut Runtime<Fake>, id: u64) {
    runtime.handle(Input::Action(UserAction::Activate(key(0, id))));
}
fn opens(calls: &[Call]) -> usize {
    calls
        .iter()
        .filter(|c| matches!(c, Call::OpenCalendar(_)))
        .count()
}

#[test]
fn one_click_sends_one_calendar_action_for_the_clicked_taskbar() {
    let mut runtime = shown(Monitors::All);
    click(&mut runtime, 2);
    assert_eq!(take(&mut runtime), vec![Call::OpenCalendar(key(0, 2))]);
}

#[test]
fn rapid_repeat_clicks_coalesce_but_a_later_deliberate_click_works() {
    let mut runtime = shown(Monitors::All);
    click(&mut runtime, 1);
    click(&mut runtime, 1);
    click(&mut runtime, 2);
    assert_eq!(opens(&take(&mut runtime)), 1, "double click is one action");
    runtime.platform.mono += ACTIVATION_COOLDOWN;
    click(&mut runtime, 1);
    assert_eq!(opens(&take(&mut runtime)), 1);
}

#[test]
fn a_click_that_is_stale_hidden_or_unknown_does_nothing() {
    let mut runtime = shown(Monitors::Primary);
    // Not shown by design (other monitor), and a key from another generation.
    click(&mut runtime, 2);
    runtime.handle(Input::Action(UserAction::Activate(key(7, 1))));
    assert_eq!(opens(&take(&mut runtime)), 0);

    // Fullscreen hides the overlay; a click queued before that is ignored.
    let mut runtime = shown(Monitors::All);
    runtime.platform.observations.insert(
        1,
        Observation {
            fullscreen: true,
            ..Observation {
                exists: true,
                visible: true,
                rect: Some(bar(0, 1, true).taskbar_rect),
                dpi: 96,
                monitor_rect: Some(bar(0, 1, true).monitor_rect),
                fullscreen: false,
                tray: None,
            }
        },
    );
    runtime.handle(Input::Poll);
    take(&mut runtime);
    click(&mut runtime, 1);
    assert_eq!(opens(&take(&mut runtime)), 0);
}

#[test]
fn held_modifiers_send_nothing_and_do_not_consume_the_cooldown_or_remove_overlays() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.calendar = CalendarOutcome::ModifiersHeld;
    click(&mut runtime, 1);
    // The user lets go: the very next click works immediately.
    runtime.platform.calendar = CalendarOutcome::Sent;
    click(&mut runtime, 1);
    let calls = take(&mut runtime);
    assert_eq!(opens(&calls), 2);
    assert!(!calls.iter().any(|c| matches!(c, Call::Hide(_))));
    assert!(
        runtime
            .diagnostics()
            .taskbars
            .iter()
            .all(|t| t.reason.is_none())
    );
}

#[test]
fn an_unknown_calendar_failure_removes_every_overlay_records_why_and_never_retries() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.calendar = CalendarOutcome::Failed("input blocked".into());
    click(&mut runtime, 1);
    let calls = take(&mut runtime);
    assert_eq!(opens(&calls), 1, "no retry of the failed action");
    assert!(calls.contains(&Call::Hide(key(0, 1))));
    assert!(calls.contains(&Call::Hide(key(0, 2))));
    for taskbar in runtime.diagnostics().taskbars {
        assert_eq!(
            taskbar.reason,
            Some(Reason::ActionFailed("input blocked".into()))
        );
    }
    // Further clicks on a removed overlay do nothing, and ticks do not bring it back.
    click(&mut runtime, 1);
    runtime.platform.now = wall("2026-10-14T12:05:00+00:00");
    runtime.handle(Input::Timer);
    let calls = take(&mut runtime);
    assert_eq!(opens(&calls), 0);
    assert_eq!(paints(&calls), 0);
}

#[test]
fn pause_then_resume_gives_a_failed_overlay_another_chance() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.calendar = CalendarOutcome::Failed("x".into());
    click(&mut runtime, 1);
    take(&mut runtime);
    runtime.platform.menu_choice = Some(MenuCommand::TogglePause);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    assert!(runtime.diagnostics().paused);
    runtime.platform.mono += ACTIVATION_COOLDOWN;
    runtime.platform.menu_choice = Some(MenuCommand::TogglePause);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls), 2, "resume repaints both clocks");
    assert!(
        runtime
            .diagnostics()
            .taskbars
            .iter()
            .all(|t| t.reason.is_none())
    );
}

#[test]
fn the_menu_reports_the_current_mode_and_dispatches_exactly_the_chosen_command() {
    let mut runtime = shown(Monitors::All);
    let at = Some(ScreenPoint { x: -100, y: 1040 });
    runtime.handle(Input::Action(UserAction::ShowMenu(at)));
    assert_eq!(take(&mut runtime), vec![Call::ShowMenu(at, false)]);

    runtime.platform.mono += ACTIVATION_COOLDOWN;
    runtime.platform.menu_choice = Some(MenuCommand::Settings);
    runtime.handle(Input::Action(UserAction::ShowMenu(at)));
    assert_eq!(
        take(&mut runtime),
        vec![Call::ShowMenu(at, false), Call::OpenSettings]
    );
    assert!(!runtime.is_finished());
}

#[test]
fn pause_hides_every_overlay_so_the_native_clock_shows_and_resume_restores_them() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.menu_choice = Some(MenuCommand::TogglePause);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Hide(key(0, 1))));
    assert!(calls.contains(&Call::Hide(key(0, 2))));
    assert!(calls.contains(&Call::PausedChanged(true)));
    let diagnostics = runtime.diagnostics();
    assert!(diagnostics.paused);
    assert!(diagnostics.taskbars.iter().all(|t| matches!(
        t.state,
        OverlayState::Hidden(clock_core::HiddenReason::Paused)
    )));

    // Ticks, rediscovery and a click while paused never show or act.
    runtime.platform.now = wall("2026-10-14T12:01:00+00:00");
    runtime.handle(Input::Timer);
    click(&mut runtime, 1);
    let calls = take(&mut runtime);
    assert_eq!(paints(&calls) + opens(&calls), 0);

    // The menu now offers Resume.
    runtime.platform.mono += ACTIVATION_COOLDOWN;
    runtime.platform.menu_choice = Some(MenuCommand::TogglePause);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    let calls = take(&mut runtime);
    assert_eq!(calls[0], Call::ShowMenu(None, true));
    assert_eq!(paints(&calls), 2);
    assert!(calls.contains(&Call::PausedChanged(false)));
    assert!(!runtime.diagnostics().paused);
}

#[test]
fn exit_from_the_menu_quits_like_any_other_exit() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.menu_choice = Some(MenuCommand::Exit);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Destroy(key(0, 1))));
    assert_eq!(calls.iter().filter(|c| **c == Call::Stop).count(), 1);
    assert!(runtime.is_finished());
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    assert!(take(&mut runtime).is_empty());
}

#[test]
fn mouse_and_keyboard_tray_events_for_one_gesture_open_one_menu() {
    let mut runtime = shown(Monitors::All);
    // NIN_SELECT and NIN_KEYSELECT can both arrive for one activation.
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    let menus = take(&mut runtime)
        .iter()
        .filter(|c| matches!(c, Call::ShowMenu(..)))
        .count();
    assert_eq!(menus, 1);
}

#[test]
fn explorer_restart_re_registers_the_tray_from_the_existing_recovery_path_only() {
    let mut runtime = shown(Monitors::All);
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    let calls = take(&mut runtime);
    assert_eq!(calls.iter().filter(|c| **c == Call::RestoreTray).count(), 1);

    // A new Explorer process noticed by the poll is the same single path.
    runtime.platform.shell = shell(11, 2);
    advance(&mut runtime, RECOVERY_POLL);
    let calls = take(&mut runtime);
    assert_eq!(calls.iter().filter(|c| **c == Call::RestoreTray).count(), 1);

    // Steady state never touches the tray: no second recovery loop.
    for _ in 0..5 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    assert!(!take(&mut runtime).contains(&Call::RestoreTray));
}

#[test]
fn pause_survives_explorer_restart() {
    let mut runtime = shown(Monitors::All);
    runtime.platform.menu_choice = Some(MenuCommand::TogglePause);
    runtime.handle(Input::Action(UserAction::ShowMenu(None)));
    runtime.handle(Input::Signal(Signal::TaskbarCreated));
    discover(&mut runtime, 1, vec![bar(1, 1, true), bar(1, 2, true)]);
    assert!(runtime.diagnostics().paused);
    assert!(
        runtime
            .platform
            .overlays
            .iter()
            .all(|k| k.generation.0 == 0)
    );
}

// ---- Settings fit checks (R10) -------------------------------------------

fn fit_format() -> CompiledFormat {
    CompiledFormat::parse("%H:%M\n%g%V.%u").unwrap()
}

#[test]
fn settings_fit_check_measures_only_selected_taskbars_with_a_known_clock_slot() {
    let mut runtime = runtime(Monitors::All);
    // id 3 has no proven clock rectangle, so there is nothing to measure there.
    let mut unknown = bar(0, 2, false);
    unknown.key = key(0, 3);
    discover(
        &mut runtime,
        0,
        vec![bar(0, 1, true), bar(0, 2, true), unknown],
    );
    runtime.platform.fit_needs_width = Some(150);
    let now = wall("2026-10-13T09:15:00+02:00");

    let primary = runtime.check(&fit_format(), 9, Monitors::Primary, now);
    assert_eq!(primary.len(), 1);
    assert!(primary[0].primary);
    assert_eq!(primary[0].overflow.required.width, 150);
    assert_eq!(primary[0].overflow.available.width, 100);

    let all = runtime.check(&fit_format(), 14, Monitors::All, now);
    assert_eq!(
        all.iter().map(|p| p.primary).collect::<Vec<_>>(),
        [true, false]
    );
    // Each taskbar is measured at its own DPI and with the draft's font size.
    let calls = &runtime.platform.fit_calls;
    assert_eq!(
        calls[1..],
        [
            (
                PixelSize {
                    width: 100,
                    height: 48
                },
                96,
                14
            ),
            (
                PixelSize {
                    width: 100,
                    height: 48
                },
                144,
                14
            )
        ]
    );
}

#[test]
fn settings_fit_check_reports_nothing_when_it_fits_or_cannot_measure() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let now = wall("2026-10-13T09:15:00+02:00");
    // A platform that cannot measure never blocks Apply.
    assert!(
        runtime
            .check(&fit_format(), 9, Monitors::All, now)
            .is_empty()
    );
    runtime.platform.fit_needs_width = Some(100);
    assert!(
        runtime
            .check(&fit_format(), 9, Monitors::All, now)
            .is_empty()
    );
    // Before discovery there are no slots at all.
    let mut fresh = Runtime::new(Fake::new(), prefs(Monitors::All));
    fresh.platform.fit_needs_width = Some(1_000);
    assert!(fresh.check(&fit_format(), 9, Monitors::All, now).is_empty());
    assert!(fresh.platform.fit_calls.is_empty());
}

// ---- Performance guards: repeated work ------------------------------------
//
// These count operations, not time: measurements, sampling sweeps, paints and
// discovery requests. Each pairs an unchanged input that must do no new work with
// a genuinely changed one that must, so a guard cannot pass by never recomputing.
// Measured CPU and memory on a real desktop are a separate procedure
// (docs/contributing.md, "Performance and resource guards").

fn measured(runtime: &Runtime<Fake>) -> usize {
    runtime.platform.fit_calls.len()
}

#[test]
fn settings_fit_checks_on_several_displays_sample_and_measure_only_what_changed() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    runtime.platform.fit_needs_width = Some(150);
    let now = wall("2026-10-13T09:15:00+02:00");
    let format = fit_format();
    let builds = WorstCase::builds();

    let first = runtime.check(&format, 9, Monitors::All, now);
    assert_eq!(first.len(), 2);
    assert_eq!(
        measured(&runtime),
        2,
        "one measurement per selected display"
    );
    assert_eq!(WorstCase::builds(), builds + 1, "one sampling sweep");
    // Every Settings refresh and the Apply after it repeat the same question.
    for seconds in [0, 1, 30, 45] {
        let later = now + chrono::TimeDelta::seconds(seconds);
        assert_eq!(runtime.check(&format, 9, Monitors::All, later), first);
    }
    assert_eq!(measured(&runtime), 2);
    assert_eq!(WorstCase::builds(), builds + 1);

    // Changing which displays are selected measures that set, not the old one.
    runtime.check(&format, 9, Monitors::Primary, now);
    assert_eq!(measured(&runtime), 3);
    // Only the second display's slot changes: both are measured again.
    runtime.snapshots.get_mut(&2).unwrap().clock_rect = Some(rect(-120, 1032, 120, 48));
    runtime.check(&format, 9, Monitors::All, now);
    assert_eq!(measured(&runtime), 5);
    assert_eq!(
        WorstCase::builds(),
        builds + 1,
        "the sweep outlives all of it"
    );

    // A different format replaces the sweep and the verdict.
    let other = CompiledFormat::parse("%H:%M").unwrap();
    runtime.check(&other, 9, Monitors::All, now);
    assert_eq!(measured(&runtime), 7);
    assert_eq!(WorstCase::builds(), builds + 2);
}

#[test]
fn a_transient_measurement_failure_is_retried_not_held_as_a_fit() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    runtime.platform.fit_needs_width = Some(150);
    runtime.platform.fit_error = true;
    let now = wall("2026-10-13T09:15:00+02:00");
    let format = fit_format();
    for round in 1..=3 {
        assert!(runtime.check(&format, 9, Monitors::All, now).is_empty());
        assert_eq!(measured(&runtime), 2 * round, "measured again each time");
    }
    runtime.platform.fit_error = false;
    assert_eq!(runtime.check(&format, 9, Monitors::All, now).len(), 2);
    assert_eq!(measured(&runtime), 8);
    runtime.check(&format, 9, Monitors::All, now);
    assert_eq!(measured(&runtime), 8, "the success is held");
}

#[test]
fn only_a_changed_format_source_resamples_the_worst_case() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    let apply = |runtime: &mut Runtime<Fake>, format: &str, font_size_pt: u8, theme: Theme| {
        let preferences = Preferences {
            update_check_enabled: false,
            format: format.into(),
            font_size_pt,
            theme,
            monitors: Monitors::All,
        }
        .validate()
        .unwrap();
        runtime.handle(Input::Preferences(preferences));
    };
    apply(&mut runtime, "%H:%M", 9, Theme::System);
    take(&mut runtime);
    let builds = WorstCase::builds();

    apply(&mut runtime, "%H:%M", 9, Theme::System);
    assert_eq!(paints(&take(&mut runtime)), 0, "identical preferences");
    apply(&mut runtime, "%H:%M", 12, Theme::Dark);
    assert_eq!(paints(&take(&mut runtime)), 2, "one repaint per display");
    assert_eq!(WorstCase::builds(), builds, "font and theme keep the sweep");
    apply(&mut runtime, "%H:%M:%S", 12, Theme::Dark);
    assert_eq!(paints(&take(&mut runtime)), 2);
    assert_eq!(
        WorstCase::builds(),
        builds + 1,
        "a new format resamples once"
    );
    apply(&mut runtime, "%H:%M:%S", 12, Theme::Dark);
    assert_eq!(paints(&take(&mut runtime)), 0);
    assert_eq!(WorstCase::builds(), builds + 1);
}

#[test]
fn an_idle_clock_with_unchanged_observations_does_no_native_work() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    // Let the transition and settling polls finish before measuring idleness.
    for _ in 0..20 {
        advance(&mut runtime, RECOVERY_POLL);
    }
    take(&mut runtime);
    for _ in 0..30 {
        advance(&mut runtime, RECOVERY_POLL);
        runtime.handle(Input::Signal(Signal::Shell(ShellEvent::ForegroundChanged)));
        runtime.handle(Input::Signal(Signal::AppearanceMaybeChanged));
    }
    let calls = take(&mut runtime);
    let work: Vec<_> = calls
        .iter()
        .filter(|call| !matches!(call, Call::Poll(_)))
        .collect();
    assert!(work.is_empty(), "idle polls only re-arm the poll: {work:?}");

    // A redelivered, identical discovery answer creates, moves and paints nothing.
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, true)]);
    let calls = take(&mut runtime);
    assert!(
        !calls.iter().any(|call| matches!(
            call,
            Call::Create(_) | Call::Destroy(_) | Call::SetBounds(_) | Call::Paint(..)
        )),
        "{calls:?}"
    );
    assert!(requests(&calls).is_empty());
}

// ---- Support: Refresh status, report data and the transition log (R11) ----------

use crate::support::{Progress, REFRESH_LIMIT, RefreshWatch, Replacement};

fn fail_discovery(runtime: &mut Runtime<Fake>, failure: DiscoveryFailure) {
    let generation = runtime.state().generation().0;
    runtime.handle(Input::Discovery(DiscoveryReport {
        generation: ShellGeneration(generation),
        shell: runtime.shell,
        outcome: Err(failure),
    }));
}

#[test]
fn diagnostics_carry_monitor_bounds_dpi_and_strategy_for_the_report() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, false)]);
    let d = runtime.diagnostics();
    assert!(d.shell_running && !d.discovery_pending);
    assert_eq!(d.discovery_completed, 1);
    let [primary, secondary] = &d.taskbars[..] else {
        panic!("two taskbars")
    };
    assert_eq!(primary.monitor_rect, rect(0, 0, 1920, 1080));
    assert_eq!((primary.dpi, secondary.dpi), (96, 144));
    assert_eq!(primary.strategy, DiscoveryStrategy::NativeChildren);
    assert!(primary.clock_located && !secondary.clock_located);
    assert_eq!(secondary.monitor_rect, rect(-1920, 0, 1920, 1080));
}

#[test]
fn refresh_status_runs_one_bounded_discovery_and_reports_the_answer() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    let watch = RefreshWatch::start(&runtime.diagnostics());
    runtime.refresh_status();
    runtime.refresh_status(); // a second click while one is queued adds nothing
    let d = runtime.diagnostics();
    assert!(d.discovery_pending);
    assert_eq!(watch.progress(&d, Duration::ZERO), Progress::Waiting);
    assert!(requests(&take(&mut runtime)).is_empty(), "settles first");
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert_eq!(requests(&take(&mut runtime)), vec![0]);
    // Nothing has answered yet: still waiting, and the UI thread is not blocked.
    let d = runtime.diagnostics();
    assert_eq!(
        watch.progress(&d, Duration::from_secs(1)),
        Progress::Waiting
    );
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let d = runtime.diagnostics();
    assert!(!d.discovery_pending);
    assert_eq!(watch.progress(&d, Duration::from_secs(2)), Progress::Done);
    assert_eq!(
        crate::support::status(&d).replacement,
        Replacement::Active,
        "a successful refresh shows the clock as active"
    );
}

#[test]
fn a_refresh_that_times_out_in_the_helper_still_completes_with_the_reason() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let watch = RefreshWatch::start(&runtime.diagnostics());
    runtime.refresh_status();
    advance(&mut runtime, TOPOLOGY_SETTLE);
    take(&mut runtime);
    // The UI Automation helper hit its deadline: a failure report, not a hang.
    fail_discovery(&mut runtime, DiscoveryFailure::Deadline);
    let d = runtime.diagnostics();
    assert_eq!(watch.progress(&d, Duration::from_secs(5)), Progress::Done);
    assert_eq!(d.last_discovery_failure, Some(DiscoveryFailure::Deadline));
    let status = crate::support::status(&d);
    assert_eq!(status.replacement, Replacement::NativeClock);
    assert!(status.explanation.contains("did not finish in time"));
}

#[test]
fn a_helper_that_never_answers_cannot_keep_refresh_waiting_forever() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let watch = RefreshWatch::start(&runtime.diagnostics());
    runtime.refresh_status();
    advance(&mut runtime, TOPOLOGY_SETTLE);
    let d = runtime.diagnostics();
    assert!(d.discovery_pending);
    assert_eq!(watch.progress(&d, REFRESH_LIMIT), Progress::TimedOut);
}

#[test]
fn refresh_is_a_no_op_when_discovery_is_off_or_the_app_is_finished() {
    let mut runtime = runtime(Monitors::All);
    runtime.discovery_disabled = true;
    take(&mut runtime);
    runtime.refresh_status();
    advance(&mut runtime, TOPOLOGY_SETTLE);
    assert!(requests(&take(&mut runtime)).is_empty());
    let watch = RefreshWatch::start(&runtime.diagnostics());
    assert_eq!(
        watch.progress(&runtime.diagnostics(), Duration::ZERO),
        Progress::Done
    );
    runtime.handle(Input::Quit);
    take(&mut runtime);
    runtime.refresh_status();
    assert!(take(&mut runtime).is_empty());
}

#[test]
fn overflow_transition_logs_dimensions_once_without_clock_text() {
    let mut runtime = runtime(Monitors::Primary);
    runtime.platform.paint_result.insert(
        1,
        PaintResult::Overflow(Overflow {
            required: PixelSize {
                width: 70,
                height: 16,
            },
            available: PixelSize {
                width: 1,
                height: 48,
            },
        }),
    );
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    let lines = runtime.transition_lines();
    assert_eq!(lines.len(), 2);
    assert_eq!(
        lines[1],
        "taskbar 1 overflow: required=70x16px, available=1x48px, discovered slot=100x48px, DPI=96, font=9pt"
    );
    assert!(
        !lines
            .iter()
            .any(|line| line.contains("12:00") || line.contains("%g"))
    );
    advance(&mut runtime, RECOVERY_POLL);
    assert!(
        runtime.transition_lines().is_empty(),
        "persistent retries stay quiet"
    );
    runtime.platform.paint_result.clear();
    advance(&mut runtime, RECOVERY_POLL);
    assert_eq!(
        runtime.transition_lines(),
        vec!["primary taskbar 1: clock replaced"]
    );
}

#[test]
fn only_state_transitions_are_logged_never_repaints_or_ticks() {
    let mut runtime = runtime(Monitors::All);
    assert!(runtime.transition_lines().is_empty());
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, false)]);
    let lines = runtime.transition_lines();
    assert_eq!(
        lines,
        vec![
            "primary taskbar 1: clock replaced".to_owned(),
            "secondary taskbar 2: Windows clock in use: ElevenClock-rs could not verify where \
             the Windows clock is on this taskbar, so it leaves the Windows clock alone. This \
             Windows version may not be supported yet"
                .to_owned(),
        ]
    );
    // A minute of ticks, repaints and a repeat of the same state: silence.
    for _ in 0..5 {
        runtime.platform.now += chrono::Duration::seconds(1);
        runtime.handle(Input::Timer);
        assert!(runtime.transition_lines().is_empty());
    }
    discover(&mut runtime, 0, vec![bar(0, 1, true), bar(0, 2, false)]);
    assert!(runtime.transition_lines().is_empty());
    // Failure, recovery, pause and removal each log once.
    runtime.refresh_status();
    advance(&mut runtime, TOPOLOGY_SETTLE);
    fail_discovery(&mut runtime, DiscoveryFailure::NoTaskbar);
    let lines = runtime.transition_lines();
    assert!(
        lines
            .iter()
            .any(|l| l.starts_with("taskbar discovery failed (1 in a row)")),
        "{lines:?}"
    );
    assert!(lines.contains(&"taskbar 1: gone".to_owned()), "{lines:?}");
    assert!(runtime.transition_lines().is_empty());
}

#[test]
fn failed_move_hides_immediately_and_recreates_at_latest_bounds() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.platform.bounds_error = true;
    let bounds = rect(1790, 1032, 100, 48);
    runtime.snapshots.get_mut(&1).unwrap().clock_rect = Some(bounds);
    let text = runtime
        .state()
        .preferences()
        .format()
        .render(runtime.platform.now)
        .unwrap();
    runtime.apply(vec![
        Effect::SetBounds {
            key: key(0, 1),
            bounds,
        },
        Effect::Paint {
            key: key(0, 1),
            text,
        },
    ]);
    let calls = take(&mut runtime);
    assert!(calls.contains(&Call::Hide(key(0, 1))));
    assert_eq!(
        paints(&calls),
        0,
        "failed move must not expose stale bounds"
    );
    assert!(matches!(
        runtime.diagnostics().taskbars[0].reason,
        Some(Reason::PlacementFailed(_))
    ));
    runtime.platform.bounds_error = false;
    runtime.retry_overlays(runtime.platform.mono);
    let calls = take(&mut runtime);
    assert_eq!(
        &calls[..2],
        &[Call::Destroy(key(0, 1)), Call::Create(key(0, 1))]
    );
    assert_eq!(paints(&calls), 1);
    assert_eq!(runtime.platform.created_bounds.last(), Some(&bounds));
    assert_eq!(runtime.diagnostics().taskbars[0].reason, None);
}

#[test]
fn creation_and_render_failures_recover_without_text_changes() {
    for creation in [false, true] {
        let mut runtime = runtime(Monitors::All);
        runtime.platform.create_error = creation;
        if !creation {
            runtime
                .platform
                .paint_result
                .insert(1, PaintResult::Failed("device lost".into()));
        }
        discover(&mut runtime, 0, vec![bar(0, 1, true)]);
        take(&mut runtime);
        runtime.platform.create_error = false;
        runtime.platform.paint_result.clear();
        advance(&mut runtime, RECOVERY_POLL);
        assert_eq!(paints(&take(&mut runtime)), 1);
        assert_eq!(runtime.diagnostics().taskbars[0].reason, None);
    }
}

#[test]
fn native_upkeep_failures_are_reported_and_retried_but_stale_reports_are_ignored() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    take(&mut runtime);
    runtime.platform.overlay_failures = vec![
        (key(0, 1), Reason::RenderFailed("upkeep failed".into())),
        (key(9, 1), Reason::RenderFailed("stale".into())),
    ];
    runtime.sync_overlay_failures();
    assert_eq!(
        runtime.diagnostics().taskbars[0].reason,
        Some(Reason::RenderFailed("upkeep failed".into()))
    );
    assert_eq!(take(&mut runtime), vec![Call::Hide(key(0, 1))]);
    advance(&mut runtime, RECOVERY_POLL);
    assert_eq!(paints(&take(&mut runtime)), 1);
    assert_eq!(runtime.diagnostics().taskbars[0].reason, None);
}

#[test]
fn fault_retries_are_throttled_and_do_not_revive_hidden_or_action_failed_clocks() {
    for reason in [
        Reason::RenderFailed("device lost".into()),
        Reason::PlacementFailed("move failed".into()),
        Reason::ActionFailed("input blocked".into()),
    ] {
        let mut runtime = runtime(Monitors::All);
        discover(&mut runtime, 0, vec![bar(0, 1, true)]);
        runtime.faults.insert(key(0, 1), reason.clone());
        runtime.platform.create_error = true;
        runtime
            .platform
            .paint_result
            .insert(1, PaintResult::Failed("still failing".into()));
        take(&mut runtime);
        let now = runtime.platform.mono;
        runtime.retry_overlays(now);
        let first = take(&mut runtime);
        if matches!(reason, Reason::ActionFailed(_)) {
            assert!(first.is_empty());
        } else {
            assert!(!first.is_empty());
        }
        runtime.retry_overlays(now + Duration::from_millis(100));
        assert!(take(&mut runtime).is_empty());
        runtime.dispatch(Event::SessionLocked);
        take(&mut runtime);
        runtime.retry_overlays(now + RECOVERY_POLL);
        assert!(take(&mut runtime).is_empty());
    }
}

#[test]
fn settings_fit_cache_reuses_measurements_and_invalidates_changed_inputs() {
    let mut runtime = runtime(Monitors::All);
    discover(&mut runtime, 0, vec![bar(0, 1, true)]);
    runtime.platform.fit_needs_width = Some(110);
    let format = CompiledFormat::parse("%H:%M").unwrap();
    let now = runtime.platform.now;
    let check =
        |runtime: &mut Runtime<Fake>, font, time| runtime.check(&format, font, Monitors::All, time);
    assert_eq!(check(&mut runtime, 9, now).len(), 1);
    assert_eq!(check(&mut runtime, 9, now).len(), 1);
    assert_eq!(runtime.platform.fit_calls.len(), 1);
    // Seconds do not change this format's rendered text.
    check(&mut runtime, 9, now + chrono::TimeDelta::seconds(1));
    assert_eq!(runtime.platform.fit_calls.len(), 1);
    check(&mut runtime, 10, now);
    assert_eq!(runtime.platform.fit_calls.len(), 2);
    runtime.platform.font_changed = true;
    runtime.appearance_changed();
    check(&mut runtime, 10, now);
    assert_eq!(runtime.platform.fit_calls.len(), 3);
    runtime.snapshots.get_mut(&1).unwrap().dpi = 120;
    check(&mut runtime, 10, now);
    assert_eq!(runtime.platform.fit_calls.len(), 4);
    runtime.snapshots.get_mut(&1).unwrap().clock_rect = Some(rect(1800, 1032, 120, 48));
    assert!(check(&mut runtime, 10, now).is_empty());
    assert_eq!(runtime.platform.fit_calls.len(), 5);
    check(&mut runtime, 10, now + chrono::TimeDelta::minutes(1));
    assert_eq!(runtime.platform.fit_calls.len(), 6);
    runtime.check(
        &CompiledFormat::parse("%S").unwrap(),
        10,
        Monitors::All,
        now,
    );
    assert_eq!(runtime.platform.fit_calls.len(), 7);
    // Unavailable measurements must not suppress a later successful check.
    runtime.platform.fit_needs_width = None;
    check(&mut runtime, 9, now);
    runtime.platform.fit_needs_width = Some(140);
    assert_eq!(check(&mut runtime, 9, now).len(), 1);
}

#[test]
fn update_policy_uses_scripted_lifecycle_and_independent_managed_input() {
    use crate::{
        preferences_store::{MemoryPreferencesStore, PreferenceSession},
        update_check::UpdateRuntime,
    };
    let mut runtime = runtime(Monitors::Primary);
    let mut session = PreferenceSession::open(MemoryPreferencesStore::default());
    let mut draft = session.active().draft();
    draft.update_check_enabled = true;
    session.apply(draft).unwrap();
    let mut updates = UpdateRuntime::default();
    let initial = runtime.update_context(false);
    assert_eq!(
        updates.schedule(&session, initial),
        Some(initial.monotonic + Duration::from_secs(180))
    );
    assert_eq!(
        updates.schedule(&session, runtime.update_context(true)),
        None
    );
    assert!(session.active().update_check_enabled());
    runtime.handle(Input::Signal(Signal::SessionLocked));
    runtime.handle(Input::Signal(Signal::Suspended));
    runtime.handle(Input::Signal(Signal::Resumed));
    assert!(
        !runtime.update_context(false).allowed,
        "resume must not unlock the session"
    );
    runtime.handle(Input::Signal(Signal::SessionUnlocked));
    let resumed = runtime.update_context(false);
    assert!(resumed.allowed);
    assert_ne!(resumed.epoch, initial.epoch);
    assert_eq!(
        updates.schedule(&session, resumed),
        Some(resumed.monotonic + Duration::from_secs(180))
    );
    runtime.platform_mut().mono += Duration::from_secs(180);
    assert!(updates.begin(&mut session, runtime.update_context(false), true));
    runtime.handle(Input::Quit);
    let quit = runtime.update_context(false);
    assert!(!quit.allowed);
    assert_eq!(updates.schedule(&session, quit), None);
    assert_eq!(
        updates.complete(
            &mut session,
            &crate::update_check::CheckResult::Available {
                version: "9.0.0".into(),
                url: "https://github.com/tjorim/ElevenClock-rs/releases/tag/v9.0.0".into()
            },
            quit
        ),
        (false, false)
    );
}

#[test]
fn update_cancellation_survives_lock_and_unlock_in_one_message_batch() {
    use crate::{
        preferences_store::{MemoryPreferencesStore, PreferenceSession},
        update_check::{CheckResult, UpdateRuntime},
    };
    let mut runtime = runtime(Monitors::Primary);
    let mut session = PreferenceSession::open(MemoryPreferencesStore::default());
    let mut updates = UpdateRuntime::default();
    assert!(updates.begin(&mut session, runtime.update_context(false), false));
    runtime.handle(Input::Signal(Signal::SessionLocked));
    runtime.handle(Input::Signal(Signal::SessionUnlocked));
    let context = runtime.update_context(false);
    assert!(context.allowed);
    updates.schedule(&session, context);
    assert!(updates.needs_cancel());
    assert!(!updates.accepts_result(&session, context));
    assert_eq!(
        updates.complete(&mut session, &CheckResult::UpToDate, context),
        (false, false)
    );
}
