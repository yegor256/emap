// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Keys;
use crate::Map;
use std::ptr;

impl<V> Iterator for Keys<V> {
    type Item = usize;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<usize> {
        while self.pos < self.max {
            let opt = unsafe { ptr::read(self.head.add(self.pos)) };
            if opt.is_some() {
                let k = self.pos;
                self.pos += 1;
                return Some(k);
            }
            self.pos += 1;
        }
        None
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
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

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
