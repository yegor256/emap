// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, Node, NodeId};
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
    fn with_capacity(cap: usize) -> Self {
        unsafe {
            let layout = Layout::array::<Node<V>>(cap).unwrap();
            let ptr = alloc(layout);
            Self {
                first_free: NodeId::new(NodeId::UNDEF),
                first_used: NodeId::new(NodeId::UNDEF),
                layout,
                head: ptr.cast(),
                len: 0,
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
        m.init_with_none();
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    fn init_with_none(&mut self) {
        let mut ptr = self.head;
        let cap = self.capacity();
        self.first_free = NodeId::new(0);
        for i in 0..cap {
            let free_next = if i + 1 == cap { NodeId::UNDEF } else { i + 1 };
            let free_prev = if i == 0 { NodeId::UNDEF } else { i - 1 };
            let node = Node::new(free_next, free_prev, None);
            unsafe {
                std::ptr::write(ptr, node);
                ptr = ptr.add(1);
            }
        }
    }

    /// Return capacity.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.layout.size() / mem::size_of::<Node<V>>()
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
        m.len = cap;
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }

    #[inline]
    pub fn init_with_some(&mut self, cap: usize, v: V) {
        let mut ptr = self.head;
        self.first_used = NodeId::new(0);
        for i in 0..cap {
            let free_next = if i + 1 == cap { NodeId::UNDEF } else { i + 1 };
            let free_prev = if i == 0 { NodeId::UNDEF } else { i - 1 };
            let node = Node::new(free_next, free_prev, Some(v.clone()));
            unsafe {
                std::ptr::write(ptr, node);
                ptr = ptr.add(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn calculates_size_of_memory() {
        let m1: Map<u8> = Map::with_capacity_none(8);
        assert_eq!(24 * 8, m1.layout.size());
        let m2: Map<bool> = Map::with_capacity_none(8);
        assert_eq!(24 * 8, m2.layout.size());
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
}
