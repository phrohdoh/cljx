
use ::core::fmt;
use crate::{UnqualifiedSymbol, QualifiedKeyword};

/// A [Symbol] with a namespace and name.
///
/// e.g.: `ns/n`
///
/// [Symbol]: crate::Value::Symbol
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedSymbol(pub(super) (String, String));

impl QualifiedSymbol {
    pub fn new(
        namespace: String,
        name: String,
    ) -> Self {
        Self((
            namespace,
            name,
        ))
    }

    pub fn namespace(&self) -> &str {
        let (namespace, _name) = &self.0;
        namespace
    }

    pub fn name(&self) -> &str {
        let (_namespace, name) = &self.0;
        name
    }
}

/// Helper to make it easier to create a [QualifiedSymbol].
pub fn new(
    namespace: impl ToString,
    name: impl ToString,
) -> QualifiedSymbol {
    QualifiedSymbol::new(
        namespace.to_string(),
        name.to_string(),
    )
}

impl<NS, N> From<(NS, N)> for QualifiedSymbol where NS: Into<UnqualifiedSymbol>, N: Into<UnqualifiedSymbol> {
    fn from((ns, n): (NS, N)) -> Self {
        let ns = ns.into();
        let n = n.into();
        Self((
            ns.name().to_owned(),
            n.name().to_owned(),
        ))
    }
}

impl From<QualifiedKeyword> for QualifiedSymbol {
    fn from(kw: QualifiedKeyword) -> Self {
        Self((
            kw.namespace().to_owned(),
            kw.name().to_owned(),
        ))
    }
}


impl fmt::Display for QualifiedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (namespace, name) = &self.0;
        write!(f, "{}/{}", namespace, name)
    }
}

impl fmt::Debug for QualifiedSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("QualifiedSymbol").field(&self.0).finish()
    }
}
