use tracing as log;
use crate::prelude::*;

pub fn eval(
    env: RcEnvironment,
    v: RcValue,
) -> RcValue {
    match v.as_ref() {
        Value::Nil(_) => v,
        Value::Symbol(symbol, _) => resolve_or_panic(env.clone(), symbol).deref().expect(&format!("attempted to deref unbound Var: {}", symbol)),
        Value::Keyword(_, _) => v,
        Value::Boolean(_, _) => v,
        Value::Integer(_, _) => v,
        Value::Float(_, _) => v,
        Value::String(_, _) => v,
        Value::List(list, _) => {
            if list.is_empty() { return v; }
            let args: Vec<RcValue> = list.iter().skip(1).map(|value| eval(env.clone(), value.to_owned())).collect();
            let v = eval(env.clone(), list.get_first().unwrap().to_owned());
            apply(env.clone(), v, args)
        },
        Value::Vector(vector, _) => Value::new_vector_rc(vector.iter().map(|value| eval(env.clone(), value.to_owned())).collect()),
        Value::Set(set, _) => Value::new_set_rc(set.iter().map(|value| eval(env.clone(), value.to_owned())).collect()),
        Value::Map(map, _) => Value::new_map_rc(map.iter().map(|(k, v)| (eval(env.clone(), k.to_owned()), eval(env.clone(), v.to_owned()))).collect()),
        Value::Var(var, _) => var.deref().expect("attempted to deref unbound Var"),
        Value::Function(_, _) => v,
        Value::Handle(_, _) => v,
    }
}

pub fn apply(
    env: RcEnvironment,
    f: RcValue,
    args: Vec<RcValue>,
) -> RcValue {
    match f.as_ref() {
        Value::Function(func, _) => func.invoke(env.clone(), args),
        Value::Handle(handle, _) => {
            if let Some(func) = handle.downcast_ref::<Function>() {
                func.invoke(env.clone(), args)
            } else {
                f
            }
        }
        // TODO: properly handle other variants
        _ => {
            eprintln!(
                "Warning: apply called on non-function value: {:?}",
                f
            );
            f
        }
    }
}

pub fn try_resolve(
    env: RcEnvironment,
    symbol: &Symbol,
) -> Result<RcVar, ResolveError> {
    match symbol {
        Symbol::Qualified(sym) => {
            log::warn!("Resolving qualified symbol: {}", sym);
            env.try_get_namespace(sym.namespace())
                .ok_or_else(|| ResolveError::NoSuchNamespace(SymbolUnqualified::new(sym.namespace())))?
                .try_get_var(sym.name())
                .map_err(ResolveError::from)
        },
        Symbol::Unqualified(sym) => {
            log::warn!("Resolving unqualified symbol: {}", sym);
            env.try_get_current_namespace()
               .map_err(|_| ResolveError::UnknownCurrentNamespace)?
               .try_get_var(sym.name())
               .map_err(ResolveError::from)
        },
    }
}

pub fn resolve_or_panic(
    env: RcEnvironment,
    symbol: &Symbol,
) -> RcVar {
    try_resolve(env, symbol)
        .expect(&format!("could not resolve: {}", symbol))
}

#[derive(Debug, Clone)]
pub enum ResolveError {
    NoSuchNamespace(SymbolUnqualified),
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    UnknownCurrentNamespace,
}

impl From<GetVarError> for ResolveError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetValueError> for ResolveError {
    fn from(get_value_err: GetValueError) -> Self {
        match get_value_err {
            GetValueError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
            GetValueError::UnboundVar(var_sym) => Self::UnboundVar(var_sym),
        }
    }
}

