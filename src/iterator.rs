use std::collections::{hash_map, HashMap};

use serde::de::DeserializeOwned;

use crate::serialization::Serializer;

pub struct DocDbIterator<'a> {
    ///
    pub(crate) map_iter: hash_map::Iter<'a, String, Vec<u8>>,
    pub(crate) serializer: &'a Serializer,
}

impl<'a> Iterator for DocDbIterator<'a> {
    type Item = DocDbIteratorItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.map_iter.next() {
            Some((key, value)) => Some(DocDbIteratorItem {
                key,
                value,
                serializer: self.serializer,
            }),
            None => None,
        }
    }
}

pub struct DocDbIteratorItem<'a> {
    ///
    key: &'a str,
    value: &'a Vec<u8>,
    serializer: &'a Serializer,
}

impl<'a> DocDbIteratorItem<'a> {
    pub fn get_key(&self) -> &str {
        self.key
    }

    pub fn get_value<T: DeserializeOwned>(&self) -> Option<T> {
        self.serializer.deserialize_data(self.value)
    }
}
