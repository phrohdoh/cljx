
use ::core::fmt;
use ::std::str::FromStr;

use crate::UnqualifiedSymbol;

/// A [Keyword] with a name only (no namespace).
///
/// e.g.: `:n`
///
/// [Keyword]: crate::Value::Keyword
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnqualifiedKeyword(String);


impl UnqualifiedKeyword {
    pub fn new(
        name: String,
    ) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}


/// Helper to make it easier to create an [UnqualifiedKeyword].
pub fn new(
    name: impl ToString,
) -> UnqualifiedKeyword {
    UnqualifiedKeyword::new(
        name.to_string(),
    )
}



impl FromStr for UnqualifiedKeyword {
    type Err = ();
    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(Self(name.to_owned()))
    }
}


impl From<&'_ str> for UnqualifiedKeyword {
    fn from(name: &'_ str) -> Self {
        Self(name.to_owned())
    }
}

impl From<&'_ String> for UnqualifiedKeyword {
    fn from(name: &'_ String) -> Self {
        Self(name.to_owned())
    }
}

impl From<UnqualifiedSymbol> for UnqualifiedKeyword {
    fn from(name: UnqualifiedSymbol) -> Self {
        Self(name.name().to_owned())
    }
}


impl fmt::Display for UnqualifiedKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, ":{}", self.0)
    }
}

impl fmt::Debug for UnqualifiedKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("UnqualifiedKeyword").field(&self.0).finish()
    }
}
