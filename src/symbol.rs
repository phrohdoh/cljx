
use core::fmt;
use crate::{UnqualifiedSymbol, QualifiedSymbol, Value};

/// A name, possibly with a namespace, usually intended to be `resolve`d into a [Value].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Unqualified(UnqualifiedSymbol),
    Qualified(QualifiedSymbol),
}


impl From<UnqualifiedSymbol> for Symbol {
    fn from(symbol: UnqualifiedSymbol) -> Self {
        Self::Unqualified(symbol)
    }
}

impl From<QualifiedSymbol> for Symbol {
    fn from(symbol: QualifiedSymbol) -> Self {
        Self::Qualified(symbol)
    }
}

impl From<(UnqualifiedSymbol, UnqualifiedSymbol)> for Symbol {
    fn from((ns, n): (UnqualifiedSymbol, UnqualifiedSymbol)) -> Self {
        Self::Qualified(QualifiedSymbol::from((ns, n)))
    }
}

impl From<(Option<UnqualifiedSymbol>, UnqualifiedSymbol)> for Symbol {
    fn from((ns, n): (Option<UnqualifiedSymbol>, UnqualifiedSymbol)) -> Self {
        match ns {
            Some(ns) => Self::Qualified(QualifiedSymbol::from((ns, n))),
            None => Self::Unqualified(n),
        }
    }
}


impl Symbol {
    pub fn is_unqualified(&self) -> bool {
        matches!(self, Self::Unqualified(..))
    }

    pub fn is_qualified(&self) -> bool {
        matches!(self, Self::Qualified(..))
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Unqualified(sym) => sym.name(),
            Self::Qualified(sym) => sym.name(),
        }
    }

    pub fn namespace(&self) -> Option<&str> {
        match self {
            Self::Qualified(kw) => Some(kw.namespace()),
            _ => None,
        }
    }

    pub fn namespace_panicing(&self) -> &str {
        self.namespace().expect(&format!("{self} is not a qualified keyword"))
    }
}


impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unqualified(sym) => write!(f, "{}", sym),
            Self::Qualified(sym) => write!(f, "{}", sym),
        }
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unqualified(sym) => write!(f, "{}", sym),
            Self::Qualified(sym) => write!(f, "{}", sym),
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn unqualified() {
        assert_eq!(
            crate::symbol!("abc"),
            crate::symbol!("abc"),
        );

        assert_ne!(
            crate::symbol!("abc"),
            crate::symbol!("xyz"),
        );
    }

    #[test]
    fn qualified() {
        assert_eq!(
            crate::symbol!("abc", "xyz"),
            crate::symbol!("abc", "xyz"),
        );

        assert_ne!(
            crate::symbol!("abc", "xyz"),
            crate::symbol!("xyz", "abc"),
        );
    }
}
