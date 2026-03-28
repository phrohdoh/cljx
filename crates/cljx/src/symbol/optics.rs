use crate::prelude::*;

pub fn view_unqualified(symbol: Symbol) -> Option<SymbolUnqualified> {
    match symbol {
        Symbol::Unqualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn view_unqualified_ref(symbol: &Symbol) -> Option<&SymbolUnqualified> {
    match symbol {
        Symbol::Unqualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn view_qualified(symbol: Symbol) -> Option<SymbolQualified> {
    match symbol {
        Symbol::Qualified(symbol) => Some(symbol),
        _ => None,
    }
}

pub fn view_qualified_ref(symbol: &Symbol) -> Option<&SymbolQualified> {
    match symbol {
        Symbol::Qualified(symbol) => Some(symbol),
        _ => None,
    }
}
