// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![allow(clippy::unit_arg)]

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use emap::Map;

const CAPACITY: usize = 65536;

fn insert_long_str(c: &mut Criterion) {
    c.bench_function("insert_long_str", |b| {
        let mut m: Map<&str> = Map::with_capacity_none(CAPACITY);
        b.iter(|| {
            for i in 0..CAPACITY {
                unsafe {
                    m.insert_unchecked(i, black_box("Hello, world! How are you doing today?"))
                };
            }
            // Prevent optimization removing the loop
            black_box(&m);
        });
    });
}

fn insert_eight_bytes(c: &mut Criterion) {
    c.bench_function("insert_eight_bytes", |b| {
        let mut m: Map<u64> = Map::with_capacity_none(CAPACITY);
        b.iter(|| {
            for i in 0..CAPACITY {
                unsafe { m.insert_unchecked(i, black_box(42)) };
            }
            // Prevent optimization removing the loop
            black_box(&m);
        });
    });
}

criterion_group!(benches, insert_long_str, insert_eight_bytes);
criterion_main!(benches);
