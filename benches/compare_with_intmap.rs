// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;
use std::time::Duration;

use criterion::{
    BatchSize, BenchmarkId, Criterion, SamplingMode, Throughput, criterion_group, criterion_main,
};
use emap::Map as EMap;
use intmap::IntMap;

/// Alias used for clarity in benchmarks.
type IntMapI32 = IntMap<usize, i32>;

const SIZES: &[usize] = &[10, 100, 1_000, 10_000, 25_000];
const PASSES: usize = 64; // how many repeated scans in read-only benches

/// Global Criterion config tuned for stability on heavy batches.
fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(8))
        .sample_size(60)
        .noise_threshold(0.01)
}

/// Builds a fresh `IntMapI32` prefilled with `size` elements.
fn setup_intmap_prefilled(size: usize, value: i32) -> IntMapI32 {
    let mut m = IntMapI32::with_capacity(size);
    for i in 0..size {
        m.insert(i, value);
    }
    m
}

/// Builds a fresh `EMap<i32>` prefilled with `size` elements using the safe API.
fn setup_emap_prefilled_safe(size: usize, value: i32) -> EMap<i32> {
    let mut m = EMap::<i32>::with_capacity_none(size);
    for i in 0..size {
        m.insert(i, value);
    }
    m
}

/// Builds a fresh `EMap<i32>` prefilled with `size` elements using the unchecked API.
/// Safety invariants are enforced by construction in this helper.
fn setup_emap_prefilled_unchecked(size: usize, value: i32) -> EMap<i32> {
    let mut m = EMap::<i32>::with_capacity_none(size);
    for i in 0..size {
        // SAFETY: each key 0..size inserted exactly once into a fresh map
        unsafe { m.insert_unchecked(i, value) };
    }
    m
}

/// Compare constructors without prefill (empty maps).
fn compare_ctors_empty(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctors_empty");
    group.sampling_mode(SamplingMode::Flat);

    for &size in SIZES {
        group.throughput(Throughput::Elements(1));

        group.bench_with_input(BenchmarkId::new("intmap_empty", size), &size, |b, &n| {
            b.iter(|| {
                let m = IntMapI32::with_capacity(black_box(n));
                black_box(m);
            });
        });

        group.bench_with_input(BenchmarkId::new("emap_empty", size), &size, |b, &n| {
            b.iter(|| {
                let m = EMap::<i32>::with_capacity_none(black_box(n));
                black_box(m);
            });
        });
    }
    group.finish();
}

/// Compare constructors with prefill to target size.
fn compare_ctors_prefill(c: &mut Criterion) {
    let mut group = c.benchmark_group("ctors_prefill");
    group.sampling_mode(SamplingMode::Flat);

    for &size in SIZES {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("intmap_ctor+fill", size), &size, |b, &n| {
            b.iter(|| {
                let mut m = IntMapI32::with_capacity(n);
                let v = black_box(42_i32);
                for i in 0..n {
                    m.insert(i, v);
                }
                black_box(m);
            });
        });

        group.bench_with_input(BenchmarkId::new("emap_ctor+fill_safe", size), &size, |b, &n| {
            b.iter(|| {
                let mut m = EMap::<i32>::with_capacity_none(n);
                let v = black_box(42_i32);
                for i in 0..n {
                    m.insert(i, v);
                }
                black_box(m);
            });
        });

        group.bench_with_input(
            BenchmarkId::new("emap_ctor+fill_unchecked", size),
            &size,
            |b, &n| {
                b.iter(|| {
                    let m = setup_emap_prefilled_unchecked(n, black_box(42_i32));
                    black_box(m);
                });
            },
        );
    }
    group.finish();
}

/// Compare insertion throughput into a fresh map per iteration.
fn compare_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_throughput");
    group.sampling_mode(SamplingMode::Flat);

    for &size in SIZES {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(BenchmarkId::new("intmap_safe", size), &size, |b, &n| {
            b.iter_batched(
                || IntMapI32::with_capacity(n),
                |mut m| {
                    let v = black_box(42_i32);
                    for i in 0..n {
                        m.insert(i, v);
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("emap_safe", size), &size, |b, &n| {
            b.iter_batched(
                || EMap::<i32>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(42_i32);
                    for i in 0..n {
                        m.insert(i, v);
                    }
                    black_box(m);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("emap_unchecked", size), &size, |b, &n| {
            b.iter_batched(
                || EMap::<i32>::with_capacity_none(n),
                |mut m| {
                    let v = black_box(42_i32);
                    for i in 0..n {
                        // SAFETY: fresh map, unique keys 0..n, capacity reserved
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

/// Compare sequential scans over values on prefilled maps.
/// We count elements visited as throughput: size * PASSES per batch.
fn compare_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan_values");
    group.sampling_mode(SamplingMode::Flat);

    for &size in SIZES {
        group.throughput(Throughput::Elements((size * PASSES) as u64));

        group.bench_with_input(BenchmarkId::new("intmap_values", size), &size, |b, &n| {
            b.iter_batched(
                || setup_intmap_prefilled(n, 42),
                |m| {
                    let mut acc = 0i64;
                    for _ in 0..PASSES {
                        for v in m.values() {
                            acc += black_box(*v as i64);
                        }
                    }
                    black_box(acc);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("emap_values", size), &size, |b, &n| {
            b.iter_batched(
                || setup_emap_prefilled_safe(n, 42),
                |m| {
                    let mut acc = 0i64;
                    for _ in 0..PASSES {
                        for v in m.values() {
                            acc += black_box(*v as i64);
                        }
                    }
                    black_box(acc);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("emap_values_unchecked", size), &size, |b, &n| {
            b.iter_batched(
                || setup_emap_prefilled_unchecked(n, 42),
                |m| {
                    let mut acc = 0i64;
                    for _ in 0..PASSES {
                        for v in m.values() {
                            acc += black_box(*v as i64);
                        }
                    }
                    black_box(acc);
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

/// Compare sequential scans over keys on prefilled maps.
/// Throughput = size * PASSES.
fn compare_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("scan_keys");
    group.sampling_mode(SamplingMode::Flat);

    for &size in SIZES {
        group.throughput(Throughput::Elements((size * PASSES) as u64));

        group.bench_with_input(BenchmarkId::new("intmap_keys", size), &size, |b, &n| {
            b.iter_batched(
                || setup_intmap_prefilled(n, 42),
                |m| {
                    let mut acc = 0usize;
                    for _ in 0..PASSES {
                        for k in m.keys() {
                            acc = acc.wrapping_add(black_box(k));
                        }
                    }
                    black_box(acc);
                },
                BatchSize::SmallInput,
            );
        });

        group.bench_with_input(BenchmarkId::new("emap_keys", size), &size, |b, &n| {
            b.iter_batched(
                || setup_emap_prefilled_safe(n, 42),
                |m| {
                    let mut acc = 0usize;
                    for _ in 0..PASSES {
                        for k in m.keys() {
                            acc = acc.wrapping_add(black_box(k));
                        }
                    }
                    black_box(acc);
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets =
        compare_ctors_empty,
        compare_ctors_prefill,
        compare_insert,
        compare_values,
        compare_keys
}
criterion_main!(benches);
