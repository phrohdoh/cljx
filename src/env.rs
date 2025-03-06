
use core::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;
use std::collections::HashMap;
use crate::{UnqualifiedSymbol, QualifiedSymbol, new_unqualified_symbol};
use crate::Var;
use crate::Namespace;
use crate::rt::Resolve;

type MutablePlace = RefCell<Namespace>;
type NsName = UnqualifiedSymbol;
type NsPlace = Rc<MutablePlace>;


type NsMapKey = NsName;
type NsMapValue = NsPlace;
type NsMapEntry = (NsMapKey, NsMapValue);
type NsMap = HashMap<NsMapKey, NsMapValue>;


/// A map of name ([UnqualifiedSymbol]) to [Namespace], or of qualified name ([QualifiedSymbol]) to [Var].
#[derive(Clone)]
pub struct Env(NsMap);

impl Env {
    pub fn new_empty() -> Self {
        Self(Default::default())
    }

    pub fn borrow(
        &mut self,
    ) -> &Self {
        self
    }

    pub fn namespaces(
        &self,
    ) -> impl Iterator<Item = NsMapEntry> {
        self.0.iter()
            .map(|(ns_name, ns)| (ns_name.clone(), ns.clone()))
    }

    pub fn remove_namespace<'e, 'ns>(
        &mut self,
        ns_name: &'ns UnqualifiedSymbol,
    ) -> Option<NsMapEntry> {
        self.0.remove_entry(ns_name)
    }

    pub fn find_or_create_namespace(
        &mut self,
        ns_name: UnqualifiedSymbol,
    ) -> NsPlace {
        self.0.entry(ns_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Namespace::new_empty(ns_name.clone()))))
            ;
        Rc::clone(&self.0[&ns_name])
    }

    pub fn find_or_create_namespace_mut(
        &mut self,
        ns_name: UnqualifiedSymbol,
    ) -> RefMut<'_, Namespace> {
        self.0.entry(ns_name.clone())
            .or_insert_with(|| Rc::new(RefCell::new(Namespace::new_empty(ns_name.clone()))))
            ;
        self.0[&ns_name].borrow_mut()
    }

    pub fn get_namespace<'e, 'ns>(
        &'e self,
        ns_name: &'ns UnqualifiedSymbol,
    ) -> Option<NsPlace> {
        self.0.get(ns_name)
            .cloned()
    }

    pub fn get_namespace_ref<'e, 'ns>(
        &'e self,
        ns_name: &'ns UnqualifiedSymbol,
    ) -> Option<Ref<'e, Namespace>> {
        self.0.get(ns_name)
            .map(|x| x.borrow())
    }

    pub fn get_namespace_mut<'e, 'ns>(
        &'e mut self,
        ns_name: &'ns UnqualifiedSymbol,
    ) -> Option<RefMut<'e, Namespace>> {
        self.0.get(ns_name)
            .map(|x| x.borrow_mut())
    }
}


impl<'env, 'k> Resolve<'env, &'k crate::QualifiedSymbol> for Env {
    type V = Option<Rc<Var>>;
    fn resolve(&'env self, k: &'k crate::QualifiedSymbol) -> Self::V {
        self.get_namespace_ref(&new_unqualified_symbol(k.namespace()))
            .and_then(|ns| ns.get_interned_or_referred_var(&new_unqualified_symbol(k.name())))
    }
}

impl<'env, 'ns, 'n> Resolve<'env, (&'ns str, &'n str)> for Env {
    type V = Option<Rc<Var>>;
    fn resolve(&'env self, (ns, n): (&'ns str, &'n str)) -> Self::V {
        self.get_namespace_ref(&new_unqualified_symbol(ns))
            .and_then(|ns| ns.get_interned_or_referred_var(&new_unqualified_symbol(n)))
    }
}

// TODO: as part of EnvExts
//impl<'env, 'ns, 'n> Resolve<'env, (&'ns UnqualifiedSymbol, &'n str)> for Env {
//    type V = Option<Rc<Var>>;
//    fn resolve(&'env self, (ns, n): (&'ns UnqualifiedSymbol, &'n str)) -> Self::V {
//        use crate::NamespaceExts;
//        self.get_namespace_ref(ns)
//            .and_then(|ns| ns.get_interned_or_referred_var(&new_unqualified_symbol(n)))
//    }
//}


impl<'env, 'ns, 'n> crate::Intern<'env, crate::QualifiedSymbol> for crate::Env {
    type V = crate::RcValue;
    type O = &'env mut Self;
    fn intern(&'env mut self, k: crate::QualifiedSymbol, v: Self::V) -> Self::O {
        // use crate::EnvExts;
        self.find_or_create_namespace_mut(new_unqualified_symbol(k.namespace()))
            .intern(new_unqualified_symbol(k.name()), v)
            ;
        self
    }
}

impl<'env, 'ns, 'n> crate::Intern<'env, (&'ns str, &'n str)> for crate::Env {
    type V = crate::RcValue;
    type O = &'env mut Self;
    fn intern(&'env mut self, (ns, n): (&'ns str, &'n str), v: Self::V) -> Self::O {
        // use crate::EnvExts;
        self.find_or_create_namespace_mut(new_unqualified_symbol(ns))
            .intern(new_unqualified_symbol(n), v)
            ;
        self
    }
}
