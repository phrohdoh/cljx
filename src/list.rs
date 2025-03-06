
use core::fmt::{self, Debug, Display};
use crate::deps::tracing;
use crate::deps::archery::{SharedPointer, RcK};
use crate::deps::rpds::{List as RawList, list::IterPtr};
use crate::value::Value;
use crate::rt::RcValue;

pub use crate::convert::IntoList;

////////////////////////////////////////////////////////////////////////////////

pub type PersistentList = RawList<Value>;


/// An ordered sequence of [Value]s with O(n) lookup and mutations applied at the beginning.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct List(PersistentList);

////////////////////////////////////////////////////////////////////////////////

impl<T> From<T> for List where T: IntoList {
    // #[tracing::instrument(name = "List::from(T:IntoList)", skip(t), level = "TRACE")]
    fn from(t: T) -> Self {
        t.into_list()
    }
}

impl IntoList for PersistentList {
    // #[tracing::instrument(name = "PersistentList::into_list", skip(self), level = "TRACE")]
    fn into_list(self) -> List {
        List::new(self)
    }
}

impl IntoList for Vec<Value> {
    // #[tracing::instrument(name = "Vec<Value>::into_list", skip(self), level = "TRACE")]
    fn into_list(self) -> List {
        List::new(PersistentList::from_iter(self))
    }
}

impl IntoList for Vec<RcValue> {
    // #[tracing::instrument(name = "Vec<RcValue>::into_list", skip(self), level = "TRACE")]
    fn into_list(self) -> List {
        List::new(PersistentList::from_iter(
            self.iter()
                .map(SharedPointer::as_ref)
                .map(Clone::clone)
        ))
    }
}

////////////////////////////////////////////////////////////////////////////////

impl List {
    // #[tracing::instrument(name = "List::new_empty", level = "TRACE")]
    pub fn new_empty() -> Self {
        Self(PersistentList::new())
    }

    #[inline]
    // #[tracing::instrument(name = "List::len", level = "TRACE")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    // #[tracing::instrument(name = "List::is_empty", level = "TRACE")]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    // #[tracing::instrument(name = "List::is_nonempty", level = "TRACE")]
    pub fn is_nonempty(&self) -> bool {
        !self.0.is_empty()
    }

    #[inline]
    // #[tracing::instrument(name = "List::iter", level = "TRACE")]
    pub fn iter<'a>(&'a self) -> IterPtr<'a, Value, RcK> {
        self.0.iter_ptr()
    }

    // #[tracing::instrument(name = "List::last", skip(self), level = "TRACE")]
    pub fn last(&self) -> Option<RcValue> {
        self.0.last_ptr().map(Clone::clone)
    }

    // #[tracing::instrument(name = "List::rest", skip(self), level = "TRACE")]
    pub fn rest(&self) -> Self {
        self.0
            .drop_first().map(|p_list| Self(p_list))
            .unwrap_or_default()
    }
}

//feature! {
//    #![feature = "list-exts"]
    impl List {
        // #[tracing::instrument(name = "List::first", skip(self), level = "DEBUG")]
        pub fn first(&self) -> Option<RcValue> {
            self.0.first_ptr().map(Clone::clone)
        }

        // #[tracing::instrument(name = "List::first_or_nil", skip(self), level = "DEBUG")]
        pub fn first_or_nil(&self) -> RcValue {
            self.first().unwrap_or(Value::Nil.into())
        }

        // #[tracing::instrument(name = "List::first_or", skip(self, or_value), level = "DEBUG")]
        pub fn first_or(&self, or_value: RcValue) -> RcValue {
            self.first().unwrap_or(or_value)
        }

        // #[tracing::instrument(name = "List::first_or_else", skip(self, else_fn), level = "DEBUG")]
        pub fn first_or_else<F: FnOnce() -> RcValue>(&self, else_fn: F) -> RcValue {
            self.first().unwrap_or_else(else_fn)
        }
    }
//}

//feature! {
//    #![not(feature = "export-inner-types")]
//    impl List {
//        #[tracing::instrument(
//            name = "List::new",
//            skip(list),
//            // ret(Display, level = "DEBUG"),
//        )]
//        pub(crate) fn new(list: PersistentList) -> Self {
//            // tracing::trace!("List::new");
//            Self(list)
//        }
//    }
//}

//feature! {
//    #![feature = "export-inner-types"]
    impl List {
        #[tracing::instrument(
            name = "List::new",
            skip(list),
            // ret(Display, level = "DEBUG"),
        )]
        pub fn new(list: PersistentList) -> Self {
            // tracing::trace!("List::new");
            Self(list)
        }
    }
//}

//feature! {
//    #![feature = "mut-api"]
    impl List {
        #[inline]
        pub fn push_front(&self, value: Value) -> Self {
            Self(self.0.push_front(value))
        }

        #[inline]
        pub fn push_front_mut(&mut self, value: Value) {
            self.0.push_front_mut(value);
        }

        #[inline]
        pub fn drop_first_mut(&mut self) {
            self.0.drop_first_mut();
        }
    }
//}

pub trait IPersistentList {
    fn len(self) -> usize;
    fn contains(self, v: &Value) -> bool;
}

impl<'a> IPersistentList for &'a List {
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

impl Default for List {
    fn default() -> Self {
        Self::new_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for List {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use crate::deps::itertools::Itertools as _;
        write!(f, "({})",
            self.iter()
                .map(|v| format!("{v}"))
                .join(" "))
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
