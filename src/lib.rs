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

#![doc(html_root_url = "https://docs.rs/emap/0.0.13")]
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
#[cfg(feature = "serde")]
mod serialization;
mod values;

use std::alloc::Layout;
use std::marker::PhantomData;

/// A map with a fixed capacity and `usize` as keys.
pub struct Map<V> {
    max: usize,
    head: *mut Option<V>,
    layout: Layout,
    #[cfg(debug_assertions)]
    initialized: bool,
}

/// Iterator over the [`Map`].
pub struct Iter<'a, V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
    _marker: PhantomData<&'a V>,
}

/// Mutable iterator over the [`Map`].
pub struct IterMut<'a, V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
    _marker: PhantomData<&'a V>,
}

/// Into-iterator over the [`Map`].
pub struct IntoIter<V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
}

/// Iterator over the values of a [`Map`].
pub struct Values<'a, V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
    _marker: PhantomData<&'a V>,
}

/// Into-iterator over the values of a [`Map`].
pub struct IntoValues<V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
}

/// Iterator over the keys of a [`Map`].
pub struct Keys<V> {
    max: usize,
    pos: usize,
    head: *mut Option<V>,
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
        for (k, _) in m.into_iter() {
            m.remove(k);
        }
        for i in 0..cap {
            assert!(!m.contains_key(i));
        }
    }
    let d = start.elapsed();
    println!("Total time: {}", d.as_millis());
}
