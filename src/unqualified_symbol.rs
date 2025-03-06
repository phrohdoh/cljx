
use ::core::fmt;
use ::std::str::FromStr;

use crate::UnqualifiedKeyword;

/// A [Symbol] with a name only (no namespace).
///
/// e.g.: `n`
///
/// [Symbol]: crate::Value::Symbol
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnqualifiedSymbol(pub(super) String);


impl UnqualifiedSymbol {
    pub fn new(
        name: String,
    ) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}


/// Helper to make it easier to create an [UnqualifiedSymbol].
pub fn new(
    name: impl ToString,
) -> UnqualifiedSymbol {
    UnqualifiedSymbol::new(
        name.to_string(),
    )
}


impl FromStr for UnqualifiedSymbol {
    type Err = ();
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(Self(name.to_owned()))
    }
}

impl From<&'_ str> for UnqualifiedSymbol {
    fn from(name: &'_ str) -> Self {
        Self(name.to_owned())
    }
}

impl From<&'_ String> for UnqualifiedSymbol {
    fn from(name: &'_ String) -> Self {
        Self(name.to_owned())
    }
}

impl From<UnqualifiedKeyword> for UnqualifiedSymbol {
    fn from(kw: UnqualifiedKeyword) -> Self {
        Self(kw.name().to_owned())
    }
}


impl fmt::Display for UnqualifiedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for UnqualifiedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnqualifiedSymbol").field(&self.0).finish()
    }
}
