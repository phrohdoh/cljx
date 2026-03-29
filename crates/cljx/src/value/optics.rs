//! This module provides optics (prisms, preview, review, modify, set) for accessing and transforming
//! different variants of `Value`. Each variant type (nil, boolean, integer, etc.) has a complete
//! set of optics functions for functional, composable access patterns.
//!
//! Common patterns:
//! - `prism_*()`: Returns a prism that can be used with lens combinators
//! - `preview_*()`: Extracts the inner value if it matches this type, returns `Option`
//! - `review_*()`: Constructs a `Value` from an inner value
//! - `modify_*()`: Applies a function to the inner value, returns the modified `Value` (or original if type doesn't match)
//! - `set_*()`: Sets the inner value, returns the new `Value`
//! - `try_modify_*()`: Applies a function, returns `Result` based on type match

use ::std::rc::Rc;
use crate::prelude::*;


// nil
// ========================================

/// Creates a prism for accessing `Value::Nil` variants.
/// This prism can preview whether a `Value` is nil and reconstruct a nil `Value`.
pub fn prism_nil() -> PrismNil<Value> {
    PrismNil::new(
        |value: &Value| match value {
            Value::Nil(_) => Some(()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Nil(_) => Some(()),
            _ => None,
        },
        || Rc::new(Value::Nil(None)),
    )
}

/// Previews whether a `Value` is nil.
/// Returns `Some(())` if the value is nil, `None` otherwise.
pub fn preview_nil(value: &Value) -> Option<()> {
    prism_nil().preview(value)
}

/// Constructs a nil `Value`.
pub fn review_nil() -> Rc<Value> {
    prism_nil().review()
}


// boolean
// ========================================

/// Creates a prism for accessing `Value::Boolean` variants as `bool`.
pub fn prism_boolean() -> Prism<Value, bool> {
    Prism::new(
        |value: &Value| match value {
            Value::Boolean(b, _) => Some(*b),
            _ => None,
        },
        |value: &Value| match value {
            Value::Boolean(_, _) => None, // bool is Copy; use preview_boolean for the value
            _ => None,
        },
        |b: bool| Rc::new(Value::Boolean(b, None)),
    )
}

/// Previews whether a `Value` is a boolean and extracts the bool value.
/// Returns `Some(bool)` if the value is boolean, `None` otherwise.
pub fn preview_boolean(value: &Value) -> Option<bool> {
    prism_boolean().preview(value)
}

/// Constructs a boolean `Value` from a bool.
pub fn review_boolean(b: bool) -> Rc<Value> {
    prism_boolean().review(b)
}

/// Applies a function to the inner bool of a `Value`, if it is a boolean.
/// Returns the modified `Value` if this is a boolean, otherwise returns the original unchanged.
pub fn modify_boolean(value: Rc<Value>, f: impl Fn(bool) -> bool) -> Rc<Value> {
    prism_boolean().modify(value, f)
}

/// Sets the bool value of a `Value`, if it is a boolean.
/// Returns a new `Value` with the updated bool if this is a boolean, otherwise returns the original unchanged.
pub fn set_boolean(value: Rc<Value>, b: bool) -> Rc<Value> {
    prism_boolean().set(value, b)
}

/// Attempts to apply a function to the inner bool of a `Value`.
/// Returns `Ok(modified_value)` if this is a boolean, `Err(original_value)` otherwise.
pub fn try_modify_boolean(value: Rc<Value>, f: impl Fn(bool) -> bool) -> Result<Rc<Value>, Rc<Value>> {
    prism_boolean().try_modify(value, f)
}


// integer
// ========================================

/// Creates a prism for accessing `Value::Integer` variants as `i64`.
pub fn prism_integer() -> Prism<Value, i64> {
    Prism::new(
        |value: &Value| match value {
            Value::Integer(i, _) => Some(*i),
            _ => None,
        },
        |value: &Value| match value {
            Value::Integer(_, _) => None, // i64 is Copy; use preview_integer for the value
            _ => None,
        },
        |i: i64| Rc::new(Value::Integer(i, None)),
    )
}

/// Previews whether a `Value` is an integer and extracts the i64 value.
/// Returns `Some(i64)` if the value is an integer, `None` otherwise.
pub fn preview_integer(value: &Value) -> Option<i64> {
    prism_integer().preview(value)
}

/// Constructs an integer `Value` from an i64.
pub fn review_integer(i: i64) -> Rc<Value> {
    prism_integer().review(i)
}

/// Applies a function to the inner i64 of a `Value`, if it is an integer.
/// Returns the modified `Value` if this is an integer, otherwise returns the original unchanged.
pub fn modify_integer(value: Rc<Value>, f: impl Fn(i64) -> i64) -> Rc<Value> {
    prism_integer().modify(value, f)
}

/// Sets the i64 value of a `Value`, if it is an integer.
/// Returns a new `Value` with the updated i64 if this is an integer, otherwise returns the original unchanged.
pub fn set_integer(value: Rc<Value>, i: i64) -> Rc<Value> {
    prism_integer().set(value, i)
}

/// Attempts to apply a function to the inner i64 of a `Value`.
/// Returns `Ok(modified_value)` if this is an integer, `Err(original_value)` otherwise.
pub fn try_modify_integer(value: Rc<Value>, f: impl Fn(i64) -> i64) -> Result<Rc<Value>, Rc<Value>> {
    prism_integer().try_modify(value, f)
}


// float
// ========================================

/// Creates a prism for accessing `Value::Float` variants as `f64`.
pub fn prism_float() -> Prism<Value, f64> {
    Prism::new(
        |value: &Value| match value {
            Value::Float(float, _) => Some(float.as_f64()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Float(_, _) => None, // f64 is Copy; use preview_float for the value
            _ => None,
        },
        |float: f64| Rc::new(Value::Float(float.into(), None)),
    )
}

/// Previews whether a `Value` is a float and extracts the f64 value.
/// Returns `Some(f64)` if the value is a float, `None` otherwise.
pub fn preview_float(value: &Value) -> Option<f64> {
    prism_float().preview(value)
}

/// Previews the `Float` value within a `Value` by reference.
/// Returns `Some(&Float)` if the value is a float, `None` otherwise.
/// This avoids cloning the `Float` wrapper type.
pub fn preview_float_ref(value: &Value) -> Option<&Float> {
    if let Value::Float(f, _) = value {
        Some(f)
    } else {
        None
    }
}

/// Constructs a float `Value` from an f64.
pub fn review_float(float: f64) -> Rc<Value> {
    prism_float().review(float)
}

/// Applies a function to the inner f64 of a `Value`, if it is a float.
/// Returns the modified `Value` if this is a float, otherwise returns the original unchanged.
pub fn modify_float(value: Rc<Value>, f: impl Fn(f64) -> f64) -> Rc<Value> {
    prism_float().modify(value, f)
}

/// Sets the f64 value of a `Value`, if it is a float.
/// Returns a new `Value` with the updated f64 if this is a float, otherwise returns the original unchanged.
pub fn set_float(value: Rc<Value>, float: f64) -> Rc<Value> {
    prism_float().set(value, float)
}

/// Attempts to apply a function to the inner f64 of a `Value`.
/// Returns `Ok(modified_value)` if this is a float, `Err(original_value)` otherwise.
pub fn try_modify_float(value: Rc<Value>, f: impl Fn(f64) -> f64) -> Result<Rc<Value>, Rc<Value>> {
    prism_float().try_modify(value, f)
}


// string
// ========================================

/// Creates a prism for accessing `Value::String` variants as `String`.
pub fn prism_string() -> Prism<Value, String> {
    Prism::new(
        |value: &Value| match value {
            Value::String(s, _) => Some(s.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::String(s, _) => Some(s),
            _ => None,
        },
        |s: String| Rc::new(Value::String(s, None)),
    )
}

/// Previews whether a `Value` is a string and extracts the String value.
/// Returns `Some(String)` if the value is a string, `None` otherwise.
pub fn preview_string(value: &Value) -> Option<String> {
    prism_string().preview(value)
}

/// Previews the `String` value within a `Value` by reference.
/// Returns `Some(&str)` if the value is a string, `None` otherwise.
/// This avoids cloning the `String` value.
pub fn preview_string_ref(value: &Value) -> Option<&str> {
    if let Value::String(s, _) = value {
        Some(s.as_str())
    } else {
        None
    }
}

/// Constructs a string `Value` from a String.
pub fn review_string(s: String) -> Rc<Value> {
    prism_string().review(s)
}

/// Applies a function to the inner String of a `Value`, if it is a string.
/// Returns the modified `Value` if this is a string, otherwise returns the original unchanged.
pub fn modify_string(value: Rc<Value>, f: impl Fn(String) -> String) -> Rc<Value> {
    prism_string().modify(value, f)
}

/// Sets the String value of a `Value`, if it is a string.
/// Returns a new `Value` with the updated String if this is a string, otherwise returns the original unchanged.
pub fn set_string(value: Rc<Value>, s: String) -> Rc<Value> {
    prism_string().set(value, s)
}

/// Attempts to apply a function to the inner String of a `Value`.
/// Returns `Ok(modified_value)` if this is a string, `Err(original_value)` otherwise.
pub fn try_modify_string(value: Rc<Value>, f: impl Fn(String) -> String) -> Result<Rc<Value>, Rc<Value>> {
    prism_string().try_modify(value, f)
}


// symbol
// ========================================

/// Creates a prism for accessing `Value::Symbol` variants as `Symbol`.
pub fn prism_symbol() -> Prism<Value, Symbol> {
    Prism::new(
        |value: &Value| match value {
            Value::Symbol(sym, _) => Some(sym.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Symbol(sym, _) => Some(sym),
            _ => None,
        },
        |sym: Symbol| Rc::new(Value::Symbol(sym, None)),
    )
}

/// Previews whether a `Value` is a symbol and extracts the Symbol value.
/// Returns `Some(Symbol)` if the value is a symbol, `None` otherwise.
pub fn preview_symbol(value: &Value) -> Option<Symbol> {
    prism_symbol().preview(value)
}

/// Previews the `Symbol` value within a `Value` by reference.
/// Returns `Some(&Symbol)` if the value is a symbol, `None` otherwise.
/// This avoids cloning the `Symbol` value.
pub fn preview_symbol_ref(value: &Value) -> Option<&Symbol> {
    if let Value::Symbol(sym, _) = value {
        Some(sym)
    } else {
        None
    }
}

/// Constructs a symbol `Value` from a Symbol.
pub fn review_symbol(sym: Symbol) -> Rc<Value> {
    prism_symbol().review(sym)
}

/// Applies a function to the inner Symbol of a `Value`, if it is a symbol.
/// Returns the modified `Value` if this is a symbol, otherwise returns the original unchanged.
pub fn modify_symbol(value: Rc<Value>, f: impl Fn(Symbol) -> Symbol) -> Rc<Value> {
    prism_symbol().modify(value, f)
}

/// Sets the Symbol value of a `Value`, if it is a symbol.
/// Returns a new `Value` with the updated Symbol if this is a symbol, otherwise returns the original unchanged.
pub fn set_symbol(value: Rc<Value>, sym: Symbol) -> Rc<Value> {
    prism_symbol().set(value, sym)
}

/// Attempts to apply a function to the inner Symbol of a `Value`.
/// Returns `Ok(modified_value)` if this is a symbol, `Err(original_value)` otherwise.
pub fn try_modify_symbol(value: Rc<Value>, f: impl Fn(Symbol) -> Symbol) -> Result<Rc<Value>, Rc<Value>> {
    prism_symbol().try_modify(value, f)
}


// keyword
// ========================================

/// Creates a prism for accessing `Value::Keyword` variants as `Keyword`.
pub fn prism_keyword() -> Prism<Value, Keyword> {
    Prism::new(
        |value: &Value| match value {
            Value::Keyword(kw, _) => Some(kw.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Keyword(kw, _) => Some(kw),
            _ => None,
        },
        |kw: Keyword| Rc::new(Value::Keyword(kw, None)),
    )
}

/// Previews whether a `Value` is a keyword and extracts the Keyword value.
/// Returns `Some(Keyword)` if the value is a keyword, `None` otherwise.
pub fn preview_keyword(value: &Value) -> Option<Keyword> {
    prism_keyword().preview(value)
}

/// Previews the `Keyword` value within a `Value` by reference.
/// Returns `Some(&Keyword)` if the value is a keyword, `None` otherwise.
/// This avoids cloning the `Keyword` value.
pub fn preview_keyword_ref(value: &Value) -> Option<&Keyword> {
    if let Value::Keyword(kw, _) = value {
        Some(kw)
    } else {
        None
    }
}

/// Constructs a keyword `Value` from a Keyword.
pub fn review_keyword(kw: Keyword) -> Rc<Value> {
    prism_keyword().review(kw)
}

/// Applies a function to the inner Keyword of a `Value`, if it is a keyword.
/// Returns the modified `Value` if this is a keyword, otherwise returns the original unchanged.
pub fn modify_keyword(value: Rc<Value>, f: impl Fn(Keyword) -> Keyword) -> Rc<Value> {
    prism_keyword().modify(value, f)
}

/// Sets the Keyword value of a `Value`, if it is a keyword.
/// Returns a new `Value` with the updated Keyword if this is a keyword, otherwise returns the original unchanged.
pub fn set_keyword(value: Rc<Value>, kw: Keyword) -> Rc<Value> {
    prism_keyword().set(value, kw)
}

/// Attempts to apply a function to the inner Keyword of a `Value`.
/// Returns `Ok(modified_value)` if this is a keyword, `Err(original_value)` otherwise.
pub fn try_modify_keyword(value: Rc<Value>, f: impl Fn(Keyword) -> Keyword) -> Result<Rc<Value>, Rc<Value>> {
    prism_keyword().try_modify(value, f)
}


// list
// ========================================

/// Creates a prism for accessing `Value::List` variants as `List`.
pub fn prism_list() -> Prism<Value, List> {
    Prism::new(
        |value: &Value| match value {
            Value::List(list, _) => Some(list.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::List(list, _) => Some(list),
            _ => None,
        },
        |list: List| Rc::new(Value::List(list, None)),
    )
}

/// Previews whether a `Value` is a list and extracts the List value.
/// Returns `Some(List)` if the value is a list, `None` otherwise.
pub fn preview_list(value: &Value) -> Option<List> {
    prism_list().preview(value)
}

/// Previews the `List` value within a `Value` by reference.
/// Returns `Some(&List)` if the value is a list, `None` otherwise.
/// This avoids cloning the `List` value.
pub fn preview_list_ref(value: &Value) -> Option<&List> {
    if let Value::List(list, _) = value {
        Some(list)
    } else {
        None
    }
}

/// Constructs a list `Value` from a List.
pub fn review_list(list: List) -> Rc<Value> {
    prism_list().review(list)
}

/// Applies a function to the inner List of a `Value`, if it is a list.
/// Returns the modified `Value` if this is a list, otherwise returns the original unchanged.
pub fn modify_list(value: Rc<Value>, f: impl Fn(List) -> List) -> Rc<Value> {
    prism_list().modify(value, f)
}

/// Sets the List value of a `Value`, if it is a list.
/// Returns a new `Value` with the updated List if this is a list, otherwise returns the original unchanged.
pub fn set_list(value: Rc<Value>, list: List) -> Rc<Value> {
    prism_list().set(value, list)
}

/// Attempts to apply a function to the inner List of a `Value`.
/// Returns `Ok(modified_value)` if this is a list, `Err(original_value)` otherwise.
pub fn try_modify_list(value: Rc<Value>, f: impl Fn(List) -> List) -> Result<Rc<Value>, Rc<Value>> {
    prism_list().try_modify(value, f)
}


// vector
// ========================================

/// Creates a prism for accessing `Value::Vector` variants as `Vector`.
pub fn prism_vector() -> Prism<Value, Vector> {
    Prism::new(
        |value: &Value| match value {
            Value::Vector(vector, _) => Some(vector.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Vector(vector, _) => Some(vector),
            _ => None,
        },
        |vector: Vector| Rc::new(Value::Vector(vector, None)),
    )
}

/// Previews whether a `Value` is a vector and extracts the Vector value.
/// Returns `Some(Vector)` if the value is a vector, `None` otherwise.
pub fn preview_vector(value: &Value) -> Option<Vector> {
    prism_vector().preview(value)
}

/// Previews the `Vector` value within a `Value` by reference.
/// Returns `Some(&Vector)` if the value is a vector, `None` otherwise.
/// This avoids cloning the `Vector` value.
pub fn preview_vector_ref(value: &Value) -> Option<&Vector> {
    if let Value::Vector(vector, _) = value {
        Some(vector)
    } else {
        None
    }
}

/// Constructs a vector `Value` from a Vector.
pub fn review_vector(vector: Vector) -> Rc<Value> {
    prism_vector().review(vector)
}

/// Applies a function to the inner Vector of a `Value`, if it is a vector.
/// Returns the modified `Value` if this is a vector, otherwise returns the original unchanged.
pub fn modify_vector(value: Rc<Value>, f: impl Fn(Vector) -> Vector) -> Rc<Value> {
    prism_vector().modify(value, f)
}

/// Sets the Vector value of a `Value`, if it is a vector.
/// Returns a new `Value` with the updated Vector if this is a vector, otherwise returns the original unchanged.
pub fn set_vector(value: Rc<Value>, vector: Vector) -> Rc<Value> {
    prism_vector().set(value, vector)
}

/// Attempts to apply a function to the inner Vector of a `Value`.
/// Returns `Ok(modified_value)` if this is a vector, `Err(original_value)` otherwise.
pub fn try_modify_vector(value: Rc<Value>, f: impl Fn(Vector) -> Vector) -> Result<Rc<Value>, Rc<Value>> {
    prism_vector().try_modify(value, f)
}


// set
// ========================================

/// Creates a prism for accessing `Value::Set` variants as `Set`.
pub fn prism_set() -> Prism<Value, Set> {
    Prism::new(
        |value: &Value| match value {
            Value::Set(set, _) => Some(set.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Set(set, _) => Some(set),
            _ => None,
        },
        |set: Set| Rc::new(Value::Set(set, None)),
    )
}

/// Previews whether a `Value` is a set and extracts the Set value.
/// Returns `Some(Set)` if the value is a set, `None` otherwise.
pub fn preview_set(value: &Value) -> Option<Set> {
    prism_set().preview(value)
}

/// Previews the `Set` value within a `Value` by reference.
/// Returns `Some(&Set)` if the value is a set, `None` otherwise.
/// This avoids cloning the `Set` value.
pub fn preview_set_ref(value: &Value) -> Option<&Set> {
    if let Value::Set(set, _) = value {
        Some(set)
    } else {
        None
    }
}

/// Constructs a set `Value` from a Set.
pub fn review_set(set: Set) -> Rc<Value> {
    prism_set().review(set)
}

/// Applies a function to the inner Set of a `Value`, if it is a set.
/// Returns the modified `Value` if this is a set, otherwise returns the original unchanged.
pub fn modify_set(value: Rc<Value>, f: impl Fn(Set) -> Set) -> Rc<Value> {
    prism_set().modify(value, f)
}

/// Sets the Set value of a `Value`, if it is a set.
/// Returns a new `Value` with the updated Set if this is a set, otherwise returns the original unchanged.
pub fn set_set(value: Rc<Value>, set: Set) -> Rc<Value> {
    prism_set().set(value, set)
}

/// Attempts to apply a function to the inner Set of a `Value`.
/// Returns `Ok(modified_value)` if this is a set, `Err(original_value)` otherwise.
pub fn try_modify_set(value: Rc<Value>, f: impl Fn(Set) -> Set) -> Result<Rc<Value>, Rc<Value>> {
    prism_set().try_modify(value, f)
}


// map
// ========================================

/// Creates a prism for accessing `Value::Map` variants as `Map`.
pub fn prism_map() -> Prism<Value, Map> {
    Prism::new(
        |value: &Value| match value {
            Value::Map(map, _) => Some(map.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Map(map, _) => Some(map),
            _ => None,
        },
        |map: Map| Rc::new(Value::Map(map, None)),
    )
}

/// Previews whether a `Value` is a map and extracts the Map value.
/// Returns `Some(Map)` if the value is a map, `None` otherwise.
pub fn preview_map(value: &Value) -> Option<Map> {
    prism_map().preview(value)
}

/// Previews the `Map` value within a `Value` by reference.
/// Returns `Some(&Map)` if the value is a map, `None` otherwise.
/// This avoids cloning the `Map` value.
pub fn preview_map_ref(value: &Value) -> Option<&Map> {
    if let Value::Map(map, _) = value {
        Some(map)
    } else {
        None
    }
}

/// Constructs a map `Value` from a Map.
pub fn review_map(map: Map) -> Rc<Value> {
    prism_map().review(map)
}

/// Applies a function to the inner Map of a `Value`, if it is a map.
/// Returns the modified `Value` if this is a map, otherwise returns the original unchanged.
pub fn modify_map(value: Rc<Value>, f: impl Fn(Map) -> Map) -> Rc<Value> {
    prism_map().modify(value, f)
}

/// Sets the Map value of a `Value`, if it is a map.
/// Returns a new `Value` with the updated Map if this is a map, otherwise returns the original unchanged.
pub fn set_map(value: Rc<Value>, map: Map) -> Rc<Value> {
    prism_map().set(value, map)
}

/// Attempts to apply a function to the inner Map of a `Value`.
/// Returns `Ok(modified_value)` if this is a map, `Err(original_value)` otherwise.
pub fn try_modify_map(value: Rc<Value>, f: impl Fn(Map) -> Map) -> Result<Rc<Value>, Rc<Value>> {
    prism_map().try_modify(value, f)
}


// var
// ========================================

/// Creates a prism for accessing `Value::Var` variants as `RcVar`.
pub fn prism_var() -> Prism<Value, RcVar> {
    Prism::new(
        |value: &Value| match value {
            Value::Var(var, _) => Some(var.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Var(rc_var, _) => Some(rc_var),
            _ => None,
        },
        |var: RcVar| Rc::new(Value::Var(var, None)),
    )
}

/// Previews whether a `Value` is a var and extracts the RcVar value.
/// Returns `Some(RcVar)` if the value is a var, `None` otherwise.
pub fn preview_var(value: &Value) -> Option<RcVar> {
    prism_var().preview(value)
}

/// Previews the `Var` value within a `Value` by reference.
/// Returns `Some(&Var)` if the value is a var, `None` otherwise.
/// This avoids cloning the `Var` value.
pub fn preview_var_ref(value: &Value) -> Option<&Var> {
    if let Value::Var(rc_var, _) = value {
        Some(rc_var.as_ref())
    } else {
        None
    }
}

/// Constructs a var `Value` from an RcVar.
pub fn review_var(var: RcVar) -> Rc<Value> {
    prism_var().review(var)
}

/// Applies a function to the inner RcVar of a `Value`, if it is a var.
/// Returns the modified `Value` if this is a var, otherwise returns the original unchanged.
pub fn modify_var(value: Rc<Value>, f: impl Fn(RcVar) -> RcVar) -> Rc<Value> {
    prism_var().modify(value, f)
}

/// Sets the RcVar value of a `Value`, if it is a var.
/// Returns a new `Value` with the updated RcVar if this is a var, otherwise returns the original unchanged.
pub fn set_var(value: Rc<Value>, var: RcVar) -> Rc<Value> {
    prism_var().set(value, var)
}

/// Attempts to apply a function to the inner RcVar of a `Value`.
/// Returns `Ok(modified_value)` if this is a var, `Err(original_value)` otherwise.
pub fn try_modify_var(value: Rc<Value>, f: impl Fn(RcVar) -> RcVar) -> Result<Rc<Value>, Rc<Value>> {
    prism_var().try_modify(value, f)
}


// function
// ========================================

/// Creates a prism for accessing `Value::Function` variants as `RcFunction`.
pub fn prism_function() -> Prism<Value, RcFunction> {
    Prism::new(
        |value: &Value| match value {
            Value::Function(function, _) => Some(function.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Function(rc_func, _) => Some(rc_func),
            _ => None,
        },
        |function: RcFunction| Rc::new(Value::Function(function, None)),
    )
}

/// Previews whether a `Value` is a function and extracts the RcFunction value.
/// Returns `Some(RcFunction)` if the value is a function, `None` otherwise.
pub fn preview_function(value: &Value) -> Option<RcFunction> {
    prism_function().preview(value)
}

/// Previews the `Function` value within a `Value` by reference.
/// Returns `Some(&Function)` if the value is a function, `None` otherwise.
/// This avoids cloning the `Function` value.
pub fn preview_function_ref(value: &Value) -> Option<&Function> {
    if let Value::Function(rc_func, _) = value {
        Some(rc_func.as_ref())
    } else {
        None
    }
}

/// Constructs a function `Value` from an RcFunction.
pub fn review_function(function: RcFunction) -> Rc<Value> {
    prism_function().review(function)
}

/// Applies a function to the inner RcFunction of a `Value`, if it is a function.
/// Returns the modified `Value` if this is a function, otherwise returns the original unchanged.
pub fn modify_function(value: Rc<Value>, f: impl Fn(RcFunction) -> RcFunction) -> Rc<Value> {
    prism_function().modify(value, f)
}

/// Sets the RcFunction value of a `Value`, if it is a function.
/// Returns a new `Value` with the updated RcFunction if this is a function, otherwise returns the original unchanged.
pub fn set_function(value: Rc<Value>, function: RcFunction) -> Rc<Value> {
    prism_function().set(value, function)
}

/// Attempts to apply a function to the inner RcFunction of a `Value`.
/// Returns `Ok(modified_value)` if this is a function, `Err(original_value)` otherwise.
pub fn try_modify_function(value: Rc<Value>, f: impl Fn(RcFunction) -> RcFunction) -> Result<Rc<Value>, Rc<Value>> {
    prism_function().try_modify(value, f)
}


// handle
// ========================================

/// Creates a prism for accessing `Value::Handle` variants as `Handle`.
pub fn prism_handle() -> Prism<Value, Handle> {
    Prism::new(
        |value: &Value| match value {
            Value::Handle(handle, _) => Some(handle.clone()),
            _ => None,
        },
        |value: &Value| match value {
            Value::Handle(handle, _) => Some(handle),
            _ => None,
        },
        |handle: Handle| Rc::new(Value::Handle(handle, None)),
    )
}

/// Previews whether a `Value` is a handle and extracts the Handle value.
/// Returns `Some(Handle)` if the value is a handle, `None` otherwise.
pub fn preview_handle(value: &Value) -> Option<Handle> {
    prism_handle().preview(value)
}

/// Previews the `Handle` value within a `Value` by reference.
/// Returns `Some(&Handle)` if the value is a handle, `None` otherwise.
/// This avoids cloning the `Handle` value.
pub fn preview_handle_ref(value: &Value) -> Option<&Handle> {
    if let Value::Handle(handle, _) = value {
        Some(handle)
    } else {
        None
    }
}

/// Constructs a handle `Value` from a Handle.
pub fn review_handle(handle: Handle) -> Rc<Value> {
    prism_handle().review(handle)
}

/// Applies a function to the inner Handle of a `Value`, if it is a handle.
/// Returns the modified `Value` if this is a handle, otherwise returns the original unchanged.
pub fn modify_handle(value: Rc<Value>, f: impl Fn(Handle) -> Handle) -> Rc<Value> {
    prism_handle().modify(value, f)
}

/// Sets the Handle value of a `Value`, if it is a handle.
/// Returns a new `Value` with the updated Handle if this is a handle, otherwise returns the original unchanged.
pub fn set_handle(value: Rc<Value>, handle: Handle) -> Rc<Value> {
    prism_handle().set(value, handle)
}

/// Attempts to apply a function to the inner Handle of a `Value`.
/// Returns `Ok(modified_value)` if this is a handle, `Err(original_value)` otherwise.
pub fn try_modify_handle(value: Rc<Value>, f: impl Fn(Handle) -> Handle) -> Result<Rc<Value>, Rc<Value>> {
    prism_handle().try_modify(value, f)
}


// meta (metadata)
// ========================================

/// Previews the metadata of a `Value`.
/// Returns `Some(Rc<Map>)` if metadata is present, `None` otherwise.
/// This works on any `Value` variant since all variants have embedded metadata.
pub fn preview_meta(value: &Value) -> Option<Rc<Map>> {
    match value {
        Value::Nil(meta) => meta.clone(),
        Value::Boolean(_, meta) => meta.clone(),
        Value::Integer(_, meta) => meta.clone(),
        Value::Float(_, meta) => meta.clone(),
        Value::String(_, meta) => meta.clone(),
        Value::Symbol(_, meta) => meta.clone(),
        Value::Keyword(_, meta) => meta.clone(),
        Value::List(_, meta) => meta.clone(),
        Value::Vector(_, meta) => meta.clone(),
        Value::Set(_, meta) => meta.clone(),
        Value::Map(_, meta) => meta.clone(),
        Value::Var(_, meta) => meta.clone(),
        Value::Function(_, meta) => meta.clone(),
        Value::Handle(_, meta) => meta.clone(),
    }
}

/// Previews the metadata of a `Value` by reference.
/// Returns `&Option<Rc<Map>>` - a reference to the metadata, which will be `&Some(rc_map)` if
/// metadata is present or `&None` if no metadata. The reference is valid as long as the `Value` is valid.
/// This avoids cloning the metadata map. This works on any `Value` variant since all have embedded metadata.
pub fn preview_meta_ref(value: &Value) -> Option<&Rc<Map>> {
    match value {
        Value::Nil(meta) => meta.as_ref(),
        Value::Boolean(_, meta) => meta.as_ref(),
        Value::Integer(_, meta) => meta.as_ref(),
        Value::Float(_, meta) => meta.as_ref(),
        Value::String(_, meta) => meta.as_ref(),
        Value::Symbol(_, meta) => meta.as_ref(),
        Value::Keyword(_, meta) => meta.as_ref(),
        Value::List(_, meta) => meta.as_ref(),
        Value::Vector(_, meta) => meta.as_ref(),
        Value::Set(_, meta) => meta.as_ref(),
        Value::Map(_, meta) => meta.as_ref(),
        Value::Var(_, meta) => meta.as_ref(),
        Value::Function(_, meta) => meta.as_ref(),
        Value::Handle(_, meta) => meta.as_ref(),
    }
}

/// Applies a function to the metadata of a `Value`.
/// Returns a new `Value` with the modified metadata. This works on any variant.
pub fn modify_meta(value: Rc<Value>, f: impl Fn(Option<Rc<Map>>) -> Option<Rc<Map>>) -> Rc<Value> {
    match (*value).clone() {
        Value::Nil(meta) => Rc::new(Value::Nil(f(meta))),
        Value::Boolean(b, meta) => Rc::new(Value::Boolean(b, f(meta))),
        Value::Integer(i, meta) => Rc::new(Value::Integer(i, f(meta))),
        Value::Float(fl, meta) => Rc::new(Value::Float(fl, f(meta))),
        Value::String(s, meta) => Rc::new(Value::String(s, f(meta))),
        Value::Symbol(sym, meta) => Rc::new(Value::Symbol(sym, f(meta))),
        Value::Keyword(kw, meta) => Rc::new(Value::Keyword(kw, f(meta))),
        Value::List(list, meta) => Rc::new(Value::List(list, f(meta))),
        Value::Vector(vec, meta) => Rc::new(Value::Vector(vec, f(meta))),
        Value::Set(set, meta) => Rc::new(Value::Set(set, f(meta))),
        Value::Map(map, meta) => Rc::new(Value::Map(map, f(meta))),
        Value::Var(var, meta) => Rc::new(Value::Var(var, f(meta))),
        Value::Function(func, meta) => Rc::new(Value::Function(func, f(meta))),
        Value::Handle(handle, meta) => Rc::new(Value::Handle(handle, f(meta))),
    }
}

/// Sets the metadata of a `Value`.
/// Returns a new `Value` with the updated metadata, replacing any existing metadata.
pub fn set_meta(value: Rc<Value>, meta: Option<Rc<Map>>) -> Rc<Value> {
    modify_meta(value, |_| meta.clone())
}

/// Attempts to get the metadata of a `Value` if it exists.
/// Returns `Ok(meta_map)` if metadata is present, `Err(original_value)` if metadata is None.
pub fn try_get_meta(value: Rc<Value>) -> Result<Rc<Map>, Rc<Value>> {
    if let Some(meta) = preview_meta(&value) {
        Ok(meta)
    } else {
        Err(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_integer() {
        let val = review_integer(42);
        let modified = modify_integer(val, |i| i * 2);
        assert_eq!(preview_integer(&modified), Some(84));
    }

    #[test]
    fn test_modify_integer_wrong_type() {
        let val = review_boolean(true);
        let modified = modify_integer(val.clone(), |i| i * 2);
        // Should return the original value unchanged
        assert_eq!(modified, val);
    }

    #[test]
    fn test_modify_boolean() {
        let val = review_boolean(true);
        let modified = modify_boolean(val, |b| !b);
        assert_eq!(preview_boolean(&modified), Some(false));
    }

    #[test]
    fn test_modify_boolean_wrong_type() {
        let val = review_integer(42);
        let modified = modify_boolean(val.clone(), |b| !b);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_modify_float() {
        let val = review_float(3.14);
        let modified = modify_float(val, |f| f * 2.0);
        assert_eq!(preview_float(&modified), Some(3.14 * 2.0));
    }

    #[test]
    fn test_modify_float_wrong_type() {
        let val = review_integer(42);
        let modified = modify_float(val.clone(), |f| f * 2.0);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_modify_string() {
        let val = review_string("hello".to_string());
        let modified = modify_string(val, |s| s.to_uppercase());
        assert_eq!(preview_string(&modified), Some("HELLO".to_string()));
    }

    #[test]
    fn test_modify_string_wrong_type() {
        let val = review_integer(42);
        let modified = modify_string(val.clone(), |s| s.to_uppercase());
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_integer_success() {
        let val = review_integer(10);
        let result = try_modify_integer(val, |i| i + 5);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_integer(&modified), Some(15));
        }
    }

    #[test]
    fn test_try_modify_integer_failure() {
        let val = review_boolean(true);
        let result = try_modify_integer(val.clone(), |i| i + 5);
        assert!(result.is_err());
        if let Err(original) = result {
            assert_eq!(original, val);
        }
    }

    #[test]
    fn test_try_modify_boolean_success() {
        let val = review_boolean(false);
        let result = try_modify_boolean(val, |b| !b);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_boolean(&modified), Some(true));
        }
    }

    #[test]
    fn test_try_modify_boolean_failure() {
        let val = review_integer(42);
        let result = try_modify_boolean(val.clone(), |b| !b);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_integer() {
        let val = review_integer(42);
        let new_val = set_integer(val, 100);
        assert_eq!(preview_integer(&new_val), Some(100));
    }

    #[test]
    fn test_set_integer_wrong_type() {
        let val = review_boolean(true);
        let new_val = set_integer(val.clone(), 100);
        assert_eq!(new_val, val);
    }

    #[test]
    fn test_set_boolean() {
        let val = review_boolean(false);
        let new_val = set_boolean(val, true);
        assert_eq!(preview_boolean(&new_val), Some(true));
    }

    #[test]
    fn test_preview_nil() {
        let val = review_nil();
        assert_eq!(preview_nil(&val), Some(()));
    }

    #[test]
    fn test_preview_nil_wrong_type() {
        let val = review_integer(42);
        assert_eq!(preview_nil(&val), None);
    }

    #[test]
    fn test_modify_symbol() {
        let sym = Symbol::new_unqualified("foo");
        let val = review_symbol(sym);
        let modified = modify_symbol(val, |s| Symbol::new_unqualified(&format!("{}_modified", s.name())));
        let result_sym = preview_symbol(&modified);
        assert!(result_sym.is_some());
        assert_eq!(result_sym.unwrap().name(), "foo_modified");
    }

    #[test]
    fn test_modify_symbol_wrong_type() {
        let val = review_integer(42);
        let modified = modify_symbol(val.clone(), |_s| Symbol::new_unqualified("bar"));
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_symbol_success() {
        let sym = Symbol::new_unqualified("test");
        let val = review_symbol(sym);
        let result = try_modify_symbol(val, |s| Symbol::new_unqualified(&s.name().to_uppercase()));
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_symbol(&modified).unwrap().name(), "TEST");
        }
    }

    #[test]
    fn test_try_modify_symbol_failure() {
        let val = review_integer(42);
        let result = try_modify_symbol(val.clone(), |_s| Symbol::new_unqualified("ignored"));
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_keyword() {
        let kw = Keyword::new_unqualified("foo");
        let val = review_keyword(kw);
        let modified = modify_keyword(val, |k| Keyword::new_unqualified(&format!("{}_modified", k.name())));
        let result_kw = preview_keyword(&modified);
        assert!(result_kw.is_some());
        assert_eq!(result_kw.unwrap().name(), "foo_modified");
    }

    #[test]
    fn test_modify_keyword_wrong_type() {
        let val = review_integer(42);
        let modified = modify_keyword(val.clone(), |_k| Keyword::new_unqualified("bar"));
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_keyword_success() {
        let kw = Keyword::new_unqualified("test");
        let val = review_keyword(kw);
        let result = try_modify_keyword(val, |k| Keyword::new_unqualified(&k.name().to_uppercase()));
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_keyword(&modified).unwrap().name(), "TEST");
        }
    }

    #[test]
    fn test_try_modify_keyword_failure() {
        let val = review_integer(42);
        let result = try_modify_keyword(val.clone(), |_k| Keyword::new_unqualified("ignored"));
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_vector() {
        let vec = Vector::new_empty();
        let val = review_vector(vec.clone());
        // Just verify that modify is called and returns the modified value
        let modified = modify_vector(val, |v| v);
        let result_vec = preview_vector(&modified);
        assert!(result_vec.is_some());
        assert_eq!(result_vec.unwrap().len(), 0);
    }

    #[test]
    fn test_modify_vector_wrong_type() {
        let val = review_integer(42);
        let modified = modify_vector(val.clone(), |v| v);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_vector_success() {
        let vec = Vector::new_empty();
        let val = review_vector(vec);
        let result = try_modify_vector(val, |v| v);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_vector(&modified).unwrap().len(), 0);
        }
    }

    #[test]
    fn test_try_modify_vector_failure() {
        let val = review_integer(42);
        let result = try_modify_vector(val.clone(), |v| v);
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_list() {
        let list = List::new_empty();
        let val = review_list(list);
        let modified = modify_list(val, |l| l);
        let result_list = preview_list(&modified);
        assert!(result_list.is_some());
        assert_eq!(result_list.unwrap().len(), 0);
    }

    #[test]
    fn test_modify_list_wrong_type() {
        let val = review_integer(42);
        let modified = modify_list(val.clone(), |l| l);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_list_success() {
        let list = List::new_empty();
        let val = review_list(list);
        let result = try_modify_list(val, |l| l);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_list(&modified).unwrap().len(), 0);
        }
    }

    #[test]
    fn test_try_modify_list_failure() {
        let val = review_integer(42);
        let result = try_modify_list(val.clone(), |l| l);
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_set() {
        let set = Set::new_empty();
        let val = review_set(set);
        let modified = modify_set(val, |s| s);
        let result_set = preview_set(&modified);
        assert!(result_set.is_some());
        assert_eq!(result_set.unwrap().len(), 0);
    }

    #[test]
    fn test_modify_set_wrong_type() {
        let val = review_integer(42);
        let modified = modify_set(val.clone(), |s| s);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_set_success() {
        let set = Set::new_empty();
        let val = review_set(set);
        let result = try_modify_set(val, |s| s);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_set(&modified).unwrap().len(), 0);
        }
    }

    #[test]
    fn test_try_modify_set_failure() {
        let val = review_integer(42);
        let result = try_modify_set(val.clone(), |s| s);
        assert!(result.is_err());
    }

    #[test]
    fn test_modify_map() {
        let map = Map::new_empty();
        let val = review_map(map);
        let modified = modify_map(val, |m| m);
        let result_map = preview_map(&modified);
        assert!(result_map.is_some());
        assert_eq!(result_map.unwrap().len(), 0);
    }

    #[test]
    fn test_modify_map_wrong_type() {
        let val = review_integer(42);
        let modified = modify_map(val.clone(), |m| m);
        assert_eq!(modified, val);
    }

    #[test]
    fn test_try_modify_map_success() {
        let map = Map::new_empty();
        let val = review_map(map);
        let result = try_modify_map(val, |m| m);
        assert!(result.is_ok());
        if let Ok(modified) = result {
            assert_eq!(preview_map(&modified).unwrap().len(), 0);
        }
    }

    #[test]
    fn test_try_modify_map_failure() {
        let val = review_integer(42);
        let result = try_modify_map(val.clone(), |m| m);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_map() {
        let map1 = Map::new_empty();
        let val = review_map(map1);
        let map2 = Map::new_empty();
        let new_val = set_map(val, map2);
        let result_map = preview_map(&new_val);
        assert!(result_map.is_some());
        assert_eq!(result_map.unwrap().len(), 0);
    }

    // meta (metadata)
    // ========================================

    #[test]
    fn test_preview_meta_none() {
        let val = review_integer(42);
        assert_eq!(preview_meta(&val), None);
    }

    #[test]
    fn test_preview_meta_with_metadata() {
        let meta = Map::new_empty();
        let val = set_meta(review_integer(42), Some(Rc::new(meta)));
        assert!(preview_meta(&val).is_some());
    }

    #[test]
    fn test_modify_meta_add() {
        let val = review_integer(42);
        let modified = modify_meta(val, |_| {
            Some(Rc::new(Map::new_empty()))
        });
        assert!(preview_meta(&modified).is_some());
        // Value should still be an integer
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_modify_meta_remove() {
        let val = set_meta(review_integer(42), Some(Rc::new(Map::new_empty())));
        assert!(preview_meta(&val).is_some());
        
        let modified = modify_meta(val, |_| None);
        assert_eq!(preview_meta(&modified), None);
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_set_meta_replaces() {
        let val = set_meta(review_integer(42), Some(Rc::new(Map::new_empty())));
        assert!(preview_meta(&val).is_some());
        
        let new_meta = Map::new_empty();
        let modified = set_meta(val, Some(Rc::new(new_meta)));
        assert!(preview_meta(&modified).is_some());
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_set_meta_to_none() {
        let val = set_meta(review_integer(42), Some(Rc::new(Map::new_empty())));
        let modified = set_meta(val, None);
        assert_eq!(preview_meta(&modified), None);
        assert_eq!(preview_integer(&modified), Some(42));
    }

    #[test]
    fn test_try_get_meta_success() {
        let meta = Map::new_empty();
        let val = set_meta(review_integer(42), Some(Rc::new(meta)));
        let result = try_get_meta(val);
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_get_meta_failure() {
        let val = review_integer(42);
        let result = try_get_meta(val.clone());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), val);
    }

    #[test]
    fn test_meta_on_various_types() {
        // Test that metadata operations work on all value types
        let values = vec![
            review_nil(),
            review_boolean(true),
            review_integer(42),
            review_float(3.14),
            review_string("test".to_string()),
            review_symbol(Symbol::new_unqualified("sym")),
            review_keyword(Keyword::new_unqualified("kw")),
            review_list(List::new_empty()),
            review_vector(Vector::new_empty()),
            review_set(Set::new_empty()),
            review_map(Map::new_empty()),
        ];
        
        for val in values {
            assert_eq!(preview_meta(&val), None);
            
            let with_meta = set_meta(val.clone(), Some(Rc::new(Map::new_empty())));
            assert!(preview_meta(&with_meta).is_some());
            
            let without_meta = set_meta(with_meta, None);
            assert_eq!(preview_meta(&without_meta), None);
        }
    }

    // preview_*_ref tests
    // ========================================

    #[test]
    fn test_preview_float_ref() {
        let val = review_float(3.14);
        let float_ref = preview_float_ref(&val);
        assert!(float_ref.is_some());
        assert_eq!(float_ref.unwrap().as_f64(), 3.14);
    }

    #[test]
    fn test_preview_string_ref() {
        let val = review_string("hello".to_string());
        let str_ref = preview_string_ref(&val);
        assert_eq!(str_ref, Some("hello"));
    }

    #[test]
    fn test_preview_symbol_ref() {
        let sym = Symbol::new_unqualified("test");
        let val = review_symbol(sym.clone());
        let sym_ref = preview_symbol_ref(&val);
        assert!(sym_ref.is_some());
        assert_eq!(sym_ref.unwrap(), &sym);
    }

    #[test]
    fn test_preview_keyword_ref() {
        let kw = Keyword::new_unqualified("test");
        let val = review_keyword(kw.clone());
        let kw_ref = preview_keyword_ref(&val);
        assert!(kw_ref.is_some());
        assert_eq!(kw_ref.unwrap(), &kw);
    }

    #[test]
    fn test_preview_list_ref() {
        let list = List::new_empty();
        let val = review_list(list.clone());
        let list_ref = preview_list_ref(&val);
        assert!(list_ref.is_some());
        assert_eq!(list_ref.unwrap().len(), 0);
    }

    #[test]
    fn test_preview_vector_ref() {
        let vector = Vector::new_empty();
        let val = review_vector(vector.clone());
        let vec_ref = preview_vector_ref(&val);
        assert!(vec_ref.is_some());
        assert_eq!(vec_ref.unwrap().len(), 0);
    }

    #[test]
    fn test_preview_set_ref() {
        let set = Set::new_empty();
        let val = review_set(set.clone());
        let set_ref = preview_set_ref(&val);
        assert!(set_ref.is_some());
        assert_eq!(set_ref.unwrap().len(), 0);
    }

    #[test]
    fn test_preview_map_ref() {
        let map = Map::new_empty();
        let val = review_map(map.clone());
        let map_ref = preview_map_ref(&val);
        assert!(map_ref.is_some());
        assert_eq!(map_ref.unwrap().len(), 0);
    }

    #[test]
    fn test_preview_var_ref() {
        let var = Var::new_bound(review_integer(42));
        let rc_var = Rc::new(var);
        let val = review_var(rc_var.clone());
        let var_ref = preview_var_ref(&val);
        assert!(var_ref.is_some());
        assert_eq!(var_ref.unwrap().is_bound(), true);
    }

    #[test]
    fn test_preview_function_ref() {
        // Functions are complex to construct; just test that the ref function is available
        // by testing with its absence (type mismatch case)
        let val = review_integer(42);
        let func_ref = preview_function_ref(&val);
        assert!(func_ref.is_none());
    }

    #[test]
    fn test_preview_handle_ref() {
        // Handles require IHandle trait; just test that it's available with type mismatch
        let val = review_integer(42);
        let handle_ref = preview_handle_ref(&val);
        assert!(handle_ref.is_none());
    }

    #[test]
    fn test_preview_meta_ref() {
        let val_no_meta = review_integer(42);
        assert!(preview_meta_ref(&val_no_meta).is_none());

        let val_with_meta = set_meta(review_integer(42), Some(Rc::new(Map::new_empty())));
        let meta_ref = preview_meta_ref(&val_with_meta);
        assert!(meta_ref.is_some());
    }

    #[test]
    fn test_preview_ref_type_mismatch() {
        let val = review_integer(42);
        
        assert!(preview_string_ref(&val).is_none());
        assert!(preview_symbol_ref(&val).is_none());
        assert!(preview_list_ref(&val).is_none());
        assert!(preview_map_ref(&val).is_none());
    }
}
