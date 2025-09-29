// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;

use criterion::{
    BatchSize, BenchmarkGroup, BenchmarkId, Criterion, Throughput, criterion_group,
    criterion_main, measurement::WallTime,
};
use emap::Map;

const CAPACITY: usize = 65_536;

/// Prepare a fresh pre-filled map for the removal benchmark.
/// This isolates per-iteration state and avoids leakage across iterations.
fn setup_prefilled_map<T: Copy>(value: T) -> Map<T> {
    let mut m = Map::with_capacity_none(CAPACITY);
    for i in 0..CAPACITY {
        // SAFETY: fresh map in this batch, keys are unique, capacity is sufficient.
        unsafe { m.insert_unchecked(i, value) };
    }
    m
}

/// Benchmark "remove" with the safe API variant.
/// Measures only the removal cost; population happens in setup.
fn bench_remove_safe<T: Copy + 'static>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    value: T,
) {
    group.bench_function(BenchmarkId::new(label, "safe"), |b| {
        b.iter_batched(
            || setup_prefilled_map(value),
            |mut m| {
                for i in 0..CAPACITY {
                    let _ = m.remove(i);
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });
}

/// Benchmark "remove_unchecked" with explicit invariants.
/// Only valid if every key exists exactly once in this batch.
fn bench_remove_unchecked<T: Copy + 'static>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    label: &str,
    value: T,
) {
    group.bench_function(BenchmarkId::new(label, "unsafe_unchecked"), |b| {
        b.iter_batched(
            || setup_prefilled_map(value),
            |mut m| {
                for i in 0..CAPACITY {
                    // SAFETY: every key i in [0..CAPACITY) exists in this fresh map.
                    unsafe { m.remove_unchecked(i) };
                }
                black_box(m);
            },
            BatchSize::SmallInput,
        );
    });
}

/// Bench group for removals across several payload sizes.
/// - bool:       minimal payload
/// - u32/u64:    4/8 bytes
/// - [u8; 1024]: larger payload to expose memcpy/alloc effects
///
/// We attach throughput to make results comparable across variants.
fn remove_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("emap_remove");
    group.throughput(Throughput::Elements(CAPACITY as u64));
    bench_remove_safe(&mut group, "bool", true);
    bench_remove_unchecked(&mut group, "bool", true);
    bench_remove_safe(&mut group, "u32", 42_u32);
    bench_remove_unchecked(&mut group, "u32", 42_u32);
    bench_remove_safe(&mut group, "u64", 42_u64);
    bench_remove_unchecked(&mut group, "u64", 42_u64);
    bench_remove_safe(&mut group, "[u8;1024]", [0_u8; 1024]);
    bench_remove_unchecked(&mut group, "[u8;1024]", [0_u8; 1024]);
    group.finish();
}

criterion_group!(benches, remove_benchmarks);
criterion_main!(benches);
