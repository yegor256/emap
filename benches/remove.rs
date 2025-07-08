// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use emap::Map;

const CAPACITY: usize = 65536;

fn remove_benchmarks(c: &mut Criterion) {
    c.bench_function("remove_big_array", |b| {
        let mut m: Map<[u8; 1024]> = Map::with_capacity_none(CAPACITY);
        for i in 0..CAPACITY {
            m.insert(i, [0; 1024]);
        }
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.remove(i));
            }
        });
    });

    c.bench_function("remove_bool", |b| {
        let mut m: Map<bool> = Map::with_capacity_none(CAPACITY);
        for i in 0..CAPACITY {
            m.insert(i, true);
        }
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.remove(i));
            }
        });
    });

    c.bench_function("remove_eight_bytes", |b| {
        let mut m: Map<u64> = Map::with_capacity_none(CAPACITY);
        for i in 0..CAPACITY {
            m.insert(i, 42);
        }
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.remove(i));
            }
        });
    });

    c.bench_function("remove_four_bytes", |b| {
        let mut m: Map<u32> = Map::with_capacity_none(CAPACITY);
        for i in 0..CAPACITY {
            m.insert(i, 42);
        }
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.remove(i));
            }
        });
    });
}

criterion_group!(benches, remove_benchmarks);
criterion_main!(benches);
