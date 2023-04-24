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

impl<V: Clone> Map<V> {
    /// Get the next key available for insertion.
    #[inline]
    #[must_use]
    pub fn next_key(&self) -> usize {
        unsafe {
            for i in 0..self.max {
                let item = &*self.head.add(i);
                if !item.is_some() {
                    return i;
                }
            }
            self.max
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn get_next_key_empty_map() -> Result<()> {
    let m: Map<&str> = Map::with_capacity(16);
    assert_eq!(0, m.next_key());
    Ok(())
}

#[test]
fn get_next_in_the_middle() -> Result<()> {
    let mut m: Map<u32> = Map::with_capacity(16);
    m.insert(0, 42);
    m.insert(1, 42);
    m.remove(&1);
    m.insert(2, 42);
    assert_eq!(1, m.next_key());
    Ok(())
}
