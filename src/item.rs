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

use crate::Item;
use crate::Item::{Absent, Present};

impl<V> Item<V> {
    pub(crate) const fn is_some(&self) -> bool {
        match self {
            Absent => false,
            Present(_) => true,
        }
    }

    pub(crate) fn unwrap(self) -> V {
        match self {
            Present(v) => v,
            Absent => panic!("Oops"),
        }
    }
}

#[cfg(test)]
use anyhow::Result;

#[test]
fn makes_absent_and_checks() -> Result<()> {
    let p: Item<u8> = Absent;
    assert!(!p.is_some());
    Ok(())
}

#[test]
fn makes_present_and_checks() -> Result<()> {
    let p = Item::Present((1, 2));
    assert!(p.is_some());
    Ok(())
}
