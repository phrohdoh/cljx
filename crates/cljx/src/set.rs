use core::fmt;
use std::ops;

use itertools::Itertools as _;

use crate::{RcValue, Value};


#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Set(Vec<RcValue>);

impl Set {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    pub fn new_empty_value() -> Value {
        Value::set(Self(Vec::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::set_rc(Self(Vec::new()))
    }

    pub fn new(elements: Vec<RcValue>) -> Self {
        Self(elements)
    }

    pub fn new_value(elements: Vec<RcValue>) -> Value {
        Value::set(Self(elements))
    }

    pub fn new_value_rc(elements: Vec<RcValue>) -> RcValue {
        Value::set_rc(Self(elements))
    }

    pub fn insert(&mut self, value: RcValue) {
        // avoid duplicates
        for v in self.0.iter() {
            if RcValue::ptr_eq(v, &value) || *v == value {
                return;
            }
        }
        self.0.push(value);
    }

    pub fn get(&self, value: &RcValue) -> Option<RcValue> {
        for value_in_set in self.0.iter() {
            if RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value {
                return Some(value_in_set.to_owned());
            }
        }
        None
    }

    pub fn get_or(&self, value: &RcValue, or: RcValue) -> RcValue {
        for value_in_set in self.0.iter() {
            if RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value {
                return value_in_set.to_owned();
            }
        }
        or
    }

    pub fn get_or_nil(&self, value: &RcValue) -> RcValue {
        for value_in_set in self.0.iter() {
            if RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value {
                return value_in_set.to_owned();
            }
        }
        Value::nil_rc()
    }

    pub fn get_panicing(&self, value: &RcValue) -> RcValue {
        for value_in_set in self.0.iter() {
            if RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value {
                return value_in_set.to_owned();
            }
        }
        panic!("Key not found in Set: {}", value);
    }

    pub fn values(&self) -> Vec<RcValue> {
        self.0.iter().map(|v| v.to_owned()).collect()
    }

    pub fn contains(&self, value: &RcValue) -> bool {
        for value_in_set in self.0.iter() {
            if RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value {
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, value: &RcValue) -> Option<RcValue> {
        let idx = self.0.iter().position(|value_in_set| RcValue::ptr_eq(value_in_set, value) || *value_in_set == *value);
        if let Some(idx) = idx {
            let value_in_set = self.0.swap_remove(idx);
            Some(value_in_set)
        } else {
            None
        }
    }

    pub fn into_value(self) -> Value {
        Value::set(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::set_rc(self)
    }
}

impl ops::Deref for Set {
    type Target = Vec<RcValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Set {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{{{}}}", self.0.iter().join(", "))
    }
}

impl fmt::Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Set([{}])", self.0.iter().map(|x| format!("{:?}", x)).join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty_creates_empty_set() {
        let set = Set::new_empty();
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn new_with_elements() {
        let set = Set::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn new_value() {
        let val = Set::new_value(vec![
            Value::integer_rc(10),
            Value::integer_rc(20),
        ]);
        assert!(val.is_set());
        if let crate::Value::Set(s, _) = val {
            assert_eq!(s.len(), 2);
        } else {
            panic!("Expected Set variant");
        }
    }

    #[test]
    fn insert_adds_new_element() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_rc(1));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Value::integer_rc(1)));
    }

    #[test]
    fn insert_prevents_duplicate_by_value_equality() {
        let mut set = Set::new_empty();
        let val1 = Value::integer_rc(42);
        let val2 = Value::integer_rc(42);
        set.insert(val1);
        set.insert(val2);
        // Should still be 1 element
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_prevents_duplicate_by_pointer_equality() {
        let mut set = Set::new_empty();
        let val = Value::integer_rc(42);
        set.insert(val.clone());
        set.insert(val.clone());
        // Should still be 1 element
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_cannot_insert_same_value_twice() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_rc(100));
        set.insert(Value::integer_rc(100));
        set.insert(Value::integer_rc(100));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn insert_allows_multiple_distinct_values() {
        let mut set = Set::new_empty();
        set.insert(Value::integer_rc(1));
        set.insert(Value::integer_rc(2));
        set.insert(Value::integer_rc(3));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn get_returns_some_for_present_value() {
        let set = Set::new(vec![Value::integer_rc(42)]);
        let result = set.get(&Value::integer_rc(42));
        assert!(result.is_some());
    }

    #[test]
    fn get_returns_none_for_missing_value() {
        let set = Set::new(vec![Value::integer_rc(42)]);
        let result = set.get(&Value::integer_rc(99));
        assert!(result.is_none());
    }

    #[test]
    fn get_or_nil_returns_nil_for_missing_value() {
        let set = Set::new(vec![Value::integer_rc(42)]);
        let result = set.get_or_nil(&Value::integer_rc(99));
        assert!(result.is_nil());
    }

    #[test]
    fn get_panicing_panics_on_missing_value() {
        let set = Set::new(vec![Value::integer_rc(42)]);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = set.get_panicing(&Value::integer_rc(99));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn remove_removes_element_and_returns_value() {
        let mut set = Set::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        let removed = set.remove(&Value::integer_rc(2));
        assert!(removed.is_some());
        assert_eq!(set.len(), 2);
        assert!(!set.contains(&Value::integer_rc(2)));
    }

    #[test]
    fn remove_returns_none_for_non_existent_value() {
        let mut set = Set::new(vec![Value::integer_rc(42)]);
        let removed = set.remove(&Value::integer_rc(99));
        assert!(removed.is_none());
    }

    #[test]
    fn remove_allows_re_insert() {
        let mut set = Set::new(vec![Value::integer_rc(42)]);
        set.remove(&Value::integer_rc(42));
        assert_eq!(set.len(), 0);
        set.insert(Value::integer_rc(42));
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Value::integer_rc(42)));
    }

    #[test]
    fn contains_returns_false_for_non_existent() {
        let set = Set::new(vec![Value::integer_rc(42)]);
        assert!(!set.contains(&Value::integer_rc(99)));
    }

    #[test]
    fn equality_with_same_elements() {
        let set1 = Set::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        let set2 = Set::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        assert_eq!(set1, set2);
    }
}

