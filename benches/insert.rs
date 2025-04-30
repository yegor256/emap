// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emap::Map;

const CAPACITY: usize = 65536;

fn insert_long_str(c: &mut Criterion) {
    c.bench_function("insert_long_str", |b| {
        let mut m: Map<&str> = Map::with_capacity(CAPACITY);
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.insert(i, black_box("Hello, world! How are you doing today?")));
            }
        })
    });
}

fn insert_eight_bytes(c: &mut Criterion) {
    c.bench_function("insert_eight_bytes", |b| {
        let mut m: Map<u64> = Map::with_capacity(CAPACITY);
        b.iter(|| {
            for i in 0..CAPACITY {
                black_box(m.insert(i, black_box(42)));
            }
        })
    });
}

criterion_group!(benches, insert_long_str, insert_eight_bytes);
criterion_main!(benches);
