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

use crate::{IntoIter, Iter, Map};

impl<'a, V: Clone + 'a> Iterator for Iter<'a, V> {
    type Item = (usize, &'a V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.pos < self.max {
                let item = &*self.head.as_ptr().add(self.pos);
                if let Present(p) = item {
                    let i = self.pos;
                    self.pos += 1;
                    return Some((i, p));
                }
                self.pos += 1;
            }
            None
        }
    }
}

impl<V: Copy> Iterator for IntoIter<V> {
    type Item = (usize, V);

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.pos < self.max {
                let item = self.head.as_ptr().add(self.pos).as_ref().unwrap();
                if let Present(v) = item {
                    let i = self.pos;
                    self.pos += 1;
                    return Some((i, *v));
                }
                self.pos += 1;
            }
            None
        }
    }
}

impl<'a, V: Copy> IntoIterator for &'a Map<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

use crate::Item::Present;
#[cfg(test)]
use anyhow::Result;

#[test]
fn empty_iterator() -> Result<()> {
    let m: Map<u32> = Map::with_capacity(16);
    assert!(m.into_iter().next().is_none());
    Ok(())
}

#[test]
fn insert_and_jump_over_next() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "foo");
    let mut iter = m.into_iter();
    assert_eq!("foo", iter.next().unwrap().1);
    assert!(iter.next().is_none());
    Ok(())
}

#[test]
fn insert_and_iterate() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
    Ok(())
}

#[test]
fn insert_and_into_iterate() -> Result<()> {
    let mut m: Map<&str> = Map::with_capacity(16);
    m.insert(0, "one");
    m.insert(1, "two");
    m.insert(2, "three");
    let mut sum = 0;
    let mut count = 0;
    for (k, _v) in m.into_iter() {
        sum += k;
        count += 1;
    }
    assert_eq!(3, count);
    assert_eq!(3, sum);
    Ok(())
}
