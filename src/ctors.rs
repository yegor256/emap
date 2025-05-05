// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;

impl<V> Drop for Map<V> {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.head.cast(), self.layout);
        }
    }
}

impl<V> Map<V> {
    /// Make it.
    ///
    /// # Panics
    ///
    /// May panic if out of memory.
    #[inline]
    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        unsafe {
            let layout = Layout::array::<Option<V>>(cap).unwrap();
            let ptr = alloc(layout);
            Self {
                max: 0,
                layout,
                head: ptr.cast(),
                #[cfg(debug_assertions)]
                initialized: false,
            }
        }
    }

    /// Make it and prepare all keys.
    ///
    /// This is a more expensive operation that `with_capacity`, because it has
    /// to go through all keys and fill them up with `None`.
    ///
    /// # Panics
    ///
    /// May panic if out of memory.
    #[inline]
    #[must_use]
    pub fn with_capacity_none(cap: usize) -> Self {
        let mut m = Self::with_capacity(cap);
        for k in 0..cap {
            m.remove(k);
        }
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    /// Return capacity.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.layout.size() / mem::size_of::<Option<V>>()
    }
}

impl<V: Clone> Map<V> {
    /// Make it and prepare all keys with some value set.
    ///
    /// This is a more expensive operation that `with_capacity`, because it has
    /// to go through all keys and fill them up with `Some`.
    ///
    /// # Panics
    ///
    /// May panic if out of memory.
    #[inline]
    #[must_use]
    pub fn with_capacity_some(cap: usize, v: V) -> Self {
        let mut m = Self::with_capacity(cap);
        m.init_with_some(cap, v);
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    #[inline]
    pub fn init_with_some(&mut self, cap: usize, v: V) {
        let mut ptr = self.head;
        // Write all elements except the last one
        for _ in 1..cap {
            unsafe {
                std::ptr::write(ptr, Some(v.clone()));
                ptr = ptr.add(1);
            }
        }
        if cap > 0 {
            unsafe {
                // We can write the last element directly without cloning needlessly
                std::ptr::write(ptr, Some(v));
            }
        }
        self.max = cap;
    }
}

#[test]
fn calculates_size_of_memory() {
    let m1: Map<u8> = Map::with_capacity_none(8);
    assert_eq!(16, m1.layout.size());
    let m2: Map<bool> = Map::with_capacity_none(8);
    assert_eq!(8, m2.layout.size());
}

#[test]
fn makes_new_map() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(0, m.len());
}

#[test]
fn returns_capacity() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert_eq!(16, m.capacity());
}

#[test]
fn with_init() {
    let m: Map<&str> = Map::with_capacity_none(16);
    assert!(!m.contains_key(8));
}

#[test]
fn drops_correctly() {
    let m: Map<Vec<u8>> = Map::with_capacity_none(16);
    assert_eq!(0, m.len());
}

#[test]
#[ignore]
fn drops_values() {
    use std::rc::Rc;
    let mut m: Map<Rc<()>> = Map::with_capacity(1);
    let v = Rc::new(());
    m.insert(0, Rc::clone(&v));
    drop(m);
    assert_eq!(Rc::strong_count(&v), 1);
}

#[cfg(test)]
#[derive(Clone, PartialEq, Eq)]
struct Foo {
    pub t: i32,
}

#[test]
fn init_with_structs() {
    let m: Map<Foo> = Map::with_capacity_none(16);
    assert_eq!(16, m.capacity());
}

#[test]
fn init_with_some() {
    let m: Map<Foo> = Map::with_capacity_some(16, Foo { t: 42 });
    assert_eq!(16, m.capacity());
    assert_eq!(16, m.len());
}

#[test]
fn init_with_empty() {
    let m: Map<Foo> = Map::with_capacity_some(0, Foo { t: 42 });
    assert_eq!(0, m.capacity());
    assert_eq!(0, m.len());
}
