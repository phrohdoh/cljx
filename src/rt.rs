
//use crate::deps::tracing::{self};
//use crate::deps::as_any::AsAny;
//use crate::symbol::Symbol;
//use crate::value::{Value, IntoValue};
//use crate::list::{List, IntoList};
//use crate::vector::{Vector, PersistentVector};
//use crate::set::{Set, PersistentSet};
//use crate::map::{Map, PersistentMap};
//use crate::rt::env::Environment;

use crate::deps::tracing;
use crate::deps::as_any::AsAny;
use crate::{UnqualifiedSymbol, Value};
use crate::{List, IntoList};
use crate::{Vector, PersistentVector};
use crate::{Set, PersistentSet};
use crate::{Map, PersistentMap};
use crate::Symbol;
use self::env::Env;
use self::namespace::Namespace;

use ::std::rc::Rc;
use ::std::cell::RefCell;
pub type RcValue   = crate::deps::archery::SharedPointer<Value, crate::deps::archery::RcK>;
//pub type ArcValue  = crate::deps::archery::SharedPointer<Value, crate::deps::archery::ArcK>;
//pub type ArcTValue = crate::deps::archery::SharedPointer<Value, crate::deps::archery::ArcTK>;


#[path = "env.rs"]
pub mod env;

#[path = "namespace.rs"]
pub mod namespace;


pub trait Resolve<'this, K> {
    type V;
    fn resolve(&'this self, k: K) -> Self::V;
}

pub trait Intern<'this, K> {
    type V;
    type O;
    fn intern(&'this mut self, k: K, v: Self::V) -> Self::O;
}


pub fn is_comment(value: &Value) -> bool {
    value.is_list_and(
        |list| list.first().is_some_and(
            |head| head.is_symbol_unqualified_and(
                |head_sym| head_sym.name() == "comment"
            )
        )
    )
}

pub fn is_quote(value: &Value) -> bool {
    value.is_list_and(
        |list| list.first().is_some_and(
            |head| head.is_symbol_unqualified_and(
                |head_sym| head_sym.name() == "quote"
            )
        )
    )
}

pub fn eval(
    env: &mut Env,
    value: RcValue,
) -> RcValue {
    // eval2(env, value).unwrap()

    match eval2(env, value.clone()) {
        Ok(rc_value) => rc_value,
        Err(EvalError::NoSuchVar(var_sym)) => {
            crate::deps::tracing::error!("no such var: #'{}, using nil", var_sym);
            Value::Nil.into()
        },
        Err(EvalError::UnboundVar(var_sym)) => {
            crate::deps::tracing::error!("unbound var: #'{}, using nil", var_sym);
            Value::Nil.into()
        },
        Err(EvalError::InvalidArity(name, arity, arities)) => {
            crate::deps::tracing::error!("Wrong number of args passed to {name}: received {arity}, expected one of {arities}");
            Value::Nil.into()
        },
        //Err(error) => {
        //    crate::deps::tracing::error!(%value, ?error);
        //    Value::Nil.into()
        //},
    }
}

#[derive(Debug)]
pub enum EvalError {
    NoSuchVar(Symbol),
    UnboundVar(Symbol),
    InvalidArity(String, u32, crate::Set),
}

// #[tracing::instrument(name = "cljx::eval2", skip(env, value))]
pub fn eval2(
    env: &mut Env,
    value: RcValue,
) -> Result<RcValue, EvalError> {
    Ok(match value.as_ref() {
        // unchanged
        Value::Nil          => Value::Nil.into(),
        Value::Boolean (x)  => Value::Boolean (*x).into(),
        Value::Integer (x)  => Value::Integer (*x).into(),
        Value::Float   (x)  => Value::Float   (*x).into(),
        Value::String  (x)  => Value::String  (x.clone()).into(),
        Value::Keyword (x)  => Value::Keyword (x.clone()).into(),

        // resolve
        // Value::Var (var) => var.deref().expect(&format!("unbound var: {}", value)),
        Value::Var (rc_var) => {
            match rc_var.deref() {
                Some(rc_value) => rc_value,
                None           => Value::Var(Rc::clone(rc_var)).into(),
            }
        },
        Value::Symbol (Symbol::Unqualified (symbol)) => {
            //let ns_name = current_ns_name_sym(env.borrow());
            //let ns_name = ns_name.name().to_owned();
            let ns_name = current_ns(env).borrow().name().name().to_owned();
            let name = symbol.name().to_owned();

            let err_sym = Symbol::from((
                UnqualifiedSymbol::new(ns_name.clone()),
                UnqualifiedSymbol::new(name.clone()),
            ));

            crate::Resolve::resolve(env, (ns_name.as_str(), name.as_str()))
               .ok_or_else(|| EvalError::NoSuchVar(err_sym.clone()))?
               .deref()
               .ok_or_else(|| EvalError::UnboundVar(err_sym))?
        },
        Value::Symbol (Symbol::Qualified   (symbol)) => {
            crate::Resolve::resolve(env, symbol)
               .ok_or_else(|| EvalError::NoSuchVar(crate::Symbol::Qualified(symbol.to_owned())))?
               .deref()
               .ok_or_else(|| EvalError::UnboundVar(crate::Symbol::Qualified(symbol.to_owned())))?
        },

        // recursively eval
        Value::Vector (vector) => {
            let mut persistent_vector = PersistentVector::new();
            for val in vector.iter() {
                let evaled = eval(env, val.to_owned().into());
                persistent_vector.push_back_mut(evaled.as_ref().clone());
            }
            Value::Vector(Vector::new(persistent_vector)).into()
        },
        Value::Set (set) => {
            let mut persistent_set = PersistentSet::new();
            for val in set.iter() {
                let evaled = eval(env, val.to_owned().into());
                persistent_set.insert_mut(evaled.as_ref().clone());
            }
            Value::Set(Set::new(persistent_set)).into()
        },
        Value::Map (map) => {
            use crate::map::IPersistentMap as _;
            let mut persistent_map = PersistentMap::new();
            for (key, val) in map.entries() {
                let evaled_key = eval(env, key.to_owned().into());
                let evaled_val = eval(env, val.to_owned().into());
                persistent_map.insert_mut(
                    evaled_key.as_ref().clone(),
                    evaled_val.as_ref().clone(),
                );
            }
            Value::Map(Map::new(persistent_map)).into()
        },

        // recursively eval then apply
        Value::List (list) => {
            if list.is_empty() {
                return Ok(Value::list_empty().into())
            }

            if is_comment(value.as_ref()) {
                return Ok(crate::nil!().into())
            }

            if is_quote(value.as_ref()) {
               return Ok(list.rest().first().unwrap())
            }

            let tail = list.rest().iter().map(|value| eval(env, value.to_owned())).collect::<Vec<_>>().into_list();
            let head = list.first().map(|head| eval(env, head)).unwrap();
            apply(env, head, tail)
        },
        Value::AFn(..) => value,
        Value::Handle(..) => value,
    })
}

//pub fn current_ns_name_sym(env: &Env) -> UnqualifiedSymbol {
//    let ns = self::current_ns(env);
//    let ns_name = match ns.as_ref() {
//        // Value::String(ns_name) => ns_name.to_owned(),
//        // Value::Keyword(Keyword::Unqualified(ns_name)) => ns_name.name().to_owned(),
//        // Value::Symbol(Symbol::Unqualified(ns_name)) => ns_name.name().to_owned(),
//        Value::Handle(rc_any) => {
//            match rc_any.downcast_ref::<RefCell<Namespace>>() {
//                Some(refcell_namespace) => {
//                    let ns = refcell_namespace.borrow();
//                    ns.name().to_owned()
//                },
//                None => todo!(),
//            }
//        },
//        _ => unimplemented!("non-Value::Handle in {NS_NAME}/*ns*"),
//    };
//    UnqualifiedSymbol::new(ns_name.as_str())
//}

//pub fn current_ns_name_sym(env: &Env) -> &UnqualifiedSymbol {
//    current_ns(env).as_ref().borrow().name_sym()
//}
//
//pub fn current_ns_name(env: &Env) -> &str {
//    current_ns(env).as_ref().borrow().name()
//}

pub fn current_ns<'env>(env: &'env Env) -> Rc<RefCell<Namespace>> {
    let shptr_value_ns: RcValue = crate::Resolve::resolve(env, (crate::clojure_core::NS_NAME, "*ns*")).unwrap().deref().unwrap();
    let Value::Handle(rc_any) = crate::deps::archery::SharedPointer::as_ref(&shptr_value_ns) else { unimplemented!() };
    let Ok(rc_refcell_namespace) = rc_any.clone().downcast::<RefCell<Namespace>>() else { unimplemented!() };
    rc_refcell_namespace
    // ns
}

/// Finds (creating if necessary) the named namespace and sets `clojure.core/*ns*` to a [Value::Handle] of the [Namespace].
pub fn set_current_ns<'env>(env: &'env mut Env, ns: UnqualifiedSymbol) {
    crate::Resolve::resolve(env, (crate::clojure_core::NS_NAME, "*ns*")).unwrap()
        .bind(Value::Handle(env.find_or_create_namespace(ns)))
}

#[tracing::instrument(
    skip(env, f),
    fields(f = %f, args = %args),
    level = "ERROR",
    // ret(Display),
)]
/// (apply f args)
pub fn apply(
    env: &mut Env,
    f: RcValue,
    args: List,
) -> RcValue {
    let f_val = f.as_ref();
    match f_val {
        Value::Symbol(Symbol::Unqualified(f_sym)) => {
            let current_ns = current_ns(env.borrow());
            let current_ns = current_ns.borrow();
            let resolved_f = crate::Resolve::resolve(env, (current_ns.name().name(), f_sym.name())).unwrap().deref().unwrap();
            // let resolved_f = current_ns.get_interned_or_referred_var(f_sym.name()).unwrap().deref().unwrap();
            let f_args = args.rest().first().unwrap().as_ref().into_list_panicing();
            crate::apply(env, resolved_f, f_args)
        },
        Value::Symbol(Symbol::Qualified(f_sym)) => {
            let resolved_f = crate::Resolve::resolve(env, f_sym).unwrap().deref().unwrap();
            let f_args = args.rest().first().unwrap().as_ref().into_list_panicing();
            crate::apply(env, resolved_f, f_args)
        },
        Value::Keyword(..) => {
            let map = args.first().expect("nothing to apply keyword to");
            let map = map.as_map_panicing();

            let not_found = args.rest().first();
            match not_found.as_ref() {
                Some(not_found) => map.get_or(f_val, not_found),
                None            => map.get_or_nil(f_val)
            }.to_owned().into()
        },
        Value::AFn(afn) => afn.apply(env, args),
        other => todo!("(apply {} {})", other, args.rest()),
    }
}

pub trait AFn: AsAny {
    fn name(&self) -> Option<String> { None }
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue;
}

impl<T> AFn for T where T: Fn(&mut Env, RcValue) -> RcValue + 'static {
    fn apply(
        &self,
        _env: &mut Env,
        _args: List,
    ) -> RcValue {
        todo!()
    }
}


// #[tracing::instrument("check-arity", level = "TRACE", skip(fn_ident, arities, args))]
pub fn check_arity<'fn_ident, 'arities, 'arity>(
    fn_ident: &'fn_ident str,
    arities: &'arities Set,
    args: &'arity List,
//) -> Result<(), EvalError> {
) -> Result<(), ()> {

    {
        use crate::IntoValue as _;
        tracing::info!("{}", crate::map!(
            (crate::keyword!("f"), crate::symbol!(fn_ident)),
            (crate::keyword!("args"), args.into_value()),
            (crate::keyword!("arities"), arities.into_value()),
        ));
    }

    let arity = args.len() as i64;
    let invalid_arities = arities.iter()
        .filter(|arity| !is_valid_arity(arity))
        .collect::<Vec<_>>();

    if !invalid_arities.is_empty() {
        panic!(
            "{fn_ident} arities must be an integer or a length-1 vector containing an integer, but contained {}",
            invalid_arities
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let variadic_arities = arities.iter()
        .filter(|arity| is_valid_variadic_arity(arity))
        .collect::<Vec<_>>();

    if variadic_arities.len() > 1 {
        panic!(
            "{fn_ident} arities must not contain multiple variadics, but contained {}: {}",
            variadic_arities.len(),
            variadic_arities
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(", "),
        );
    }

    {
        let direct_support = {
            use crate::set::IPersistentSet as _;
            arities.contains(&crate::integer!(arity))
        };
        let variadic_support = variadic_arities.first().is_some_and(|variadic_min| variadic_min.is_integer_and(|variadic_min| dbg!(arity >= variadic_min) ));
        let is_arity_supported = direct_support || variadic_support ;

        if !is_arity_supported {
            //let _reconstructed_call = {
            //    let mut l = crate::list_inner!();
            //    let reversed_args = args.iter().map(ToOwned::to_owned).collect::<Vec<_>>().into_iter().rev();
            //    for a in reversed_args {
            //        l.push_front_mut(a.as_ref().to_owned());
            //    }
            //    l.push_front_mut(crate::symbol!(fn_ident));
            //    l
            //};

            // panic!("Wrong number of args passed to {fn_ident}: received {arity}, expected one of {arities}; {reconstructed_call}");

            return Err(EvalError::InvalidArity(
                fn_ident.to_owned(),
                arity as u32,
                arities.to_owned(),
            )).inspect_err(|error| {
                let EvalError::InvalidArity(fn_ident, arity, arities) = error else { unreachable!() };
                crate::deps::tracing::error!(
                    "Wrong number of args passed to {fn_ident}: received {arity}, expected one of {arities}",
                    arities = format!("{}", arities),
                );
            }).map_err(|_| ());

        }
    }

    fn is_valid_variadic_arity(v: &Value) -> bool {
        match v {
            Value::Vector(vs) => vs.len() == 1 && vs.first().unwrap().is_integer(),
            _ => false,
        }
    }

    fn is_valid_arity(v: &Value) -> bool {
        match v {
            Value::Integer(_) => true,
            Value::Vector(_) => is_valid_variadic_arity(v),
            _ => false,
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_arity_v2() {
        super::check_arity(
            "my-fn",
            &crate::set_inner!(crate::integer!(0)),
            &crate::list_inner!(),
        );
    }
}
