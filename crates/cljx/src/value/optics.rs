use std::rc::Rc;
use crate::prelude::*;

/// Helper: extract metadata from any Value, preserving it across transformations.
pub fn meta(value: &Value) -> Option<Rc<Map>> {
    match value {
        Value::Nil(m) => m.clone(),
        Value::Boolean(_, m) => m.clone(),
        Value::Integer(_, m) => m.clone(),
        Value::Float(_, m) => m.clone(),
        Value::String(_, m) => m.clone(),
        Value::Symbol(_, m) => m.clone(),
        Value::Keyword(_, m) => m.clone(),
        Value::List(_, m) => m.clone(),
        Value::Vector(_, m) => m.clone(),
        Value::Set(_, m) => m.clone(),
        Value::Map(_, m) => m.clone(),
        Value::Var(_, m) => m.clone(),
        Value::Function(_, m) => m.clone(),
        Value::Handle(_, m) => m.clone(),
    }
}

pub fn meta_ref(value: &Value) -> &Option<Rc<Map>> {
    match value {
        Value::Nil(m) => m,
        Value::Boolean(_, m) => m,
        Value::Integer(_, m) => m,
        Value::Float(_, m) => m,
        Value::String(_, m) => m,
        Value::Symbol(_, m) => m,
        Value::Keyword(_, m) => m,
        Value::List(_, m) => m,
        Value::Vector(_, m) => m,
        Value::Set(_, m) => m,
        Value::Map(_, m) => m,
        Value::Var(_, m) => m,
        Value::Function(_, m) => m,
        Value::Handle(_, m) => m,
    }
}

pub fn view_meta(value: &Value) -> Option<Map> {
    meta(value).as_ref().map(|rc_map| (**rc_map).clone())
}

pub fn view_meta_ref(value: &Value) -> Option<&Map> {
    meta_ref(value).as_ref().and_then(|rc_map| Some(rc_map.as_ref()))
}

pub fn set_meta(value: &Value, meta: Option<Rc<Map>>) -> RcValue {
    value.with_meta_rc(meta)
}


/// View the unit value within a [`Value::Nil`], returning `None` if the variant doesn't match.
pub fn view_nil(value: &Value) -> Option<()> {
    if let Value::Nil(_) = value {
        Some(())
    } else {
        None
    }
}

/// Set (recreate) a `Nil` variant, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Nil`].
pub fn set_nil(value: &Value, _: ()) -> Option<Value> {
    if let Value::Nil(_) = value {
        Some(Value::Nil(meta(value)))
    } else {
        None
    }
}


/// View the boolean value within a [`Value::Boolean`], returning `None` if the variant doesn't match.
pub fn view_boolean(value: &Value) -> Option<bool> {
    if let Value::Boolean(b, _) = value {
        Some(*b)
    } else {
        None
    }
}

/// Set a boolean value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Boolean`].
pub fn set_boolean(value: &Value, b: bool) -> Option<Value> {
    if let Value::Boolean(_, _) = value {
        Some(Value::Boolean(b, meta(value)))
    } else {
        None
    }
}


/// View the integer value within a [`Value::Integer`], returning `None` if the variant doesn't match.
pub fn view_integer(value: &Value) -> Option<i64> {
    if let Value::Integer(i, _) = value {
        Some(*i)
    } else {
        None
    }
}

/// Set an integer value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Integer`].
pub fn set_integer(value: &Value, i: i64) -> Option<Value> {
    if let Value::Integer(_, _) = value {
        Some(Value::Integer(i, meta(value)))
    } else {
        None
    }
}


/// View the [`Float`] within a [`Value::Float`], returning `None` if the variant doesn't match.
pub fn view_float(value: &Value) -> Option<Float> {
    if let Value::Float(f, _) = value {
        Some(f.clone())
    } else {
        None
    }
}

/// Set a [`Float`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Float`].
pub fn set_float(value: &Value, f: Float) -> Option<Value> {
    if let Value::Float(_, _) = value {
        Some(Value::Float(f, meta(value)))
    } else {
        None
    }
}


/// View the [`String`] within a [`Value::String`], returning `None` if the variant doesn't match.
pub fn view_string(value: &Value) -> Option<String> {
    if let Value::String(s, _) = value {
        Some(s.clone())
    } else {
        None
    }
}

/// Set a [`String`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::String`].
pub fn set_string(value: &Value, s: String) -> Option<Value> {
    if let Value::String(_, _) = value {
        Some(Value::String(s, meta(value)))
    } else {
        None
    }
}


/// View the [`Symbol`] within a [`Value::Symbol`], returning `None` if the variant doesn't match.
pub fn view_symbol(value: &Value) -> Option<Symbol> {
    if let Value::Symbol(s, _) = value {
        Some(s.clone())
    } else {
        None
    }
}

/// Set a [`Symbol`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Symbol`].
pub fn set_symbol(value: &Value, s: Symbol) -> Option<Value> {
    if let Value::Symbol(_, _) = value {
        Some(Value::Symbol(s, meta(value)))
    } else {
        None
    }
}


/// View the [`Keyword`] within a [`Value::Keyword`], returning `None` if the variant doesn't match.
pub fn view_keyword(value: &Value) -> Option<Keyword> {
    if let Value::Keyword(k, _) = value {
        Some(k.clone())
    } else {
        None
    }
}

/// Set a [`Keyword`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Keyword`].
pub fn set_keyword(value: &Value, k: Keyword) -> Option<Value> {
    if let Value::Keyword(_, _) = value {
        Some(Value::Keyword(k, meta(value)))
    } else {
        None
    }
}


/// View the [`List`] within a [`Value::List`], returning `None` if the variant doesn't match.
///
/// Since [`List`] is immutable, this returns an owned clone. The clone is `O(1)` due to
/// structural sharing in the underlying persistent data structure.
pub fn view_list(value: &Value) -> Option<List> {
    if let Value::List(list, _) = value {
        Some(list.clone())
    } else {
        None
    }
}

/// Set a [`List`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::List`].
pub fn set_list(value: &Value, list: List) -> Option<Value> {
    if let Value::List(_, meta) = value {
        Some(Value::List(list, meta.clone()))
    } else {
        None
    }
}

/// Create a [`Prism`] focusing on the `List` variant of [`Value`].
pub fn prism_list() -> Prism<Value, List> {
    Prism::new(view_list, set_list)
}

/// View the [`List`] within a [`Value::List`] by reference.
/// Returns `None` if the variant doesn't match.
pub fn view_list_ref(value: &Value) -> Option<&List> {
    if let Value::List(list, _) = value {
        Some(list)
    } else {
        None
    }
}

/// Create a [`PrismRef`] focusing on the `List` variant of [`Value`] with borrowed access.
pub fn prism_list_ref<'s>() -> PrismRef<'s, Value, List> {
    PrismRef::new(view_list_ref, set_list)
}


/// View the [`Vector`] within a [`Value::Vector`], returning `None` if the variant doesn't match.
///
/// Since [`Vector`] is immutable, this returns an owned clone. The clone is `O(1)` due to
/// structural sharing in the underlying persistent data structure.
pub fn view_vector(value: &Value) -> Option<Vector> {
    if let Value::Vector(v, _) = value {
        Some(v.clone())
    } else {
        None
    }
}

/// Set a [`Vector`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Vector`].
pub fn set_vector(value: &Value, v: Vector) -> Option<Value> {
    if let Value::Vector(_, _) = value {
        Some(Value::Vector(v, meta(value)))
    } else {
        None
    }
}

/// View the [`Vector`] within a [`Value::Vector`] by reference.
/// Returns `None` if the variant doesn't match.
pub fn view_vector_ref(value: &Value) -> Option<&Vector> {
    if let Value::Vector(vector, _) = value {
        Some(vector)
    } else {
        None
    }
}

/// Create a [`PrismRef`] focusing on the `Vector` variant of [`Value`] with borrowed access.
pub fn prism_vector_ref<'s>() -> PrismRef<'s, Value, Vector> {
    PrismRef::new(view_vector_ref, set_vector)
}



/// View the [`Set`] within a [`Value::Set`], returning `None` if the variant doesn't match.
///
/// Since [`Set`] is immutable, this returns an owned clone. The clone is `O(1)` due to
/// structural sharing in the underlying persistent data structure.
pub fn view_set(value: &Value) -> Option<Set> {
    if let Value::Set(s, _) = value {
        Some(s.clone())
    } else {
        None
    }
}

/// Set a [`Set`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Set`].
pub fn set_set(value: &Value, s: Set) -> Option<Value> {
    if let Value::Set(_, _) = value {
        Some(Value::Set(s, meta(value)))
    } else {
        None
    }
}


/// View the [`Map`] within a [`Value::Map`], returning `None` if the variant doesn't match.
///
/// Since [`Map`] is immutable, this returns an owned clone. The clone is `O(1)` due to
/// structural sharing in the underlying persistent data structure.
pub fn view_map(value: &Value) -> Option<Map> {
    if let Value::Map(m, _) = value {
        Some(m.clone())
    } else {
        None
    }
}

/// Set a [`Map`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Map`].
pub fn set_map(value: &Value, m: Map) -> Option<Value> {
    if let Value::Map(_, _) = value {
        Some(Value::Map(m, meta(value)))
    } else {
        None
    }
}


/// View the [`RcVar`] within a [`Value::Var`], returning `None` if the variant doesn't match.
pub fn view_var(value: &Value) -> Option<RcVar> {
    if let Value::Var(v, _) = value {
        Some(v.clone())
    } else {
        None
    }
}

/// Set an [`RcVar`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Var`].
pub fn set_var(value: &Value, v: RcVar) -> Option<Value> {
    if let Value::Var(_, _) = value {
        Some(Value::Var(v, meta(value)))
    } else {
        None
    }
}


/// View the [`RcFunction`] within a [`Value::Function`], returning `None` if the variant doesn't match.
pub fn view_function(value: &Value) -> Option<RcFunction> {
    if let Value::Function(f, _) = value {
        Some(f.clone())
    } else {
        None
    }
}

/// Set an [`RcFunction`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Function`].
pub fn set_function(value: &Value, f: RcFunction) -> Option<Value> {
    if let Value::Function(_, _) = value {
        Some(Value::Function(f, meta(value)))
    } else {
        None
    }
}


/// View the [`Handle`] within a [`Value::Handle`], returning `None` if the variant doesn't match.
pub fn view_handle(value: &Value) -> Option<Handle> {
    if let Value::Handle(h, _) = value {
        Some(h.clone())
    } else {
        None
    }
}

/// Set a [`Handle`] value, preserving metadata from the original `Value`.
/// Returns `None` if the `Value` is not a [`Value::Handle`].
pub fn set_handle(value: &Value, h: Handle) -> Option<Value> {
    if let Value::Handle(_, _) = value {
        Some(Value::Handle(h, meta(value)))
    } else {
        None
    }
}


/// Create a [`Prism`] focusing on the `Nil` variant of [`Value`].
pub fn prism_nil() -> Prism<Value, ()> {
    Prism::new(view_nil, set_nil)
}

/// Create a [`Prism`] focusing on the `Boolean` variant of [`Value`].
pub fn prism_boolean() -> Prism<Value, bool> {
    Prism::new(view_boolean, set_boolean)
}

/// Create a [`Prism`] focusing on the `Integer` variant of [`Value`].
pub fn prism_integer() -> Prism<Value, i64> {
    Prism::new(view_integer, set_integer)
}

/// Create a [`Prism`] focusing on the `Float` variant of [`Value`].
pub fn prism_float() -> Prism<Value, Float> {
    Prism::new(view_float, set_float)
}

/// Create a [`Prism`] focusing on the `String` variant of [`Value`].
pub fn prism_string() -> Prism<Value, String> {
    Prism::new(view_string, set_string)
}

/// Create a [`Prism`] focusing on the `Symbol` variant of [`Value`].
pub fn prism_symbol() -> Prism<Value, Symbol> {
    Prism::new(view_symbol, set_symbol)
}

/// Create a [`Prism`] focusing on the `Keyword` variant of [`Value`].
pub fn prism_keyword() -> Prism<Value, Keyword> {
    Prism::new(view_keyword, set_keyword)
}

/// Create a [`Prism`] focusing on the `Vector` variant of [`Value`].
pub fn prism_vector() -> Prism<Value, Vector> {
    Prism::new(view_vector, set_vector)
}

/// Create a [`Prism`] focusing on the `Set` variant of [`Value`].
pub fn prism_set() -> Prism<Value, Set> {
    Prism::new(view_set, set_set)
}

/// Create a [`Prism`] focusing on the `Map` variant of [`Value`].
pub fn prism_map() -> Prism<Value, Map> {
    Prism::new(view_map, set_map)
}

/// Create a [`Prism`] focusing on the `Var` variant of [`Value`].
pub fn prism_var() -> Prism<Value, RcVar> {
    Prism::new(view_var, set_var)
}

/// Create a [`Prism`] focusing on the `Function` variant of [`Value`].
pub fn prism_function() -> Prism<Value, RcFunction> {
    Prism::new(view_function, set_function)
}

/// Create a [`Prism`] focusing on the `Handle` variant of [`Value`].
pub fn prism_handle() -> Prism<Value, Handle> {
    Prism::new(view_handle, set_handle)
}
