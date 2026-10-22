//! Informative reducer timings for a four-display setup. Nothing gates on these:
//! the operation-count guards in the tests are the regression checks, and this
//! shows how large the per-event cost is when you change the reducer.
use chrono::{DateTime, FixedOffset};
use clock_core::{
    DiscoveryStrategy, Event, MonitorId, Monitors, PhysicalRect, Preferences, ShellGeneration,
    State, TaskbarId, TaskbarKey, TaskbarSnapshot, TaskbarVisibility, reduce,
};
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const DISPLAYS: u64 = 4;

fn wall(value: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_rfc3339(value).unwrap()
}
fn preferences(format: &str) -> clock_core::ValidatedPreferences {
    Preferences {
        format: format.into(),
        monitors: Monitors::All,
        ..Preferences::default()
    }
    .validate()
    .unwrap()
}
/// Side-by-side 1920x1080 displays, each with a bottom taskbar and its clock slot.
fn snapshots() -> Vec<TaskbarSnapshot> {
    (1..=DISPLAYS)
        .map(|id| {
            let left = (id as i32 - 1) * 1920;
            TaskbarSnapshot {
                key: TaskbarKey {
                    id: TaskbarId(id),
                    generation: ShellGeneration(0),
                },
                monitor: MonitorId(id),
                monitor_rect: PhysicalRect::new(left, 0, 1920, 1080).unwrap(),
                taskbar_rect: PhysicalRect::new(left, 1032, 1920, 48).unwrap(),
                clock_rect: Some(PhysicalRect::new(left + 1800, 1032, 120, 48).unwrap()),
                dpi: 96,
                primary: id == 1,
                visibility: TaskbarVisibility::Visible,
                strategy: DiscoveryStrategy::NativeChildren,
            }
        })
        .collect()
}
fn discovery() -> Event {
    Event::DiscoveryResult {
        generation: ShellGeneration(0),
        snapshots: snapshots(),
    }
}
fn ready(format: &str) -> State {
    let (state, _) = reduce(
        State::new(preferences(format)),
        Event::Tick(wall("2026-10-14T12:00:00+00:00")),
    );
    reduce(state, discovery()).0
}

fn benchmark(c: &mut Criterion) {
    let minute = ready("%H:%M");
    c.bench_function("state/tick_unchanged_text_4_displays", |b| {
        let event = Event::Tick(wall("2026-10-14T12:00:30+00:00"));
        b.iter(|| reduce(black_box(minute.clone()), black_box(event.clone())))
    });
    c.bench_function("state/tick_new_minute_4_displays", |b| {
        let event = Event::Tick(wall("2026-10-14T12:01:00+00:00"));
        b.iter(|| reduce(black_box(minute.clone()), black_box(event.clone())))
    });
    c.bench_function("state/discovery_unchanged_4_displays", |b| {
        let event = discovery();
        b.iter(|| reduce(black_box(minute.clone()), black_box(event.clone())))
    });
    c.bench_function("state/preferences_unchanged_4_displays", |b| {
        let event = Event::PreferencesApplied(preferences("%H:%M"));
        b.iter(|| reduce(black_box(minute.clone()), black_box(event.clone())))
    });
    c.bench_function("state/preferences_new_format_4_displays", |b| {
        let event = Event::PreferencesApplied(preferences("%H:%M:%S"));
        b.iter(|| reduce(black_box(minute.clone()), black_box(event.clone())))
    });
}
criterion_group! { name = benches; config = Criterion::default().sample_size(20).warm_up_time(std::time::Duration::from_millis(500)).measurement_time(std::time::Duration::from_secs(1)); targets = benchmark }
criterion_main!(benches);
