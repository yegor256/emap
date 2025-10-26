// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! A fixed-capacity map keyed by `usize`.
//!
//! The capacity is set at construction time and does not grow.
//!
//! # Example
//!
//! ```
//! use emap::Map;
//! let mut m: Map<&str> = Map::with_capacity_none(10);
//! m.insert(0, "Hello, world!");
//! m.insert(1, "Good bye!");
//! assert_eq!(2, m.len());
//! ```
//!
//! An attempt to add an element when the map is full returns [`MapFullError`].

#![doc(html_root_url = "https://docs.rs/emap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]

mod clone;
mod ctors;
mod debug;
mod error;
mod index;
mod iterators;
mod keys;
mod map;
mod next_key;
pub mod node;
#[cfg(feature = "serde")]
mod serialization;
mod values;

pub use crate::error::MapFullError;
use crate::node::{Node, NodeId};
use std::alloc::Layout;
use std::marker::PhantomData;

/// A fixed-capacity map keyed by `usize`.
pub struct Map<V> {
    first_free: NodeId,
    first_used: NodeId,
    head: *mut Node<V>,
    layout: Layout,
    len: usize,
    #[cfg(debug_assertions)]
    initialized: bool,
}

/// Iterator over a [`Map`].
pub struct Iter<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

/// Mutable iterator over a [`Map`].
pub struct IterMut<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a mut V>,
}

/// Into-iterator over a [`Map`] that yields immutable references to values.
pub struct IntoIter<'a, V> {
    inner: Iter<'a, V>,
}

/// Borrowing iterator over values.
pub struct Values<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

/// Owning iterator over values.
pub struct IntoValues<V> {
    current: NodeId,
    head: *const Node<V>,
}

/// Iterator over keys.
pub struct Keys<V> {
    current: NodeId,
    head: *mut Node<V>,
}

#[cfg(test)]
use std::time::Instant;

/// Basic performance smoke test.
///
/// Run with:
///
/// ```text
/// cargo test --release -- perf -- --nocapture
/// ```
#[test]
fn perf() {
    let cap = 256;
    let mut m: Map<&str> = Map::with_capacity_none(cap);
    let start = Instant::now();
    for _ in 0..1000 {
        m.clear();
        for _ in 0..cap {
            assert!(m.push("Hello, world!").is_ok());
        }
        for i in 0..cap {
            m.remove(i);
        }
        let mut keys = Vec::with_capacity(m.len());
        for (k, _) in &m {
            keys.push(k);
        }
        for key in keys {
            m.remove(key);
        }
        for i in 0..cap {
            assert!(!m.contains_key(i));
        }
    }
    let _duration = start.elapsed();
}
