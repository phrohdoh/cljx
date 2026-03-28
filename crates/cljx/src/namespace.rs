use ::std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::prelude::*;


pub type RcNamespace = Rc<Namespace>;
pub type NamedVars = HashMap<SymbolUnqualified, RcVar>;
pub type Aliases = HashMap<SymbolUnqualified, RcNamespace>;
pub type Imports = HashMap<SymbolUnqualified, String>;
pub type Refers = HashMap<SymbolUnqualified, RcVar>;

#[derive(Debug)]
pub struct Namespace {
    name: SymbolUnqualified,
    mappings: RefCell<NamedVars>,
    aliases: RefCell<Aliases>,
    imports: RefCell<Imports>,
    refers: RefCell<Refers>,
}

// constructors
impl Namespace {
    pub fn new_empty(
        name: &str,
    ) -> Self {
        Self {
            name: SymbolUnqualified::new(name),
            mappings: RefCell::new(NamedVars::new()),
            aliases: RefCell::new(Aliases::new()),
            imports: RefCell::new(Imports::new()),
            refers: RefCell::new(Refers::new()),
        }
    }

    pub fn new_empty_rc(
        name: &str,
    ) -> Rc<Self> {
        Rc::new(Self {
            name: SymbolUnqualified::new(name),
            mappings: RefCell::new(NamedVars::new()),
            aliases: RefCell::new(Aliases::new()),
            imports: RefCell::new(Imports::new()),
            refers: RefCell::new(Refers::new()),
        })
    }

    pub fn new_from_named_values<'ns_name, 'name, I>(
        name: &'ns_name str,
        values: I,
    ) -> Self
    where
        I: IntoIterator<Item = (&'name str, Value)>,
    {
        Self {
            name: SymbolUnqualified::new(name),
            mappings: RefCell::new(
                values.into_iter()
                    .map(|(n, v)| (SymbolUnqualified::new(n), Rc::new(Var::new_bound(v))))
                    .collect()
            ),
            aliases: RefCell::new(Aliases::new()),
            imports: RefCell::new(Imports::new()),
            refers: RefCell::new(Refers::new()),
        }
    }

    pub fn new_from_named_vars<'ns_name, 'name, I>(
        name: &'ns_name str,
        vars: I,
    ) -> Self
    where
        I: IntoIterator<Item = (&'name str, Var)>,
    {
        Self {
            name: SymbolUnqualified::new(name),
            mappings: RefCell::new(
                vars.into_iter()
                    .map(|(n, v)| (SymbolUnqualified::new(n), Rc::new(v)))
                    .collect()
            ),
            aliases: RefCell::new(Aliases::new()),
            imports: RefCell::new(Imports::new()),
            refers: RefCell::new(Refers::new()),
        }
    }

    pub fn new_from_named_vars_rc<'ns_name, 'name, I>(
        name: &'ns_name str,
        vars: I,
    ) -> Self
    where
        I: IntoIterator<Item = (&'name str, RcVar)>,
    {
        Self {
            name: SymbolUnqualified::new(name),
            mappings: RefCell::new(
                vars.into_iter()
                    .map(|(n, v)| (SymbolUnqualified::new(n), v))
                    .collect()
            ),
            aliases: RefCell::new(Aliases::new()),
            imports: RefCell::new(Imports::new()),
            refers: RefCell::new(Refers::new()),
        }
    }
}


// read errors
#[derive(Debug)]
pub enum GetVarError {
    NoSuchVar(SymbolQualified),
}

#[derive(Debug)]
pub enum GetValueError {
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
}

#[derive(Debug)]
pub enum GetFunctionError {
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    ValueIsNotFunction(SymbolQualified),
}

#[derive(Debug)]
pub enum GetHandleError {
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    IncorrectValueType(SymbolQualified),
    IncorrectHandleType(SymbolQualified),
}

// read error conversions
impl From<GetVarError> for GetValueError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetVarError> for GetFunctionError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetValueError> for GetFunctionError {
    fn from(get_value_err: GetValueError) -> Self {
        match get_value_err {
            GetValueError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
            GetValueError::UnboundVar(var_sym) => Self::UnboundVar(var_sym),
        }
    }
}

impl From<GetVarError> for GetHandleError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetValueError> for GetHandleError {
    fn from(get_value_err: GetValueError) -> Self {
        match get_value_err {
            GetValueError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
            GetValueError::UnboundVar(var_sym) => Self::UnboundVar(var_sym),
        }
    }
}

// reads
impl Namespace {
    pub fn name_str(&self) -> &str {
        self.name.name()
    }

    pub fn names(&self) -> Vec<String> {
        self.mappings.borrow()
            .keys()
            .map(|sym| sym.name().to_owned())
            .collect()
    }

    pub fn vars(&self) -> Vec<RcVar> {
        self.mappings.borrow()
            .values()
            .cloned()
            .collect()
    }

    pub fn entries(&self) -> Vec<(String, RcVar)> {
        self.mappings.borrow()
            .iter()
            .map(|(sym, var)| (sym.name().to_owned(), var.clone()))
            .collect()
    }

    pub fn aliases(&self) -> Aliases {
        self.aliases.borrow().clone()
    }

    pub fn imports(&self) -> Imports {
        self.imports.borrow().clone()
    }

    pub fn refers(&self) -> Refers {
        self.refers.borrow().clone()
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn contains_var(
        &self,
        name: &str,
    ) -> bool {
        self.mappings.borrow()
            .contains_key(&SymbolUnqualified::new(name))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_var(
        &self,
        name: &str,
    ) -> Result<RcVar, GetVarError> {
        self.mappings.borrow()
            .get(&SymbolUnqualified::new(name))
            .cloned()
            .ok_or(GetVarError::NoSuchVar(SymbolQualified::new(
                self.name_str(),
                name,
            )))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_value(
        &self,
        name: &str,
    ) -> Result<RcValue, GetValueError> {
        self.try_get_var(name)?
            .deref()
            .ok_or(GetValueError::UnboundVar(SymbolQualified::new(
                self.name_str(),
                name,
            )))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_var_or_panic(
        &self,
        name: &str,
    ) -> RcVar {
        let var_sym = SymbolQualified::new(self.name_str(), name);
        match self.try_get_var(name) {
            Ok(var) => var,
            Err(err) => panic!("error getting var #'{}: {:?}", var_sym, err),
        }
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_value_or_panic(
        &self,
        name: &str,
    ) -> RcValue {
        let var_sym = SymbolQualified::new(self.name_str(), name);
        self.get_var_or_panic(name)
            .deref()
            .expect(&format!("deref of unbound var: #'{}", var_sym))
    }

    pub fn try_get_handle<T: IHandle + Clone>(
        &self,
        name: &str,
    ) -> Result<T, GetHandleError> {
        let var_sym = SymbolQualified::new(self.name_str(), name);
        let value = self.try_get_value(name)?;
        match value.try_get_handle() {
            Ok(t) => Ok(t),
            Err(value::GetHandleError::IncorrectHandleType) => Err(GetHandleError::IncorrectHandleType(var_sym)),
            Err(value::GetHandleError::IncorrectValueType) => Err(GetHandleError::IncorrectValueType(var_sym)),
        }
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_function(
        &self,
        name: &str,
    ) -> Result<RcFunction, GetFunctionError> {
        let var_sym = SymbolQualified::new(self.name_str(), name);
        match self.try_get_value(name)?.as_ref() {
            Value::Function(func_rc, _) => Ok(func_rc.clone()),
            Value::Handle(handle, _) => {
                if let Some(func_ref) = handle.downcast_ref::<Function>() {
                    Ok(Rc::new(func_ref.clone()))
                } else {
                    Err(GetFunctionError::ValueIsNotFunction(var_sym))
                }
            },
            _ => Err(GetFunctionError::ValueIsNotFunction(var_sym)),
        }
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_function_or_panic(
        &self,
        name: &str,
    ) -> RcFunction {
        let var_sym = SymbolQualified::new(self.name_str(), name);
        self.try_get_function(name)
            .expect(&format!("unable to get #'{} as function", var_sym))
    }

    #[tracing::instrument(ret, fields(ns_name, name), level = "info")]
    pub fn get_function_value(
        &self,
        name: &str,
    ) -> Value {
        Value::function(self.get_function_or_panic(name))
    }

    #[tracing::instrument(ret, fields(ns_name, name), level = "info")]
    pub fn get_function_value_rc(
        &self,
        name: &str,
    ) -> RcValue {
        Rc::new(self.get_function_value(name))
    }
}

// writes
impl Namespace {
    pub fn insert_var(
        &self,
        name: &str,
        var: impl Into<RcVar>,
    ) -> &Self {
        self.mappings.borrow_mut()
            .insert(SymbolUnqualified::new(name), var.into());
        self
    }

    pub fn insert_function(
        &self,
        name: &str,
        function: Function,
    ) -> &Self {
        self.insert_var(name, Var::new_bound(Value::function_rc(Rc::new(function))));
        self
    }

    pub fn insert_vars<'a, I, V>(
        &self,
        vars: I,
    ) -> &Self
    where
        I: IntoIterator<Item = (&'a str, V)>,
        V: Into<RcVar>,
    {
        vars.into_iter()
            .for_each(move |(name, var)| {
                self.insert_var(name, var.into());
            });
        self
    }

    pub fn remove_var(
        &self,
        name: &str,
    ) {
        self.mappings.borrow_mut()
            .remove(&SymbolUnqualified::new(name));
    }

    pub fn remove_vars<'this, 'a, I>(
        &'this self,
        names_iter: I,
    )
    where
        I: IntoIterator<Item = &'a str>,
    {
        for name in names_iter {
            self.mappings.borrow_mut()
                .remove(&SymbolUnqualified::new(name));
        }
    }

    pub fn add_alias(&self, alias: &str, ns: RcNamespace) {
        self.aliases.borrow_mut().insert(SymbolUnqualified::new(alias), ns);
    }

    pub fn remove_alias(&self, alias: &str) {
        self.aliases.borrow_mut().remove(&SymbolUnqualified::new(alias));
    }

    pub fn add_import(&self, simple: &str, fqn: String) {
        self.imports.borrow_mut().insert(SymbolUnqualified::new(simple), fqn);
    }

    pub fn add_refer(&self, name: &str, var: RcVar) {
        self.refers.borrow_mut().insert(SymbolUnqualified::new(name), var);
    }
}

impl Namespace {
    pub fn bind_value(
        &self,
        name: &str,
        value: Value,
    ) -> &Self {
        match self.try_get_var(name) {
            Ok(var) => {
                var.bind(value);
            },
            Err(_) => {
                self.insert_var(name, Var::new_bound(value));
            },
        }

        self
    }

    pub fn bind_value_rc(
        &self,
        name: &str,
        value: RcValue,
    ) -> &Self {
        match self.try_get_var(name) {
            Ok(var) => {
                var.bind(value);
            },
            Err(_) => {
                self.insert_var(name, Var::new_bound(value));
            },
        }

        self
    }

    pub fn bind_handle(
        &self,
        name: &str,
        handle: Handle,
    ) -> &Self {
        self.bind_value(name, Value::handle(handle));
        self
    }

    pub fn bind_function(
        &self,
        name: &str,
        function: Function,
    ) -> &Self {
        self.bind_value_rc(name, Rc::new(Value::function(Rc::new(function))));
        self
    }

    pub fn bind_macro(
        &self,
        name: &str,
        function: Function,
    ) -> &Self {
        self.insert_var(
            name,
            Var::new_bound_with_meta(
                Value::function(Rc::new(function)),
                Some(Rc::new(Map::new(vec![(
                    Value::keyword_unqualified_rc("macro"),
                    Value::boolean_rc(true),
                )]))),
            ));
        self
    }

    /// Build and bind a function with custom `IFunction` implementations.
    ///
    /// This is the primary way to bind multi-arity functions to a namespace.
    /// It accepts a collection of `(FunctionArity, RcDynIFunction)` pairs,
    /// allowing you to bind custom `IFunction` implementations, closures,
    /// or any combination thereof.
    ///
    /// ## Example with closures
    /// ```
    /// # use cljx::prelude::*;
    /// # let mut env_builder = Environment::builder();
    /// # env_builder.set_current_namespace_var("clojure.core", "*ns*");
    /// # let env = env_builder.build();
    /// # let ns = env.create_namespace("clojure.core");
    /// ns.build_and_bind_function(
    ///     "+",
    ///     vec![closure_fn(FunctionArity::AtLeast(0), |_env, args| {
    ///         let mut sum = 0i64;
    ///         for arg in args {
    ///             // ... accumulate sum
    ///         }
    ///         Value::integer_rc(sum)
    ///     })],
    /// );
    /// ```
    ///
    /// ## Example with custom struct
    /// ```
    /// # use ::std::cell::RefCell;
    /// # use ::std::rc::Rc;
    /// # use cljx::prelude::*;
    /// struct Counter { state: RefCell<i32> }
    /// impl IFunction for Counter {
    ///     fn invoke(&self, env: RcEnvironment, args: Vec<RcValue>) -> RcValue {
    ///         todo!()
    ///     }
    /// }
    ///
    /// # let mut env_builder = Environment::builder();
    /// # env_builder.set_current_namespace_var("clojure.core", "*ns*");
    /// # let env = env_builder.build();
    /// # let ns = env.create_namespace("clojure.core");
    /// ns.build_and_bind_function(
    ///     "counter",
    ///     vec![(
    ///         FunctionArity::AtLeast(0),
    ///         Rc::new(Counter { state: RefCell::new(0) }),
    ///     )],
    /// );
    /// ```
    pub fn build_and_bind_function(
        &self,
        name: &str,
        arities: Vec<(FunctionArity, RcDynIFunction)>,
    ) -> &Self {
        self.bind_function(name, build_function(name, arities))
    }

    pub fn build_and_bind_macro(
        &self,
        name: &str,
        arities: Vec<(FunctionArity, RcDynIFunction)>,
    ) -> &Self {
        self.bind_macro(name, build_function(name, arities))
    }
}

impl IHandle for Namespace {}
impl IHandle for Rc<Namespace> {}

// conversions into handle
impl Namespace {
    pub fn into_handle(self: Rc<Self>) -> Handle {
        Handle::new(self)
    }

    pub fn into_handle_value(self: Rc<Self>) -> Value {
        Value::handle(self.into_handle())
    }

    pub fn into_handle_value_rc(self: Rc<Self>) -> RcValue {
        Rc::new(Value::handle(self.into_handle()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_empty() {
        let ns_name = "my-namespace";
        let ns = Namespace::new_empty(ns_name);
        assert_eq!(ns.name, SymbolUnqualified::new(ns_name));
        assert!(ns.mappings.borrow().is_empty());
    }

    #[test]
    fn new_from() {
        let ns_name = "my-namespace";
        let var_1_name = "my-var-1";
        let var_2_name = "my-var-2";
        let ns = Namespace::new_from_named_vars(
            ns_name,
            [(
                var_1_name,
                Var::new_bound(Value::nil()),
            ), (
                var_2_name,
                Var::new_bound(Value::boolean(true)),
            )],
        );
        assert!(ns.contains_var(&var_1_name));
        assert!(ns.contains_var(&var_2_name));
        assert_eq!(ns.try_get_var(&var_1_name).unwrap().deref().unwrap(), Value::nil().into_value_rc());
        assert_eq!(ns.try_get_var(&var_2_name).unwrap().deref().unwrap(), Value::boolean(true).into_value_rc());
    }

    #[test]
    fn insert_var() {
        let ns_name = "my-namespace";
        let ns = Namespace::new_empty(ns_name);
        let var_name = "my-var";
        let var = Rc::new(Var::new_bound(Value::nil()));
        ns.insert_var(var_name, var.clone());
        assert!(ns.contains_var(var_name));
        let got_var = ns.try_get_var(var_name).expect("var must be present");
        assert_eq!(*got_var, *var);
    }

    #[test]
    fn insert_vars() {
        let ns_name = "my-namespace";
        let ns = Namespace::new_empty(ns_name);
        let var_1_name = "my-var-1";
        let var_2_name = "my-var-2";
        ns.insert_vars([(
            var_1_name,
            Var::new_bound(Value::nil()),
        ), (
            var_2_name,
            Var::new_bound(Value::boolean(true)),
        )]);
        assert!(ns.contains_var(&var_1_name));
        assert!(ns.contains_var(&var_2_name));
        assert_eq!(ns.try_get_var(&var_1_name).unwrap().deref().unwrap(), Value::nil().into_value_rc());
        assert_eq!(ns.try_get_var(&var_2_name).unwrap().deref().unwrap(), Value::boolean(true).into_value_rc());
    }

    #[test]
    fn remove_var() {
        let ns_name = "my-namespace";
        let ns = Namespace::new_empty(ns_name);
        let var_name = "my-var";
        let var = Var::new_bound(Value::nil());
        ns.insert_var(var_name, var.clone());
        assert!(ns.contains_var(&var_name));
        ns.remove_var(&var_name);
        assert!(!ns.contains_var(&var_name));
    }

    #[test]
    fn remove_vars() {
        let ns_name = "my-namespace";
        let ns = Namespace::new_empty(ns_name);
        let var_1_name = "my-var-1";
        let var_2_name = "my-var-2";
        let var_1 = Var::new_bound(Value::nil());
        let var_2 = Var::new_bound(Value::boolean(true));
        ns.insert_var(var_1_name, var_1);
        ns.insert_var(var_2_name, var_2);
        assert!(ns.contains_var(var_1_name));
        assert!(ns.contains_var(var_2_name));
        ns.remove_vars([var_1_name, var_2_name].into_iter());
        // let removed_vars: HashMap<_, _> = .collect();
        // assert_eq!(removed_vars.get(&var_1_name).unwrap().as_ref().unwrap(), &var_1);
        // assert_eq!(removed_vars.get(&var_2_name).unwrap().as_ref().unwrap(), &var_2);
        assert!(!ns.contains_var(&var_1_name));
        assert!(!ns.contains_var(&var_2_name));
    }
}
