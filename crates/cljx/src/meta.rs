use core::fmt;
use std::rc::Rc;
use crate::{Map, RcValue};

// TODO: this probably ought to be a protocol

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Meta(Option<Map>);


pub type RcMeta = Rc<Meta>;


impl Default for Meta {
    fn default() -> Self {
        Self::new_empty()
    }
}


impl Meta {
    pub fn new_empty() -> Self {
        Self(None)
    }

    pub fn new_empty_rc() -> RcMeta {
        Rc::new(Self(None))
    }

    pub fn new(map: Map) -> Self {
        Self(Some(map))
    }

    pub fn new_rc(map: Map) -> RcMeta {
        Rc::new(Self(Some(map)))
    }

    pub fn into_meta_rc(self) -> RcMeta {
        Rc::new(self)
    }

    /// Generic helper to insert a key-value pair into the Meta map.
    /// Returns a new RcMeta with the updated map (doesn't mutate in place).
    pub fn insert(meta: RcMeta, key: RcValue, value: RcValue) -> RcMeta {
        match &meta.0 {
            None => {
                // No existing map, create a new one
                let mut new_map = Map::new_empty();
                new_map.insert(key, value);
                Rc::new(Meta::new(new_map))
            }
            Some(existing_map) => {
                // Clone existing entries
                let mut entries: Vec<(RcValue, RcValue)> = existing_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

                // Find and update, or insert if not found
                let mut found = false;
                for (k, v) in entries.iter_mut() {
                    if RcValue::ptr_eq(k, &key) || *k == key {
                        *v = value.clone();
                        found = true;
                        break;
                    }
                }
                if !found {
                    entries.push((key, value));
                }

                Rc::new(Meta::new(Map::new(entries)))
            }
        }
    }

    /// Generic helper to retrieve a value by key from the Meta map.
    pub fn get(meta: &Meta, key: &RcValue) -> Option<RcValue> {
        meta.0.as_ref().and_then(|map| map.get(key))
    }
}

impl fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            None => write!(f, "{{}}"),
            Some(map) => write!(f, "{}", map),
        }
    }
}
