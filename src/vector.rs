
use core::fmt::{self, Debug, Display};
use crate::deps::tracing;
use crate::deps::archery::{SharedPointer, RcK};
use crate::deps::rpds::{self, vector::IterPtr};
use crate::value::Value;
use crate::rt::RcValue;

pub use crate::convert::IntoVector;

pub type PersistentVector = rpds::Vector<Value>;

/// An ordered sequence of [Value]s with O(1) lookup and mutations applied at the end.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Vector(PersistentVector);

impl<T> From<T> for Vector where T: IntoVector {
    // #[tracing::instrument(name = "Vector::from(T:IntoVector)", skip(t), level = "TRACE")]
    fn from(t: T) -> Self {
        t.into_vector()
    }
}

impl IntoVector for PersistentVector {
    // #[tracing::instrument(name = "PersistentVector::into_vector", skip(self), level = "TRACE")]
    fn into_vector(self) -> Vector {
        Vector::new(self)
    }
}

impl IntoVector for Vec<Value> {
    // #[tracing::instrument(name = "Vec<Value>::into_vector", skip(self), level = "TRACE")]
    fn into_vector(self) -> Vector {
        Vector::new(PersistentVector::from_iter(self))
    }
}

impl IntoVector for Vec<RcValue> {
    // #[tracing::instrument(name = "Vec<RcValue>::into_vector", skip(self), level = "TRACE")]
    fn into_vector(self) -> Vector {
        Vector::new(PersistentVector::from_iter(
            self.iter()
                .map(SharedPointer::as_ref)
                .map(Clone::clone)
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Vector {
    // #[tracing::instrument(name = "Vector::new_empty", level = "TRACE")]
    pub fn new_empty() -> Self {
        Self(PersistentVector::new())
    }

    #[inline]
    // #[tracing::instrument(name = "Vector::len", level = "TRACE")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    // #[tracing::instrument(name = "Vector::is_empty", level = "TRACE")]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    // #[tracing::instrument(name = "Vector::is_nonempty", level = "TRACE")]
    pub fn is_nonempty(&self) -> bool {
        !self.0.is_empty()
    }

    #[inline]
    // #[tracing::instrument(name = "Vector::iter", level = "TRACE")]
    pub fn iter<'a>(&'a self) -> IterPtr<'a, Value, RcK> {
        self.0.iter_ptr()
    }

    // #[tracing::instrument(name = "Vector::last", skip(self), level = "TRACE")]
    pub fn last(&self) -> Option<RcValue> {
        self.0.last_ptr().map(Clone::clone)
    }

    // #[tracing::instrument(name = "Vector::rest", skip(self), level = "TRACE")]
    pub fn rest(&self) -> Self {
        let values = self.0.iter_ptr()
            .skip(1)
            .map(SharedPointer::as_ref)
            .map(ToOwned::to_owned)
            // .collect::<Vec<_>>()
            ;
        Self(PersistentVector::from_iter(values))
    }
}

impl Vector {
    // #[tracing::instrument(name = "Vector::first", skip(self), level = "DEBUG")]
    pub fn first(&self) -> Option<RcValue> {
        self.0.first_ptr().map(Clone::clone)
    }

    // #[tracing::instrument(name = "Vector::first_or_nil", skip(self), level = "DEBUG")]
    pub fn first_or_nil(&self) -> RcValue {
        self.first().unwrap_or(Value::Nil.into())
    }

    // #[tracing::instrument(name = "Vector::first_or", skip(self, or_value), level = "DEBUG")]
    pub fn first_or(&self, or_value: RcValue) -> RcValue {
        self.first().unwrap_or(or_value)
    }

    // #[tracing::instrument(name = "Vector::first_or_else", skip(self, else_fn), level = "DEBUG")]
    pub fn first_or_else<F: FnOnce() -> RcValue>(&self, else_fn: F) -> RcValue {
        self.first().unwrap_or_else(else_fn)
    }
}

//feature! {
//    #![feature = "export-inner-types"]
    impl Vector {
        #[tracing::instrument(
            name = "Vector::new",
            skip(list),
            // ret(Display, level = "DEBUG"),
        )]
        pub fn new(list: PersistentVector) -> Self {
            tracing::trace!("Vector::new");
            Self(list)
        }
    }
//}

//feature! {
//    #![feature = "mut-api"]
    impl Vector {
        #[inline]
        pub fn push_back(&self, value: Value) -> Self {
            Self(self.0.push_back(value))
        }

        #[inline]
        pub fn push_back_mut(&mut self, value: Value) {
            self.0.push_back_mut(value);
        }

        #[inline]
        pub fn drop_last_mut(&mut self) -> bool {
            self.0.drop_last_mut()
        }
    }
//}

pub trait IPersistentVector {
    fn len(self) -> usize;
    fn contains(self, v: &Value) -> bool;
}

impl<'a> IPersistentVector for &'a Vector {
    #[inline]
    fn len(self) -> usize {
        self.0.len()
    }

    #[inline]
    fn contains(self, v: &Value) -> bool {
        self.0.iter().find(|x| *x == v).is_some()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Default for Vector {
    fn default() -> Self {
        Self::new_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for Vector {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use crate::deps::itertools::Itertools as _;
        write!(f, "[{}]",
            self.iter()
                .map(|v| format!("{v}"))
                .join(" "))
    }
}

impl Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
