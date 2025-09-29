// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use std::ops::{Index, IndexMut};

impl<V> Index<usize> for Map<V> {
    type Output = V;

    #[inline]
    fn index(&self, key: usize) -> &V {
        self.get(key).expect("No entry found for key")
    }
}

impl<V> IndexMut<usize> for Map<V> {
    #[inline]
    fn index_mut(&mut self, key: usize) -> &mut V {
        self.get_mut(key).expect("No entry found for key")
    }
}

#[cfg(test)]
use std::borrow::Borrow;

#[test]
fn index() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(1, "first");
    assert_eq!("first", m[1]);
}

#[test]
fn index_mut() {
    let mut m: Map<i32> = Map::with_capacity_none(16);
    m.insert(1, 10);
    m[1] += 55;
    assert_eq!(65, m[1]);
}

#[test]
fn wrong_index() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(2, "first");
    m.insert(8, "second");
    m.remove(8);
    assert_eq!(m.get(8), None);
}

#[cfg(test)]
#[derive(Clone, PartialEq, Eq)]
struct Container {
    pub t: i32,
}

#[cfg(test)]
impl Borrow<i32> for Container {
    fn borrow(&self) -> &i32 {
        &self.t
    }
}

#[test]
fn index_by_borrow() {
    let mut m: Map<Container> = Map::with_capacity_none(16);
    m.insert(2, Container { t: 10 });
    assert_eq!(10, m[2].t);
}
