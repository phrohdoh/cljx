
use crate::{new_unqualified_symbol, Value, Env};

pub fn make_env(short_env: Vec<(&'static str, Vec<(&'static str, Value)>)>) -> Env {
    let mut env = Env::new_empty();
    for (ns, vals) in short_env {
        let mut ns = env.find_or_create_namespace_mut(new_unqualified_symbol(ns));
        for (n, val) in vals {
            ns.intern(new_unqualified_symbol(n), val.into());
        }
    }
    env
}
