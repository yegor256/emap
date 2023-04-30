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

use crate::Keys;
use crate::Map;
use std::ptr;

impl<V> Iterator for Keys<V> {
    type Item = usize;

    #[inline]
    #[must_use]
    fn next(&mut self) -> Option<usize> {
        while self.pos < self.max {
            let opt = unsafe { ptr::read(self.head.add(self.pos)) };
            if opt.is_some() {
                let k = self.pos;
                self.pos += 1;
                return Some(k);
            }
            self.pos += 1;
        }
        None
    }
}

impl<V: Clone> Map<V> {
    /// Make an iterator over all keys.
    ///
    /// # Panics
    ///
    /// It may panic in debug mode, if the [`Map`] is not initialized.
    #[inline]
    #[must_use]
    pub const fn keys(&self) -> Keys<V> {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't keys() non-initialized Map");
        Keys {
            max: self.max,
            pos: 0,
            head: self.head,
        }
    }
}

#[test]
fn empty_keys() {
    let m: Map<u32> = Map::with_capacity_none(16);
    assert!(m.keys().next().is_none());
}

#[test]
fn insert_and_jump_over_next_key() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "foo");
    let mut keys = m.keys();
    assert_eq!(0, keys.next().unwrap());
    assert!(keys.next().is_none());
}
