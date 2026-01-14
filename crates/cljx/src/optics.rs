//! Optics (Prisms, Lenses, Traversals) for focusing on parts of Values and other types.
//!
//! This module provides composable, zero-cost abstractions for accessing and transforming
//! nested data structures without exhaustive pattern matching boilerplate.
//!
//! Type-specific optics are found in their respective modules:
//! - Value optics: `cljx::value::optics` module
//! - Var optics: `cljx::var::optics` module
//! - Namespace optics: `cljx::namespace::optics` module
//! - Environment optics: `cljx::environment::optics` module

/// A Prism focuses on an optional part of a structure.
///
/// Unlike a Lens (which focuses on a required part), a Prism may or may not find its target.
/// It provides two operations:
/// - `view`: extract (or clone) the inner value (returns `Option`)
/// - `set`: reconstruct the structure with a new inner value, preserving context
///
/// **Immutable Semantics**: When working with immutable persistent data structures
/// (like List, Vector, Set, Map from the `im` crate), `set` does not mutate in place.
/// Instead, it produces a new structure with the updated inner value. The `view` operation
/// clones the inner value (with O(1) clone cost due to structural sharing).
pub trait IPrismOwn<S, A> {
    /// Extract (or clone) the inner value from the structure, if this prism matches.
    fn view(&self, source: &S) -> Option<A>;

    /// Reconstruct the structure with a new inner value, preserving metadata and context.
    /// Only produces output if the prism matches; otherwise returns None.
    /// For immutable structures, this produces a new reconstructed structure.
    fn set(&self, source: &S, value: A) -> Option<S>;
}

/// A generic Prism implementation that wraps view and set operations.
/// Implements the IPrism trait for composable focusing on nested structures.
#[derive(Copy, Clone)]
pub struct Prism<S, A> {
    view_fn: fn(&S) -> Option<A>,
    set_fn: fn(&S, A) -> Option<S>,
}

impl<S, A> Prism<S, A> {
    /// Create a new prism with the given view and set functions.
    pub fn new(
        view_fn: fn(&S) -> Option<A>,
        set_fn: fn(&S, A) -> Option<S>,
    ) -> Self {
        Prism { view_fn, set_fn }
    }
}

impl<S, A> IPrismOwn<S, A> for Prism<S, A> {
    fn view(&self, source: &S) -> Option<A> {
        (self.view_fn)(source)
    }

    fn set(&self, source: &S, value: A) -> Option<S> {
        (self.set_fn)(source, value)
    }
}



/// A Prism focuses on an optional part of a structure.
/// A PrismRef's `view` operation results in a reference to the focused part of a structure.
///
/// Unlike a Lens (which focuses on a required part), a Prism may or may not find its target.
/// It provides two operations:
/// - `view`: reference the inner value (returns `Option`)
/// - `set`: reconstruct the structure with a new inner value, preserving context
///
/// **Immutable Semantics**: When working with immutable persistent data structures
/// (like [`List`], [`Vector`], [`Set`], [`Map`] from the `im` crate), `set` does not mutate in place.
/// Instead, it produces a new structure with the updated inner value.
pub trait IPrismRef<'s, S, A> {
    /// Obtain a reference to the inner value, if the prism matches.
    fn view(&self, source: &'s S) -> Option<&'s A>;
    fn set(&self, source: &'s S, value: A) -> Option<S>;
}

/// A generic Prism implementation that wraps `view` and `set` operations.
/// Implements the [`IPrismRef`] trait for composable focusing on nested structures.
#[derive(Copy, Clone)]
pub struct PrismRef<'s, S, A> {
    view_fn: fn(&'s S) -> Option<&'s A>,
    set_fn: fn(&'s S, A) -> Option<S>,
}

impl<'s, S, A> PrismRef<'s, S, A> {
    pub fn new(
        view_fn: fn(&'s S) -> Option<&'s A>,
        set_fn: fn(&'s S, A) -> Option<S>,
    ) -> Self {
        Self {
            view_fn,
            set_fn,
        }
    }
}

impl<'s, S, A> IPrismRef<'s, S, A> for PrismRef<'s, S, A> {
    fn view(&self, source: &'s S) -> Option<&'s A> {
        (self.view_fn)(source)
    }

    fn set(&self, source: &'s S, value: A) -> Option<S> {
        (self.set_fn)(source, value)
    }
}
