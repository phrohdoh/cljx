
use cljx::RcValue;
use cljx::Env;
use cljx::read_one;
// use cljx::clojure_core::{ListFn, FirstFn, SecondFn, LastFn, RestFn};
// use cljx::utils as cljx_utils;

pub fn read(input: &str) -> RcValue {
    read_one(input).unwrap().unwrap().into()
}

pub fn make_env() -> Env {
    let mut env = Env::new_empty();
    cljx::clojure_core::add_to_env(&mut env);
    cljx::clojure_edn::add_to_env(&mut env);
    cljx::cljx_core::add_to_env(&mut env);
    cljx::user::add_to_env(&mut env);
    env
}
