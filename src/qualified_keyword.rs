
use ::core::fmt;
use crate::{QualifiedSymbol, UnqualifiedKeyword};

/// A [Keyword] with a namespace and name.
///
/// e.g.: `:ns/n`
///
/// [Keyword]: crate::Value::Keyword
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedKeyword((String, String));


impl QualifiedKeyword {
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
        &self.0.0
    }

    pub fn name(&self) -> &str {
        &self.0.1
    }
}


/// Helper to make it easier to create a [QualifiedKeyword].
pub fn new(
    namespace: impl ToString,
    name: impl ToString,
) -> QualifiedKeyword {
    QualifiedKeyword::new(
        namespace.to_string(),
        name.to_string(),
    )
}


impl<NS, N> From<(NS, N)> for QualifiedKeyword where NS: Into<UnqualifiedKeyword>, N: Into<UnqualifiedKeyword> {
    fn from((ns, n): (NS, N)) -> Self {
        let ns = ns.into();
        let n = n.into();
        Self((
            ns.name().to_owned(),
            n.name().to_owned(),
        ))
    }
}

//impl From<(UnqualifiedKeyword, UnqualifiedKeyword)> for QualifiedKeyword where NS: Into<UnqualifiedKeyword>, N: Into<UnqualifiedKeyword> {
//    fn from((ns, n): (UnqualifiedKeyword, UnqualifiedKeyword)) -> Self {
//        Self((
//            ns.name().to_owned(),
//            n.name().to_owned(),
//        ))
//    }
//}

impl From<QualifiedSymbol> for QualifiedKeyword {
    fn from(kw: QualifiedSymbol) -> Self {
        Self((
            kw.namespace().to_owned(),
            kw.name().to_owned(),
        ))
    }
}


impl fmt::Display for QualifiedKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (namespace, name) = &self.0;
        write!(f, ":{}/{}", namespace, name)
    }
}

impl fmt::Debug for QualifiedKeyword {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QualifiedKeyword").field(&self.0).finish()
    }
}
