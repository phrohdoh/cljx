use std::rc::Rc;
use crate::prelude::*;

pub fn new_unset() -> Option<Map> {
    None
}

pub fn new_unset_rc() -> Rc<Option<Map>> {
    Rc::new(None)
}

pub fn new(map: Map) -> Option<Map> {
    Some(map)
}

pub fn new_rc(map: Map) -> Rc<Option<Map>> {
    Rc::new(Some(map))
}

pub fn into_meta_rc(meta: Option<Map>) -> Rc<Option<Map>> {
    Rc::new(meta)
}

// Helper functions for Option<Map> access
pub fn inner_ref(meta: &Option<Map>) -> Option<&Map> {
    meta.as_ref()
}

/// Trait for metadata operations on Rc<Option<Map>>.
/// Provides methods for associating key-value pairs and retrieving values.
pub trait MetaOps {
    /// Insert or update a key-value pair in the metadata map.
    /// Returns a new Rc<Option<Map>> with the updated map (doesn't mutate in place).
    fn assoc(&self, key: RcValue, value: RcValue) -> Rc<Option<Map>>;

    /// Retrieve a value by key from the metadata map.
    fn get(&self, key: &RcValue) -> Option<RcValue>;
}

impl MetaOps for Rc<Option<Map>> {
    fn assoc(&self, key: RcValue, value: RcValue) -> Rc<Option<Map>> {
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
pub fn display_meta(meta: &Rc<Option<Map>>) -> String {
    match meta.as_ref() {
        None => "{}".to_string(),
        Some(map) => format!("{}", map),
    }
}
