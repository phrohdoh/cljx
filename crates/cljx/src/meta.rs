use std::rc::Rc;
use crate::prelude::*;

pub fn new_unset() -> Option<Map> {
    None
}

pub fn new(map: Map) -> Option<Map> {
    Some(map)
}

/// Trait for metadata operations on metadata types.
/// Provides methods for associating key-value pairs and retrieving values.
pub trait MetaOps {
    /// Insert or update a key-value pair in the metadata map.
    /// Returns a new Option<Rc<Map>> with the updated map (doesn't mutate in place).
    fn assoc(&self, key: RcValue, value: RcValue) -> Option<Rc<Map>>;

    /// Retrieve a value by key from the metadata map.
    fn get(&self, key: &RcValue) -> Option<RcValue>;
}

impl MetaOps for Option<Rc<Map>> {
    fn assoc(&self, key: RcValue, value: RcValue) -> Option<Rc<Map>> {
        match self {
            None => {
                // No existing map, create a new one
                let mut new_map = Map::new_empty();
                new_map.insert(key, value);
                Some(Rc::new(new_map))
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
                Some(Rc::new(Map::new(entries)))
            }
        }
    }

    fn get(&self, key: &RcValue) -> Option<RcValue> {
        self.as_ref().and_then(|map| map.get(key))
    }
}

// Helper function to format metadata for display
pub fn display_meta(meta: &Rc<Option<Map>>) -> String {
    match meta.as_ref() {
        None => "{}".to_string(),
        Some(map) => format!("{}", map),
    }
}
