// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#[cfg(test)]
use crate::node::NodeId;
use crate::{IntoIter, Iter, IterMut, Map};
#[cfg(test)]
use std::convert::TryFrom;
use std::marker::PhantomData;

impl<'a, V> Iterator for Iter<'a, V> {
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
    fn next(&mut self) -> Option<Self::Item> {
        while self.current.is_def() {
            let index = self.current.get();
            let node = unsafe { &*self.head.add(index) };
            self.current = node.get_next();
            if let Some(value) = node.get() {
                return Some((index, value));
            }
        }
        None
    }
}

impl<'a, V> Iterator for IterMut<'a, V> {
    type Item = (usize, &'a mut V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.current.is_def() {
            let index = self.current.get();
            let node = unsafe { &mut *self.head.add(index) };
            self.current = node.get_next();
            if let Some(value) = node.get_mut() {
                return Some((index, value));
            }
        }
        None
    }
}

impl<'a, V> Iterator for IntoIter<'a, V> {
    type Item = (usize, &'a V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a, V> IntoIter<'a, V> {
    #[inline]
    pub(crate) const fn new(inner: Iter<'a, V>) -> Self {
        Self { inner }
    }
}

impl<'a, V> IntoIterator for &'a Map<V> {
    type Item = (usize, &'a V);
    type IntoIter = Iter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut Map<V> {
    type Item = (usize, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V> Map<V> {
    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter() non-initialized Map");
        Iter { current: self.first_used, head: self.head, _marker: PhantomData }
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
    pub fn iter_mut(&mut self) -> IterMut<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't iter_mut() non-initialized Map");
        IterMut { current: self.first_used, head: self.head, _marker: PhantomData }
    }

    /// Make an iterator over all items.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<'_, V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't into_iter() non-initialized Map");
        IntoIter::new(self.iter())
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
    assert_eq!("foo", *iter.next().unwrap().1);
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
    for (k, _v) in &m {
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
fn iterate_mutably_without_function() {
    let mut m: Map<u64> = Map::with_capacity_none(16);
    m.insert(0, 1);
    m.insert(1, 2);
    for (_, value) in &mut m {
        *value *= 2;
    }
    assert_eq!(Some(&2), m.get(0));
    assert_eq!(Some(&4), m.get(1));
}

#[test]
fn iterate_and_mutate() {
    let mut m: Map<u64> = Map::with_capacity_none(16);
    m.insert(0, 16);
    m.insert(1, 32);
    m.insert(2, 64);
    for (_, v) in &mut m {
        *v += 1;
    }
    let mut sum = 0;
    for v in m.values() {
        sum += v;
    }
    assert_eq!(115, sum);
}

#[test]
fn iterate_non_clone_values() {
    struct NoClone {
        id: usize,
    }

    let mut map: Map<NoClone> = Map::with_capacity_none(4);
    map.insert(0, NoClone { id: 1 });
    map.insert(1, NoClone { id: 3 });

    for (idx, value) in &mut map {
        value.id += idx;
    }

    let mut seen = [false; 2];
    let mut sum = 0;
    for (idx, value) in &map {
        sum += value.id;
        seen[idx] = true;
    }
    assert!(seen.iter().all(|flag| *flag));
    assert_eq!(5, sum);

    let mut borrowed_sum = 0;
    for (_, value) in &map {
        borrowed_sum += value.id;
    }
    assert_eq!(5, borrowed_sum);

    let mut owned_sum = 0;
    for (_, value) in map.into_iter() {
        owned_sum += value.id;
    }
    assert_eq!(5, owned_sum);
}

#[test]
fn iterator_skips_nodes_without_values() {
    let mut map: Map<u32> = Map::with_capacity_none(2);
    map.insert(0, 10);
    map.insert(1, 20);

    unsafe {
        let removed = &mut *map.head.add(0);
        removed.replace_value(None);
        removed.update_next(NodeId::new(NodeId::UNDEF));
        removed.update_prev(NodeId::new(NodeId::UNDEF));

        map.first_free = NodeId::new(0);
        map.first_used = NodeId::new(1);
        map.len = 1;

        let remaining = &mut *map.head.add(1);
        remaining.update_prev(NodeId::new(NodeId::UNDEF));
        remaining.update_next(NodeId::new(NodeId::UNDEF));
    }

    let collected: Vec<_> = map.iter().map(|(k, v)| (k, *v)).collect();
    assert_eq!(collected, vec![(1, 20)]);
}

#[test]
fn iterator_mut_skips_nodes_without_values() {
    let mut map: Map<u32> = Map::with_capacity_none(3);
    map.insert(0, 5);
    map.insert(1, 15);
    map.insert(2, 25);

    unsafe {
        let removed = &mut *map.head.add(0);
        removed.replace_value(None);
        removed.update_next(NodeId::new(NodeId::UNDEF));
        removed.update_prev(NodeId::new(NodeId::UNDEF));

        map.first_free = NodeId::new(0);
        map.first_used = NodeId::new(1);
        map.len = 2;

        let middle = &mut *map.head.add(1);
        middle.update_prev(NodeId::new(NodeId::UNDEF));
        middle.update_next(NodeId::new(2));

        let tail = &mut *map.head.add(2);
        tail.update_prev(NodeId::new(1));
        tail.update_next(NodeId::new(NodeId::UNDEF));
    }

    for (index, value) in &mut map {
        let Ok(offset) = u32::try_from(index) else {
            panic!("index {index} overflows u32");
        };
        *value += offset;
    }

    assert_eq!(map.get(0), None);
    assert_eq!(map.get(1).copied(), Some(16));
    assert_eq!(map.get(2).copied(), Some(27));
}
