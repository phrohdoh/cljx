
use ::core::cell::RefCell;
use ::std::rc::Rc;
use crate::{RcValue, Value};

type MutablePlace = RefCell<Option<RcValue>>;

/// A mutable place, possibly with a bound [Value].
///
/// e.g.: `#'clojure.core/assoc`
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Var(MutablePlace);


impl From<Var> for Value {
    fn from(var: Var) -> Self {
        Self::Var(var.into())
    }
}

impl From<Rc<Var>> for Value {
    fn from(var: Rc<Var>) -> Self {
        Self::Var(var.into())
    }
}


impl Var {
    pub fn new_unbound() -> Self {
        Self(MutablePlace::new(None))
    }

    pub fn new_bound(v: RcValue) -> Self {
        Self(MutablePlace::new(Some(v)))
    }

    pub fn is_bound(&self) -> bool {
        self.0.borrow().is_some()
    }

    pub fn is_unbound(&self) -> bool {
        self.0.borrow().is_none()
    }

    pub fn deref(&self) -> Option<RcValue> {
        self.0.borrow().as_ref().cloned()
    }

    pub fn bind(&self, v: impl Into<RcValue>) {
        self::bind(self, v.into());
    }

    pub fn unbind(&self) {
        *self.0.borrow_mut() = None;
    }
}

fn bind(var: &Var, value: RcValue) {
    crate::deps::tracing::trace!(
    // eprintln!(
        "(bind {} {})",
        "some-var", // var.deref().unwrap_or(crate::Value::Nil.into()),
        value,
    );
    *var.0.borrow_mut() = Some(value);
}
