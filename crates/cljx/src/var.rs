use std::{cell::RefCell, hash::{Hasher, Hash}};
use std::rc::Rc;
use std::collections::HashSet;
use crate::prelude::*;

pub type RcVar = Rc<Var>;

thread_local! {
    /// Track which Vars are currently being accessed by this thread.
    /// This allows safe re-entrant access: the same thread can call deref/bind
    /// multiple times on the same Var without panicking.
    ///
    /// Key: pointer address of the Var's inner structure
    /// Enables support for:
    /// - Recursive functions accessing their own bindings
    /// - Let bindings where the RHS eval accesses the variable being bound
    static ACQUIRING_VARS: RefCell<HashSet<usize>> = RefCell::new(HashSet::new());
}

/// A mutable (possibly empty) [Value]-holding place that supports re-entrant access.
///
/// e.g.: `#'clojure.core/*ns*`
///
/// Key difference from RefCell-based Var: This allows the same thread to call
/// deref() or bind() multiple times without panicking. The inner RefCell only
/// holds the value, not a lock guard, so multiple borrows from the same thread
/// are safe as long as they're not simultaneous (which they aren't in our interpreter).
#[derive(Clone, Debug)]
pub struct Var {
    inner: Rc<VarInner>,
}

#[derive(Debug)]
struct VarInner {
    /// The actual value stored in this variable
    value: RefCell<Option<RcValue>>,
    /// Track recursion depth for this variable (per thread)
    depth: RefCell<usize>,
}

impl Var {
    /// Create a new unbound variable
    pub fn new_unbound() -> Self {
        Self {
            inner: Rc::new(VarInner {
                value: RefCell::new(None),
                depth: RefCell::new(0),
            }),
        }
    }

    /// Create a new variable bound to a value
    pub fn new_bound(rc_value: impl Into<RcValue>) -> Self {
        Self {
            inner: Rc::new(VarInner {
                value: RefCell::new(Some(rc_value.into())),
                depth: RefCell::new(0),
            }),
        }
    }

    /// Check if the variable is unbound
    pub fn is_unbound(&self) -> bool {
        self.inner.value.borrow().is_none()
    }

    /// Check if the variable is bound
    pub fn is_bound(&self) -> bool {
        self.inner.value.borrow().is_some()
    }

    /// Get the value, allowing re-entrant access from the same thread.
    /// Supports Scenario 1: Let bindings where the RHS eval accesses the variable
    /// Supports Scenario 2: Recursive functions accessing their own binding
    pub fn deref(&self) -> Option<RcValue> {
        let var_ptr = self.inner.as_ref() as *const VarInner as usize;
        
        // Track this variable's access depth for this thread
        let was_first_entry = {
            let mut depth = self.inner.depth.borrow_mut();
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

        let result = self.inner.value.borrow().as_ref().cloned();

        // Release our acquisition if this was the outermost call
        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        // Decrement depth
        *self.inner.depth.borrow_mut() -= 1;

        result
    }

    /// Bind a value to this variable, allowing re-entrant calls from the same thread.
    /// Supports Scenario 1: Let bindings where the RHS eval modifies the variable
    /// Supports Scenario 2: Recursive functions modifying their own binding
    pub fn bind(&self, rc_value: impl Into<RcValue>) {
        let var_ptr = self.inner.as_ref() as *const VarInner as usize;
        
        let was_first_entry = {
            let mut depth = self.inner.depth.borrow_mut();
            let was_first = *depth == 0;
            *depth += 1;
            was_first
        };

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().insert(var_ptr);
            });
        }

        *self.inner.value.borrow_mut() = Some(rc_value.into());

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        *self.inner.depth.borrow_mut() -= 1;
    }

    /// Unbind the variable (set to None), allowing re-entrant calls from the same thread.
    pub fn unbind(&self) {
        let var_ptr = self.inner.as_ref() as *const VarInner as usize;
        
        let was_first_entry = {
            let mut depth = self.inner.depth.borrow_mut();
            let was_first = *depth == 0;
            *depth += 1;
            was_first
        };

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().insert(var_ptr);
            });
        }

        *self.inner.value.borrow_mut() = None;

        if was_first_entry {
            ACQUIRING_VARS.with(|vars| {
                vars.borrow_mut().remove(&var_ptr);
            });
        }

        *self.inner.depth.borrow_mut() -= 1;
    }

    pub fn into_value(var: RcVar) -> Value {
        Value::Var(var, meta::new_empty_rc())
    }

    pub fn into_value_rc(var: RcVar) -> RcValue {
        Rc::new(Value::Var(var, meta::new_empty_rc()))
    }
}

impl From<Value> for Var {
    fn from(value: Value) -> Self {
        Var::new_bound(value)
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Var {}

impl PartialOrd for Var {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_ptr = self.inner.as_ref() as *const VarInner as usize;
        let other_ptr = other.inner.as_ref() as *const VarInner as usize;
        self_ptr.partial_cmp(&other_ptr)
    }
}

impl Ord for Var {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_ptr = self.inner.as_ref() as *const VarInner as usize;
        let other_ptr = other.inner.as_ref() as *const VarInner as usize;
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
        let ptr = self.inner.as_ref() as *const VarInner as usize;
        ptr.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_unbound() {
        let var = Var::new_unbound();
        assert!(var.is_unbound());
        assert!(!var.is_bound());
        assert_eq!(var.deref(), None);
    }

    #[test]
    fn test_new_bound() {
        let val = Rc::new(Value::from(42.0));
        let var = Var::new_bound(val.clone());
        assert!(!var.is_unbound());
        assert!(var.is_bound());
        assert_eq!(var.deref(), Some(val));
    }

    #[test]
    fn test_bind_unbind() {
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
    fn test_reentrant_access_same_thread() {
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
    fn test_reentrant_bind_during_deref() {
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
    fn test_recursive_function_scenario() {
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
    fn test_multiple_derefs() {
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
}