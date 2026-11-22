//! Wall snapshots choose alignment; the runtime waits using a monotonic timer.
use crate::Cadence;
use std::time::Duration;
use time::OffsetDateTime;

/// Strictly positive wait to the next local second/minute boundary. Date-only
/// formats use Minute. Each wake/resume must supply a fresh snapshot; never add
/// this duration to the previous wall time.
pub fn next_delay(snapshot: OffsetDateTime, cadence: Cadence) -> Duration {
    let nanos = u64::from(snapshot.nanosecond());
    let period = match cadence {
        Cadence::Second => 1_000_000_000,
        Cadence::Minute => 60_000_000_000,
    };
    let elapsed = match cadence {
        Cadence::Second => nanos,
        Cadence::Minute => u64::from(snapshot.second()) * 1_000_000_000 + nanos,
    };
    Duration::from_nanos(period - elapsed)
}
