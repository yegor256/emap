// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![feature(test)]

extern crate test;
use emap::Map;
use test::Bencher;

const CAPACITY: usize = 65536;

#[bench]
fn remove_big_array(b: &mut Bencher) {
    let mut m: Map<[u8; 1024]> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.remove(i);
        }
    });
}

#[bench]
fn remove_bool(b: &mut Bencher) {
    let mut m: Map<bool> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.remove(i);
        }
    });
}

#[bench]
fn remove_eight_bytes(b: &mut Bencher) {
    let mut m: Map<u64> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.remove(i);
        }
    });
}

#[bench]
fn remove_four_bytes(b: &mut Bencher) {
    let mut m: Map<u32> = Map::with_capacity(CAPACITY);
    b.iter(|| {
        for i in 0..CAPACITY {
            m.remove(i);
        }
    });
}
