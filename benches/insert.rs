// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![feature(test)]

extern crate test;
use emap::Map;
use test::Bencher;

const CAPACITY: usize = 65536;

#[bench]
fn insert_long_str(b: &mut Bencher) {
    let mut m: Map<&str> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.insert(i, &"Hello, world! How are you doing today?");
        }
    });
}

#[bench]
fn insert_eight_bytes(b: &mut Bencher) {
    let mut m: Map<u64> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.insert(i, 42);
        }
    });
}
