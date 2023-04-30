// Copyright (c) 2023 Yegor Bugayenko
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::{IntoIter, IntoValues, Iter, Keys, Map, Values};
use std::marker::PhantomData;
use std::ptr;

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
            max: self.max,
            pos: 0,
            head: self.head,
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
        let mut busy = 0;
        for i in 0..self.max {
            if self.get(i).is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Does the map contain this key?
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to refer to they key that is outside
    /// of the boundary of this map. It will not return `None`, it will panic.
    /// However, in "release" mode it will not panic, but will lead to
    /// undefined behavior.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn contains_key(&self, k: usize) -> bool {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "Over the boundary");
        matches!(unsafe { &*self.head.add(k) }, Some(_))
    }

    /// Remove by key.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to refer to they key that is outside
    /// of the boundary of this map. It will not return `None`, it will panic.
    /// However, in "release" mode it will not panic, but will lead to
    /// undefined behavior.
    #[inline]
    pub fn remove(&mut self, k: usize) {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "Over the boundary");
        unsafe {
            ptr::write(self.head.add(k), None);
        }
    }

    /// Push to the rightmost position and return the key.
    #[inline]
    pub fn push(&mut self, v: V) -> usize {
        let k = self.next_key();
        self.insert(k, v);
        k
    }

    /// Insert a single pair into the map.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to refer to they key that is outside
    /// of the boundary of this map. It will not return `None`, it will panic.
    /// However, in "release" mode it will not panic, but will lead to
    /// undefined behavior.
    #[inline]
    pub fn insert(&mut self, k: usize, v: V) {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "Over the boundary");
        unsafe {
            ptr::write(self.head.add(k), Some(v));
        }
        if self.max <= k {
            self.max = k + 1;
        }
    }

    /// Get a reference to a single value.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to refer to they key that is outside
    /// of the boundary of this map. It will not return `None`, it will panic.
    /// However, in "release" mode it will not panic, but will lead to
    /// undefined behavior.
    #[inline]
    #[must_use]
    pub fn get(&self, k: usize) -> Option<&V> {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "Over the boundary");
        unsafe { &*self.head.add(k) }.as_ref()
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Panics
    ///
    /// It may panic if you attempt to refer to they key that is outside
    /// of the boundary of this map. It will not return `None`, it will panic.
    /// However, in "release" mode it will not panic, but will lead to
    /// undefined behavior.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, k: usize) -> Option<&mut V> {
        #[cfg(debug_assertions)]
        assert!(k < self.capacity(), "Over the boundary");
        unsafe { &mut *(self.head.add(k)) }.as_mut()
    }

    /// Remove all items from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        self.max = 0;
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    pub fn retain<F: Fn(&usize, &V) -> bool>(&mut self, f: F) {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't do retain() on non-initialized Map");
        for i in 0..self.max {
            if let Some(p) = self.get_mut(i) {
                if !f(&i, p) {
                    unsafe {
                        ptr::write(self.head.add(i), None);
                    }
                }
            }
        }
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
fn pushes_into() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(0, m.push("one"));
    assert_eq!(1, m.push("two"));
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn insert_out_of_boundary() {
    let mut m: Map<&str> = Map::with_capacity(1);
    m.insert(5, "one");
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn get_out_of_boundary() {
    let m: Map<&str> = Map::with_capacity(1);
    m.get(5).unwrap();
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn remove_out_of_boundary() {
    let mut m: Map<&str> = Map::with_capacity(1);
    m.remove(5);
}
