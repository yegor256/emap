// SPDX-FileCopyrightText: Copyright (c) 2023-2025 Yegor Bugayenko
// SPDX-License-Identifier: MIT

use crate::Map;
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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

impl<'de, V: Clone + Deserialize<'de>> Visitor<'de> for Vi<V> {
    type Value = Map<V>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a Map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map: HashMap<usize, V> = HashMap::new();
        while let Some((key, value)) = access.next_entry()? {
            map.insert(key, value);
        }
        let mut m: Self::Value = Map::with_capacity_none(map.len());
        for (k, v) in map.iter() {
            m.insert(*k, v.clone());
        }
        Ok(m)
    }
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
use bincode::{config::standard, serde};

#[test]
fn serialize_and_deserialize() {
    let mut before: Map<u8> = Map::with_capacity_none(2);
    before.insert(0, 42);
    before.insert(1, 42);
    let bytes: Vec<u8> = serde::encode_to_vec(&before, standard()).unwrap();
    let (after, _): (Map<u8>, usize) = serde::decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(42, after.into_iter().next().unwrap().1);
}

#[test]
fn serde_big_map() {
    let cap = 256;
    let mut before: Map<u8> = Map::with_capacity_none(cap);
    before.insert(0, 42);
    before.insert(1, 42);
    let bytes: Vec<u8> = serde::encode_to_vec(&before, standard()).unwrap();
    let (after, _): (Map<u8>, usize) = serde::decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(2, after.capacity());
}
