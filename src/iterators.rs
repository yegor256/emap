// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{IntoIter, Iter, IterMut, Map};
use std::marker::PhantomData;

impl<'a, V: Clone + 'a> Iterator for Iter<'a, V> {
    type Item = (usize, &'a V);

    /// This is an implementation of the `next` function that returns the next item in an iterator if it
    /// exists, or `None` if the end of the iterator has been reached.
    ///
    /// Returns:
    ///
    /// The `next` function returns an `Option` that contains a tuple of `(i, p)` where `i` is the index of
    /// the item and `p` is a reference to the item. If there are no more items to iterate over, it returns
    /// `None`.
    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let item = unsafe { &*self.head.add(self.pos) };
            if let Some(p) = item {
                let i = self.pos;
                self.pos += 1;
                return Some((i, p));
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, V: Clone + 'a> Iterator for IterMut<'a, V> {
    type Item = (usize, &'a mut V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let item = unsafe { &mut *self.head.add(self.pos) };
            if let Some(p) = item {
                let i = self.pos;
                self.pos += 1;
                return Some((i, p));
            }
            self.pos += 1;
        }
        None
    }
}

impl<V: Copy> Iterator for IntoIter<V> {
    type Item = (usize, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let item = unsafe { &*self.head.add(self.pos) };
            if let Some(v) = item {
                let i = self.pos;
                self.pos += 1;
                return Some((i, *v));
            }
            self.pos += 1;
        }
        None
    }
}

impl<'a, V: Copy> IntoIterator for &'a Map<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

impl<V: Clone> Map<V> {
    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter() non-initialized Map");
        Iter {
            max: self.max,
            pos: 0,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make a mutable iterator over all items.
    ///
    /// For example:
    ///
    /// ```
    /// use emap::Map;
    /// let mut m: Map<String> = Map::with_capacity_none(16);
    /// m.insert(0, "Jeff".to_string());
    /// m.insert(1, "Lebowski".to_string());
    /// for (_, v) in m.iter_mut() {
    ///   *v = v.to_lowercase();
    /// }
    /// ```
    ///
    /// The keys are not mutable, only the values, for obvious reasons.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn iter_mut(&self) -> IterMut<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter_mut() non-initialized Map");
        IterMut {
            max: self.max,
            pos: 0,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_iter() non-initialized Map");
        IntoIter {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

#[test]
fn empty_iterator() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.into_iter().next().is_none());
}

#[test]
fn insert_and_jump_over_next() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "foo");
    let mut iter = m.into_iter();
    assert_eq!("foo", iter.next().unwrap().1);
    assert!(iter.next().is_none());
}

#[test]
fn insert_and_iterate() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
}

#[test]
fn insert_and_into_iterate() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.into_iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
}

#[test]
fn iterate_without_function() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "test");
    let mut count = 0;
    for (_, _) in &m {
        count += 1;
    }
    assert_eq!(1, count);
}

#[test]
fn iterate_and_mutate() {
    let mut m: Map<u64> = Map::with_capacity_none(16);
    m.insert(0, 16);
    m.insert(1, 32);
    m.insert(2, 64);
    for (_, v) in m.iter_mut() {
        *v += 1;
    }
    let mut sum = 0;
    for v in m.values() {
        sum += v;
    }
    assert_eq!(115, sum);
}
