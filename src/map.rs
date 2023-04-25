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

use crate::Item::{Absent, Present};
use crate::{IntoIter, IntoValues, Iter, Map, Values};
use std::marker::PhantomData;
use std::ptr;

impl<V: Clone> Map<V> {
    /// Make an iterator over all items.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<V> {
        Iter {
            max: self.max,
            pos: 0,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make an iterator over all items.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<V> {
        IntoIter {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }

    /// Make an iterator over all values.
    #[inline]
    #[must_use]
    pub const fn values(&self) -> Values<V> {
        Values {
            max: self.max,
            pos: 0,
            head: self.head,
            _marker: PhantomData,
        }
    }

    /// Make an iterator over all items.
    #[inline]
    #[must_use]
    pub const fn into_values(&self) -> IntoValues<V> {
        IntoValues {
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
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let mut busy = 0;
        for i in 0..self.max {
            if self.get(&i).is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Does the map contain this key?
    #[inline]
    #[must_use]
    pub fn contains_key(&self, k: &usize) -> bool {
        self.get(k).is_some()
    }

    /// Remove by key.
    #[inline]
    pub fn remove(&mut self, k: &usize) {
        unsafe {
            ptr::write(self.head.add(*k), Absent);
        }
    }

    /// Push to the rightmost position and return the key.
    #[inline]
    pub fn push(&mut self, v: V) -> usize {
        self.insert(self.max, v);
        self.max
    }

    /// Insert a single pair into the map.
    #[inline]
    pub fn insert(&mut self, k: usize, v: V) {
        unsafe {
            *self.head.add(k) = Present(v);
        }
        if self.max <= k {
            self.max = k + 1;
        }
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn get(&self, k: &usize) -> Option<&V> {
        let item = unsafe { self.head.add(*k) };
        if let Present(p) = unsafe { &*item } {
            return Some(p);
        }
        None
    }

    /// Get a mutable reference to a single value.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, k: &usize) -> Option<&mut V> {
        let item = unsafe { &mut *(self.head.add(*k)) };
        if let Present(p) = item {
            return Some(p);
        }
        None
    }

    /// Remove all items from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        self.max = 0;
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F: Fn(&usize, &V) -> bool>(&mut self, f: F) {
        for i in 0..self.max {
            let item = self.get_mut(&i);
            if let Some(p) = item {
                if !f(&i, p) {
                    unsafe {
                        *(self.head.add(i)) = Absent;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn insert_and_check_length() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "zero");
    assert_eq!(1, m.len());
    m.insert(1, "first");
    assert_eq!(2, m.len());
    m.insert(1, "first");
    assert_eq!(2, m.len());
    Ok(())
}

#[test]
fn empty_length() -> Result<()> {
    let m: Map<u32> = Map::with_capacity(16);
    assert_eq!(0, m.len());
    Ok(())
}

#[test]
fn is_empty_check() -> Result<()> {
    let mut m: Map<u32> = Map::with_capacity(16);
    assert!(m.is_empty());
    m.insert(0, 42);
    assert!(!m.is_empty());
    Ok(())
}

#[test]
fn insert_and_gets() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "zero");
    m.insert(1, "one");
    assert_eq!("one", *m.get(&1).unwrap());
    Ok(())
}

#[test]
fn insert_and_gets_mut() -> Result<()> {
    let mut m: Map<[i32; 3]> = Map::with_capacity(16);
    m.insert(0, [1, 2, 3]);
    let a = m.get_mut(&0).unwrap();
    a[0] = 500;
    assert_eq!(500, m.get(&0).unwrap()[0]);
    Ok(())
}

#[test]
fn checks_key() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    assert!(m.contains_key(&0));
    m.insert(8, "");
    m.remove(&8);
    assert!(!m.contains_key(&8));
    Ok(())
}

#[test]
fn gets_missing_key() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(&1);
    assert!(m.get(&1).is_none());
    Ok(())
}

#[test]
fn mut_gets_missing_key() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(&1);
    assert!(m.get_mut(&1).is_none());
    Ok(())
}

#[test]
fn removes_simple_pair() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    m.remove(&0);
    m.remove(&1);
    assert!(m.get(&0).is_none());
    Ok(())
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Foo {
    v: [u32; 3],
}

#[test]
fn insert_struct() -> Result<()> {
    let mut m: Map<Foo> = Map::with_capacity(16);
    let foo = Foo { v: [1, 2, 100] };
    m.insert(0, foo);
    assert_eq!(100, m.into_iter().next().unwrap().1.v[2]);
    Ok(())
}

#[test]
fn large_map_in_heap() -> Result<()> {
    let m: Box<Map<[u64; 10]>> = Box::new(Map::with_capacity(16));
    assert_eq!(0, m.len());
    Ok(())
}

#[test]
fn clears_it_up() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(7, "one");
    m.clear();
    assert_eq!(0, m.len());
    Ok(())
}
