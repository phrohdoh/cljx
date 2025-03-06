
use core::fmt;
use crate::{QualifiedKeyword, UnqualifiedKeyword, Symbol};

/// A [Symbol] that is always `resolve`d into itself.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Keyword {
    Unqualified(UnqualifiedKeyword),
    Qualified(QualifiedKeyword),
}


impl From<UnqualifiedKeyword> for Keyword {
    fn from(keyword: UnqualifiedKeyword) -> Self {
        Self::Unqualified(keyword)
    }
}

impl From<QualifiedKeyword> for Keyword {
    fn from(keyword: QualifiedKeyword) -> Self {
        Self::Qualified(keyword)
    }
}

impl From<(UnqualifiedKeyword, UnqualifiedKeyword)> for Keyword {
    fn from((ns, n): (UnqualifiedKeyword, UnqualifiedKeyword)) -> Self {
        Self::Qualified(QualifiedKeyword::from((ns, n)))
    }
}

impl From<(Option<UnqualifiedKeyword>, UnqualifiedKeyword)> for Keyword {
    fn from((ns, n): (Option<UnqualifiedKeyword>, UnqualifiedKeyword)) -> Self {
        match ns {
            Some(ns) => Self::Qualified(QualifiedKeyword::from((ns, n))),
            None => Self::Unqualified(n),
        }
    }
}


impl Keyword {
    pub fn is_unqualified(&self) -> bool {
        matches!(self, Self::Unqualified(..))
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified(..))
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Unqualified(kw) => kw.name(),
            Self::Qualified(kw) => kw.name(),
        }
    }

    pub fn maybe_namespace(&self) -> Option<&str> {
        match self {
            Self::Qualified(kw) => Some(kw.namespace()),
            _ => None,
        }
    }

    pub fn namespace(&self) -> &str {
        self.maybe_namespace().expect(&format!("{self} is not a qualified keyword"))
    }
}

impl Keyword {
    pub fn try_as_unqualified(&self) -> Result<&UnqualifiedKeyword, &Self> {
        match self {
            Self::Unqualified(kw) => Ok(kw),
            _ => Err(self),
        }
    }

    pub fn as_unqualified(&self) -> &UnqualifiedKeyword {
        match self {
            Self::Unqualified(kw) => kw,
            _ => panic!("{self} is not an unqualified keyword"),
        }
    }
}

impl Keyword {
    pub fn try_as_qualified(&self) -> Result<&QualifiedKeyword, &Self> {
        match self {
            Self::Qualified(kw) => Ok(kw),
            _ => Err(self),
        }
    }

    pub fn as_qualified(&self) -> &QualifiedKeyword {
        match self {
            Self::Qualified(kw) => kw,
            _ => panic!("{self} is not an qualified keyword"),
        }
    }
}

// -----------------------------------------------------------------------------

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unqualified(sym) => write!(f, "{}", sym),
            Self::Qualified(sym) => write!(f, "{}", sym),
        }
    }
}

impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unqualified(sym) => write!(f, "{:?}", sym),
            Self::Qualified(sym)   => write!(f, "{:?}", sym),
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::value::Value;

    #[test]
    fn unqualified() {
        assert_eq!(
            crate::keyword!("abc"),
            crate::keyword!("abc"),
        );

        assert_ne!(
            crate::keyword!("abc"),
            crate::keyword!("xyz"),
        );
    }

    #[test]
    fn qualified() {
        assert_eq!(
            crate::keyword!("abc", "xyz"),
            crate::keyword!("abc", "xyz"),
        );

        assert_ne!(
            crate::keyword!("abc", "xyz"),
            crate::keyword!("xyz", "abc"),
        );
    }

    #[test]
    fn as_map_key() {
        fn _test(run: usize) {
            let k = crate::keyword!("en", "US");
            let v = crate::string!("Wood");
            let m = crate::map!((k.clone(), v.clone()));
            assert_eq!(
                m.as_map_panicing().get_or_else(&k, || panic!("[run {run}] expect map {map} to have key {key}", run = run, map = m, key = k)),
                &v,
            );
        }
        for i in 0..1 {
            _test(i);
        }
    }

    #[test]
    #[ignore]
    fn as_map_key_complex() {
        fn _test(run: usize) {
            let m = crate::map!(
                (Value::Nil,
                 crate::keyword!("en", "US")),
                (crate::keyword!("en", "US"),
                 crate::string!("Wood")),
            );
            let m = m.as_map_panicing();

            let k = m.get_or_else(&Value::Nil, || panic!("[run {run}] expect map {m} to have key nil"));
            let v = m.get_or_else(k,           || panic!("[run {run}] expect map {m} to have key {k}"));

            assert_eq!(v, &crate::string!("Wood"));
        }
        for i in 1..=10 {
            _test(i);
        }
    }
}
