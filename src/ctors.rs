// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, Node, NodeId};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::mem;
use std::ptr;
use std::ptr::NonNull;

impl<V> Drop for Map<V> {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        if self.initialized {
            unsafe { self.drop_all_initialized_nodes() };
        } else {
            unsafe { self.drop_used_nodes() };
        }

        #[cfg(not(debug_assertions))]
        unsafe {
            self.drop_used_nodes();
        }

        if self.layout.size() != 0 {
            unsafe {
                dealloc(self.head.cast(), self.layout);
            }
        }
    }
}

impl<V> Map<V> {
    /// Create a map with the given capacity without initializing values.
    ///
    /// # Panics
    ///
    /// Panics on allocation error.
    #[inline]
    #[must_use]
    fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<Node<V>>(cap).expect("invalid layout");
        if layout.size() == 0 {
            return Self {
                first_free: NodeId::new(NodeId::UNDEF),
                first_used: NodeId::new(NodeId::UNDEF),
                layout,
                head: NonNull::<Node<V>>::dangling().as_ptr(),
                len: 0,
                #[cfg(debug_assertions)]
                initialized: false,
            };
        }
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
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

    /// Create a map and initialize all slots with `None`.
    ///
    /// More expensive than `with_capacity` since it initializes every slot.
    ///
    /// # Panics
    ///
    /// Panics on allocation error.
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

    /// Initialize all slots as free and link the free-list.
    #[inline]
    fn init_with_none(&mut self) {
        let cap = self.capacity();
        self.first_free = NodeId::new(if cap == 0 { NodeId::UNDEF } else { 0 });
        let mut p = self.head;
        for i in 0..cap {
            let free_next = if i + 1 == cap { NodeId::UNDEF } else { i + 1 };
            let free_prev = if i == 0 { NodeId::UNDEF } else { i - 1 };
            let node = Node::new(free_next, free_prev, None);
            unsafe {
                ptr::write(p, node);
                p = p.add(1);
            }
        }
    }

    /// Return the map capacity.
    #[inline]
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.layout.size() / mem::size_of::<Node<V>>()
    }
}

impl<V: Clone> Map<V> {
    /// Create a map and fill all slots with the provided value.
    ///
    /// # Panics
    ///
    /// Panics on allocation error or if `Clone` panics during initialization.
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

    /// Fill all slots with `Some(v.clone())` and build the used-list.
    ///
    /// # Panics
    ///
    /// Panics if `cap` exceeds [`Map::capacity`].
    #[inline]
    pub fn init_with_some(&mut self, cap: usize, v: V) {
        let capacity = self.capacity();
        assert!(
            cap <= capacity,
            "initialization bound {cap} exceeds capacity {capacity}",
        );
        let mut previous_used = NodeId::new(NodeId::UNDEF);
        self.first_free = NodeId::new(NodeId::UNDEF);
        self.first_used = NodeId::new(NodeId::UNDEF);
        self.len = 0;

        for index in 0..cap {
            let cloned = v.clone();
            let node = Node::new(NodeId::UNDEF, previous_used.get(), Some(cloned));
            unsafe {
                ptr::write(self.head.add(index), node);
            }
            if previous_used.is_def() {
                unsafe {
                    let previous_node = &mut *self.head.add(previous_used.get());
                    previous_node.update_next(NodeId::new(index));
                }
            } else {
                self.first_used = NodeId::new(index);
            }
            previous_used = NodeId::new(index);
            self.len = index + 1;
        }
    }
}

impl<V> Map<V> {
    /// Drop values reachable through the used-list only.
    ///
    /// # Safety
    ///
    /// `self.head` must point to a valid array of `Node<V>` of size `capacity()`.
    #[inline]
    unsafe fn drop_used_nodes(&mut self) {
        let mut current = self.first_used;
        while current.is_def() {
            let node = unsafe { &mut *self.head.add(current.get()) };
            if let Some(value) = node.take_value() {
                drop(value);
            }
            current = node.get_next();
        }
    }

    /// Drop all values in initialized slots regardless of list integrity.
    ///
    /// Enabled only in debug builds to detect leaks when invariants are violated.
    ///
    /// # Safety
    ///
    /// `self.head` must point to a valid array of `Node<V>` of size `capacity()`.
    #[cfg(debug_assertions)]
    #[inline]
    unsafe fn drop_all_initialized_nodes(&mut self) {
        for index in 0..self.capacity() {
            let node = unsafe { &mut *self.head.add(index) };
            if let Some(value) = node.take_value() {
                drop(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    /// Out-of-bounds insert must panic in debug builds.
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn insert_out_of_boundary() {
        let mut m: Map<&str> = Map::with_capacity(1);
        m.insert(5, "one");
    }

    /// Out-of-bounds get must panic in debug builds.
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn get_out_of_boundary() {
        let m: Map<&str> = Map::with_capacity(1);
        m.get(5).unwrap();
    }

    /// Out-of-bounds remove must panic in debug builds.
    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn remove_out_of_boundary() {
        let mut m: Map<&str> = Map::with_capacity(1);
        m.remove(5);
    }

    /// Memory layout size must reflect Node<V> array size.
    #[test]
    fn calculates_size_of_memory() {
        let m1: Map<u8> = Map::with_capacity_none(8);
        assert_eq!(24 * 8, m1.layout.size());
        let m2: Map<bool> = Map::with_capacity_none(8);
        assert_eq!(24 * 8, m2.layout.size());
    }

    /// New map must start empty.
    #[test]
    fn makes_new_map() {
        let m: Map<&str> = Map::with_capacity_none(16);
        assert_eq!(0, m.len());
    }

    #[test]
    fn zero_capacity_map_behaves_consistently() {
        let mut map: Map<u8> = Map::with_capacity_none(0);
        assert_eq!(map.capacity(), 0);
        assert!(map.next_key().is_err());
        assert!(map.push(42).is_err());
        map.clear();
        assert_eq!(map.len(), 0);
    }

    /// Capacity must match constructor argument.
    #[test]
    fn returns_capacity() {
        let m: Map<&str> = Map::with_capacity_none(16);
        assert_eq!(16, m.capacity());
    }

    /// Contains check must return false for uninitialized slot.
    #[test]
    fn with_init() {
        let m: Map<&str> = Map::with_capacity_none(16);
        assert!(!m.contains_key(8));
    }

    /// Dropping an empty map should be safe.
    #[test]
    fn drops_correctly() {
        let m: Map<Vec<u8>> = Map::with_capacity_none(16);
        assert_eq!(0, m.len());
        drop(m);
    }

    /// Values must be dropped on map drop.
    #[test]
    fn drops_values() {
        let mut m: Map<Rc<()>> = Map::with_capacity_none(1);
        let v = Rc::new(());
        m.insert(0, Rc::clone(&v));
        drop(m);
        assert_eq!(Rc::strong_count(&v), 1);
    }

    /// Multiple values must be dropped on map drop.
    #[test]
    fn drops_multiple_values() {
        let mut m: Map<Rc<()>> = Map::with_capacity_none(3);
        let a = Rc::new(());
        let b = Rc::new(());
        let c = Rc::new(());

        m.insert(0, Rc::clone(&a));
        m.insert(1, Rc::clone(&b));
        m.insert(2, Rc::clone(&c));

        drop(m);

        assert_eq!(Rc::strong_count(&a), 1);
        assert_eq!(Rc::strong_count(&b), 1);
        assert_eq!(Rc::strong_count(&c), 1);
    }

    /// Values must not leak after repeated insert/remove cycles.
    #[test]
    fn drops_values_after_remove_cycles() {
        let mut m: Map<Rc<()>> = Map::with_capacity_none(2);
        let value = Rc::new(());

        for _ in 0..3 {
            m.insert(0, Rc::clone(&value));
            m.remove(0);
            assert_eq!(Rc::strong_count(&value), 1);
        }

        m.insert(0, Rc::clone(&value));
        drop(m);
        assert_eq!(Rc::strong_count(&value), 1);
    }

    #[derive(Clone, PartialEq, Eq)]
    struct Foo {
        pub t: i32,
    }

    /// Struct values must initialize as expected.
    #[test]
    fn init_with_structs() {
        let m: Map<Foo> = Map::with_capacity_none(16);
        assert_eq!(16, m.capacity());
    }

    /// `with_capacity_some` must fill all slots and set len accordingly.
    #[test]
    fn init_with_some() {
        let m: Map<Foo> = Map::with_capacity_some(16, Foo { t: 42 });
        assert_eq!(16, m.capacity());
        assert_eq!(16, m.len());
    }

    /// Zero capacity must be supported.
    #[test]
    fn init_with_empty() {
        let m: Map<Foo> = Map::with_capacity_some(0, Foo { t: 42 });
        assert_eq!(0, m.capacity());
        assert_eq!(0, m.len());
    }

    /// Oversized initialization bounds must panic instead of causing UB.
    #[test]
    #[should_panic(expected = "initialization bound")]
    fn init_with_some_panics_when_bound_exceeds_capacity() {
        let mut map: Map<Foo> = Map::with_capacity(1);
        map.init_with_some(2, Foo { t: 1 });
    }

    /// Partial initialization that panics must still be drop-safe.
    #[test]
    fn drop_after_boundary_panic_without_initialization() {
        let result = catch_unwind(AssertUnwindSafe(|| {
            let mut map: Map<&str> = Map::with_capacity(1);
            map.insert(1, "boom");
        }));
        assert!(result.is_err());
    }

    #[derive(Debug)]
    struct PanicOnClone {
        clones: Rc<Cell<usize>>,
        active: Rc<Cell<usize>>,
        panic_after: usize,
    }

    impl PanicOnClone {
        fn new(panic_after: usize, clones: Rc<Cell<usize>>, active: Rc<Cell<usize>>) -> Self {
            active.set(active.get() + 1);
            Self {
                clones,
                active,
                panic_after,
            }
        }
    }

    impl Clone for PanicOnClone {
        fn clone(&self) -> Self {
            if self.clones.get() >= self.panic_after {
                panic!("clone limit reached");
            }
            self.clones.set(self.clones.get() + 1);
            self.active.set(self.active.get() + 1);
            Self {
                clones: Rc::clone(&self.clones),
                active: Rc::clone(&self.active),
                panic_after: self.panic_after,
            }
        }
    }

    impl Drop for PanicOnClone {
        fn drop(&mut self) {
            let current = self.active.get();
            self.active.set(current.saturating_sub(1));
        }
    }

    /// If cloning panics during `with_capacity_some`, no values must leak.
    #[test]
    fn drop_after_partial_with_capacity_some_panics() {
        let clones = Rc::new(Cell::new(0));
        let active = Rc::new(Cell::new(0));

        let result = catch_unwind(AssertUnwindSafe(|| {
            let value = PanicOnClone::new(1, Rc::clone(&clones), Rc::clone(&active));
            let _ = Map::with_capacity_some(3, value);
        }));

        assert!(result.is_err());
        assert_eq!(clones.get(), 1);
        assert_eq!(active.get(), 0);
    }
}
