
use cljx::eval;
// use cljx::clojure_core::{ListFn, FirstFn, SecondFn, LastFn, RestFn};

#[path = "utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[test]
fn rest_fn() {
    let mut env = make_env();

    assert_eq!(
        eval(&mut env, read("(clojure.core/rest ())")),
        read("()"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/rest (clojure.core/list :a :b :c))")),
        read("(:b :c)"),
    );


    assert_eq!(
        eval(&mut env, read("(clojure.core/rest [])")),
        read("()"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/rest [:a :b :c])")),
        read("(:b :c)"),
    );
}
