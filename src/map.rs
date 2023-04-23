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
use crate::{IntoIter, Iter, Map};

impl<V: Clone + Copy, const N: usize> Default for Map<V, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Copy, const N: usize> Map<V, N> {
    /// Make an iterator over all items.
    #[inline]
    #[must_use]
    pub const fn iter(&self) -> Iter<V, N> {
        Iter {
            filled: self.filled,
            pos: 0,
            items: &self.items,
        }
    }

    /// Make an iterator over all items.
    #[inline]
    #[must_use]
    pub const fn into_iter(&self) -> IntoIter<V, N> {
        IntoIter {
            next: self.filled,
            pos: 0,
            items: &self.items,
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
        for i in 0..self.filled {
            if self.items[i].is_some() {
                busy += 1;
            }
        }
        busy
    }

    /// Does the map contain this key?
    #[inline]
    pub const fn contains_key(&self, k: &usize) -> bool {
        self.items[*k].is_some()
    }

    /// Remove by key.
    #[inline]
    pub fn remove(&mut self, k: &usize) {
        self.items[*k] = Absent;
    }

    /// Insert a single pair into the map.
    ///
    /// # Panics
    ///
    /// It may panic if there are too many items in the map already.
    #[inline]
    pub fn insert(&mut self, k: usize, v: V) {
        self.items[k] = Present(v);
        if self.filled <= k {
            self.filled = k + 1;
        }
    }

    /// Get a reference to a single value.
    #[inline]
    #[must_use]
    pub const fn get(&self, k: &usize) -> Option<&V> {
        if let Present(p) = &self.items[*k] {
            return Some(p);
        }
        None
    }

    /// Get a mutable reference to a single value.
    ///
    /// # Panics
    ///
    /// If can't turn it into a mutable state.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, k: &usize) -> Option<&mut V> {
        if let Present(v) = &mut self.items[*k] {
            return Some(v);
        }
        None
    }

    /// Remove all items from it, but keep the space intact for future use.
    #[inline]
    pub fn clear(&mut self) {
        self.filled = 0;
    }

    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F: Fn(&usize, &V) -> bool>(&mut self, f: F) {
        for i in 0..self.filled {
            if let Present(p) = &self.items[i] {
                if !f(&i, p) {
                    self.items[i] = Absent;
                }
            }
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn insert_and_check_length() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
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
    let m: Map<u32, 10> = Map::new();
    assert_eq!(0, m.len());
    Ok(())
}

#[test]
fn is_empty_check() -> Result<()> {
    let mut m: Map<u32, 10> = Map::new();
    assert!(m.is_empty());
    m.insert(0, 42);
    assert!(!m.is_empty());
    Ok(())
}

#[test]
fn insert_and_gets() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "zero");
    m.insert(1, "one");
    assert_eq!("one", *m.get(&1).unwrap());
    Ok(())
}

#[test]
fn insert_and_gets_mut() -> Result<()> {
    let mut m: Map<[i32; 3], 10> = Map::new();
    m.insert(0, [1, 2, 3]);
    let a = m.get_mut(&0).unwrap();
    a[0] = 500;
    assert_eq!(500, m.get(&0).unwrap()[0]);
    Ok(())
}

#[test]
fn checks_key() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "one");
    assert!(m.contains_key(&0));
    assert!(!m.contains_key(&8));
    Ok(())
}

#[test]
fn gets_missing_key() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(&1);
    assert!(m.get(&1).is_none());
    Ok(())
}

#[test]
fn mut_gets_missing_key() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "one");
    m.insert(1, "one");
    m.remove(&1);
    assert!(m.get_mut(&1).is_none());
    Ok(())
}

#[test]
fn removes_simple_pair() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
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
    let mut m: Map<Foo, 8> = Map::new();
    let foo = Foo { v: [1, 2, 100] };
    m.insert(0, foo);
    assert_eq!(100, m.into_iter().next().unwrap().1.v[2]);
    Ok(())
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct Composite {
    r: Map<u8, 1>,
}

#[test]
fn insert_composite() -> Result<()> {
    let mut m: Map<Composite, 8> = Map::new();
    let c = Composite { r: Map::new() };
    m.insert(0, c);
    assert_eq!(0, m.into_iter().next().unwrap().1.r.len());
    Ok(())
}

#[derive(Clone, Copy)]
struct Bar {}

#[test]
fn large_map_in_heap() -> Result<()> {
    let m: Box<Map<[u64; 10], 10>> = Box::new(Map::new());
    assert_eq!(0, m.len());
    Ok(())
}

#[test]
fn clears_it_up() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(7, "one");
    m.clear();
    assert_eq!(0, m.len());
    Ok(())
}
