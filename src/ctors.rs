// SPDX-FileCopyrightText: Copyright (c) 2023 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::func::calc_capacity_ub;
use crate::Map;
use std::alloc::{alloc, dealloc, Layout};

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
        let size = cap;

        unsafe {
            let layout = Layout::array::<Option<V>>(cap).unwrap();
            let ptr = alloc(layout);

            Self {
                max: 0,
                size,
                layout,
                head: ptr.cast(),
                #[cfg(debug_assertions)]
                initialized: false,
            }
        }
    }

    /// Make it, while allocating memory with alignment alignment
    /// and a size not less than necessary, but at the same time
    /// a multiple of `reg_size`
    ///
    /// # Panics
    ///
    /// May panic if out of memory.
    fn with_capacity_align(size: usize, align: usize, reg_size: usize) -> Self {
        // align must be power of two
        assert!(align != 0 && align.is_power_of_two());
        let align = align.max(std::mem::align_of::<Option<V>>());
        let cap = calc_capacity_ub(std::mem::size_of::<Option<V>>() * size, reg_size);
        unsafe {
            let layout = Layout::from_size_align(cap, align).unwrap();
            let ptr = alloc(layout);
            assert!(ptr as usize % align == 0);
            Self {
                max: 0,
                size,
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
        self.size
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
        for k in 0..cap {
            m.insert(k, v.clone());
        }
        #[cfg(debug_assertions)]
        {
            m.initialized = true;
        }
        m
    }
}

macro_rules! impl_with_capacity_some_sse {
    ($type:ty) => {
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        impl Map<$type> {
            /// Make it and prepare all keys with some value set using sse.
            ///
            /// This method is implemented for primitive types and allows you to
            /// use sse2 vector registers for filling. It works faster than
            /// `with_capacity_some`.
            ///
            /// # Panics
            ///
            /// May panic if out of memory.
            #[inline]
            #[must_use]
            pub fn with_capacity_some_sse(cap: usize, value: $type) -> Self {
                let mut m = Self::with_capacity_align(cap, 16, 16);
                m.init_sse(value);
                #[cfg(debug_assertions)]
                {
                    m.initialized = true;
                }
                m
            }
        }

        #[cfg(not(all(target_arch = "x86_64", target_feature = "sse2")))]
        impl Map<$type> {
            pub fn with_capacity_some_sse(cap: usize, value: $type) -> Self {
                log::warn!("SSE2 not available, using fallback");
                Self::with_capacity_some(cap, value)
            }
        }
    };
}

impl_with_capacity_some_sse!(i8);
impl_with_capacity_some_sse!(i16);
impl_with_capacity_some_sse!(i32);
impl_with_capacity_some_sse!(u8);
impl_with_capacity_some_sse!(u16);
impl_with_capacity_some_sse!(u32);

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
}

#[test]
fn init_with_some_sse_neg() {
    let value = -13131_i32;
    let size = 127;
    let m: Map<i32> = Map::<i32>::with_capacity_some_sse(size, value);

    for i in 0..size {
        assert_eq!(*m.get(i).unwrap(), value);
    }
    assert_eq!(m.len(), size);
    assert_eq!(m.capacity(), size);
}

#[cfg(test)]
macro_rules! test_sse_impl {
    ($type:ty, $value:expr) => {
        paste::item! {
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            #[test]
            fn [<test_sse_ $type>]() {
                let sizes: [usize; 8] = [1, 2, 3, 4, 5, 13, 16, 25];
                for size in sizes {
                    let m: Map<$type> = Map::<$type>::with_capacity_some_sse(size, $value);

                    for i in 0..size {
                        assert_eq!(*m.get(i).unwrap(), $value);
                    }
                    assert_eq!(m.len(), size);
                    assert_eq!(m.capacity(), size);
                }
            }
        }
    };
}

#[cfg(test)]
test_sse_impl!(i8, 42_i8);
#[cfg(test)]
test_sse_impl!(i16, 1234_i16);
#[cfg(test)]
test_sse_impl!(i32, 0x11223344_i32);
#[cfg(test)]
test_sse_impl!(u8, 0xFF_u8);
#[cfg(test)]
test_sse_impl!(u16, 0xABCD_u16);
#[cfg(test)]
test_sse_impl!(u32, 0xDEADBEEF_u32);
