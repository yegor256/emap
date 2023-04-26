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

impl<V: Clone> Clone for Map<V> {
    fn clone(&self) -> Self {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't clone() non-initialized Map");
        let mut m = Self::with_capacity_init(self.layout.size());
        for (k, v) in self.iter() {
            m.insert(k, v.clone());
        }
        println!("clone!");
        m
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn map_can_be_cloned() -> Result<()> {
    let mut m: Map<u8> = Map::with_capacity_init(16);
    m.insert(0, 42);
    assert_eq!(42, *m.clone().get(0).unwrap());
    Ok(())
}

#[test]
#[ignore]
fn empty_clone() -> Result<()> {
    let m: Map<u8> = Map::with_capacity_init(16);
    assert!(m.clone().is_empty());
    Ok(())
}

#[test]
#[ignore]
fn larger_map_can_be_cloned() -> Result<()> {
    let cap = 16;
    let mut m: Map<u8> = Map::with_capacity(cap);
    m.insert(1, 42);
    m.insert(2, 42);
    assert_eq!(2, m.clone().len());
    assert_eq!(cap, m.clone().capacity());
    Ok(())
}

#[derive(Clone)]
struct Foo {
    _m: Map<u64>,
}

#[test]
#[ignore]
fn clone_of_wrapper() -> Result<()> {
    let mut f: Foo = Foo {
        _m: Map::with_capacity_init(16),
    };
    f._m.insert(7, 42);
    assert_eq!(1, f.clone()._m.len());
    Ok(())
}
