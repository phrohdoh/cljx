use core::fmt;
use std::{ops, rc::Rc};
use itertools::Itertools;
use crate::{Value, RcValue};

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub struct List(Vec<RcValue>);

impl List {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }

    pub fn new_empty_value() -> Value {
        Value::list(Self(Vec::new()))
    }

    pub fn new_empty_value_rc() -> RcValue {
        Value::list_rc(Self(Vec::new()))
    }

    pub fn new(elements: Vec<RcValue>) -> Self {
        Self(elements)
    }

    pub fn new_value(elements: Vec<RcValue>) -> Value {
        Value::list(Self(elements))
    }

    pub fn new_value_rc(elements: Vec<RcValue>) -> RcValue {
        Value::list_rc(Self(elements))
    }

    pub fn into_value(self) -> Value {
        Value::list(self)
    }

    pub fn into_value_rc(self) -> RcValue {
        Value::list_rc(self)
    }

    pub fn push(&mut self, value: RcValue) {
        self.push_front(value)
    }

    pub fn push_front(&mut self, value: RcValue) {
        self.0.insert(0, value);
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

impl ops::Deref for List {
    type Target = Vec<RcValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.0.iter().join(" "))
    }
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "List([{}])", self.0.iter().map(|x| format!("{:?}", x)).join(", "))
    }
}

// Generate tests for List
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display() {
        let list = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(format!("{}", list), "(1 2 3)");
    }

    #[test]
    fn debug() {
        let list = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(format!("{:?}", list), "List([Value::Integer(1), Value::Integer(2), Value::Integer(3)])");
    }

    #[test]
    fn push_front() {
        // arrange
        let mut list = List::new_empty();
        // act
        list.push_front(Value::integer_rc(1));
        list.push_front(Value::integer_rc(2));
        list.push_front(Value::integer_rc(3));
        // assert
        assert_eq!(list.len(), 3);
        assert_eq!(*list[0], Value::integer(3));
        assert_eq!(*list[1], Value::integer(2));
        assert_eq!(*list[2], Value::integer(1));
    }

    #[test]
    fn get_nth_panicing_given_index_in_bounds() {
        // arrange
        let list = List::new(vec![
            /* 0 */ Value::integer_rc(3),
            /* 1 */ Value::integer_rc(7),
            /* 2 */ Value::integer_rc(9),
        ]);
        // act
        let nth_1 = list.get_nth_panicing(1);
        // assert
        assert_eq!(*nth_1, Value::integer(7));
    }

    #[test]
    #[should_panic]
    fn get_nth_panicing_given_index_out_of_bounds_panics() {
        for (index, list) in vec![
            (0, List::new_empty()),
            (10, List::new_empty()),
            (1, List::new(vec![
                Rc::new(Value::nil()),
            ])),
        ] {
            let _ = list.get_nth_panicing(index);
        }
    }

    #[test]
    fn get_nth_or_nil() {
        // arrange
        let list = List::new(vec![
            /* 0 */ Value::keyword_unqualified_rc("vanilla"),
            /* 1 */ Value::keyword_unqualified_rc("chocolate"),
            /* 2 */ Value::keyword_unqualified_rc("strawberry"),
        ]);
        // act
        let nth_3 = list.get_nth_or_nil(3);
        // assert
        assert!(nth_3.is_nil());
    }

    #[test]
    fn get_nth_or() {
        // arrange
        let list = List::new(vec![
            /* 0 */ Value::keyword_unqualified_rc("red"),
            /* 1 */ Value::keyword_unqualified_rc("green"),
            /* 2 */ Value::keyword_unqualified_rc("blue"),
        ]);
        let or_value = Value::keyword_unqualified_rc("unknown");
        // act
        let nth_5 = list.get_nth_or(5, or_value.clone());
        // assert
        assert_eq!(*nth_5, *or_value);
    }

    #[test]
    fn new_empty_creates_empty_list() {
        let list = List::new_empty();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn new_with_elements() {
        let list = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        assert_eq!(list.len(), 3);
        assert_eq!(*list[0], Value::integer(1));
    }

    #[test]
    fn new_empty_value() {
        let val = List::new_empty_value();
        assert!(val.is_list());
        if let crate::Value::List(list, _) = val {
            assert_eq!(list.len(), 0);
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn new_value() {
        let val = List::new_value(vec![
            Value::integer_rc(10),
            Value::integer_rc(20),
        ]);
        assert!(val.is_list());
        if let crate::Value::List(list, _) = val {
            assert_eq!(list.len(), 2);
            assert_eq!(*list[0], Value::integer(10));
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn length_increases_with_push_front() {
        let mut list = List::new_empty();
        assert_eq!(list.len(), 0);
        list.push_front(Value::integer_rc(1));
        assert_eq!(list.len(), 1);
        list.push_front(Value::integer_rc(2));
        assert_eq!(list.len(), 2);
        list.push_front(Value::integer_rc(3));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn lifo_ordering_with_push_front() {
        let mut list = List::new_empty();
        list.push_front(Value::integer_rc(1));
        list.push_front(Value::integer_rc(2));
        list.push_front(Value::integer_rc(3));
        // newest (3) should be at front, then 2, then 1
        assert_eq!(*list[0], Value::integer(3));
        assert_eq!(*list[1], Value::integer(2));
        assert_eq!(*list[2], Value::integer(1));
    }

    #[test]
    fn multiple_pushes_preserve_order() {
        let mut list = List::new_empty();
        for i in 1..=5 {
            list.push_front(Value::integer_rc(i));
        }
        // Order should be: 5, 4, 3, 2, 1
        assert_eq!(*list[0], Value::integer(5));
        assert_eq!(*list[1], Value::integer(4));
        assert_eq!(*list[2], Value::integer(3));
        assert_eq!(*list[3], Value::integer(2));
        assert_eq!(*list[4], Value::integer(1));
    }

    #[test]
    fn equality_with_same_elements() {
        let list1 = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        let list2 = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
        ]);
        assert_eq!(list1, list2);
    }

    #[test]
    fn inequality_with_different_elements() {
        let list1 = List::new(vec![Value::integer_rc(1)]);
        let list2 = List::new(vec![Value::integer_rc(2)]);
        assert_ne!(list1, list2);
    }

    #[test]
    fn cloned_list_equals_original() {
        let list1 = List::new(vec![
            Value::integer_rc(1),
            Value::integer_rc(2),
            Value::integer_rc(3),
        ]);
        let list2 = list1.clone();
        assert_eq!(list1, list2);
    }

    #[test]
    fn into_value() {
        let list = List::new(vec![Value::integer_rc(42)]);
        let val = list.into_value();
        assert!(val.is_list());
    }

    #[test]
    fn into_value_rc() {
        let list = List::new(vec![Value::integer_rc(99)]);
        let rc_val = list.into_value_rc();
        assert!(rc_val.is_list());
    }
}
