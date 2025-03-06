
use cljx::eval;
// use cljx::clojure_core::{ListFn, FirstFn, SecondFn, LastFn, RestFn};

#[path = "utils/mod.rs"]
mod test_utils;
use test_utils::*;

#[test]
fn list_fn() {
    let mut env = make_env();

    assert_eq!(
        eval(&mut env, read("(clojure.core/list)")),
        read("()"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/list :a :b :c)")),
        read("(:a :b :c)"),
    );
}

#[test]
fn first_fn() {
    let mut env = make_env();

    assert_eq!(
        eval(&mut env, read("(clojure.core/first ())")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/first (clojure.core/list))")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/first (clojure.core/list :a :b :c))")),
        read(":a"),
    );
}

#[test]
fn second_fn() {
    let mut env = make_env();

    assert_eq!(
        eval(&mut env, read("(clojure.core/second ())")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/second (clojure.core/list))")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/second (clojure.core/list :a :b :c))")),
        read(":b"),
    );
}

#[test]
fn last_fn() {
    let mut env = make_env();

    assert_eq!(
        eval(&mut env, read("(clojure.core/last ())")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/last (clojure.core/list))")),
        read("nil"),
    );

    assert_eq!(
        eval(&mut env, read("(clojure.core/last (clojure.core/list :a :b :c))")),
        read(":c"),
    );
}
