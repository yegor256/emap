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

use crate::{IntoValues, Values};

impl<'a, V: Clone + 'a> Iterator for Values<'a, V> {
    type Item = &'a V;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let opt = unsafe { &*self.head.add(self.pos) };
            self.pos += 1;
            if opt.is_some() {
                return opt.as_ref();
            }
        }
        None
    }
}

impl<V: Copy> Iterator for IntoValues<V> {
    type Item = V;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.max {
            let opt = unsafe { &*self.head.add(self.pos) };
            self.pos += 1;
            if opt.is_some() {
                return *opt;
            }
        }
        None
    }
}

#[cfg(test)]
use anyhow::Result;

#[cfg(test)]
use crate::Map;

#[test]
fn empty_values() -> Result<()> {
    let m: Map<u32> = Map::with_capacity_init(16);
    assert!(m.values().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity_init(16);
    m.insert(0, "foo");
    let mut values = m.into_values();
    assert_eq!("foo", values.next().unwrap());
    assert!(values.next().is_none());
    Ok(())
}

#[test]
fn count_them_all() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity_init(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    assert_eq!(3, m.into_values().count());
    Ok(())
}
