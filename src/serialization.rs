// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use crate::node::{Node, NodeId};
use serde::de::{Error as DeError, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::alloc::Layout;
use std::fmt::Formatter;
use std::marker::PhantomData;

/// Upper bound for the space reserved from an untrusted size hint.
const MAX_PREALLOCATED_ENTRIES: usize = 1024;

/// Entries read from the wire, together with the capacity they demand.
type Entries<V> = (Vec<(usize, V)>, usize);

impl<V: Clone + Serialize> Serialize for Map<V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[cfg(debug_assertions)]
        assert!(self.initialized, "Can't serialize() non-initialized Map");
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (a, v) in self.iter() {
            map.serialize_entry(&a, &v)?;
        }
        map.end()
    }
}

struct Vi<V>(PhantomData<V>);

impl<'de, V: Clone + Deserialize<'de>> Visitor<'de> for Vi<V> {
    type Value = Map<V>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let (entries, cap) = collect_entries::<M, V>(&mut access)?;
        let mut m: Self::Value = Map::with_capacity_none(cap);
        for (key, value) in entries {
            m.insert(key, value);
        }
        Ok(m)
    }
}

/// Read every entry and report the capacity the highest key demands.
///
/// # Errors
///
/// Fails on the reserved `usize::MAX` key and on a capacity that no `Layout`
/// can describe.
fn collect_entries<'de, M, V>(access: &mut M) -> Result<Entries<V>, M::Error>
where
    M: MapAccess<'de>,
    V: Deserialize<'de>,
{
    let mut entries: Vec<(usize, V)> = Vec::new();
    if let Some(hint) = access.size_hint() {
        entries.reserve(hint.min(MAX_PREALLOCATED_ENTRIES));
    }
    let mut max_key: Option<usize> = None;
    while let Some((key, value)) = access.next_entry::<usize, V>()? {
        if key == NodeId::UNDEF {
            return Err(DeError::custom(
                "the key usize::MAX is reserved and cannot be used",
            ));
        }
        max_key = Some(max_key.map_or(key, |seen: usize| seen.max(key)));
        entries.push((key, value));
    }
    let cap = max_key.map_or(0, |key| key + 1);
    if Layout::array::<Node<V>>(cap).is_err() {
        return Err(DeError::custom(
            "the highest key requires a capacity beyond addressable memory",
        ));
    }
    Ok((entries, cap))
}

impl<'de, V: Clone + Deserialize<'de>> Deserialize<'de> for Map<V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vi(PhantomData))
    }
}

#[cfg(test)]
use bincode::{deserialize, serialize};

#[test]
fn serialize_and_deserialize() {
    let mut before: Map<u8> = Map::with_capacity_none(2);
    before.insert(0, 42);
    before.insert(1, 42);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();
    assert_eq!(42, after.into_iter().next().unwrap().1);
}

#[test]
fn serde_big_map() {
    let cap = 256;
    let mut before: Map<u8> = Map::with_capacity_none(cap);
    before.insert(0, 42);
    before.insert(1, 42);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();
    assert_eq!(2, after.capacity());
}

/// A map whose highest key exceeds the number of entries must survive a round
/// trip, since the capacity has to follow the key and not the entry count.
#[test]
fn keeps_sparse_keys_over_a_round_trip() {
    let mut before: Map<u8> = Map::with_capacity_none(32);
    before.insert(0, 1);
    before.insert(31, 2);
    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();
    assert_eq!(32, after.capacity());
    assert_eq!(2, after.len());
    assert_eq!(Some(&1), after.get(0));
    assert_eq!(Some(&2), after.get(31));
}

/// The reserved sentinel key must be reported as an error instead of being
/// inserted.
#[test]
fn rejects_the_reserved_key() {
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as ValueError, MapDeserializer};
    let entry = std::iter::once((NodeId::UNDEF.into_deserializer(), 0u8.into_deserializer()));
    let deserializer = MapDeserializer::<_, ValueError>::new(entry);
    let err = Map::<u8>::deserialize(deserializer).unwrap_err();
    assert!(err.to_string().contains("is reserved and cannot be used"));
}

/// A key that demands more memory than the address space can hold must be
/// reported as an error instead of aborting on allocation failure.
#[test]
fn rejects_a_key_beyond_addressable_memory() {
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as ValueError, MapDeserializer};
    let entry = std::iter::once((
        (NodeId::UNDEF - 1).into_deserializer(),
        0u8.into_deserializer(),
    ));
    let deserializer = MapDeserializer::<_, ValueError>::new(entry);
    let err = Map::<u8>::deserialize(deserializer).unwrap_err();
    assert!(err.to_string().contains("beyond addressable memory"));
}
