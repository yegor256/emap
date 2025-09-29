// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;
use std::time::Duration;

use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, SamplingMode, Throughput,
};
use emap::Map;

const CAPACITY: usize = 65_536;

/// Global Criterion config tuned for heavy batches:
/// - Longer measurement avoids "unable to complete 100 samples in 5.0s"
/// - Smaller sample size reduces total runtime and noise for large batches
/// - Warmup is trimmed to keep total run time sane
fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(8))
        .sample_size(60)
}

/// Benchmarks insertion of CAPACITY long string values into a fresh Map per iteration,
/// measuring only the insertion workload (allocation is in setup).
///
/// Safety:
/// - Uses safe `insert` API variant to avoid UB across iterations.
/// - A fresh map is constructed for every measured iteration to prevent state leakage.
fn bench_insert_long_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_insert_long_str");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(CAPACITY as u64));

    let value: &str = "Hello, world! How are you doing today?";
    group.bench_function(BenchmarkId::from_parameter("safe"), |b| {
        b.iter_batched(
            || Map::<&str>::with_capacity_none(CAPACITY),
            |mut m| {
                let v = black_box(value);
                for i in 0..CAPACITY {
                    m.insert(i, v);
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::from_parameter("unsafe_unchecked"), |b| {
        b.iter_batched(
            || Map::<&str>::with_capacity_none(CAPACITY),
            |mut m| {
                let v = black_box(value);
                for i in 0..CAPACITY {
                    // SAFETY: each key i is inserted exactly once
                    unsafe { m.insert_unchecked(i, v) };
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Benchmarks insertion of CAPACITY u64 values into a fresh Map per iteration.
/// Uses safe API for the primary measurement.
fn bench_insert_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_insert_u64");
    group.sampling_mode(SamplingMode::Flat);
    group.throughput(Throughput::Elements(CAPACITY as u64));

    let val: u64 = 42;
    group.bench_function(BenchmarkId::from_parameter("safe"), |b| {
        b.iter_batched(
            || Map::<u64>::with_capacity_none(CAPACITY),
            |mut m| {
                let v = black_box(val);
                for i in 0..CAPACITY {
                    m.insert(i, v);
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::from_parameter("unsafe_unchecked"), |b| {
        b.iter_batched(
            || Map::<u64>::with_capacity_none(CAPACITY),
            |mut m| {
                let v = black_box(val);
                for i in 0..CAPACITY {
                    // SAFETY: each key i is inserted exactly once
                    unsafe { m.insert_unchecked(i, v) };
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_insert_long_str, bench_insert_u64
}
criterion_main!(benches);
