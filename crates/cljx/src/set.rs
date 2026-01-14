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

