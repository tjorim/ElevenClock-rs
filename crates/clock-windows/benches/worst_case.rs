//! Informative timing for the worst-case sampling sweep that Settings fit checks,
//! preference changes and the first paint of a format all depend on. Nothing gates
//! on it and it is not a Criterion benchmark: Criterion's `alloca` dependency would
//! break the documented Linux cross-target lint of this crate. Run it with
//! `cargo bench -p clock-windows --bench worst_case --locked`; the operation-count
//! guards in `runtime/tests.rs` are what catch an avoidable extra sweep.
use clock_core::format::{CompiledFormat, PRESETS};
use clock_windows::render::WorstCase;
use std::{hint::black_box, time::Instant};

const RUNS: usize = 15;

fn main() {
    println!("worst-case sweep, {RUNS} runs each (min / median):");
    for (name, source) in PRESETS
        .iter()
        .map(|preset| (preset.name, preset.format))
        .chain([("seconds, two lines", "%H:%M:%S\n%a %d %b %Y")])
    {
        let format = CompiledFormat::parse(source).expect("preset formats are valid");
        let mut times: Vec<_> = (0..RUNS)
            .map(|_| {
                let start = Instant::now();
                black_box(WorstCase::for_format(black_box(&format)));
                start.elapsed()
            })
            .collect();
        times.sort();
        println!(
            "  {name:<24} {:>10.2?} / {:>10.2?}",
            times[0],
            times[RUNS / 2]
        );
    }
}
