// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;
use std::time::Duration;

use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, SamplingMode, Throughput,
};
use emap::Map;

/// Sizes used for scale-out benchmarks.
const SIZES: &[usize] = &[16_384, 32_768, 65_536, 131_072];

/// Global Criterion config tuned for heavy batches.
/// - Longer measurement avoids "unable to complete 100 samples in 5.0s"
/// - Moderate sample size keeps runtime reasonable
/// - Flat sampling stabilizes timing for batched workloads
fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(8))
        .sample_size(60)
        .noise_threshold(0.01)
}

/// Batch-throughput benchmark: CAP inserts of an &str (pointer+len only).
///
/// Measures write-hot path without allocation costs in the value.
fn bench_insert_str(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_insert_str");
    group.sampling_mode(SamplingMode::Flat);

    let value: &str = "Hello, world! How are you doing today?";

    for &cap in SIZES {
        group.throughput(Throughput::Elements(cap as u64));

        group.bench_with_input(BenchmarkId::new("safe", cap), &cap, |b, &n| {
            b.iter_batched(
                || Map::<&str>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(value);
                    for i in 0..n {
                        m.insert(i, v);
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });

        #[cfg(feature = "bench_unchecked")]
        group.bench_with_input(BenchmarkId::new("unsafe_unchecked", cap), &cap, |b, &n| {
            b.iter_batched(
                || Map::<&str>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(value);
                    for i in 0..n {
                        // SAFETY: fresh map, unique keys 0..n-1, sufficient capacity.
                        unsafe { m.insert_unchecked(i, v) };
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Batch-throughput benchmark: CAP inserts of a u64.
fn bench_insert_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_insert_u64");
    group.sampling_mode(SamplingMode::Flat);

    let val: u64 = 42;

    for &cap in SIZES {
        group.throughput(Throughput::Elements(cap as u64));

        group.bench_with_input(BenchmarkId::new("safe", cap), &cap, |b, &n| {
            b.iter_batched(
                || Map::<u64>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(val);
                    for i in 0..n {
                        m.insert(i, v);
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });

        #[cfg(feature = "bench_unchecked")]
        group.bench_with_input(BenchmarkId::new("unsafe_unchecked", cap), &cap, |b, &n| {
            b.iter_batched(
                || Map::<u64>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(val);
                    for i in 0..n {
                        // SAFETY: fresh map, unique keys 0..n-1, sufficient capacity.
                        unsafe { m.insert_unchecked(i, v) };
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Batch-throughput benchmark: CAP inserts of an owned `String`.
///
/// This includes allocation and memcpy cost for the value payload.
fn bench_insert_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_insert_string");
    group.sampling_mode(SamplingMode::Flat);

    let payload = String::from("Hello, world! How are you doing today?");

    for &cap in SIZES {
        group.throughput(Throughput::Elements(cap as u64));

        group.bench_with_input(BenchmarkId::new("safe_clone", cap), &cap, |b, &n| {
            b.iter_batched(
                || Map::<String>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(&payload);
                    for i in 0..n {
                        m.insert(i, v.clone());
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Microbenchmark: latency of a single insert into a pre-allocated map.
///
/// Measures per-op latency rather than batch throughput.
fn bench_single_insert_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_single_insert_latency");
    group.sampling_mode(SamplingMode::Flat);

    let cap = 65_536usize;
    group.throughput(Throughput::Elements(1));

    group.bench_function(BenchmarkId::from_parameter("u64_safe"), |b| {
        b.iter_batched(
            || (Map::<u64>::with_capacity_none(cap), 777u64, 123usize),
            |(mut m, v, k)| {
                m.insert(k, v);
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });

    #[cfg(feature = "bench_unchecked")]
    group.bench_function(BenchmarkId::from_parameter("u64_unsafe_unchecked"), |b| {
        b.iter_batched(
            || (Map::<u64>::with_capacity_none(cap), 777u64, 123usize),
            |(mut m, v, k)| {
                // SAFETY: unique key into a fresh map with sufficient capacity.
                unsafe { m.insert_unchecked(k, v) };
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
    targets =
        bench_insert_str,
        bench_insert_u64,
        bench_insert_string,
        bench_single_insert_latency
}
criterion_main!(benches);
