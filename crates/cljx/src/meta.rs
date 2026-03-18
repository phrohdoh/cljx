use std::rc::Rc;
use crate::prelude::*;

// TODO: this probably ought to be a protocol

/// Meta is now a type alias for Option<Map>, removing the newtype wrapper.
pub type Meta = Option<Map>;

pub type RcMeta = Rc<Meta>;

// Helper functions for Meta construction
#[inline]
pub fn new_empty() -> Meta {
    None
}

#[inline]
pub fn new_empty_rc() -> RcMeta {
    Rc::new(None)
}

#[inline]
pub fn new(map: Map) -> Meta {
    Some(map)
}

#[inline]
pub fn new_rc(map: Map) -> RcMeta {
    Rc::new(Some(map))
}

#[inline]
pub fn into_meta_rc(meta: Meta) -> RcMeta {
    Rc::new(meta)
}

// Helper functions for Meta access
#[inline]
pub fn inner_ref(meta: &Meta) -> Option<&Map> {
    meta.as_ref()
}

#[inline]
pub fn inner(meta: Meta) -> Option<Map> {
    meta
}

/// Trait for metadata operations on RcMeta.
/// Provides methods for associating key-value pairs and retrieving values.
pub trait MetaOps {
    /// Insert or update a key-value pair in the metadata map.
    /// Returns a new RcMeta with the updated map (doesn't mutate in place).
    fn assoc(&self, key: RcValue, value: RcValue) -> RcMeta;

    /// Retrieve a value by key from the metadata map.
    fn get(&self, key: &RcValue) -> Option<RcValue>;
}

impl MetaOps for RcMeta {
    fn assoc(&self, key: RcValue, value: RcValue) -> RcMeta {
        match self.as_ref() {
            None => {
                // No existing map, create a new one
                let mut new_map = Map::new_empty();
                new_map.insert(key, value);
                Rc::new(Some(new_map))
            }
            Some(existing_map) => {
                // Clone existing entries
                let mut entries: Vec<(RcValue, RcValue)> = existing_map
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();

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

                Rc::new(Some(Map::new(entries)))
            }
        }
    }

    fn get(&self, key: &RcValue) -> Option<RcValue> {
        self.as_ref().as_ref().and_then(|map| map.get(key))
    }
}

// Helper function to format metadata for display
pub fn display_meta(meta: &RcMeta) -> String {
    match meta.as_ref() {
        None => "{}".to_string(),
        Some(map) => format!("{}", map),
    }
}
