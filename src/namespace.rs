
use ::std::{
    rc::Rc,
    collections::HashMap,
};

use crate::{
    UnqualifiedSymbol,
    RcValue,
    Var,
    AFn,
};


/// A map of name ([UnqualifiedSymbol]) to [Var].
///
/// [UnqualifiedSymbol]: crate::UnqualifiedSymbol
/// [Var]: crate::Var
#[derive(Clone)]
pub struct Namespace {
    name: UnqualifiedSymbol,
    interned_vars: HashMap<UnqualifiedSymbol, Rc<Var>>,
    referred_vars: HashMap<UnqualifiedSymbol, Rc<Var>>,
    ns_aliases: HashMap<UnqualifiedSymbol, UnqualifiedSymbol>,
}

impl Namespace {
    pub fn new_empty(name: UnqualifiedSymbol) -> Self {
        Self {
            name,
            interned_vars: HashMap::new(),
            referred_vars: HashMap::new(),
            ns_aliases: HashMap::new(),
        }
    }
}

impl Namespace {
    pub fn name(&self) -> &UnqualifiedSymbol {
        &self.name
    }

    pub fn interns(&self) -> Vec<(&UnqualifiedSymbol, Rc<Var>)> {
        self.interned_vars.iter()
            .map(|(k, v)| (k, Rc::clone(v)))
            .collect()
    }

    pub fn declare(&mut self, name: UnqualifiedSymbol) -> &mut Self {
        self.interned_vars.insert(name, Var::new_unbound().into());
        self
    }

    pub fn intern(&mut self, name: UnqualifiedSymbol, rc_value: RcValue) -> &mut Self {
        match self.interned_vars.get(&name) {
            Some(decl) => { decl.bind(rc_value); },
            None       => { self.interned_vars.insert(name, Var::new_bound(rc_value).into()); }
        }
        self
    }

    pub fn get_interned_var(&self, name: &UnqualifiedSymbol) -> Option<Rc<Var>> {
        self.interned_vars.get(name).cloned()
    }

    pub fn get_referred_var(&self, name: &UnqualifiedSymbol) -> Option<Rc<Var>> {
        self.referred_vars.get(name).cloned()
    }

    pub fn get_interned_or_referred_var(&self, name: &UnqualifiedSymbol) -> Option<Rc<Var>> {
        self.get_interned_var(name)
            .or_else(|| self.get_referred_var(name))
    }
}
