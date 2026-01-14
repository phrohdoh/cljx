use core::fmt;
use std::{ops, rc::Rc};

use itertools::Itertools as _;

use crate::{RcValue, Value};


pub type RcMap = Rc<Map>;

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Map(Vec<(RcValue, RcValue)>);

impl Map {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    pub fn new_empty_value() -> Value {
        Value::map(Self(Vec::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::map_rc(Self(Vec::new()))
    }

    pub fn new(entries: Vec<(RcValue, RcValue)>) -> Self {
        Self(entries)
    }

    pub fn new_value(entries: Vec<(RcValue, RcValue)>) -> Value {
        Value::map(Self(entries))
    }

    pub fn new_value_rc(entries: Vec<(RcValue, RcValue)>) -> RcValue {
        Value::map_rc(Self(entries))
    }

    pub fn insert(&mut self, key: RcValue, value: RcValue) {
        // replace existing entry if key exists
        for (k, v) in self.0.iter_mut() {
            if RcValue::ptr_eq(k, &key) || *k == key {
                *v = value;
                return;
            }
        }

        // otherwise, add new entry
        self.0.push((key, value));
    }

    pub fn get(&self, key: &RcValue) -> Option<RcValue> {
        for (k, v) in self.0.iter() {
            if RcValue::ptr_eq(k, key) || *k == *key {
                return Some(v.to_owned());
            }
        }
        None
    }

    pub fn get_or(&self, key: &RcValue, or: RcValue) -> RcValue {
        for (k, v) in self.0.iter() {
            if RcValue::ptr_eq(k, key) || *k == *key {
                return v.to_owned();
            }
        }
        or
    }

    pub fn get_or_nil(&self, key: &RcValue) -> RcValue {
        for (k, v) in self.0.iter() {
            if RcValue::ptr_eq(k, key) || *k == *key {
                return v.to_owned();
            }
        }
        Value::nil_rc()
    }

    pub fn get_panicing(&self, key: &RcValue) -> RcValue {
        for (k, v) in self.0.iter() {
            if RcValue::ptr_eq(k, key) || *k == *key {
                return v.to_owned();
            }
        }
        panic!("Key not found in Map: {}", key);
    }

    pub fn keys(&self) -> Vec<RcValue> {
        self.0.iter().map(|(k, _v)| k.to_owned()).collect()
    }

    pub fn values(&self) -> Vec<RcValue> {
        self.0.iter().map(|(_k, v)| v.to_owned()).collect()
    }

    pub fn contains_key(&self, key: &RcValue) -> bool {
        for (k, _v) in self.0.iter() {
            if RcValue::ptr_eq(k, key) || *k == *key {
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, key: &RcValue) -> Option<RcValue> {
        let idx = self.0.iter().position(|(k, _v)| RcValue::ptr_eq(k, key) || *k == *key);
        if let Some(idx) = idx {
            let (_k, v) = self.0.swap_remove(idx);
            Some(v)
        } else {
            None
        }
    }

    pub fn into_value(self) -> Value {
        Value::map(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::map_rc(self)
    }
}

impl ops::Deref for Map {
    type Target = Vec<(RcValue, RcValue)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

