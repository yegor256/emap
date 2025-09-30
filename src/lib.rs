// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! There is a map with a fixed capacity and integers as keys.
//!
//! For example, here is a map with a few keys can be created:
//!
//! ```
//! use emap::Map;
//! let mut m : Map<&str> = Map::with_capacity_none(10);
//! m.insert(0, "Hello, world!");
//! m.insert(1, "Good bye!");
//! assert_eq!(2, m.len());
//! ```
//!
//! The map
//! will have exactly ten elements. An attempt to add an 11th element will lead
//! to a panic.

#![doc(html_root_url = "https://docs.rs/emap/0.0.0")]
#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::multiple_crate_versions)]

mod clone;
mod ctors;
mod debug;
mod index;
mod iterators;
mod keys;
mod map;
mod next_key;
pub mod node;
#[cfg(feature = "serde")]
mod serialization;
mod values;

use crate::node::{Node, NodeId};

use std::alloc::Layout;
use std::marker::PhantomData;
/// A map with a fixed capacity and `usize` as keys.
pub struct Map<V> {
    first_free: NodeId, // head of free list
    first_used: NodeId, // head of elements list
    head: *mut Node<V>,
    layout: Layout,
    len: usize,
    #[cfg(debug_assertions)]
    initialized: bool,
}

/// Iterator over the [`Map`].
pub struct Iter<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

/// Mutable iterator over the [`Map`].
pub struct IterMut<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

/// Into-iterator over the [`Map`] that yields immutable references to stored values.
pub struct IntoIter<'a, V> {
    inner: Iter<'a, V>,
}

pub struct Values<'a, V> {
    current: NodeId,
    head: *mut Node<V>,
    _marker: PhantomData<&'a V>,
}

/// Into-iterator over the values of a [`Map`].
pub struct IntoValues<V> {
    current: NodeId,
    head: *mut Node<V>,
}

/// Iterator over the keys of a [`Map`].
pub struct Keys<V> {
    current: NodeId,
    head: *mut Node<V>,
}

#[cfg(test)]
use std::time::Instant;

/// Run it like this from command line:
///
/// ```text
/// $ cargo test --release -- perf --nocapture
/// ```
#[test]
fn perf() {
    let cap = 256;
    let mut m: Map<&str> = Map::with_capacity_none(cap);
    let start = Instant::now();
    for _ in 0..1000 {
        m.clear();
        for _ in 0..cap {
            m.push("Hello, world!");
        }
        for i in 0..cap {
            m.remove(i);
        }
        let mut keys = Vec::with_capacity(m.len());
        for (k, _) in m.iter() {
            keys.push(k);
        }
        for key in keys {
            m.remove(key);
        }
        for i in 0..cap {
            assert!(!m.contains_key(i));
        }
    }
    let d = start.elapsed();
    println!("Total time: {}", d.as_millis());
}
