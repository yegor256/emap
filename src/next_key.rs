// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;

impl<V> Map<V> {
    /// Get the next key available for insertion.
    ///
    /// # Panics
    ///
    /// If no more keys left.
    #[inline]
    #[must_use]
    pub fn next_key(&self) -> usize {
        if self.first_free.is_def() {
            self.first_free.get()
        } else {
            panic!("No more keys available left");
        }
    }
}

#[test]
fn get_next_key_empty_map() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(0, m.next_key());
}

#[test]
fn get_next_in_the_middle() {
    let mut m: Map<u32> = Map::with_capacity_none(16);
    m.insert(0, 42);
    m.insert(1, 42);
    m.remove(1);
    m.insert(2, 42);
    assert_eq!(1, m.next_key());
}

#[test]
fn restore_next_key() {
    let mut m: Map<u32> = Map::with_capacity_none(4);
    m.insert(0, 42);
    m.insert(1, 42);
    m.insert(2, 42);
    m.insert(3, 42);
    m.remove(2);
    assert_eq!(2, m.next_key());
}

#[test]
fn reset_next_key_on_clear() {
    let mut m: Map<u32> = Map::with_capacity_none(16);
    m.insert(0, 42);
    assert_eq!(1, m.next_key());
    m.clear();
    assert_eq!(0, m.next_key());
}

#[test]
#[should_panic]
fn panics_on_end_of_keys() {
    let mut m: Map<u32> = Map::with_capacity_none(1);
    m.insert(0, 42);
    assert_ne!(1, m.next_key());
}
