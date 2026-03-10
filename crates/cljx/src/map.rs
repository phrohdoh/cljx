use core::fmt;
use std::rc::Rc;
use itertools::Itertools as _;
use crate::{RcValue, Value};

pub type RcMap = Rc<Map>;

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Map(im::OrdMap<RcValue, RcValue>);

impl Map {
    pub fn new_empty() -> Self {
        Self(im::OrdMap::new())
    }

    pub fn new_empty_value() -> Value {
        Value::map(Self(im::OrdMap::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::map_rc(Self(im::OrdMap::new()))
    }

    pub fn new(entries: Vec<(RcValue, RcValue)>) -> Self {
        let mut map = im::OrdMap::new();
        for (key, value) in entries {
            map.insert(key, value);
        }
        Self(map)
    }

    pub fn new_value(entries: Vec<(RcValue, RcValue)>) -> Value {
        Value::map(Self::new(entries))
    }

    pub fn new_value_rc(entries: Vec<(RcValue, RcValue)>) -> RcValue {
        Value::map_rc(Self::new(entries))
    }

    pub fn insert(&mut self, key: RcValue, value: RcValue) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &RcValue) -> Option<RcValue> {
        self.0.get(key).cloned()
    }

    pub fn get_or(&self, key: &RcValue, or: RcValue) -> RcValue {
        self.0.get(key).cloned().unwrap_or(or)
    }

    pub fn get_or_nil(&self, key: &RcValue) -> RcValue {
        self.0.get(key).cloned().unwrap_or(Value::nil_rc())
    }

    pub fn get_or_panic(&self, key: &RcValue) -> RcValue {
        self.0.get(key).cloned().unwrap_or_else(|| panic!("Key not found in Map: {}", key))
    }

    pub fn keys(&self) -> Vec<RcValue> {
        self.0.iter().map(|(k, _v)| (*k).clone()).collect()
    }

    pub fn values(&self) -> Vec<RcValue> {
        self.0.iter().map(|(_k, v)| (*v).clone()).collect()
    }

    pub fn contains_key(&self, key: &RcValue) -> bool {
        self.0.contains_key(key)
    }

    pub fn remove(&mut self, key: &RcValue) -> Option<RcValue> {
        self.0.remove(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_value(self) -> Value {
        Value::map(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::map_rc(self)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&RcValue, &RcValue)> {
        self.0.iter()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", self.0.iter().map(|(k, v)| format!("{} {}", k, v)).join(", "))
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Map([{}])", self.0.iter().map(|(k, v)| format!("[{:?}, {:?}]", k, v)).join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_empty_map() {
        let map = Map::new_empty();
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn new_with_entries() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
        ]);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn new_value() {
        let val = Map::new_value(vec![
            (Value::keyword_unqualified_rc("x"), Value::integer_rc(10)),
        ]);
        assert!(val.is_map());
        if let crate::Value::Map(m, _) = val {
            assert_eq!(m.len(), 1);
        } else {
            panic!("Expected Map variant");
        }
    }

    #[test]
    fn insert_adds_new_key_value_pair() {
        let mut map = Map::new_empty();
        let key = Value::keyword_unqualified_rc("name");
        let val = Value::string_rc("Alice".to_string());
        map.insert(key, val);
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn insert_replaces_value_for_existing_key_by_value_equality() {
        let mut map = Map::new_empty();
        let key1 = Value::keyword_unqualified_rc("key");
        let key2 = Value::keyword_unqualified_rc("key");
        map.insert(key1, Value::integer_rc(100));
        map.insert(key2, Value::integer_rc(200));
        // Should have 1 entry with value 200
        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(&Value::keyword_unqualified_rc("key")).unwrap(), Value::integer(200));
    }

    #[test]
    fn insert_replaces_value_for_existing_key_by_pointer_equality() {
        let mut map = Map::new_empty();
        let key = Value::keyword_unqualified_rc("key");
        map.insert(key.clone(), Value::integer_rc(100));
        map.insert(key.clone(), Value::integer_rc(200));
        // Should have 1 entry with value 200
        assert_eq!(map.len(), 1);
        assert_eq!(*map.get(&key).unwrap(), Value::integer(200));
    }

    #[test]
    fn insert_multiple_distinct_keys() {
        let mut map = Map::new_empty();
        map.insert(Value::keyword_unqualified_rc("a"), Value::integer_rc(1));
        map.insert(Value::keyword_unqualified_rc("b"), Value::integer_rc(2));
        map.insert(Value::keyword_unqualified_rc("c"), Value::integer_rc(3));
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn get_returns_some_for_present_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("answer"), Value::integer_rc(42)),
        ]);
        let result = map.get(&Value::keyword_unqualified_rc("answer"));
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), Value::integer(42));
    }

    #[test]
    fn get_returns_none_for_missing_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        let result = map.get(&Value::keyword_unqualified_rc("b"));
        assert!(result.is_none());
    }

    #[test]
    fn get_or_returns_default_for_missing_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        let default = Value::integer_rc(999);
        let result = map.get_or(&Value::keyword_unqualified_rc("b"), default.clone());
        assert_eq!(*result, *default);
    }

    #[test]
    fn get_or_nil_returns_nil_for_missing_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        let result = map.get_or_nil(&Value::keyword_unqualified_rc("b"));
        assert!(result.is_nil());
    }

    #[test]
    fn get_or_panic_panics_on_missing_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = map.get_or_panic(&Value::keyword_unqualified_rc("b"));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn keys_returns_all_keys() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
            (Value::keyword_unqualified_rc("c"), Value::integer_rc(3)),
        ]);
        let keys = map.keys();
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn values_returns_all_values() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
            (Value::keyword_unqualified_rc("c"), Value::integer_rc(3)),
        ]);
        let values = map.values();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn contains_key_returns_true_for_present_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("present"), Value::integer_rc(1)),
        ]);
        assert!(map.contains_key(&Value::keyword_unqualified_rc("present")));
    }

    #[test]
    fn contains_key_returns_false_for_missing_key() {
        let map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        assert!(!map.contains_key(&Value::keyword_unqualified_rc("b")));
    }

    #[test]
    fn remove_removes_entry_and_returns_value() {
        let mut map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
        ]);
        let removed = map.remove(&Value::keyword_unqualified_rc("b"));
        assert!(removed.is_some());
        assert_eq!(*removed.unwrap(), Value::integer(2));
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn remove_returns_none_for_non_existent_key() {
        let mut map = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
        ]);
        let removed = map.remove(&Value::keyword_unqualified_rc("b"));
        assert!(removed.is_none());
    }

    #[test]
    fn equality_with_same_entries() {
        let map1 = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
        ]);
        let map2 = Map::new(vec![
            (Value::keyword_unqualified_rc("a"), Value::integer_rc(1)),
            (Value::keyword_unqualified_rc("b"), Value::integer_rc(2)),
        ]);
        assert_eq!(map1, map2);
    }
}

