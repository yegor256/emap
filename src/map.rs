// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, MapFullError, NodeId};

impl<V> Map<V> {
    /// Is it empty?
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the total number of items inside.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't do len() on non-initialized Map");
        self.len
    }

    /// Does the map contain this key?
    ///
    /// # Panics
    ///
    /// Panics if k is out of bound.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn contains_key(&self, k: usize) -> bool {
        self.assert_boundaries(k);
        unsafe { self.contains_key_unchecked(k) }
    }

    /// Does the map contain this key?
    ///
    /// # Safety
    ///
    /// In debug builds, this may panic if `k` is out of bounds, but in release builds,
    /// passing an out-of-bounds `k` will result in undefined behavior.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub unsafe fn contains_key_unchecked(&self, k: usize) -> bool {
        self.assert_boundaries_debug(k);
        unsafe { &*self.head.add(k) }.is_some()
    }

    /// Remove by key.
    ///
    /// # Panics
    ///
    /// Panics if k is out of bound.
    #[inline]
    pub fn remove(&mut self, k: usize) {
        self.assert_boundaries(k);
        unsafe { self.remove_unchecked(k) }
    }

    /// Remove by key.
    ///
    /// # Safety
    ///
    /// In debug builds, this may panic if `k` is out of bounds, but in release builds,
    /// passing an out-of-bounds `k` will result in undefined behavior.
    #[inline]
    pub unsafe fn remove_unchecked(&mut self, k: usize) {
        self.assert_boundaries_debug(k);
        let node = unsafe { &mut *self.head.add(k) };

        if node.is_none() {
            return;
        }

        let prev_used = node.get_prev();
        let next_used = node.get_next();

        // 1. remove node from element list
        if prev_used.is_undef() {
            self.first_used = next_used;
        } else {
            let prev_node = unsafe { &mut *self.head.add(prev_used.get()) };
            prev_node.update_next(next_used);
        }

        if next_used.is_def() {
            let next_node = unsafe { &mut *self.head.add(next_used.get()) };
            next_node.update_prev(prev_used);
        }

        // 2. insert node into free list
        node.update_next(self.first_free);
        node.update_prev(NodeId::new(NodeId::UNDEF));

        if self.first_free.is_def() {
            let next_free = unsafe { &mut *self.head.add(self.first_free.get()) };
            next_free.update_prev(NodeId::new(k));
        }

        self.first_free = NodeId::new(k);
        let previous = node.replace_value(None);
        drop(previous);
        self.len -= 1;
    }

    /// Push to the rightmost position and return the key.
    ///
    /// # Errors
    ///
    /// Returns [`MapFullError`] if the map has no free slots left.
    ///
    /// # Examples
    ///
    /// ```
    /// use emap::{Map, MapFullError};
    /// let mut map: Map<&str> = Map::with_capacity_none(1);
    /// assert_eq!(map.push("hello"), Ok(0));
    /// assert_eq!(map.push("world"), Err(MapFullError));
    /// ```
    #[inline]
    pub fn push(&mut self, v: V) -> Result<usize, MapFullError> {
        self.try_push(v)
    }

    /// Try to push to the rightmost position and return the key.
    ///
    /// This is equivalent to [`Map::push`] and is retained for callers that
    /// prefer the explicit "try" naming convention.
    ///
    /// # Errors
    ///
    /// Returns [`MapFullError`] if the map has no free slots left.
    ///
    /// # Examples
    ///
    /// ```
    /// use emap::Map;
    /// let mut map: Map<&str> = Map::with_capacity_none(1);
    /// assert_eq!(map.try_push("hello"), Ok(0));
    /// assert!(map.try_push("world").is_err());
    /// ```
    #[inline]
    pub fn try_push(&mut self, v: V) -> Result<usize, MapFullError> {
        let key = self.try_next_key()?;
        self.insert(key, v);
        Ok(key)
    }

    /// Insert a single pair into the map.
    ///
    /// # Panics
    ///
    /// Panics if k is out of bound.
    pub fn insert(&mut self, k: usize, v: V) {
        self.assert_boundaries(k);
        unsafe { self.insert_unchecked(k, v) }
    }

    /// Insert a single pair into the map.
    ///
    /// # Safety
    ///
    /// In debug builds, this may panic if `k` is out of bounds, but in release builds,
    /// passing an out-of-bounds `k` will result in undefined behavior.
    #[inline]
    pub unsafe fn insert_unchecked(&mut self, k: usize, v: V) {
        self.assert_boundaries_debug(k);
        let node = unsafe { &mut *self.head.add(k) };

        if node.is_some() {
            let previous = node.replace_value(Some(v));
            drop(previous);
            return;
        }

        // 1. remove node from free list
        if node.get_prev().is_undef() {
            self.first_free = node.get_next();
        } else {
            let prev_free = unsafe { &mut *self.head.add(node.get_prev().get()) };
            prev_free.update_next(node.get_next());
        }

        if node.get_next().is_def() {
            let next_free = unsafe { &mut *self.head.add(node.get_next().get()) };
            next_free.update_prev(node.get_prev());
        }

        // 2. insert node into element list
        node.update_next(self.first_used);
        node.update_prev(NodeId::new(NodeId::UNDEF));

        if self.first_used.is_def() {
            let next_used = unsafe { &mut *self.head.add(self.first_used.get()) };
            next_used.update_prev(NodeId::new(k));
        }

        self.first_used = NodeId::new(k);
        let previous = node.replace_value(Some(v));
        drop(previous);
        self.len += 1;
    }

    /// Get a reference to a single value.
    ///
    /// # Panics
    ///
    /// Panics if k is out of bound.
    #[must_use]
    pub fn get(&self, k: usize) -> Option<&V> {
        self.assert_boundaries(k);
        unsafe { self.get_unchecked(k) }
    }

    /// Get a reference to a single value.
    ///
    /// # Safety
    ///
    /// In debug builds, this may panic if `k` is out of bounds, but in release builds,
    /// passing an out-of-bounds `k` will result in undefined behavior.
    #[inline]
    #[must_use]
    pub unsafe fn get_unchecked(&self, k: usize) -> Option<&V> {
        self.assert_boundaries_debug(k);
        unsafe { &*self.head.add(k) }.get()
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Panics
    ///
    /// Panics if k is out of bound.
    pub fn get_mut(&mut self, k: usize) -> Option<&mut V> {
        self.assert_boundaries(k);
        unsafe { self.get_mut_unchecked(k) }
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Safety
    ///
    /// In debug builds, this may panic if `k` is out of bounds, but in release builds,
    /// passing an out-of-bounds `k` will result in undefined behavior.
    #[inline]
    #[must_use]
    pub unsafe fn get_mut_unchecked(&mut self, k: usize) -> Option<&mut V> {
        self.assert_boundaries_debug(k);
        unsafe { &mut *(self.head.add(k)) }.get_mut()
    }

    /// Remove all items from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        while self.first_used.is_def() {
            self.remove(self.first_used.get());
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// The predicate is applied to each occupied key/value pair. If the predicate
    /// returns `false` for a given pair, that entry is removed. The iteration
    /// order follows the internal key sequence provided by `keys()`.
    ///
    /// The predicate receives the key by reference and a mutable reference to the
    /// value. Any mutation performed by the predicate is applied in place prior
    /// to deciding whether the entry should be removed.
    ///
    /// # Panics
    ///
    /// Panics in debug mode if the map is not initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// # use emap::Map;
    /// let mut m: Map<i32> = Map::with_capacity_none(4);
    /// m.insert(0, 10);
    /// m.insert(1, 20);
    /// m.insert(2, 30);
    /// m.retain(|_, v| {
    ///     *v += 5;
    ///     *v >= 25
    /// });
    /// assert_eq!(m.len(), 2);
    /// assert!(m.get(0).is_none());
    /// assert_eq!(*m.get(1).unwrap(), 25);
    /// assert_eq!(*m.get(2).unwrap(), 35);
    /// ```
    #[inline]
    pub fn retain<F: FnMut(&usize, &mut V) -> bool>(&mut self, mut f: F) {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't do retain() on non-initialized Map");
        for i in self.keys() {
            let mut should_remove = false;
            {
                if let Some(value) = self.get_mut(i) {
                    should_remove = !f(&i, value);
                }
            }
            if should_remove {
                self.remove(i);
            }
        }
    }

    /// Check the boundary condition only in debug mode.
    #[inline]
    #[allow(unused_variables)]
    fn assert_boundaries_debug(&self, k: usize) {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "The key {k} is over the boundary {}", self.capacity());
    }

    /// Check the boundary condition.
    #[inline]
    #[allow(unused_variables)]
    fn assert_boundaries(&self, k: usize) {
        assert!(k < self.capacity(), "The key {k} is over the boundary {}", self.capacity());
    }
}

#[test]
fn insert_and_check_length() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "zero");
    assert_eq!(1, m.len());
    m.insert(1, "first");
    assert_eq!(2, m.len());
    m.insert(1, "first");
    assert_eq!(2, m.len());
}

#[test]
fn empty_length() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert_eq!(0, m.len());
}

#[test]
fn is_empty_check() {
    let mut m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.is_empty());
    m.insert(0, 42);
    assert!(!m.is_empty());
}

#[test]
fn insert_and_gets() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "zero");
    m.insert(1, "one");
    assert_eq!("one", *m.get(1).unwrap());
}

#[test]
fn insert_and_gets_mut() {
    let mut m: Map<[i32; 3]> = Map::with_capacity_none(16);
    m.insert(0, [1, 2, 3]);
    let a = m.get_mut(0).unwrap();
    a[0] = 500;
    assert_eq!(500, m.get(0).unwrap()[0]);
}

#[test]
fn checks_key() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    assert!(m.contains_key(0));
    m.insert(8, "");
    m.remove(8);
    assert!(!m.contains_key(8));
}

#[test]
fn gets_missing_key() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(1);
    assert!(m.get(1).is_none());
}

#[test]
fn mut_gets_missing_key() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(1);
    assert!(m.get_mut(1).is_none());
}

#[test]
fn removes_simple_pair() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.remove(0);
    m.remove(1);
    assert!(m.get(0).is_none());
}

#[test]
fn replacing_value_drops_old_reference() {
    use std::rc::Rc;

    let mut m: Map<Rc<()>> = Map::with_capacity_none(2);
    let original = Rc::new(());
    let replacement = Rc::new(());

    m.insert(0, Rc::clone(&original));
    assert_eq!(Rc::strong_count(&original), 2);

    m.insert(0, Rc::clone(&replacement));
    assert_eq!(Rc::strong_count(&original), 1);
    assert_eq!(Rc::strong_count(&replacement), 2);

    drop(m);

    assert_eq!(Rc::strong_count(&replacement), 1);
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Foo {
    v: [u32; 3],
}

#[test]
fn insert_struct() {
    let mut m: Map<Foo> = Map::with_capacity_none(16);
    let foo = Foo { v: [1, 2, 100] };
    m.insert(0, foo);
    assert_eq!(100, m.into_iter().next().unwrap().1.v[2]);
}

#[test]
fn large_map_in_heap() {
    let m: Box<Map<[u64; 10]>> = Box::new(Map::with_capacity_none(16));
    assert_eq!(0, m.len());
}

#[test]
fn clears_it_up() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(7, "one");
    m.clear();
    assert_eq!(0, m.len());
}

#[test]
fn retain_allows_mutation() {
    let mut m: Map<i32> = Map::with_capacity_none(4);
    m.insert(0, 10);
    m.insert(1, 20);
    m.insert(2, 30);

    m.retain(|key, value| {
        if *key % 2 == 0 {
            *value += 5;
        } else {
            *value += 7;
        }
        *value >= 25
    });

    assert_eq!(2, m.len());
    assert!(m.get(0).is_none());
    assert_eq!(Some(&27), m.get(1));
    assert_eq!(Some(&35), m.get(2));
}

#[test]
fn pushes_into() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(Ok(0), m.push("one"));
    assert_eq!(Ok(1), m.push("two"));
}

#[test]
fn push_updates_length() {
    let mut m: Map<&str> = Map::with_capacity_none(8);
    let values = ["one", "two", "three", "four"];

    for (index, value) in values.into_iter().enumerate() {
        assert_eq!(Ok(index), m.push(value));
        assert_eq!(index + 1, m.len());
    }
}

#[test]
fn push_reports_error_on_full_map() {
    let mut map: Map<&str> = Map::with_capacity_none(1);
    assert_eq!(Ok(0), map.push("alpha"));
    assert_eq!(Err(MapFullError), map.push("beta"));
}

#[test]
fn try_push_provides_next_slot() {
    let mut map: Map<&str> = Map::with_capacity_none(2);
    assert_eq!(Ok(0), map.try_push("alpha"));
    assert_eq!(Ok(1), map.try_push("beta"));
    assert!(map.try_push("gamma").is_err());
}

#[test]
fn try_push_does_not_modify_on_error() {
    let mut map: Map<&str> = Map::with_capacity_none(1);
    assert!(map.try_push("alpha").is_ok());
    assert!(map.try_push("beta").is_err());
    assert_eq!(Some(&"alpha"), map.get(0));
    assert_eq!(1, map.len());
}

#[test]
fn insert_in_free_list_head() {
    let mut m: Map<i32> = Map::with_capacity_none(3);
    m.insert(0, 1);
    assert_eq!(m.next_key(), Ok(1));
    m.insert(1, 1);
    assert_eq!(m.next_key(), Ok(2));
}

#[test]
fn insert_in_free_list_mid() {
    let mut m: Map<i32> = Map::with_capacity_none(3);
    m.insert(1, 1);
    assert_eq!(m.next_key(), Ok(0));
}

#[test]
fn insert_in_free_list_reinsert() {
    let mut m: Map<i32> = Map::with_capacity_none(3);
    m.insert(1, 1);
    assert_eq!(m.next_key(), Ok(0));
    m.insert(1, 1);
    assert_eq!(m.next_key(), Ok(0));
    m.insert(0, 1);
    m.insert(0, 1);
    assert_eq!(m.next_key(), Ok(2));
}

#[test]
fn len_remove_insert() {
    let mut m: Map<i32> = Map::with_capacity_none(3);
    assert_eq!(m.len(), 0);
    m.clear();
    assert_eq!(m.len(), 0);
    m.insert(0, 1);
    assert_eq!(m.len(), 1);
    m.remove(0);
    assert_eq!(m.len(), 0);
    m.clear();
    assert_eq!(m.len(), 0);
}

#[test]
fn default_clear() {
    let mut m: Map<i32> = Map::with_capacity_none(3);
    m.insert(0, 0);
    m.insert(1, 1);
    m.insert(2, 2);
    m.clear();
    assert_eq!(m.len(), 0);
    m.insert(0, 0);
    m.insert(1, 1);
    m.clear();
    assert_eq!(m.len(), 0);
}

#[test]
fn clear_and_len() {
    let mut m: Map<&i32> = Map::with_capacity_none(3);
    for _ in 0..2 {
        for i in 0..3 {
            m.insert(i, &42);
        }
        m.clear();
        assert_eq!(0, m.len());
    }
}

#[test]
fn first_used_remove() {
    let mut m: Map<i32> = Map::with_capacity_none(2);
    m.insert(0, 1);
    assert_eq!(m.first_used.get(), 0);
    m.insert(1, 2);
    assert_eq!(m.first_used.get(), 1);
    assert_eq!(m.len(), 2);
    m.remove(0);
    assert_eq!(m.first_used.get(), 1);
    m.remove(1);
    assert_eq!(m.len(), 0);
    assert!(m.first_used.is_undef());
}

#[test]
fn insert_and_remove() {
    let mut m: Map<i32> = Map::with_capacity_none(7);
    assert_eq!(m.next_key(), Ok(0));
    m.insert(1, 11);
    assert_eq!(m.next_key(), Ok(0));
    m.insert(0, 10);
    assert_eq!(m.next_key(), Ok(2));
    m.insert(2, 12);
    m.insert(5, 15);
    assert_eq!(m.next_key(), Ok(3));
    m.remove(0);
    assert_eq!(m.next_key(), Ok(0));
}
