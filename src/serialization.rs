// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

//! Serde support for `emap::Map`.
//!
//! The map is serialized as a standard map from `usize` keys to values `V`.
//! Keys must be strictly less than [`usize::MAX`], as the maximum value is
//! reserved internally as a sentinel. The binary layout is compatible with
//! serde-based codecs such as `bincode` when the corresponding feature flags
//! are enabled.

use std::alloc::Layout;
use std::fmt::{Formatter, Result as FmtResult};
use std::marker::PhantomData;

use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::node::Node;
use crate::Map;

/// Serializes [`Map<V>`] as a map from `usize` to `V`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "serde")]
/// # {
/// use emap::Map;
/// use bincode::serde::{decode_from_slice, encode_to_vec};
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
        const MAX_PREALLOCATED_ENTRIES: usize = 1024;
        let mut entries: Vec<(usize, V)> = Vec::new();
        if let Some(hint) = access.size_hint() {
            entries.reserve(hint.min(MAX_PREALLOCATED_ENTRIES));
        }

        let mut max_key: Option<usize> = None;
        while let Some((k, v)) = access.next_entry()? {
            if k == usize::MAX {
                return Err(DeError::custom(
                    "key usize::MAX is reserved and cannot be used",
                ));
            }
            if let Some(mk) = max_key {
                if k > mk {
                    max_key = Some(k);
                }
            } else {
                max_key = Some(k);
            }
            entries.push((k, v));
        }
        let cap = match max_key {
            Some(mk) => mk
                .checked_add(1)
                .ok_or_else(|| DeError::custom("key range exceeds supported maximum"))?,
            None => 0,
        };

        if Layout::array::<Node<V>>(cap).is_err() {
            return Err(DeError::custom(
                "calculated capacity exceeds addressable memory",
            ));
        }
        let mut m: Self::Value = Map::with_capacity_none(cap);
        for (k, v) in entries {
            m.insert(k, v);
        }
        Ok(m)
    }
}

/// Deserializes data produced by [`Serialize`] back into [`Map<V>`].
///
/// Capacity is set to `max_key + 1` so that all entries can be inserted
/// directly by key without reallocation.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "serde")]
/// # {
/// use emap::Map;
/// use bincode::serde::{decode_from_slice, encode_to_vec};
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

    #[test]
    fn deserialize_rejects_reserved_key() {
        use serde::de::value::{Error as ValueError, MapDeserializer};
        use serde::de::IntoDeserializer;

        let entry = std::iter::once((usize::MAX.into_deserializer(), 0u8.into_deserializer()));
        let deserializer = MapDeserializer::<_, ValueError>::new(entry);
        let err = Map::<u8>::deserialize(deserializer).unwrap_err();
        assert!(err.to_string().contains("reserved and cannot be used"));
    }

    #[test]
    fn deserialize_rejects_capacity_overflow() {
        use serde::de::value::{Error as ValueError, MapDeserializer};
        use serde::de::IntoDeserializer;

        let large_key = usize::MAX - 1;
        let entry = std::iter::once((large_key.into_deserializer(), 0u8.into_deserializer()));
        let deserializer = MapDeserializer::<_, ValueError>::new(entry);
        let err = Map::<u8>::deserialize(deserializer).unwrap_err();
        assert!(err
            .to_string()
            .contains("capacity exceeds addressable memory"));
    }
}
