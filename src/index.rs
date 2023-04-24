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

use crate::Map;
use std::ops::{Index, IndexMut};

impl<V: Clone, const N: usize> Index<usize> for Map<V, N> {
    type Output = V;

    #[inline]
    fn index(&self, key: usize) -> &V {
        self.get(&key).expect("no entry found for key")
    }
}

impl<V: Clone, const N: usize> IndexMut<usize> for Map<V, N> {
    #[inline]
    fn index_mut(&mut self, key: usize) -> &mut V {
        self.get_mut(&key).expect("no entry found for key")
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use std::borrow::Borrow;

#[test]
fn index() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(1, "first");
    assert_eq!("first", m[1]);
    Ok(())
}

#[test]
fn index_mut() -> Result<()> {
    let mut m: Map<i32, 10> = Map::new();
    m.insert(1, 10);
    m[1] += 55;
    assert_eq!(65, m[1]);
    Ok(())
}

#[test]
#[should_panic]
fn wrong_index() -> () {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(2, "first");
    m.insert(8, "second");
    m.remove(&8);
    m[8];
}

#[cfg(test)]
#[derive(Clone, PartialEq, Eq)]
struct Container {
    pub t: i32,
}

#[cfg(test)]
impl Borrow<i32> for Container {
    fn borrow(&self) -> &i32 {
        &self.t
    }
}

#[test]
fn index_by_borrow() -> Result<()> {
    let mut m: Map<Container, 10> = Map::new();
    m.insert(2, Container { t: 10 });
    assert_eq!(10, m[2].t);
    Ok(())
}
