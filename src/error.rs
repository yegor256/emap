// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Error returned when an operation requires a free slot but the map is full.
///
/// # Examples
///
/// ```
/// use emap::{Map, MapFullError};
/// let mut map: Map<u8> = Map::with_capacity_none(1);
/// map.insert(0, 7);
/// assert!(matches!(map.try_push(8), Err(MapFullError)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapFullError;

impl Display for MapFullError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("map capacity is exhausted")
    }
}

impl Error for MapFullError {}
