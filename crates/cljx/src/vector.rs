use core::fmt;
use std::{ops, rc::Rc};
use itertools::Itertools;
use crate::{Value, RcValue};

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct Vector(Vec<RcValue>);

impl Vector {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    pub fn new_empty_value() -> Value {
        Value::vector(Self(Vec::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::vector_rc(Self(Vec::new()))
    }

    pub fn new(elements: Vec<RcValue>) -> Self {
        Self(elements)
    }

    pub fn new_value(elements: Vec<RcValue>) -> Value {
        Value::vector(Self(elements))
    }

    pub fn new_value_rc(elements: Vec<RcValue>) -> RcValue {
        Value::vector_rc(Self(elements))
    }

    pub fn into_value(self) -> Value {
        Value::vector(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::vector_rc(self)
    }

    pub fn push(&mut self, value: RcValue) {
        self.push_back(value)
    }

    pub fn push_back(&mut self, value: RcValue) {
        self.0.push(value);
    }

    pub fn get_nth_panicing(&self, n: usize) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap()
    }

    pub fn get_nth_or_nil(&self, n: usize) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or(Value::nil_rc())
    }

    pub fn get_nth_or(&self, n: usize, or: RcValue) -> RcValue {
        self.0.get(n).map(|v| v.to_owned()).unwrap_or(or)
    }
}

impl ops::Deref for Vector {
    type Target = Vec<RcValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    #[test]
    fn display() {
        let vector = Vector::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(format!("{}", vector), "[1 2 3]");
    }

    #[test]
    fn debug() {
        let vector = Vector::new(vec![
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
        assert_eq!(*vector[0], Value::integer(1));
        assert_eq!(*vector[1], Value::integer(2));
        assert_eq!(*vector[2], Value::integer(3));
    }

    #[test]
    fn get_nth_panicing_given_index_in_bounds() {
        // arrange
        let vector = Vector::new(vec![
            /* 0 */ Value::integer_rc(3),
            /* 1 */ Value::integer_rc(7),
            /* 2 */ Value::integer_rc(9),
        ]);
        // act
        let nth_1 = vector.get_nth_panicing(1);
        // assert
        assert_eq!(*nth_1, Value::integer(7));
    }

    #[test]
    #[should_panic]
    fn get_nth_panicing_given_index_out_of_bounds_panics() {
        for (index, vector) in vec![
            (0, Vector::new_empty()),
            (10, Vector::new_empty()),
            (1, Vector::new(vec![
                Rc::new(Value::nil()),
            ])),
        ] {
            let _ = vector.get_nth_panicing(index);
        }
    }

    #[test]
    fn get_nth_or_nil() {
        // arrange
        let vector = Vector::new(vec![
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
        let vector = Vector::new(vec![
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
}
