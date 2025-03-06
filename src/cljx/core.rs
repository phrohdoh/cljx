
use crate::value::{Value, IntoValue as _};
use crate::rt::env::Env;
use crate::rt::RcValue;
use crate::list::List;
use crate::map::Map;
use crate::new_unqualified_symbol;


pub const NS_NAME: &str = "cljx.core";


crate::defn!(pub AllNsMapFn, "all-ns-map", AllNsMapFn::apply, "Similar to https://clojuredocs.org/clojure.core/all-ns.\n\nReturns a map of namespace name as a symbol to a Namespace Handle of all namespaces.");

impl AllNsMapFn {
    #[tracing::instrument(
        name = "cljx.core/all-ns-map",
        skip(self, env, args),
    )]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            &format!("{NS_NAME}/all-ns-map"),
            &crate::set_inner!(crate::integer!(0)),
            &args,
        ).unwrap();
        env.namespaces()
            .into_iter()
            .map(|(ns_name, ns)| (
                crate::symbol!(ns_name),
                Value::Handle(ns),
            ))
            .collect::<Map>()
            .into_value()
            .into()
    }
}


crate::defn!(pub EnvMapFn, "env-map", EnvMapFn::apply);

impl EnvMapFn {
    #[tracing::instrument(
        name = "user/env-map"
        skip(self, env, args),
    )]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            &format!("{NS_NAME}/env-map"),
            &crate::set_inner!(crate::integer!(0)),
            &args,
        ).unwrap();

        let all_ns_map = crate::Resolve::resolve(env, ("user", "all-ns-map")).unwrap().deref().unwrap();
        let all_ns_map = all_ns_map.as_afn_panicing();

        let all_ns = all_ns_map.apply(env, crate::list_inner!());
        let all_ns = all_ns.as_ref();
        let Value::Map(all_ns) = all_ns else {
            tracing::error!("{}", crate::map!((crate::keyword!("all-ns"), all_ns.to_owned())));
            return Value::Nil.into()
        };

        use crate::IPersistentMap as _;
        let all_ns_names = all_ns.keys().collect::<List>();

        let ns_map_fn = crate::Resolve::resolve(env, ("clojure.core", "ns-map")).unwrap().deref().unwrap();
        let ns_map_fn = ns_map_fn.as_afn_panicing();

        let env_map = {
            let mut m = crate::PersistentMap::new();
            for ns_name in all_ns_names.iter() {
                let ns_map = ns_map_fn.apply(env, crate::list_inner!(ns_name.as_ref().to_owned())).as_ref().to_owned();
                m.insert_mut(ns_name.as_ref().to_owned(), ns_map);
            }
            Map::new(m)
        };
        crate::into_value!(env_map).into()

        // todo!()
        // Map::new(m).into_value().into()
        // m.into_value().into()

        //env.namespaces()
        //    .iter()
        //    .map(|(ns_name, ns)| (
        //        crate::symbol!(ns_name.name()),
        //        Value::Handle(ns.to_owned()),
        //    ))
        //    .collect::<Map>()
        //    .into_value()
        //    .into()
    }
}

/// Add the `cljx.core` namespace to an [Env].
pub fn add_to_env(env: &mut Env) {
    env.find_or_create_namespace_mut(new_unqualified_symbol(NS_NAME))
       .intern(new_unqualified_symbol("all-ns-map") , AllNsMapFn .into_value().into())
       .intern(new_unqualified_symbol("env-map")    , EnvMapFn   .into_value().into())
       ;

    //env.find_or_create_namespace_mut("clojure.core")
    //   .mutate_var("*ns*", |var| { var.bind(crate::symbol!(NS_NAME).into()); });
}
