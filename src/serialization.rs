// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

#![cfg(feature = "serde")]

use crate::Map;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Formatter, Result as FmtResult};
use std::marker::PhantomData;

/// Serializes the map as a map of `usize` keys to values `V`.
///
/// The output is compatible with serde-based codecs such as bincode (with its
/// `serde` feature enabled).
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "serde")]
/// # {
/// use emap::Map;
/// use bincode::serde::{encode_to_vec, decode_from_slice};
/// use bincode::config::standard;
///
/// let mut m: Map<u8> = Map::with_capacity_none(4);
/// m.insert(0, 7);
/// m.insert(2, 9);
///
/// let bytes = encode_to_vec(&m, standard()).unwrap();
/// let (back, _): (Map<u8>, usize) = decode_from_slice(&bytes, standard()).unwrap();
///
/// assert_eq!(back.get(0), Some(&7));
/// assert_eq!(back.get(2), Some(&9));
/// # }
/// ```
impl<V: Serialize> Serialize for Map<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't serialize() non-initialized Map");
        let mut out = serializer.serialize_map(Some(self.len()))?;
        for k in self.keys() {
            if let Some(v) = self.get(k) {
                out.serialize_entry(&k, v)?;
            }
        }
        out.end()
    }
}

struct Vi<V>(PhantomData<V>);

impl<'de, V: Deserialize<'de>> Visitor<'de> for Vi<V> {
    type Value = Map<V>;

    fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a Map<usize, V> serialized as a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut entries: Vec<(usize, V)> = Vec::with_capacity(access.size_hint().unwrap_or(0));
        let mut max_key: Option<usize> = None;
        while let Some((k, v)) = access.next_entry()? {
            if let Some(mk) = max_key {
                if k > mk {
                    max_key = Some(k);
                }
            } else {
                max_key = Some(k);
            }
            entries.push((k, v));
        }
        let cap = max_key.map(|mk| mk.saturating_add(1)).unwrap_or(0);
        let mut m: Self::Value = Map::with_capacity_none(cap);
        for (k, v) in entries {
            m.insert(k, v);
        }
        Ok(m)
    }
}

/// Deserializes a map previously produced by [`Serialize`] into `Map<V>`.
///
/// Capacity is chosen as `max_key + 1`, ensuring all entries can be inserted
/// directly by key.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "serde")]
/// # {
/// use emap::Map;
/// use bincode::serde::{encode_to_vec, decode_from_slice};
/// use bincode::config::standard;
///
/// let mut before: Map<String> = Map::with_capacity_none(8);
/// before.insert(1, "a".to_string());
/// before.insert(6, "b".to_string());
///
/// let bytes = encode_to_vec(&before, standard()).unwrap();
/// let (after, _): (Map<String>, usize) = decode_from_slice(&bytes, standard()).unwrap();
///
/// assert_eq!(after.get(1), Some(&"a".to_string()));
/// assert_eq!(after.get(6), Some(&"b".to_string()));
/// # }
/// ```
impl<'de, V: Deserialize<'de>> Deserialize<'de> for Map<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi(PhantomData))
    }
}

#[cfg(all(test, feature = "serde"))]
mod tests {
    use super::*;
    use bincode::config::standard;
    use bincode::serde::{decode_from_slice, encode_to_vec};

#[test]
fn serialize_and_deserialize() {
    let mut before: Map<u8> = Map::with_capacity_none(2);
    before.insert(0, 42);
    before.insert(1, 42);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();
    assert_eq!(42, *after.into_iter().next().unwrap().1);
}

#[test]
fn serialize_and_deserialize_roundtrip() {
    let mut before: Map<u8> = Map::with_capacity_none(2);
    before.insert(0, 42);
    before.insert(1, 99);
    let bytes = encode_to_vec(&before, standard()).unwrap();
    let (after, _): (Map<u8>, usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(after.len(), 2);
    assert_eq!(after.get(0), Some(&42));
    assert_eq!(after.get(1), Some(&99));
}

    #[test]
    fn sparse_keys_capacity_is_max_key_plus_one() {
        let mut before: Map<u8> = Map::with_capacity_none(32);
        before.insert(0, 1);
        before.insert(31, 2);
        let bytes = encode_to_vec(&before, standard()).unwrap();
        let (after, _): (Map<u8>, usize) = decode_from_slice(&bytes, standard()).unwrap();
        assert_eq!(after.capacity(), 32);
        assert_eq!(after.get(31), Some(&2));
    }
}

