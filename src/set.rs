
use core::fmt::{self, Debug, Display};
use crate::deps::archery::SharedPointer;
use crate::deps::rpds::set::red_black_tree_set::{
    RedBlackTreeSet as RawSet,
    IterRcPtr as RawSetIterRcPtr,
};
use crate::value::Value;
use crate::rt::RcValue;


pub use crate::convert::IntoSet;


pub type PersistentSet = RawSet<Value>;

/// An unordered unique set of [Value]s.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Set(PersistentSet);

type PersistentSetIterRcPtr<'a> = RawSetIterRcPtr<'a, Value>;

////////////////////////////////////////////////////////////////////////////////

impl Set {
    pub fn new_empty() -> Self {
        Self(PersistentSet::new())
    }

    pub fn new(set: PersistentSet) -> Self {
        Self(set)
    }

    #[inline]
    // #[tracing::instrument(name = "Set::len", level = "TRACE")]
    pub fn len(&self) -> usize {
        self.0.size()
    }

    #[inline]
    // #[tracing::instrument(name = "Set::is_empty", level = "TRACE")]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait IPersistentSet {
    fn len(self) -> usize;
    fn contains(self, v: &Value) -> bool;
}

impl<'a> IPersistentSet for &'a Set {
    #[inline]
    fn len(self) -> usize {
        self.0.size()
    }

    #[inline]
    fn contains(self, v: &Value) -> bool {
        self.0.contains(v)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<T> From<T> for Set where T: IntoSet {
    // #[tracing::instrument(name = "Set::from(T:IntoSet)", skip(t), level = "TRACE")]
    fn from(t: T) -> Self {
        t.into_set()
    }
}

impl IntoSet for PersistentSet {
    // #[tracing::instrument(name = "PersistentSet::into_set", skip(self), level = "TRACE")]
    fn into_set(self) -> Set {
        Set::new(self)
    }
}

impl IntoSet for Vec<Value> {
    // #[tracing::instrument(name = "Vec<Value>::into_set", skip(self), level = "TRACE")]
    fn into_set(self) -> Set {
        Set::new(PersistentSet::from_iter(self))
    }
}

impl IntoSet for Vec<RcValue> {
    // #[tracing::instrument(name = "Vec<RcValue>::into_set", skip(self), level = "TRACE")]
    fn into_set(self) -> Set {
        Set::new(PersistentSet::from_iter(
            self.iter()
                .map(SharedPointer::as_ref)
                .map(Clone::clone)
        ))
    }
}


////////////////////////////////////////////////////////////////////////////////

impl Set {
    #[inline]
    pub fn iter<'a>(&'a self) -> PersistentSetIterRcPtr<'a> {
        self.0.iter_ptr()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Default for Set {
    fn default() -> Self {
        Self::new_empty()
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for Set {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use crate::deps::itertools::Itertools as _;
        write!(f, "#{{{}}}",
            self.iter()
                .map(|v| format!("{v}"))
                .join(" "))
    }
}

impl Debug for Set {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
