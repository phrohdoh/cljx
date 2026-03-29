//! Optics for Vectors: prisms to focus on Vector variants within Values,
//! and helpers for accessing elements.
//!
//! Note: Since Vector is an immutable persistent data structure (from the `im` crate),
//! `view_*` functions clone the Vector (necessary to return owned data), and `set_*` functions
//! produce a new Value with a new Vector rather than mutating in place. This follows standard
//! functional optics semantics.
use crate::prelude::*;

/// View the Vector focused by this prism: clone the inner Vector from the Value if it is a Vector variant.
///
/// Since Vector is immutable, this returns an owned clone of the persistent Vector structure.
/// The clone is O(1) due to structural sharing in the underlying `im::Vector`.
pub fn view(value: &Value) -> Option<Vector> {
    if let Value::Vector(v, _) = value {
        Some(v.clone())
    } else {
        None
    }
}

/// Set the Vector focused by this prism: produce a new Value with the given Vector,
/// preserving the metadata from the original Value.
///
/// This does not mutate the original Value; it creates a new one with the new Vector.
pub fn set(value: &Value, v: Vector) -> Option<Value> {
    if let Value::Vector(_, m) = value {
        Some(Value::Vector(v, m.clone()))
    } else {
        None
    }
}

// =============================================================================
// Element access helpers for Vectors
// =============================================================================

/// View the first element of a Vector inside a Value, or None if the Value is not a Vector or is empty.
///
/// Returns a cloned reference to the first element (since Vector is persistent and immutable).
pub fn view_first(value: &Value) -> Option<RcValue> {
    view(value)
        .and_then(|vector| vector.get_first().map(|v| v.to_owned()))
}

/// View the second element (index 1) of a Vector inside a Value, or None if not present.
///
/// Returns a cloned reference to the element (since Vector is persistent and immutable).
pub fn view_second(value: &Value) -> Option<RcValue> {
    view(value)
        .and_then(|vector| vector.get_second().map(|v| v.to_owned()))
}

/// View the last element of a Vector inside a Value, or None if the Value is not a Vector or is empty.
///
/// Returns a cloned reference to the last element (since Vector is persistent and immutable).
pub fn view_last(value: &Value) -> Option<RcValue> {
    view(value)
        .and_then(|vector| vector.get_last().map(|v| v.to_owned()))
}

/// View the element at index n of a Vector inside a Value, or None if out of bounds.
///
/// Returns a cloned reference to the element (since Vector is persistent and immutable).
pub fn view_nth(value: &Value, n: usize) -> Option<RcValue> {
    view(value)
        .and_then(|vector| vector.get_nth_or_nil(n).into())
}
