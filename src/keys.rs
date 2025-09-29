// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Keys;
use crate::Map;
use std::mem;

impl<V> Iterator for Keys<V> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_def() {
            let mut next = unsafe { &*self.head.add(self.current.get()) }.get_next();
            mem::swap(&mut self.current, &mut next);
            Some(next.get())
        } else {
            None
        }
    }
}

impl<V> Map<V> {
    /// Make an iterator over all keys.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn keys(&self) -> Keys<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't keys() non-initialized Map");
        Keys {
            current: self.first_used,
            head: self.head,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn empty_keys() {
        let m: Map<u32> = Map::with_capacity_none(16);
        assert!(m.keys().next().is_none());
    }

    #[test]
    fn insert_and_jump_over_next_key() {
        let mut m: Map<&str> = Map::with_capacity_none(16);
        m.insert(0, "foo");
        let mut keys = m.keys();
        assert_eq!(0, keys.next().unwrap());
        assert!(keys.next().is_none());
    }

    #[test]
    fn keys_empty_map() {
        let map: Map<u32> = Map::with_capacity_none(10);
        let keys: Vec<_> = map.keys().collect();
        assert!(keys.is_empty());
    }

    #[test]
    fn keys_basic() {
        let mut map = Map::with_capacity_none(5);
        map.insert(1, 10);
        map.insert(3, 30);

        let keys: HashSet<_> = map.keys().collect();
        assert_eq!(keys, [1, 3].iter().copied().collect());
    }

    #[test]
    fn keys_remove() {
        let mut map = Map::with_capacity_none(5);
        map.insert(0, 10);
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);
        map.insert(4, 40);

        map.remove(0);
        map.remove(2);
        map.remove(4);
        let keys: HashSet<_> = map.keys().collect();
        assert_eq!(keys, [1, 3].iter().copied().collect());
    }

    #[test]
    fn keys_full_map() {
        let mut map = Map::with_capacity_some(3, 0);
        map.insert(0, 100);
        map.insert(1, 200);
        map.insert(2, 300);

        let keys: HashSet<_> = map.keys().collect();
        assert_eq!(keys, [0, 1, 2].iter().copied().collect());
    }

    #[test]
    fn keys_with_gaps() {
        let mut map = Map::with_capacity_none(5);
        map.insert(4, 400);
        map.insert(1, 100);

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&1));
        assert!(keys.contains(&4));
    }

    #[test]
    fn keys_duplicate_insert() {
        let mut map = Map::with_capacity_none(3);
        map.insert(1, 10);
        map.insert(1, 20);

        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0], 1);
    }

    #[test]
    fn keys_after_remove() {
        let mut map = Map::with_capacity_none(4);
        map.insert(0, 1);
        map.insert(1, 2);
        map.insert(2, 3);
        map.remove(1);

        let keys: HashSet<_> = map.keys().collect();
        assert_eq!(keys, [0, 2].iter().copied().collect());
    }

    #[test]
    fn keys_all_keys_present() {
        let mut map = Map::with_capacity_none(100);
        let expected_keys: HashSet<_> = (0..100).step_by(3).collect();

        for &key in &expected_keys {
            map.insert(key, key * 10);
        }

        let actual_keys: HashSet<_> = map.keys().collect();
        assert_eq!(actual_keys, expected_keys);
    }
}
