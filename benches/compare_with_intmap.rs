// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emap::Map;
use intmap::IntMap;

type IntMapI32 = IntMap<usize, i32>;

fn compare_ctors(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_with_capacity_some");
    let sizes: [usize; 5] = [10, 100, 1000, 10_000, 25_000];

    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("intmap", size), size, |b, size| {
            b.iter(|| {
                let mut map = IntMapI32::with_capacity(*size);
                for i in 0..*size {
                    map.insert(i, black_box(42_i32));
                }
                black_box(map);
            })
        });

        group.bench_with_input(BenchmarkId::new("map", size), size, |b, size| {
            b.iter(|| {
                black_box(Map::<i32>::with_capacity_some(
                    black_box(*size),
                    black_box(42_i32),
                ));
            })
        });
    }
    group.finish();
}

fn compare_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_insert");
    let sizes: [usize; 5] = [10, 100, 1000, 10_000, 25_000];

    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("intmap", size), size, |b, size| {
            let mut map = IntMapI32::with_capacity(*size);
            b.iter(|| {
                for i in 0..*size {
                    black_box(map.insert(i, black_box(42_i32)));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("map", size), size, |b, size| {
            let mut map = Map::<i32>::with_capacity_none(*size);
            b.iter(|| {
                for i in 0..*size {
                    black_box(map.insert(black_box(i), black_box(42_i32)));
                }
            });
        });
    }
    group.finish();
}

fn compare_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_values");
    let sizes: [usize; 5] = [10, 100, 1000, 10_000, 25_000];

    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("intmap", size), size, |b, size| {
            let mut map = IntMapI32::with_capacity(*size);
            for i in 0..*size {
                map.insert(i, 42_i32);
            }
            b.iter(|| {
                let mut sum = 0;
                for _ in 0..*size {
                    for s in black_box(map.values()) {
                        sum += black_box(*s);
                    }
                }
                black_box(sum)
            })
        });

        group.bench_with_input(BenchmarkId::new("map", size), size, |b, size| {
            let map = Map::<i32>::with_capacity_some(*size, 42_i32);
            b.iter(|| {
                let mut sum = 0;
                for _ in 0..*size {
                    for s in black_box(map.values()) {
                        sum += black_box(*s);
                    }
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn compare_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare_keys");
    let sizes: [usize; 5] = [10, 100, 1000, 10_000, 25_000];

    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("intmap", size), size, |b, size| {
            let mut map = IntMapI32::with_capacity(*size);
            for i in 0..*size {
                map.insert(i, 42_i32);
            }
            b.iter(|| {
                let mut sum = 0;
                for _ in 0..*size {
                    for k in black_box(map.keys()) {
                        sum += black_box(k);
                    }
                }
                black_box(sum)
            })
        });

        group.bench_with_input(BenchmarkId::new("map", size), size, |b, size| {
            let map = Map::<i32>::with_capacity_some(*size, 42_i32);
            b.iter(|| {
                let mut sum = 0;
                for _ in 0..*size {
                    for k in black_box(map.keys()) {
                        sum += black_box(k);
                    }
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_secs(2))
        .measurement_time(std::time::Duration::from_secs(3))
        .sample_size(20);
    targets = compare_ctors, compare_insert, compare_values, compare_keys
}
criterion_main!(benches);
