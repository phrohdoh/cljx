use core::fmt;
use itertools::Itertools;
use crate::prelude::*;

pub mod optics;
pub mod partials;

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Vector(im::Vector<RcValue>);

impl From<Vec<RcValue>> for Vector {
    fn from(elements: Vec<RcValue>) -> Self {
        Self(im::Vector::from(elements))
    }
}

impl Vector {
    pub fn new_empty() -> Self {
        Self(im::Vector::new())
    }

    pub fn new_empty_value() -> Value {
        Value::vector(Self(im::Vector::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::vector_rc(Self(im::Vector::new()))
    }

    pub fn new_value(elements: Vec<RcValue>) -> Value {
        Value::vector(Self::from(elements))
    }

    pub fn new_value_rc(elements: Vec<RcValue>) -> RcValue {
        Value::vector_rc(Self::from(elements))
    }


    pub fn into_value(self) -> Value {
        Value::vector(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::vector_rc(self)
    }


    pub fn push_back(&mut self, value: RcValue) {
        self.0.push_back(value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RcValue> {
        self.0.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = RcValue> {
        self.0.into_iter()
    }
}

impl Vector {
    pub fn get_first(&self) -> Option<RcValue> {
        self.0.front().cloned()
    }

    pub fn get_first_ref(&self) -> Option<&Value> {
        self.0.front().map(RcValue::as_ref)
    }

    pub fn get_first_or(&self, default: RcValue) -> RcValue {
        self.0.front().cloned().unwrap_or(default)
    }

    pub fn get_first_or_nil(&self) -> RcValue {
        self.0.front().cloned().unwrap_or_else(Value::nil_rc)
    }

    pub fn get_first_or_else(&self, else_fn: impl FnOnce(&Self) -> RcValue) -> RcValue {
        self.0.front().cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_first_or_panic(&self) -> RcValue {
        self.0.front().cloned().unwrap()
    }


    pub fn get_second(&self) -> Option<RcValue> {
        self.0.get(1).cloned()
    }

    pub fn get_second_ref(&self) -> Option<&Value> {
        self.0.get(1).map(RcValue::as_ref)
    }

    pub fn get_second_or(&self, default: RcValue) -> RcValue {
        self.0.get(1).cloned().unwrap_or(default)
    }

    pub fn get_second_or_nil(&self) -> RcValue {
        self.0.get(1).cloned().unwrap_or_else(Value::nil_rc)
    }

    pub fn get_second_or_else(&self, else_fn: impl FnOnce(&Self) -> RcValue) -> RcValue {
        self.0.get(1).cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_second_or_panic(&self) -> RcValue {
        self.0.get(1).cloned().unwrap()
    }



    pub fn get_last(&self) -> Option<RcValue> {
        self.0.last().cloned()
    }

    pub fn get_last_ref(&self) -> Option<&Value> {
        self.0.last().map(RcValue::as_ref)
    }

    pub fn get_last_or(&self, default: RcValue) -> RcValue {
        self.0.last().cloned().unwrap_or(default)
    }

    pub fn get_last_or_nil(&self) -> RcValue {
        self.0.last().cloned().unwrap_or_else(Value::nil_rc)
    }

    pub fn get_last_or_else(&self, else_fn: impl FnOnce(&Self) -> RcValue) -> RcValue {
        self.0.last().cloned().unwrap_or_else(|| else_fn(self))
    }

    pub fn get_last_or_panic(&self) -> RcValue {
        self.0.last().cloned().unwrap()
    }


    pub fn get_nth(&self, n: usize) -> Option<RcValue> {
        self.0.get(n).cloned()
    }

    pub fn get_nth_ref(&self, n: usize) -> Option<&Value> {
        self.0.get(n).map(RcValue::as_ref)
    }

    pub fn get_nth_or(&self, n: usize, or: RcValue) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or(or)
    }

    pub fn get_nth_or_nil(&self, n: usize) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or_else(Value::nil_rc)
    }

    pub fn get_nth_or_else(&self, n: usize, else_fn: impl FnOnce(&Self) -> RcValue) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or_else(|| else_fn(self))
    }

    pub fn get_nth_or_panic(&self, n: usize) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap()
    }
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.iter().join(" "))
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector([{}])", self.0.iter().map(|x| format!("{:?}", x)).join(", "))
    }
}

// Generate tests for Vector
#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn display() {
        let vector = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(format!("{}", vector), "[1 2 3]");
    }

    #[test]
    fn debug() {
        let vector = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(format!("{:?}", vector), "Vector([Value::Integer(1), Value::Integer(2), Value::Integer(3)])");
    }

    #[test]
    fn push_back() {
        let mut vector = Vector::new_empty();
        vector.push_back(Value::integer_rc(1));
        vector.push_back(Value::integer_rc(2));
        vector.push_back(Value::integer_rc(3));
        assert_eq!(vector.len(), 3);
        let mut iter = vector.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
    }

    #[test]
    fn get_nth_or_panic_given_index_in_bounds() {
        // arrange
        let vector = Vector::from(vec![
            /* 0 */ Value::integer_rc(3),
            /* 1 */ Value::integer_rc(7),
            /* 2 */ Value::integer_rc(9),
        ]);
        // act
        let nth_1 = vector.get_nth_or_panic(1);
        // assert
        assert_eq!(*nth_1, Value::integer(7));
    }

    #[test]
    #[should_panic]
    fn get_nth_or_panic_given_index_out_of_bounds_panics() {
        for (index, vector) in vec![
            (0, Vector::new_empty()),
            (10, Vector::new_empty()),
            (1, Vector::from(vec![
                Value::nil_rc(),
            ])),
        ] {
            let _ = vector.get_nth_or_panic(index);
        }
    }

    #[test]
    fn get_nth_or_nil() {
        // arrange
        let vector = Vector::from(vec![
            /* 0 */ Value::keyword_unqualified_rc("vanilla"),
            /* 1 */ Value::keyword_unqualified_rc("chocolate"),
            /* 2 */ Value::keyword_unqualified_rc("strawberry"),
        ]);
        // act
        let nth_3 = vector.get_nth_or_nil(3);
        // assert
        assert!(nth_3.is_nil());
    }

    #[test]
    fn get_nth_or() {
        // arrange
        let vector = Vector::from(vec![
            /* 0 */ Value::keyword_unqualified_rc("red"),
            /* 1 */ Value::keyword_unqualified_rc("green"),
            /* 2 */ Value::keyword_unqualified_rc("blue"),
        ]);
        let or_value = Value::keyword_unqualified_rc("unknown");
        // act
        let nth_5 = vector.get_nth_or(5, or_value.clone());
        // assert
        assert_eq!(*nth_5, *or_value);
    }

    #[test]
    fn new_empty_creates_empty_vector() {
        let vector = Vector::new_empty();
        assert_eq!(vector.len(), 0);
    }

    #[test]
    fn new_with_elements() {
        let vector = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(vector.len(), 3);
        let mut iter = vector.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
    }

    #[test]
    fn new_empty_value() {
        let val = Vector::new_empty_value();
        assert!(val.is_vector());
        if let Value::Vector(v, _) = val {
            assert_eq!(v.len(), 0);
        } else {
            panic!("Expected Vector variant");
        }
    }

    #[test]
    fn new_value() {
        let val = Vector::new_value(vec![
            Value::integer_rc(10),
            Value::integer_rc(20),
        ]);
        assert!(val.is_vector());
        if let Value::Vector(v, _) = val {
            assert_eq!(v.len(), 2);
            let mut iter = v.0.iter();
            assert_eq!(**iter.next().unwrap(), Value::integer(10));
        } else {
            panic!("Expected Vector variant");
        }
    }

    #[test]
    fn length_increases_with_push_back() {
        let mut vector = Vector::new_empty();
        assert_eq!(vector.len(), 0);
        vector.push_back(Value::integer_rc(1));
        assert_eq!(vector.len(), 1);
        vector.push_back(Value::integer_rc(2));
        assert_eq!(vector.len(), 2);
        vector.push_back(Value::integer_rc(3));
        assert_eq!(vector.len(), 3);
    }

    #[test]
    fn fifo_ordering_with_push_back() {
        let mut vector = Vector::new_empty();
        vector.push_back(Value::integer_rc(1));
        vector.push_back(Value::integer_rc(2));
        vector.push_back(Value::integer_rc(3));
        // insertion order should be preserved: 1, 2, 3
        let mut iter = vector.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
    }

    #[test]
    fn multiple_pushes_preserve_insertion_order() {
        let mut vector = Vector::new_empty();
        for i in 1..=5 {
            vector.push_back(Value::integer_rc(i));
        }
        // Order should be: 1, 2, 3, 4, 5
        let mut iter = vector.0.iter();
        assert_eq!(**iter.next().unwrap(), Value::integer(1));
        assert_eq!(**iter.next().unwrap(), Value::integer(2));
        assert_eq!(**iter.next().unwrap(), Value::integer(3));
        assert_eq!(**iter.next().unwrap(), Value::integer(4));
        assert_eq!(**iter.next().unwrap(), Value::integer(5));
    }

    #[test]
    fn equality_with_same_elements() {
        let vector1 = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        let vector2 = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        assert_eq!(vector1, vector2);
    }

    #[test]
    fn inequality_with_different_elements() {
        let vector1 = Vector::from(vec![Value::integer_rc(1)]);
        let vector2 = Vector::from(vec![Value::integer_rc(2)]);
        assert_ne!(vector1, vector2);
    }

    #[test]
    fn cloned_vector_equals_original() {
        let vector1 = Vector::from(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        let vector2 = vector1.clone();
        assert_eq!(vector1, vector2);
    }

    #[test]
    fn into_value() {
        let vector = Vector::from(vec![Value::integer_rc(42)]);
        let val = vector.into_value();
        assert!(val.is_vector());
    }

    #[test]
    fn into_value_rc() {
        let vector = Vector::from(vec![Value::integer_rc(99)]);
        let rc_val = vector.into_value_rc();
        assert!(rc_val.is_vector());
    }
}
