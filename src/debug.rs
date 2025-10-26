// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

impl<V: Clone + Display> Display for Map<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <&Self as Debug>::fmt(&self, f)
    }
}

impl<V: Clone + Display> Debug for Map<V> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't debug() non-initialized Map");
        let mut parts = vec![];
        for (k, v) in self {
            parts.push(format!("{k}: {v}"));
        }
        parts.sort_unstable();
        f.write_str(format!("{{{}}}", parts.join(", ").as_str()).as_str())
    }
}

#[test]
fn debugs_map() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    assert_eq!("{0: one, 1: two}", format!("{m:?}"));
}

#[test]
fn displays_map() {
    let mut m: Map<&str> = Map::with_capacity_none(16);
    m.insert(0, "one");
    m.insert(1, "two");
    assert_eq!("{0: one, 1: two}", m.to_string());
}
