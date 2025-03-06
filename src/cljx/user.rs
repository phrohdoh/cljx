
use crate::rt::env::Env;
use crate::value::{Value, IntoValue as _};
use crate::new_unqualified_symbol;

pub const NS_NAME: &str = "user";

/// Add the `user` namespace to an [Env].
pub fn add_to_env(env: &mut Env) {
    env.find_or_create_namespace_mut(new_unqualified_symbol(NS_NAME))
       .intern(new_unqualified_symbol("all-ns-map") , crate::cljx_core::AllNsMapFn .into_value().into())
       .intern(new_unqualified_symbol("env-map")    , crate::cljx_core::EnvMapFn   .into_value().into())
       .intern(new_unqualified_symbol("NIL")        , Value::Nil.into())
       .declare(new_unqualified_symbol("UBV"))
       ;

    //env.find_or_create_namespace_mut("clojure.core")
    //   .mutate_var("*ns*", |var| { var.bind(crate::symbol!(NS_NAME).into()); });
}
