// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use crate::{IntoValues, Values};
use std::marker::PhantomData;

impl<'a, V: 'a> Iterator for Values<'a, V> {
    type Item = &'a V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.current < self.end {
                let opt = &*self.current;
                self.current = self.current.add(1);
                if let Some(value) = opt {
                    return Some(value);
                }
            }
            None
        }
    }
}

impl<V: Copy> Iterator for IntoValues<V> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let opt = unsafe { &*self.head.add(self.pos) };
            self.pos += 1;
            if opt.is_some() {
                return *opt;
            }
        }
        None
    }
}

impl<V> Map<V> {
    /// Make an iterator over all values.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn values(&self) -> Values<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't values() non-initialized Map");
        Values {
            current: self.head,
            end: unsafe { self.head.add(self.max) },
            _marker: PhantomData,
        }
    }

    /// Make an into-iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn into_values(&self) -> IntoValues<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_values() non-initialized Map");
        IntoValues {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

#[test]
fn empty_values() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.values().next().is_none());
}

#[test]
fn insert_and_jump_over_next() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "foo");
    let mut values = m.into_values();
    assert_eq!("foo", values.next().unwrap());
    assert!(values.next().is_none());
}

#[test]
fn count_them_all() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    assert_eq!(3, m.into_values().count());
}
