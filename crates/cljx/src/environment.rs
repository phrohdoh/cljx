use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::prelude::*;

pub type RcEnvironment = Rc<Environment>;
pub type Namespaces = HashMap<SymbolUnqualified, RcNamespace>;

#[derive(Debug)]
pub struct Environment {
    namespaces: RefCell<Namespaces>,
}

// constructors
impl Environment {
    pub fn new_empty() -> Self {
        Self {
            namespaces: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_empty_rc() -> Rc<Self> {
        Rc::new(Self::new_empty())
    }

    pub fn new_from_namespaced_values<'a, 'b, I>(
        namespaced_values_iter: I,
    ) -> Self
    where
        I: IntoIterator<Item = (&'a str, Vec<(&'b str, Value)>)>
    {
        namespaced_values_iter.into_iter()
            .fold(
                Self::new_empty(),
                |env, (ns_name, named_values)| {
                    let ns = Namespace::new_from_named_values(
                        ns_name,
                        named_values,
                    );
                    env.namespaces.borrow_mut()
                        .insert(SymbolUnqualified::new(ns_name), Rc::new(ns));
                    env
                },
            )
    }
}

// reads
impl Environment {
    // pub fn namespace_names(&self) -> Vec<String> {
    //     self.namespaces.borrow()
    //         .keys()
    //         .map(|sym_u| sym_u.name().to_owned())
    //         .collect()
    // }

    // pub fn namespaces(&self) -> Vec<RcNamespace> {
    //     self.namespaces.borrow()
    //         .values()
    //         .map(|ns| ns.to_owned())
    //         .collect()
    // }

    // pub fn namespace_handles(&self) -> Vec<Handle> {
    //     self.namespaces.borrow()
    //         .values()
    //         .map(|ns| Handle::new(ns.to_owned()))
    //         .collect()
    // }

    // pub fn namespaces_entry_values(&self) -> Vec<(RcValue, RcValue)> {
    //     self.namespaces.borrow()
    //         .iter()
    //         .map(|(sym_u, ns)| (
    //             Rc::new(Value::symbol(Symbol::Unqualified(sym_u.to_owned()))),
    //             Rc::new(Value::handle(Handle::new(ns.to_owned()))),
    //         ))
    //         .collect()
    // }

    // pub fn namespace_map(&self) -> Map {
    //     let entries = self.namespaces_entry_values();
    //     Map::new(entries)
    // }

    // pub fn namespace_map_value(&self) -> RcValue {
    //     Rc::new(Value::map(self.namespace_map()))
    // }

    #[tracing::instrument(ret, level = "info")]
    pub fn all_namespaces(&self) -> Vec<RcNamespace> {
        self.namespaces.borrow()
            .values()
            .cloned()
            .collect()
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn has_namespace(
        &self,
        name: &str,
    ) -> bool {
        self.try_get_namespace(name)
            .is_some()
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace(
        &self,
        name: &str,
    ) -> Option<RcNamespace> {
        self.namespaces.borrow()
            .get(&SymbolUnqualified::new(name))
            .cloned()
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace_handle(
        &self,
        name: &str,
    ) -> Option<Handle> {
        self.try_get_namespace(name)
            .map(|ns| Handle::new(ns))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn try_get_namespace_handle_value(
        &self,
        name: &str,
    ) -> Option<RcValue> {
        self.try_get_namespace_handle(name)
            .map(|handle| Rc::new(Value::handle(handle)))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_or_panic(
        &self,
        name: &str,
    ) -> RcNamespace {
        self.try_get_namespace(name)
            .expect(&format!("no such namespace: {}", name))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_handle_or_panic(
        &self,
        name: &str,
    ) -> Handle {
        self.try_get_namespace_handle(name)
            .expect(&format!("no such namespace: {}", name))
    }

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn get_namespace_handle_value_or_panic(
        &self,
        name: &str,
    ) -> RcValue {
        self.try_get_namespace_handle_value(name)
            .expect(&format!("no such namespace: {}", name))
    }
}

// writes
impl Environment {
    /*
    #[tracing::instrument(ret, level = "info")]
    pub fn bind_value_rc<'env>(
        &'env self,
        (ns_name, name): (&str, &str),
        value_rc: RcValue,
    ) {
        let ns = match self.try_get_namespace(&ns_name) {
            Some(ns) => ns,
            None => {
                self.create_namespace(ns_name);
                self.get_namespace_or_panic(&ns_name)
            },
        };
        match ns.try_get_var(name) {
            Some(var) => { var.as_ref().bind(value_rc); },
            None => { ns.insert_var(name, Var::new_bound(value_rc)); },
        }
    }

    #[tracing::instrument(ret, level = "info")]
    pub fn bind_value<'env>(
        &'env self,
        fqn: (&str, &str),
        value: Value,
    ) {
        let value_rc = Rc::new(value);
        self.bind_value_rc(fqn, value_rc);
    }

    #[tracing::instrument(ret, level = "info", skip(arities))]
    pub fn bind_function<'env>(
        &'env self,
        (ns_name, name): (&str, &str),
        arities: Vec<(
            FunctionArity,
            Box<dyn Fn(RcEnvironment, Vec<RcValue>) -> RcValue>,
        )>,
    ) -> RcValue {
        let value = Value::function({
            let mut builder = Function::builder();
            builder.set_name(name.to_owned());
            for (arity, body) in arities {
                builder.add_body(arity, body);
            }
            Rc::new(builder.build())
        });
        let rc_value = Rc::new(value);
        self.bind_value_rc(
            (ns_name, name),
            rc_value.clone(),
        );
        rc_value
    }
    */

    #[tracing::instrument(ret, fields(name), level = "info")]
    pub fn create_namespace(
        &self,
        name: &str,
    ) -> RcNamespace {
        match self.try_get_namespace(name) {
            Some(ns) => ns,
            None => {
                self.namespaces.borrow_mut()
                    .insert(
                        SymbolUnqualified::new(name),
                        Rc::new(Namespace::new_empty(name)),
                    );
                self.get_namespace_or_panic(name)
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_nonexistent_namespace() {
        // arrange
        let env = Environment::new_empty_rc();
        let ns_name = "my-ns";
        let has_ns_before_act = env.has_namespace(&ns_name);
        // act
        env.create_namespace(ns_name);
        // assert
        let has_ns_after_act = env.has_namespace(&ns_name);
        assert!(!has_ns_before_act);
        assert!(has_ns_after_act);
    }

    #[test]
    fn create_existent_namespace_does_not_overwrite() {
        // arrange
        let env = Environment::new_empty_rc();
        let ns_name = "my-ns";
        let name = "my-var";
        let value = Value::integer(42);
        let var = Var::new_bound(value);
        env.create_namespace(ns_name)
           .insert_var(name, var);
        // act
        let ns = env.create_namespace(ns_name);
        eprintln!("{:?}", ns);
        // assert
        let var = ns.try_get_var(name);
        assert!(var.is_ok(), "var expected to be present");
        let var = var.unwrap();
        let val = var.deref();
        assert!(val.is_some(), "var expected to be bound");
        let val = val.unwrap();
        assert_eq!(*val, Value::integer(42), "value of var is unchanged");
    }

    #[test]
    fn get_var() {
        // arrange
        let ns_name = "my-ns";
        let name = "my-var";
        let value = Value::keyword_unqualified("my-val");
        let env = Environment::new_from_namespaced_values(vec![
            (ns_name, vec![
                (name,
                 value),
            ]),
        ]);
        // act
        let ns = env.get_namespace_or_panic("my-ns");
        let var = ns.try_get_var("my-var");
        // assert
        assert!(var.is_ok());
    }
}
