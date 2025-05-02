// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Keys;
use crate::Map;
use std::mem;

impl<V> Iterator for Keys<V> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.current < self.end {
                let opt = &*self.current;
                if opt.is_some() {
                    let pos = (self.current as usize - self.start as usize) / mem::size_of::<V>();
                    self.current = self.current.add(1);
                    return Some(pos);
                }
                self.current = self.current.add(1);
            }
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
            start: self.head,
            current: self.head,
            end: unsafe { self.head.add(self.max) },
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
