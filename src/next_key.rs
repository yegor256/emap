// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, MapFullError};

impl<V> Map<V> {
    /// Get the next key available for insertion.
    ///
    /// # Panics
    ///
    /// If no more keys left.
    #[inline]
    #[must_use]
    pub fn next_key(&self) -> usize {
        match self.try_next_key() {
            Ok(next) => next,
            Err(error) => panic!("{error}"),
        }
    }

    /// Get the next key available for insertion without panicking on overflow.
    ///
    /// # Errors
    ///
    /// Returns [`MapFullError`] if the map has no free slots left.
    ///
    /// # Examples
    ///
    /// ```
    /// use emap::Map;
    /// let mut map: Map<&str> = Map::with_capacity_none(2);
    /// map.insert(0, "hello");
    /// assert_eq!(map.try_next_key(), Ok(1));
    /// map.insert(1, "world");
    /// assert!(map.try_next_key().is_err());
    /// ```
    #[inline]
    pub const fn try_next_key(&self) -> Result<usize, MapFullError> {
        if self.first_free.is_def() {
            Ok(self.first_free.get())
        } else {
            Err(MapFullError)
        }
    }
}

#[test]
fn get_next_key_empty_map() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(0, m.next_key());
}

#[test]
#[should_panic]
fn next_key_panics_for_zero_capacity() {
    let m: Map<&str> = Map::with_capacity_none(0);
    let _ = m.next_key();
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
fn try_next_key_reports_free_slot() {
    let mut map: Map<u32> = Map::with_capacity_none(4);
    map.insert(1, 7);
    assert_eq!(Ok(0), map.try_next_key());
}

#[test]
fn try_next_key_reports_full_map() {
    let mut map: Map<u32> = Map::with_capacity_none(2);
    map.insert(0, 11);
    map.insert(1, 12);
    assert!(map.try_next_key().is_err());
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
