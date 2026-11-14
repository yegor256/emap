// SPDX-FileCopyrightText: Copyright (c) 2023-2026 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::{Map, Node};
use serde::de::{Error as _, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::alloc::Layout;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::marker::PhantomData;

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

impl<'de, V: Deserialize<'de>> Visitor<'de> for Vi<V> {
    type Value = Map<V>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut entries: HashMap<usize, V> = HashMap::new();
        let mut required_capacity = 0;
        while let Some((key, value)) = access.next_entry::<usize, V>()? {
            let capacity = key
                .checked_add(1)
                .ok_or_else(|| M::Error::custom("emap key cannot fit into a map capacity"))?;
            required_capacity = required_capacity.max(capacity);
            entries.insert(key, value);
        }
        Layout::array::<Node<V>>(required_capacity)
            .map_err(|_| M::Error::custom("emap capacity exceeds the maximum allocation size"))?;
        let mut map = Map::with_capacity_none(required_capacity);
        for (key, value) in entries {
            map.insert(key, value);
        }
        Ok(map)
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for Map<V> {
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

#[test]
fn serde_empty_map_uses_zero_capacity() {
    let before: Map<u8> = Map::with_capacity_none(16);

    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();

    assert_eq!(0, after.capacity());
    assert_eq!(0, after.len());
}

#[test]
fn serde_sparse_keys_preserve_values() {
    let mut before: Map<u8> = Map::with_capacity_none(512);
    before.insert(1, 41);
    before.insert(255, 42);

    let bytes: Vec<u8> = serialize(&before).unwrap();
    let after: Map<u8> = deserialize(&bytes).unwrap();

    assert_eq!(256, after.capacity());
    assert_eq!(2, after.len());
    assert_eq!(Some(&41), after.get(1));
    assert_eq!(Some(&42), after.get(255));
}

#[test]
fn serde_rejects_key_that_cannot_fit_capacity() {
    use serde::de::value::{Error, MapDeserializer};

    let entries = [(usize::MAX, 42_u8)];
    let deserializer = MapDeserializer::<_, Error>::new(entries.into_iter());
    let result = Map::<u8>::deserialize(deserializer);

    assert!(result.is_err());
}

#[test]
fn serde_rejects_key_whose_node_array_layout_overflows() {
    use serde::de::value::{Error, MapDeserializer};

    let capacity = (isize::MAX as usize / size_of::<crate::Node<u8>>()) + 1;
    let entries = [(capacity - 1, 42_u8)];
    let deserializer = MapDeserializer::<_, Error>::new(entries.into_iter());
    let result = Map::<u8>::deserialize(deserializer);

    assert!(result.is_err());
}

#[test]
fn serde_duplicate_keys_keep_last_value() {
    use serde::de::value::{Error, MapDeserializer};

    let entries = [(3_usize, 41_u8), (3, 42)];
    let deserializer = MapDeserializer::<_, Error>::new(entries.into_iter());
    let map = Map::<u8>::deserialize(deserializer).unwrap();

    assert_eq!(4, map.capacity());
    assert_eq!(1, map.len());
    assert_eq!(Some(&42), map.get(3));
}

#[test]
fn serde_deserializes_values_without_clone() {
    use serde::de::value::{Error, MapDeserializer};

    struct NonClone(u8);

    impl<'de> Deserialize<'de> for NonClone {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            u8::deserialize(deserializer).map(Self)
        }
    }

    let entries = [(7_usize, 42_u8)];
    let deserializer = MapDeserializer::<_, Error>::new(entries.into_iter());
    let map = Map::<NonClone>::deserialize(deserializer).unwrap();

    assert_eq!(8, map.capacity());
    assert_eq!(1, map.len());
    assert_eq!(Some(42), map.get(7).map(|value| value.0));
}
