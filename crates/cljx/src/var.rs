use std::{cell::RefCell, hash::{Hasher, Hash}};
use std::rc::Rc;
use std::collections::HashSet;
use crate::prelude::*;

pub type RcVar = Rc<Var>;

pub mod optics;

thread_local! {
    /// Track which Vars are currently being accessed by this thread.
    /// This allows safe re-entrant access: the same thread can call deref/bind
    /// multiple times on the same Var without panicking.
    ///
    /// Key: pointer address of the Var on the heap (stable via Rc)
    /// Enables support for:
    /// - Recursive functions accessing their own bindings
    /// - Let bindings where the RHS eval accesses the variable being bound
    static ACQUIRING_VARS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

/// A mutable (possibly empty) [`Value`]-holding place that supports re-entrant access.
///
/// e.g.: `#'clojure.core/*ns*`
///
/// This allows the same thread to call [`Var::deref`] or [`Var::bind`] multiple
/// times without panicking. Each [`Var`] is typically wrapped in [`Rc`]
/// (as [`RcVar`]), and multiple borrows from the same thread are safe as long
/// as they're not simultaneous (which they aren't in our interpreter). The
/// re-entrancy tracking uses the stable heap address of the [`Var`] provided
/// by [`Rc`] clones.
#[derive(Clone, Debug)]
pub struct Var {
    /// The actual value stored in this variable
    value: RefCell<Option<RcValue>>,
    /// Track recursion depth for this variable (per thread)
    depth: RefCell<usize>,
    /// Metadata associated with this Var (independent from the bound value's metadata)
    meta: RefCell<Rc<Option<Map>>>,
}

impl Var {
    /// Create a new unbound variable
    pub fn new_unbound() -> Self {
        Self {
            value: RefCell::new(None),
            depth: RefCell::new(0),
            meta: RefCell::new(meta::new_unset_rc()),
        }
    }

    /// Create a new variable bound to a value
    pub fn new_bound(rc_value: impl Into<RcValue>) -> Self {
        Self {
            value: RefCell::new(Some(rc_value.into())),
            depth: RefCell::new(0),
            meta: RefCell::new(meta::new_unset_rc()),
        }
    }

    /// Create a new unbound variable with initial metadata
    pub fn new_unbound_with_meta(meta: Rc<Option<Map>>) -> Self {
        Self {
            value: RefCell::new(None),
            depth: RefCell::new(0),
            meta: RefCell::new(meta),
        }
    }

    /// Create a new variable bound to a value with initial metadata
    pub fn new_bound_with_meta(rc_value: impl Into<RcValue>, meta: Rc<Option<Map>>) -> Self {
        Self {
            value: RefCell::new(Some(rc_value.into())),
            depth: RefCell::new(0),
            meta: RefCell::new(meta),
        }
    }

    /// Check if the variable is unbound
    pub fn is_unbound(&self) -> bool {
        self.value.borrow().is_none()
    }

    /// Check if the variable is bound
    pub fn is_bound(&self) -> bool {
        self.value.borrow().is_some()
    }

    /// Get the value, allowing re-entrant access from the same thread.
    /// Supports Scenario 1: Let bindings where the RHS eval accesses the variable
    /// Supports Scenario 2: Recursive functions accessing their own binding
    pub fn deref(&self) -> Option<RcValue> {
        let var_ptr = self as *const Var as usize;
        
        // Track this variable's access depth for this thread
        let was_first_entry = {
            let mut depth = self.depth.borrow_mut();
            let was_first = *depth == 0;
            *depth += 1;
            was_first
        };

        // If this is the first time we're acquiring this var in this thread,
        // record it for potential future deadlock detection
        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().insert(var_ptr);
            });
        }

        let result = self.value.borrow().as_ref().cloned();

        // Release our acquisition if this was the outermost call
        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        // Decrement depth
        *self.depth.borrow_mut() -= 1;

        result
    }

    /// Bind a value to this variable, allowing re-entrant calls from the same thread.
    /// Supports Scenario 1: Let bindings where the RHS eval modifies the variable
    /// Supports Scenario 2: Recursive functions modifying their own binding
    pub fn bind(&self, rc_value: impl Into<RcValue>) {
        let var_ptr = self as *const Var as usize;
        
        let was_first_entry = {
            let mut depth = self.depth.borrow_mut();
            let was_first = *depth == 0;
            *depth += 1;
            was_first
        };

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().insert(var_ptr);
            });
        }

        *self.value.borrow_mut() = Some(rc_value.into());

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        *self.depth.borrow_mut() -= 1;
    }

    /// Unbind the variable (set to None), allowing re-entrant calls from the same thread.
    pub fn unbind(&self) {
        let var_ptr = self as *const Var as usize;
        
        let was_first_entry = {
            let mut depth = self.depth.borrow_mut();
            let was_first = *depth == 0;
            *depth += 1;
            was_first
        };

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().insert(var_ptr);
            });
        }

        *self.value.borrow_mut() = None;

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        *self.depth.borrow_mut() -= 1;
    }

    /// Get the current metadata associated with this Var.
    /// 
    /// Each Var has its own independent metadata, separate from the metadata
    /// of the value it holds. This returns the Var's metadata.
    pub fn meta(&self) -> Rc<Option<Map>> {
        self.meta.borrow().clone()
    }

    /// Get a value from the metadata map by key.
    /// 
    /// Returns None if the metadata is empty (None) or if the key is not found.
    pub fn get_meta(&self, key: &RcValue) -> Option<RcValue> {
        self.meta.borrow().get(key)
    }

    /// Replace the entire metadata of this Var in-place.
    pub fn set_meta(&self, meta: Rc<Option<Map>>) {
        *self.meta.borrow_mut() = meta;
    }

    /// Associate a key-value pair in the metadata, in-place.
    /// 
    /// This updates or inserts the key-value pair in the current metadata map,
    /// storing the result back in the Var's metadata. Returns the new metadata.
    pub fn assoc_meta(&self, key: RcValue, value: RcValue) -> Rc<Option<Map>> {
        let new_meta = self.meta.borrow().assoc(key, value);
        *self.meta.borrow_mut() = new_meta.clone();
        new_meta
    }

    pub fn into_value(var: RcVar) -> Value {
        Value::Var(var.clone(), var.meta())
    }

    pub fn into_value_rc(var: RcVar) -> RcValue {
        Rc::new(Value::Var(var.clone(), var.meta()))
    }
}

impl From<Value> for Var {
    fn from(value: Value) -> Self {
        Var::new_bound(value)
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for Var {}

impl PartialOrd for Var {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_ptr = self as *const Var as usize;
        let other_ptr = other as *const Var as usize;
        self_ptr.partial_cmp(&other_ptr)
    }
}

impl Ord for Var {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ptr = self as *const Var as usize;
        let other_ptr = other as *const Var as usize;
        self_ptr.cmp(&other_ptr)
    }
}

impl From<RcValue> for Var {
    fn from(value: RcValue) -> Self {
        Var::new_bound(value)
    }
}

impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = self as *const Var as usize;
        ptr.hash(state);
    }
}

impl MetaOps for Var {
    /// Associate a key-value pair in this Var's metadata.
    /// 
    /// This delegates to assoc_meta() which mutates in-place.
    fn assoc(&self, key: RcValue, value: RcValue) -> Rc<Option<Map>> {
        self.assoc_meta(key, value)
    }

    /// Get a value from this Var's metadata by key.
    fn get(&self, key: &RcValue) -> Option<RcValue> {
        self.get_meta(key)
    }
}

impl MetaOps for RcVar {
    /// Associate a key-value pair in this Var's metadata.
    fn assoc(&self, key: RcValue, value: RcValue) -> Rc<Option<Map>> {
        Var::assoc(self, key, value)
    }

    /// Get a value from this Var's metadata by key.
    fn get(&self, key: &RcValue) -> Option<RcValue> {
        Var::get(self, key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_unbound() {
        let var = Var::new_unbound();
        assert!(var.is_unbound());
        assert!(!var.is_bound());
        assert_eq!(var.deref(), None);
    }

    #[test]
    fn new_bound() {
        let val = Rc::new(Value::from(42.0));
        let var = Var::new_bound(val.clone());
        assert!(!var.is_unbound());
        assert!(var.is_bound());
        assert_eq!(var.deref(), Some(val));
    }

    #[test]
    fn bind_unbind() {
        let var = Var::new_unbound();
        let val = Rc::new(Value::from(42.0));
        
        var.bind(val.clone());
        assert!(var.is_bound());
        assert_eq!(var.deref(), Some(val));
        
        var.unbind();
        assert!(var.is_unbound());
        assert_eq!(var.deref(), None);
    }

    #[test]
    fn reentrant_access_same_thread() {
        // This is the critical test - same thread can deref multiple times
        let var = Var::new_bound(Rc::new(Value::from(42.0)));
        
        // First access
        let val1 = var.deref();
        
        // Re-entrant access (same thread, same var)
        let val2 = var.deref();
        
        // Both should succeed and return the same value
        assert_eq!(val1, val2);
        assert_eq!(val1, Some(Rc::new(Value::from(42.0))));
    }

    #[test]
    fn reentrant_bind_during_deref() {
        // Simulate Scenario 1: let binding where RHS accesses the variable
        let var = Var::new_bound(Rc::new(Value::from(10.0)));
        
        // "During" deref, we bind (re-entrant)
        let val = var.deref();
        assert_eq!(val, Some(Rc::new(Value::from(10.0))));
        
        // Re-entrant bind should work
        var.bind(Rc::new(Value::from(20.0)));
        
        // And the change should be visible in new deref
        assert_eq!(var.deref(), Some(Rc::new(Value::from(20.0))));
    }

    #[test]
    fn recursive_function_scenario() {
        // Simulate Scenario 2: recursive function accessing its own binding
        // fib is bound to itself, and during eval of (fib ...) it accesses fib
        let fib_var = Var::new_bound(Rc::new(Value::from(42.0))); // Simulating the function value
        
        // "During" evaluation, we access fib again
        let val_outer = fib_var.deref();
        let val_inner = fib_var.deref();
        
        assert_eq!(val_outer, val_inner);
        assert_eq!(val_inner, Some(Rc::new(Value::from(42.0))));
    }

    #[test]
    fn multiple_derefs() {
        // Test deeply nested re-entrant access
        let var = Var::new_bound(Rc::new(Value::from(99.0)));
        
        let val1 = var.deref();
        let val2 = var.deref();
        let val3 = var.deref();
        let val4 = var.deref();
        
        assert_eq!(val1, Some(Rc::new(Value::from(99.0))));
        assert_eq!(val2, Some(Rc::new(Value::from(99.0))));
        assert_eq!(val3, Some(Rc::new(Value::from(99.0))));
        assert_eq!(val4, Some(Rc::new(Value::from(99.0))));
    }

    // ===== Metadata Tests =====

    #[test]
    fn meta_default_empty() {
        let var = Var::new_unbound();
        let meta = var.meta();
        assert!(meta.as_ref().is_none());
    }

    #[test]
    fn get_meta_empty() {
        let var = Var::new_unbound();
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("test")));
        assert_eq!(var.get_meta(&key), None);
    }

    #[test]
    fn set_meta_and_get() {
        let var = Var::new_unbound();
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("name")));
        let value = Rc::new(Value::string("my-var".to_string()));
        
        // Create a metadata map and set it
        let mut meta_map = Map::new_empty();
        meta_map.insert(key.clone(), value.clone());
        let meta = meta::new_rc(meta_map);
        
        var.set_meta(meta);
        
        // Verify we can get it back
        let retrieved = var.get_meta(&key);
        assert_eq!(retrieved, Some(value));
    }

    #[test]
    fn assoc_meta_in_place() {
        let var = Var::new_unbound();
        let key1 = Rc::new(Value::keyword(Keyword::new_unqualified("key1")));
        let value1 = Rc::new(Value::from(10.0));
        
        // assoc_meta on empty should create new map
        let new_meta = var.assoc_meta(key1.clone(), value1.clone());
        
        // Metadata should now contain the key-value pair
        assert_eq!(var.get_meta(&key1), Some(value1));
        
        // All the same Rc pointer
        assert!(Rc::ptr_eq(&var.meta(), &new_meta));
    }

    #[test]
    fn assoc_meta_multiple_keys() {
        let var = Var::new_unbound();
        
        let key1 = Rc::new(Value::keyword(Keyword::new_unqualified("key1")));
        let value1 = Rc::new(Value::from(10.0));
        let key2 = Rc::new(Value::keyword(Keyword::new_unqualified("key2")));
        let value2 = Rc::new(Value::from(20.0));
        
        var.assoc_meta(key1.clone(), value1.clone());
        var.assoc_meta(key2.clone(), value2.clone());
        
        assert_eq!(var.get_meta(&key1), Some(value1));
        assert_eq!(var.get_meta(&key2), Some(value2));
    }

    #[test]
    fn assoc_meta_update() {
        let var = Var::new_unbound();
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("counter")));
        let value1 = Rc::new(Value::from(10.0));
        let value2 = Rc::new(Value::from(20.0));
        
        var.assoc_meta(key.clone(), value1);
        assert_eq!(var.get_meta(&key), Some(Rc::new(Value::from(10.0))));
        
        var.assoc_meta(key.clone(), value2);
        assert_eq!(var.get_meta(&key), Some(Rc::new(Value::from(20.0))));
    }

    #[test]
    fn new_unbound_with_meta() {
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("test")));
        let value = Rc::new(Value::string("initial".to_string()));
        
        let mut meta_map = Map::new_empty();
        meta_map.insert(key.clone(), value.clone());
        let meta = meta::new_rc(meta_map);
        
        let var = Var::new_unbound_with_meta(meta);
        
        assert!(var.is_unbound());
        assert_eq!(var.get_meta(&key), Some(value));
    }

    #[test]
    fn new_bound_with_meta() {
        let bound_value = Rc::new(Value::from(42.0));
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("test")));
        let meta_value = Rc::new(Value::string("meta".to_string()));
        
        let mut meta_map = Map::new_empty();
        meta_map.insert(key.clone(), meta_value.clone());
        let meta = meta::new_rc(meta_map);
        
        let var = Var::new_bound_with_meta(bound_value.clone(), meta);
        
        assert!(var.is_bound());
        assert_eq!(var.deref(), Some(bound_value));
        assert_eq!(var.get_meta(&key), Some(meta_value));
    }

    #[test]
    fn meta_persists_across_bind() {
        let var = Var::new_unbound();
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("persistent")));
        let key_value = Rc::new(Value::string("yes".to_string()));
        var.assoc_meta(key.clone(), key_value.clone());
        
        // Bind a new value
        let new_value = Rc::new(Value::from(99.0));
        var.bind(new_value);
        
        // Metadata should persist
        assert_eq!(var.get_meta(&key), Some(key_value));
        assert_eq!(var.deref(), Some(Rc::new(Value::from(99.0))));
    }

    #[test]
    fn meta_persists_across_unbind() {
        let var = Var::new_bound(Rc::new(Value::from(42.0)));
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("persistent")));
        let key_value = Rc::new(Value::string("yes".to_string()));
        var.assoc_meta(key.clone(), key_value.clone());
        
        // Unbind the value
        var.unbind();
        
        // Metadata should persist
        assert_eq!(var.get_meta(&key), Some(key_value));
        assert!(var.is_unbound());
    }

    #[test]
    fn metaops_trait_assoc() {
        let var = Var::new_unbound();
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("via-trait")));
        let value = Rc::new(Value::string("trait-value".to_string()));
        
        // Use MetaOps trait method
        let new_meta = MetaOps::assoc(&var, key.clone(), value.clone());
        
        assert_eq!(var.get_meta(&key), Some(value));
        assert!(Rc::ptr_eq(&var.meta(), &new_meta));
    }

    #[test]
    fn metaops_trait_get() {
        let var = Var::new_unbound();
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("via-trait")));
        let value = Rc::new(Value::string("trait-value".to_string()));
        
        var.assoc_meta(key.clone(), value.clone());
        
        // Use MetaOps trait method
        let retrieved = MetaOps::get(&var, &key);
        assert_eq!(retrieved, Some(value));
    }

    #[test]
    fn metaops_trait_on_rcvar() {
        let var: RcVar = Rc::new(Var::new_unbound());
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("rc-var")));
        let value = Rc::new(Value::string("rc-value".to_string()));
        
        // Use MetaOps trait on RcVar
        let new_meta = MetaOps::assoc(&var, key.clone(), value.clone());
        
        assert_eq!(MetaOps::get(&var, &key), Some(value));
        assert!(Rc::ptr_eq(&var.meta(), &new_meta));
    }

    #[test]
    fn var_metadata_independence() {
        // Test the user's scenario: two vars with independent metadata, one bound to another
        let var1 = Rc::new(Var::new_bound(Rc::new(Value::from(42.0))));
        let var2 = Rc::new(Var::new_bound(Rc::new(Value::Var(var1.clone(), var1.meta()))));
        
        // Set metadata on each var
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("id")));
        let var1_meta_value = Rc::new(Value::string("var1".to_string()));
        let var2_meta_value = Rc::new(Value::string("var2".to_string()));
        
        var1.assoc_meta(key.clone(), var1_meta_value.clone());
        var2.assoc_meta(key.clone(), var2_meta_value.clone());
        
        // Verify independence
        assert_eq!(var1.get_meta(&key), Some(var1_meta_value.clone()));
        assert_eq!(var2.get_meta(&key), Some(var2_meta_value));
        
        // var2 is bound to var1, but metadata is separate
        if let Some(var2_value_rc) = var2.deref() {
            if let Value::Var(deref_var, _) = var2_value_rc.as_ref() {
                assert_eq!(deref_var.get_meta(&key), Some(var1_meta_value));
            } else {
                panic!("Expected var2 to be bound to a Var");
            }
        } else {
            panic!("Expected var2 to be bound");
        }
    }

    #[test]
    fn value_var_wrapper_shares_var_metadata() {
        // When creating a Value::Var, it should share the Var's metadata
        let var = Rc::new(Var::new_unbound());
        
        let key = Rc::new(Value::keyword(Keyword::new_unqualified("wrapper-test")));
        let value = Rc::new(Value::string("shared".to_string()));
        
        var.assoc_meta(key.clone(), value.clone());
        
        // Create Value::Var using Var::into_value()
        let value_var = Var::into_value(var.clone());
        
        // Extract the metadata from Value::Var
        if let Value::Var(_, meta) = value_var {
            // The wrapper's metadata should be the same Rc pointer as var.meta()
            assert!(Rc::ptr_eq(&meta, &var.meta()));
        } else {
            panic!("Expected Value::Var variant");
        }
    }
}