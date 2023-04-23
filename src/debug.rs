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
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

impl<V: Clone + Copy + Display, const N: usize> Display for Map<V, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl<V: Clone + Copy + Display, const N: usize> Debug for Map<V, N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut parts = vec![];
        for i in 0..self.filled {
            if let Present(v) = &self.items[i] {
                parts.push(format!("{i}: {v}"));
            }
        }
        f.write_str(format!("{{{}}}", parts.join(", ").as_str()).as_str())
    }
}

use crate::Item::Present;
#[cfg(test)]
use anyhow::Result;

#[test]
fn debugs_map() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "one");
    m.insert(1, "two");
    assert_eq!("{0: one, 1: two}", format!("{:?}", m));
    Ok(())
}

#[test]
fn displays_map() -> Result<()> {
    let mut m: Map<&str, 10> = Map::new();
    m.insert(0, "one");
    m.insert(1, "two");
    assert_eq!("{0: one, 1: two}", format!("{}", m));
    Ok(())
}
