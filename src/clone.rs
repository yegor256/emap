// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<V: Clone> Clone for Map<V> {
    fn clone(&self) -> Self {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't clone() non-initialized Map");
        let mut m = Self::with_capacity_none(self.capacity());
        for (k, v) in self.iter() {
            m.insert(k, v.clone());
        }
        m
    }
}

#[test]
fn map_can_be_cloned() {
    let mut m: Map<u8> = Map::with_capacity_none(16);
    m.insert(0, 42);
    assert_eq!(42, *m.clone().get(0).unwrap());
}

#[test]
#[ignore]
#[allow(clippy::redundant_clone)]
fn empty_clone() {
    let m: Map<u8> = Map::with_capacity_none(16);
    assert!(m.clone().is_empty());
}

#[test]
#[ignore]
fn larger_map_can_be_cloned() {
    let cap = 16;
    let mut m: Map<u8> = Map::with_capacity_none(cap);
    m.insert(1, 42);
    m.insert(2, 42);
    assert_eq!(2, m.clone().len());
    assert_eq!(cap, m.clone().capacity());
}

#[derive(Clone)]
#[allow(dead_code)]
struct Foo {
    m: Map<u64>,
}

#[test]
#[ignore]
fn clone_of_wrapper() {
    let mut f: Foo = Foo { m: Map::with_capacity_none(16) };
    f.m.insert(7, 42);
    assert_eq!(1, f.clone().m.len());
}
