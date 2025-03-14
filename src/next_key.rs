// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
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
        self.next_key_gte(0)
    }

    /// Get the next key available for insertion, which is "greater or equal"
    /// than the number provided.
    ///
    /// # Panics
    ///
    /// If no more keys left.
    ///
    /// It may also panic in "debug" mode if the Map is not initialized.
    #[inline]
    #[must_use]
    pub fn next_key_gte(&self, k: usize) -> usize {
        #[cfg(debug_assertions)]
        assert!(
            self.initialized,
            "Can't do next_key_gte() on non-initialized Map"
        );
        let mut i = k;
        loop {
            if i == self.max {
                break;
            }
            if i > self.max {
                return i;
            }
            if self.get(i).is_none() {
                return i;
            }
            i += 1;
        }
        assert_ne!(self.max, self.layout.size(), "No more keys available left");
        self.max
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
fn get_next_over() {
    let mut m: Map<u32> = Map::with_capacity_none(16);
    m.insert(2, 42);
    assert_eq!(5, m.next_key_gte(5));
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
