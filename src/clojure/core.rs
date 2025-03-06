
use ::core::cell::RefCell;
use crate::keyword::Keyword;
use crate::Symbol;
use crate::value::{Value, IntoValue};
use crate::map::Map;
use crate::list::List;
use crate::vector::Vector;
use crate::rt::env::Env;
use crate::rt::RcValue;
use crate::rt::namespace::Namespace;
use crate::{new_unqualified_symbol, UnqualifiedSymbol};

pub const NS_NAME: &str = "clojure.core";

// pub use def_macro::DefMacro;

//crate::defn!(pub DoMacro, DoMacro::apply);
//
//impl DoMacro {
//    #[tracing::instrument(
//        name = "clojure.core/do"
//        skip(self, env, args),
//        fields(args = %args),
//    )]
//    fn apply(&self, env: &mut Env, args: List) -> RcValue {
//        crate::assert_arity_v2(
//            "clojure.core/do",
//            &crate::set_inner!(crate::vector!(crate::integer!(0))),
//            &args,
//        );
//        let mut ret = Value::Nil;
//        for arg in args.iter() {
//            ret = arg.as_ref().to_owned();
//        }
//        ret.into()
//    }
//}


crate::defn!(pub NsMapFn, "ns-map", NsMapFn::apply, "https://clojuredocs.org/clojure.core/ns-map\n\nReturns a map of all the mappings for the namespace.");

impl NsMapFn {
    //#[tracing::instrument(
    //    name = "clojure.core/ns-map"
    //    skip(self, env, args),
    //    fields(args = %args),
    //)]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        if crate::check_arity(
            "clojure.core/ns-map",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).is_err() {
            return Value::Nil.into();
        }

        let ns_name = args.first().unwrap();
        let ns_name = match ns_name.as_ref() {
            Value::String(ns_name) => ns_name.to_owned(),
            Value::Keyword(Keyword::Unqualified(ns_name)) => ns_name.name().to_owned(),
            Value::Symbol(Symbol::Unqualified(ns_name)) => ns_name.name().to_owned(),
            val @ Value::Handle(rc_any) => {
                match rc_any.downcast_ref::<RefCell<Namespace>>() {
                    Some(refcell_namespace) => {
                        let ns = refcell_namespace.borrow();
                        ns.name().name().to_owned()
                    },
                    None => todo!("(clojure.core/ns-map {})", val),
                }
            },
            other => todo!("(clojure.core/ns-map {})", other),
        };

        if let Some(ns) = env.get_namespace_ref(&UnqualifiedSymbol::new(ns_name)) {
            let ns_map = ns.interns()
                .into_iter()
                .map(|(k, v)| (crate::symbol!(k.name()), Value::Var(v.clone())))
                // .map(|(k, v)| (crate::symbol!(k.name()), v.deref().unwrap().as_ref().to_owned()))
                .collect::<Map>()
                .into_value();
            ns_map
        } else {
            Value::Nil
        }.into()
    }
}



crate::defn!(pub FindNsFn, "find-ns", FindNsFn::apply, "https://clojuredocs.org/clojure.core/find-ns\n\nReturns the namespace named by the symbol or nil if it doesn't exist.");

impl FindNsFn {
    //#[tracing::instrument(
    //    name = "clojure.core/find-ns"
    //    skip(self, env, args),
    //)]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            "clojure.core/find-ns",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let name_fn = crate::Resolve::resolve(env, ("clojure.core", "name")).unwrap().deref().unwrap();
        let name_fn = name_fn.as_afn_panicing();
        let ns_name = name_fn.apply(env, crate::list_inner!(args.first().unwrap().as_ref().to_owned()));
        let Value::String(ns_name) = ns_name.as_ref() else { unreachable!() };
        let ns = match env.get_namespace(&new_unqualified_symbol(ns_name)) {
            Some(ns) => ns,
            _ => return Value::Nil.into(),
        };
        Value::Handle(ns).into()
    }
}


crate::defn!(pub RemoveNsFn, "remove-ns", RemoveNsFn::apply, "https://clojuredocs.org/clojure.core/remove-ns\n\nRemoves the namespace named by the symbol.");

impl RemoveNsFn {
    //#[tracing::instrument(
    //    name = "clojure.core/remove-ns"
    //    skip(self, env, args),
    //)]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            "clojure.core/remove-ns",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let name_fn = crate::Resolve::resolve(env, ("clojure.core", "name")).unwrap().deref().unwrap();
        let name_fn = name_fn.as_afn_panicing();
        let ns_name = name_fn.apply(env, crate::list_inner!(args.first().unwrap().as_ref().to_owned()));
        let Value::String(ns_name) = ns_name.as_ref() else { unreachable!() };
        match env.remove_namespace(&new_unqualified_symbol(ns_name)) {
            Some((_, ns)) => Value::Handle(ns),
            _ => Value::Nil,
        }.into()
    }
}


crate::defn!(pub CreateNsFn, "create-ns", CreateNsFn::apply, "https://clojuredocs.org/clojure.core/create-ns\n\nCreate a new namespace named by the symbol if one doesn't already exist, returns it or the already-existing namespace of the same name.");

impl CreateNsFn {
    //#[tracing::instrument(
    //    name = "clojure.core/create-ns"
    //    skip(self, env, args),
    //)]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            "clojure.core/create-ns",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let name_fn = crate::Resolve::resolve(env, ("clojure.core", "name")).unwrap().deref().unwrap();
        let name_fn = name_fn.as_afn_panicing();
        let ns_name = name_fn.apply(env, crate::list_inner!(args.first().unwrap().as_ref().to_owned()));
        let Value::String(ns_name) = ns_name.as_ref() else { unreachable!() };
        let ns = env.find_or_create_namespace(new_unqualified_symbol(ns_name));
        //let ns = match env.get_namespace(ns_name) {
        //    Some(ns) => ns,
        //    _ => return Value::Nil.into(),
        //};
        Value::Handle(ns).into()
    }
}


crate::defn!(pub NsPublicsFn, "ns-publics", NsPublicsFn::apply, "https://clojuredocs.org/clojure.core/ns-publics\n\nReturns a map of the public intern mappings for the namespace.");

impl NsPublicsFn {
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/ns-publics",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let ns_name = args.first().unwrap();

        let ns_name = match ns_name.as_ref() {
            Value::String(ns_name) => ns_name.to_owned(),
            Value::Keyword(Keyword::Unqualified(ns_name)) => ns_name.name().to_owned(),
            Value::Symbol(Symbol::Unqualified(ns_name)) => ns_name.name().to_owned(),
            val @ Value::Handle(rc_any) => {
                match rc_any.downcast_ref::<RefCell<Namespace>>() {
                    Some(refcell_namespace) => {
                        let ns = refcell_namespace.borrow();
                        ns.name().name().to_owned()
                    },
                    None => todo!("(clojure.core/ns-publics {})", val),
                }
            },
            other => todo!("(clojure.core/ns-publics {})", other),
        };

        if let Some(ns) = env.get_namespace_ref(&UnqualifiedSymbol::new(ns_name)) {
            let ns_map = ns.interns()
                .into_iter()
                .map(|(k, v)| (crate::symbol!(k.name()), Value::Var(v.clone())))
                .collect::<Map>()
                .into_value();
            ns_map
        } else {
            Value::Nil
        }.into()
    }
}


////////////////////////////////////////////////////////////////////////////////
crate::defn!(pub DeclareFn, "declare", DeclareFn::apply, "https://clojuredocs.org/clojure.core/declare\n\ndefs the supplied var names with no bindings, useful for making forward declarations.",);

impl DeclareFn {
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        //crate::assert_arity_v2(
        //    "clojure.core/declare",
        //    &crate::set_inner!(crate::vector!(crate::integer!(1))),
        //    dbg!(&args),
        //);

        let current_ns = crate::current_ns(env.borrow());
        let current_ns = current_ns.borrow();
        let mut current_ns = env.get_namespace_mut(&UnqualifiedSymbol::from(current_ns.name().name())).unwrap();

        args.iter().for_each(|var_name| {
            match var_name.as_ref() {
                Value::Symbol(Symbol::Unqualified(var_name)) => {
                    current_ns.declare(var_name.to_owned());
                },
                other => {
                    todo!("({NS_NAME}/declare {})", other);
                },
            }
        });

        Value::Nil.into()
    }
}
////////////////////////////////////////////////////////////////////////////////



crate::defn!(pub ReferFn, "refer", ReferFn::apply, "For each public interned var in the namespace named by the symbol, adds a mapping from the name of the var to the var to the current namespace.");

impl ReferFn {
    //#[tracing::instrument(
    //    name = "clojure.core/refer"
    //    skip(self, env, args),
    //)]
    fn apply(&self, env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            "clojure.core/refer",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let ns_name = args.first().unwrap();
        let Value::Symbol(ns_name) = ns_name.as_ref() else { unimplemented!() };
        let ns_name = ns_name.name();
        let ns = env.get_namespace_ref(&UnqualifiedSymbol::from(ns_name));
        if ns.is_some() {
            todo!("(find-ns '{})", ns_name);
        } else {
            todo!("no such ns: {}", ns_name);
        }
        // Value::Nil.into()
    }
}



crate::defn!(pub ListFn, "list", ListFn::apply, "Creates a new list containing the items.");

impl ListFn {
    //#[tracing::instrument(
    //    name = "clojure.core/list"
    //    skip(self, _env, args),
    //)]
    fn apply(&self, _env: &mut Env, args: List) -> RcValue {
        args.into_value().into()
    }
}



crate::defn!(pub VectorFn, "vector", VectorFn::apply, "Creates a new vector containing the args.",);

impl VectorFn {
    //#[tracing::instrument(
    //    name = "clojure.core/vector"
    //    skip(self, _env, args),
    //)]
    fn apply(&self, _env: &mut Env, args: List) -> RcValue {
        crate::check_arity(
            "clojure.core/vector",
            &crate::set_inner!(crate::vector!(crate::integer!(0))),
            &args,
        ).unwrap();
        args.iter()
            .map(|ptr| ptr.as_ref().to_owned())
            .collect::<Vector>()
            .into_value()
            .into()
    }
}



crate::defn!(pub PrnFn, "prn", PrnFn::apply, "Print the value in a `read`-able format with a trailing newline.");

impl PrnFn {
    //#[tracing::instrument(
    //    name = "clojure.core/prn"
    //    skip(self, _env, args),
    //)]
    fn apply(&self, _env: &mut Env, args: List) -> RcValue {
        args.iter().for_each(|v| println!("{}", v));
        Value::Nil.into()
    }
}



crate::defn!(pub FirstFn, "first", FirstFn::apply, "Returns the first item in the collection. If coll is nil, returns nil.");

impl FirstFn {
    //#[tracing::instrument(
    //    name = "clojure.core/first"
    //    skip(self, _env, args),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity("clojure.core/first", &crate::set_inner!(crate::integer!(1)), &args).unwrap();
        // todo!()
        let arg1 = args.first().unwrap();
        // assert!(matches!(arg1.as_ref(), Value::List(..) | Value::Vector(..)), "{}", arg1);
        match arg1.as_ref() {
            Value::Nil                                 => Value::Nil.into(),
            Value::List(list)     if list.is_empty()   => Value::Nil.into(),
            Value::Vector(vector) if vector.is_empty() => Value::Nil.into(),
            Value::List(list)                          => list.first().unwrap().clone(),
            Value::Vector(vector)                      => vector.first().unwrap().clone(),
            // _ => unreachable!(),
            _ => todo!(),
        }
    }
}



crate::defn!(pub SecondFn, "second", SecondFn::apply, "Same as (first (next x))");

impl SecondFn {
    //#[tracing::instrument(
    //    "clojure.core/second"
    //    skip(self, _env),
    //    fields(args = %args),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/second",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        let arg1 = args.first().unwrap();
        assert!(matches!(arg1.as_ref(), Value::List(..) | Value::Vector(..)), "{}", arg1);

        match arg1.as_ref() {
            Value::List(list) if list.is_empty() => Value::Nil.into(),
            Value::List(list)                    => list.iter().nth(1).unwrap().clone().into(),
            Value::Vector(vector) if vector.is_empty() => Value::Nil.into(),
            Value::Vector(vector)                      => vector.iter().nth(1).unwrap().clone().into(),
            arg1 => {
                assert!(
                    matches!(arg1, Value::List(..) | Value::Vector(..)),
                    "clojure.core/second invalid application: {}", arg1);
                todo!()
            },
        }
    }
}



crate::defn!(pub LastFn, "last", LastFn::apply, "Return the last item in coll");

impl LastFn {
    //#[tracing::instrument(
    //    "clojure.core/last"
    //    skip(self, _env, args),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        assert_eq!(1, args.len());
        let arg1 = args.first().unwrap();
        assert!(matches!(arg1.as_ref(), Value::List(..) | Value::Vector(..)), "{}", arg1);

        match arg1.as_ref() {
            Value::List(list) if list.is_empty() => Value::Nil.into(),
            Value::List(list)                    => list.iter().last().unwrap().clone().into(),
            Value::Vector(vector) if vector.is_empty() => Value::Nil.into(),
            Value::Vector(vector)                      => vector.iter().last().unwrap().clone().into(),
            arg1 => {
                assert!(
                    matches!(arg1, Value::List(..) | Value::Vector(..)),
                    "clojure.core/last invalid application: {}", arg1);
                todo!()
            },
        }
    }
}



crate::defn!(pub RestFn, "rest", RestFn::apply, "Returns a possibly empty list or vector of the items after the first.");

impl RestFn {
    //#[tracing::instrument(
    //    "clojure.core/rest"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        assert_eq!(1, args.len(), "{}", args);
        let arg1 = args.first().unwrap();
        assert!(
            arg1.is_list() || arg1.is_vector(),
            "(rest {})", arg1,
        );

        let rest = match arg1.as_ref() {
            Value::List(list) => {
                if list.is_empty() { return Value::list_empty().into(); }
                let values: Vec<Value> = list.iter().skip(1).map(|ptr| ptr.as_ref().to_owned()).collect::<Vec<_>>();
                values
            },
            Value::Vector(vector) => {
                if vector.is_empty() { return Value::list_empty().into(); }
                let values: Vec<Value> = vector.iter().skip(1).map(|ptr| ptr.as_ref().to_owned()).collect::<Vec<_>>();
                values
            },
            _ => unreachable!(),
        };

        let rest = Value::list(rest);
        // tracing::Span::current().record("rest", format!("{}", rest));
        rest.into()
    }
}



crate::defn!(
    pub ApplyFn,
    "apply",
    ApplyFn::apply,
    "https://clojuredocs.org/clojure.core/apply\n\n`(apply f args)`\n\nApplies fn `f` to the argument list formed by prepending intervening arguments to args.",
);

impl ApplyFn {
    //#[tracing::instrument(
    //    "clojure.core/apply"
    //    skip(self, env, args),
    //    fields(
    //        // args = %args,
    //        f = %args.first().unwrap(),
    //        f_args = %args.rest().first().unwrap(),
    //    ),
    //    level = "INFO",
    //)]
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/apply",
            &crate::set_inner!(crate::vector!(crate::integer!(2))),
            &args,
        ).unwrap();
        let f = args.first().unwrap();
        let last_arg = args.last().unwrap().into_list_panicing();
        let f_args = match args.len() {
            0 | 1 => unreachable!(),
            2 => { tracing::info!("last_arg = {last_arg}"); crate::list_inner!(last_arg.into_value()) },
            len => todo!("(clojure.core/apply/{len} {f} ,,, {last_arg})"),
        };
        tracing::info!("(clojure.core/apply {f} {f_args})");
        crate::apply(env, f, f_args)
    }
}



crate::defn!(pub EvalFn, "eval", EvalFn::apply, "Evaluates the form data structure (not text!) and returns the result.");

impl EvalFn {
    //#[tracing::instrument(
    //    "clojure.core/eval"
    //    skip(self, env),
    //    fields(args = %args),
    //)]
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/eval",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let to_eval = args.first().unwrap(); // just asserted
        crate::rt::eval(env, to_eval)
    }
}



crate::defn!(pub KeywordFn, "keyword", KeywordFn::apply, "Returns a Keyword with the given namespace and name.");

impl KeywordFn {
    //#[tracing::instrument(
    //    "clojure.core/keyword"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/keyword",
            &crate::set_inner!(
                crate::integer!(1),
                crate::integer!(2),
            ),
            &args,
        ).unwrap();

        let mut iter = args.iter();
        let ns = iter.next().map(|x| x.as_ref());
        let n = iter.next().map(|x| x.as_ref());

        match (ns, n) {
            // (keyword "foo")
            (Some(Value::String(n)),                         None)                                          => crate::keyword!(n),
            // (keyword "foo" "bar")
            (Some(Value::String(ns)),                        Some(Value::String(n)))                        => crate::keyword!(ns, n),
            // (keyword "foo" :bar)
            (Some(Value::String(ns)),                        Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::keyword!(ns, n.name()),
            // (keyword "foo" 'bar)
            (Some(Value::String(ns)),                        Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::keyword!(ns, n.name()),

            // (keyword :foo)
            (Some(Value::Keyword(n)),                         None)                                          => crate::keyword!(n.name()),
            // (keyword :foo "bar")
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::String(n)))                        => crate::keyword!(ns.name(), n),
            // (keyword :foo :bar)
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::keyword!(ns.name(), n.name()),
            // (keyword :foo 'bar)
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::keyword!(ns.name(), n.name()),

            // (keyword 'foo)
            (Some(Value::Symbol(n)),                         None)                                         => crate::keyword!(n.name()),
            // (keyword 'foo "bar")
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::String(n)))                        => crate::keyword!(ns.name(), n),
            // (keyword 'foo :bar)
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::keyword!(ns.name(), n.name()),
            // (keyword 'foo 'bar)
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::keyword!(ns.name(), n.name()),

            (Some(ns),                                       Some(n))                                       => unimplemented!("(clojure.core/keyword {ns} {n})"),
            (Some(n),                                        None)                                          => unimplemented!("(clojure.core/keyword {n})"),
            (None,                                           None)                                          => unimplemented!("(clojure.core/keyword)"),
            (None,                                           Some(_))                                       => unreachable!(),
        }.into()
    }
}



crate::defn!(pub SymbolFn, "symbol", SymbolFn::apply, "Returns a Symbol with the given namespace and name.");

impl SymbolFn {
    //#[tracing::instrument(
    //    "clojure.core/symbol"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/symbol",
            &crate::set_inner!(
                crate::integer!(1),
                crate::integer!(2),
            ),
            &args,
        ).unwrap();

        let mut iter = args.iter();
        let ns = iter.next().map(|x| x.as_ref());
        let n = iter.next().map(|x| x.as_ref());

        match (ns, n) {
            // (symbol "foo")
            (Some(Value::String(n)),                         None)                                          => crate::symbol!(n),
            // (symbol "foo" "bar")
            (Some(Value::String(ns)),                        Some(Value::String(n)))                        => crate::symbol!(ns, n),
            // (symbol "foo" :bar)
            (Some(Value::String(ns)),                        Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::symbol!(ns, n.name()),
            // (symbol "foo" 'bar)
            (Some(Value::String(ns)),                        Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::symbol!(ns, n.name()),

            // (symbol :foo)
            (Some(Value::Keyword(n)),                         None)                                          => crate::symbol!(n.name()),
            // (symbol :foo "bar")
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::String(n)))                        => crate::symbol!(ns.name(), n),
            // (symbol :foo :bar)
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::symbol!(ns.name(), n.name()),
            // (symbol :foo 'bar)
            (Some(Value::Keyword(Keyword::Unqualified(ns))),  Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::symbol!(ns.name(), n.name()),

            // (symbol 'foo)
            (Some(Value::Symbol(n)),                         None)                                         => crate::symbol!(n.name()),
            // (symbol 'foo "bar")
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::String(n)))                        => crate::symbol!(ns.name(), n),
            // (symbol 'foo :bar)
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::Keyword(Keyword::Unqualified(n)))) => crate::symbol!(ns.name(), n.name()),
            // (symbol 'foo 'bar)
            (Some(Value::Symbol(Symbol::Unqualified(ns))),  Some(Value::Symbol(Symbol::Unqualified(n))))   => crate::symbol!(ns.name(), n.name()),

            (Some(ns),                                       Some(n))                                       => unimplemented!("(clojure.core/symbol {ns} {n})"),
            (Some(n),                                        None)                                          => unimplemented!("(clojure.core/symbol {n})"),
            (None,                                           None)                                          => unimplemented!("(clojure.core/symbol)"),
            (None,                                           Some(_))                                       => unreachable!(),
        }.into()
    }
}



crate::defn!(pub VecFn, "vec", VecFn::apply, "Creates a new vector containing the contents of coll.");

impl VecFn {
    //#[tracing::instrument(
    //    "clojure.core/vec"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/vec",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        use crate::value::IntoValue as _;
        let coll = args.first().unwrap();
        match coll.as_ref() {
            Value::Vector(_) => coll,
            Value::List(list) => list.iter().map(|ptr| ptr.as_ref().to_owned()).collect::<Vector>().into_value().into(),
            _ => todo!("(clojure.core/vec {})", coll),
        }
    }
}



crate::defn!(pub NamespaceFn, "namespace", NamespaceFn::apply, "Returns the namespace String of a symbol or keyword, or nil if not present.");

impl NamespaceFn {
    //#[tracing::instrument(
    //    "clojure.core/namespace"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/namespace",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        let x = args.first();
        let x = x.as_ref().map(|x| x.as_ref());
        match x {
            Some(Value::Symbol(Symbol::Qualified(x)))   => crate::string!(x.namespace()),
            Some(Value::Keyword(Keyword::Qualified(x))) => crate::string!(x.namespace()),
            _                                           => crate::nil!(),
            //Some(x)                                   => unimplemented!("(clojure.core/namespace {})", x),
            //None                                      => unimplemented!("(clojure.core/namespace)"),
        }.into()
    }
}



crate::defn!(pub NameFn, "name", NameFn::apply, "Returns the name String of a string, symbol, or keyword.");

impl NameFn {
    //#[tracing::instrument(
    //    "clojure.core/name"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/name",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        let x = args.first();
        let x = x.as_ref().map(|x| x.as_ref());
        match x {
            Some(Value::String(x))  => crate::string!(x),
            Some(Value::Symbol(x))  => crate::string!(x.name()),
            Some(Value::Keyword(x)) => crate::string!(x.name()),
            _                       => crate::nil!(),
            //Some(x)               => unimplemented!("(clojure.core/name {})", x),
            //None                  => unimplemented!("(clojure.core/name)"),
        }.into()
    }
}



crate::defn!(pub SlurpFn, "slurp", SlurpFn::apply, "Opens a reader on f and reads all its contents, returning a string.");

impl SlurpFn {
    //#[tracing::instrument(
    //    "clojure.core/slurp"
    //    skip(self, _env, args),
    //    fields(rest),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/slurp",
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        let x = args.first();
        let Value::String(x) = x.as_ref().map(|x| x.as_ref()).unwrap() else {
            return Value::Nil.into()
        };
        crate::string!(::std::fs::read_to_string(x).unwrap()).into()
    }
}



crate::defn!(pub ConjFn, "conj", ConjFn::apply, "conj\\[oin\\]. Returns a new collection with the xs 'added'. (conj nil item) returns (item). (conj coll) returns coll. (conj) returns []. The 'addition' may happen at different 'places' depending on the concrete type.");

impl ConjFn {
    //#[tracing::instrument(
    //    "clojure.core/conj"
    //    skip(self, _env, args),
    //    level = "ERROR",
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            "clojure.core/conj",
            &crate::set_inner!(
                crate::integer!(0),
                crate::integer!(1),
                crate::vector!(crate::integer!(2)),
            ),
            &args,
        ).unwrap();
        match args.len() {
            0 => crate::vector!().into(),
            1 => args.first().unwrap(),
            _ => {
                let coll = args.first().unwrap();
                match coll.as_ref() {
                    Value::List(list) => {
                        let value = args.rest().first().unwrap();
                        list.push_front(value.as_ref().to_owned()).into_value()
                    },
                    Value::Vector(vector) => {
                        let value = args.rest().first().unwrap();
                        vector.push_back(value.as_ref().to_owned()).into_value()
                    },
                    other => unimplemented!("(cljx::clojure_core::ConjFn::apply {other} {args})"),
                }.into()
            },
        }
    }
}


//crate::defn!(pub ReadFn, ReadFn::apply);
//
//impl ReadFn {
//    #[tracing::instrument(
//        "clojure.core/read"
//        skip(self, _env, args),
//        fields(rest),
//    )]
//    fn apply(
//        &self,
//        _env: &mut Env,
//        args: List,
//    ) -> RcValue {
//        crate::assert_arity_v2(
//            "clojure.core/read",
//            &crate::set_inner!(crate::integer!(1)),
//            &Value::Integer(args.len() as i64),
//        );
//        let x = args.first();
//        let Value::String(x) = x.as_ref().map(|x| x.as_ref()).unwrap() else {
//            return Value::Nil.into()
//        };
//        match crate::read::read(x) {
//            Ok(Some(v)) => v,
//            _ => Value::Nil,
//        }.into()
//    }
//}



crate::defn!(
    pub KeysFn,
    "keys",
    |_, _, args: crate::list::List| {
        crate::check_arity(&format!("{NS_NAME}/keys"), &crate::set_inner!(crate::integer!(1)), &args).unwrap();
        let coll = args.first().unwrap();
        crate::deps::tracing::debug!(target: "clojure-core--keys", coll = %coll);
        if let Value::Map(m) = coll.as_ref() {
            use crate::map::IPersistentMap as _;
            m.keys().collect::<List>().into_value()
        } else {
            Value::Nil
        }.into()
    },
    "https://clojuredocs.org/clojure.core/keys\n\nReturns a list of the map's keys."
);

crate::defn!(
    pub ValsFn,
    "vals",
    |_, _, args: crate::list::List| {
        crate::check_arity(&format!("{NS_NAME}/vals"), &crate::set_inner!(crate::integer!(1)), &args).unwrap();
        let coll = args.first().unwrap();
        crate::deps::tracing::debug!(target: "clojure-core--vals", coll = %coll);
        if let Value::Map(m) = coll.as_ref() {
            use crate::map::IPersistentMap as _;
            m.values().collect::<List>().into_value()
        } else {
            Value::Nil
        }.into()
    },
    "https://clojuredocs.org/clojure.core/values\n\nReturns a list of the map's values."
);

crate::defn!(
    pub MapFn,
    "map",
    MapFn::apply,
    "https://clojuredocs.org/clojure.core/map\n\n"
);

impl MapFn {
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(&format!("{NS_NAME}/map"), &crate::set_inner!(crate::integer!(2)), &args).unwrap();

        let mut args = args.iter();
        let func = args.next().unwrap();
        let coll = args.next().unwrap();
        tracing::debug!("({NS_NAME}/map {func} {coll})");

        let func = func.to_owned();

        match coll.as_ref() {
            Value::List (xs) => {
                xs.iter()
                  .map(|x| crate::apply(
                      env,
                      func.clone(),
                      crate::list_inner!(x.as_ref().to_owned()),
                    ).as_ref().to_owned())
                  .collect::<List>()
                  .into_value()
            },
            Value::Vector (xs) => {
                xs.iter()
                  .map(|x| crate::apply(
                      env,
                      func.clone(),
                      crate::list_inner!(x.as_ref().to_owned()),
                    ).as_ref().to_owned())
                  .collect::<List>()
                  .into_value()
            },
            Value::Set (xs) => {
                xs.iter()
                  .map(|x| crate::apply(
                      env,
                      func.clone(),
                      crate::list_inner!(x.to_owned()),
                    ).as_ref().to_owned())
                  .collect::<List>()
                  .into_value()
            },
            Value::Map (xs) => {
                use crate::IPersistentMap as _;
                xs.entries()
                  .map(|(k, v)| {
                      crate::apply(
                          env,
                          func.clone(),
                          crate::list_inner!(
                              crate::vector!(
                                  k.to_owned(),
                                  v.to_owned(),
                              ),
                          ),
                      )
                  })
                  .map(|rc_value| rc_value.as_ref().to_owned())
                  .collect::<List>()
                  .into_value()
            }
            _ => todo!("{} does not map", coll),
        }.into()
    }
}


crate::defn!(pub InNsFn, "in-ns", InNsFn::apply, "https://clojuredocs.org/clojure.core/in-ns\n\nSets `clojure.core/*ns*` to the namespace named by the symbol, creating it if needed.");

impl InNsFn {
    fn apply(
        &self,
        env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            &format!("{NS_NAME}/in-ns"),
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();

        let new_ns = args.first().unwrap();
        let new_ns_name = match new_ns.as_ref() {
            Value::String(ns_name) => ns_name.to_owned(),
            Value::Keyword(Keyword::Unqualified(ns_name)) => ns_name.name().to_owned(),
            Value::Symbol(Symbol::Unqualified(ns_name)) => ns_name.name().to_owned(),
            Value::Handle(rc_any) => {
                match rc_any.downcast_ref::<RefCell<Namespace>>() {
                    Some(refcell_namespace) => {
                        let ns = refcell_namespace.borrow();
                        ns.name().name().to_owned()
                    },
                    None => todo!(),
                }
            },
            other => todo!("({NS_NAME}/in-ns {})", other),
        };

        crate::Resolve::resolve(env, (NS_NAME, "*ns*")).unwrap()
            .bind(Value::Handle(env.find_or_create_namespace(UnqualifiedSymbol::new(new_ns_name))));

        Value::Nil.into()
    }
}


/// Add the `clojure.core` namespace to an [Env].
pub fn add_to_env(env: &mut Env) {
    let ns_name = new_unqualified_symbol(NS_NAME);

    { // drop second borrow of env
        let mut ns = env.find_or_create_namespace_mut(ns_name.clone());

        ns.declare(new_unqualified_symbol("*ns*"))

        //.intern(new_unqualified_symbol("def")        , DefMacro    .into_value().into())
        //.intern(new_unqualified_symbol("do")         , DoMacro     .into_value().into())

          .intern(new_unqualified_symbol("conj")       , ConjFn      .into_value().into())
          .intern(new_unqualified_symbol("create-ns")  , CreateNsFn  .into_value().into())
          .intern(new_unqualified_symbol("declare")    , DeclareFn   .into_value().into())
          .intern(new_unqualified_symbol("eval")       , EvalFn      .into_value().into())
          .intern(new_unqualified_symbol("find-ns")    , FindNsFn    .into_value().into())
          .intern(new_unqualified_symbol("first")      , FirstFn     .into_value().into())
          .intern(new_unqualified_symbol("in-ns")      , InNsFn      .into_value().into())
          .intern(new_unqualified_symbol("keys")       , KeysFn      .into_value().into())
          .intern(new_unqualified_symbol("keyword")    , KeywordFn   .into_value().into())
          .intern(new_unqualified_symbol("last")       , LastFn      .into_value().into())
          .intern(new_unqualified_symbol("list")       , ListFn      .into_value().into())
          .intern(new_unqualified_symbol("map")        , MapFn       .into_value().into())
          .intern(new_unqualified_symbol("name")       , NameFn      .into_value().into())
          .intern(new_unqualified_symbol("namespace")  , NamespaceFn .into_value().into())
          .intern(new_unqualified_symbol("ns-map")     , NsMapFn     .into_value().into())
          .intern(new_unqualified_symbol("ns-publics") , NsPublicsFn .into_value().into())
          .intern(new_unqualified_symbol("prn")        , PrnFn       .into_value().into())
          .intern(new_unqualified_symbol("rest")       , RestFn      .into_value().into())
          .intern(new_unqualified_symbol("second")     , SecondFn    .into_value().into())
          .intern(new_unqualified_symbol("slurp")      , SlurpFn     .into_value().into())
          .intern(new_unqualified_symbol("symbol")     , SymbolFn    .into_value().into())
          .intern(new_unqualified_symbol("vals")       , ValsFn      .into_value().into())
          .intern(new_unqualified_symbol("vec")        , VecFn       .into_value().into())
          .intern(new_unqualified_symbol("vector")     , VectorFn    .into_value().into())

        ;
    }

    crate::set_current_ns(env, ns_name);
}

/// Create a new `cljx::Env` with `clojure.core`
pub fn new_env() -> Env {
    let mut env = Env::new_empty();
    self::add_to_env(&mut env);
    env
}



mod def_macro {
    use crate::value::Value;
    use crate::symbol::Symbol;
    use crate::list::List;
    use crate::rt::env::Env;
    use crate::rt::{RcValue, AFn};
    use crate::new_unqualified_symbol;


    pub struct DefMacro;

    impl AFn for DefMacro {
        //#[tracing::instrument(
        //    name = "DefMacro::apply"
        //    skip(self, env, args),
        //)]
        fn apply(
            &self,
            env: &mut Env,
            args: List,
        ) -> RcValue {
            assert!(args.len() >= 1);
            let mut args_iter = args.iter();
            let name = args_iter.next().unwrap();

            assert!(name.is_symbol_qualified(), "{}", name);
            let Value::Symbol(Symbol::Qualified(symbol)) = name.as_ref() else { unreachable!() };

            let ns = symbol.namespace();
            let n = new_unqualified_symbol(symbol.name());

            let value = args_iter.next()
                .map(RcValue::as_ref)
                .map(ToOwned::to_owned)
                .unwrap_or(Value::Nil);
            env.find_or_create_namespace_mut(new_unqualified_symbol(ns))
               .intern(n, value.into());

            Value::Nil.into()
        }
    }
}



#[cfg(test)]
mod keyword_tests {
    use crate::value::Value;
    use crate::list::List;
    use crate::rt::{env::Env, Resolve as _};

    fn test_ret_eq(
        args: List,
        expected_ret: Value,
    ) -> Value {
        // arrange
        let mut env = Env::new_empty();
        crate::clojure_core::add_to_env(&mut env);
        let keyword_fn = env.resolve(("clojure.core", "keyword")).unwrap().deref().unwrap();
        // act
        let ret = keyword_fn.as_afn_panicing().apply(&mut env, args);
        // assert
        assert_eq!(ret.as_ref(), &expected_ret);
        ret.as_ref().to_owned()
    }

    #[test]
    #[should_panic] // wrong number of args
    fn no_args() {
        // arrange
        let mut env = Env::new_empty();
        crate::clojure_core::add_to_env(&mut env);
        let keyword_fn = env.resolve(("clojure.core", "keyword")).unwrap().deref().unwrap();
        // act
        let args = crate::list_inner!();
        keyword_fn.as_afn_panicing().apply(&mut env, args);
        // assert (should_panic)
    }


    #[test]
    fn string() {
        test_ret_eq(
            crate::list_inner!(crate::string!("n")),
            crate::keyword!("n"),
        );
    }

    #[test]
    fn string_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::string!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn string_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::keyword!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn string_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::symbol!("n")),
            crate::keyword!("ns", "n"),
        );
    }


    #[test]
    fn unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("n")),
            crate::keyword!("n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::string!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::keyword!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::symbol!("n")),
            crate::keyword!("ns", "n"),
        );
    }


    #[test]
    fn unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("n")),
            crate::keyword!("n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::string!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::keyword!("n")),
            crate::keyword!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::symbol!("n")),
            crate::keyword!("ns", "n"),
        );
    }
}


#[cfg(test)]
mod symbol_tests {
    use crate::value::Value;
    use crate::list::List;
    use crate::rt::{env::Env, Resolve as _};

    fn test_ret_eq(
        args: List,
        expected_ret: Value,
    ) -> Value {
        // arrange
        let mut env = Env::new_empty();
        crate::clojure_core::add_to_env(&mut env);
        let symbol_fn = env.resolve(("clojure.core", "symbol")).unwrap().deref().unwrap();
        // act
        let ret = symbol_fn.as_afn_panicing().apply(&mut env, args);
        // assert
        assert_eq!(ret.as_ref(), &expected_ret);
        ret.as_ref().to_owned()
    }

    #[test]
    #[should_panic] // wrong number of args
    fn no_args() {
        // arrange
        let mut env = Env::new_empty();
        crate::clojure_core::add_to_env(&mut env);
        let symbol_fn = env.resolve(("clojure.core", "symbol")).unwrap().deref().unwrap();
        // act
        let args = crate::list_inner!();
        symbol_fn.as_afn_panicing().apply(&mut env, args);
        // assert (should_panic)
    }


    #[test]
    fn string() {
        test_ret_eq(
            crate::list_inner!(crate::string!("n")),
            crate::symbol!("n"),
        );
    }

    #[test]
    fn string_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::string!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn string_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::keyword!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn string_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::string!("ns"), crate::symbol!("n")),
            crate::symbol!("ns", "n"),
        );
    }


    #[test]
    fn unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("n")),
            crate::symbol!("n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::string!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::keyword!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_keyword_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::keyword!("ns"), crate::symbol!("n")),
            crate::symbol!("ns", "n"),
        );
    }


    #[test]
    fn unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("n")),
            crate::symbol!("n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_string() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::string!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_unqualified_keyword() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::keyword!("n")),
            crate::symbol!("ns", "n"),
        );
    }

    #[test]
    fn unqualified_symbol_and_unqualified_symbol() {
        test_ret_eq(
            crate::list_inner!(crate::symbol!("ns"), crate::symbol!("n")),
            crate::symbol!("ns", "n"),
        );
    }
}

#[cfg(test)]
mod first_tests {
    use crate::{Value, List};

    //fn test_ret_eq(
    //    args: List,
    //    expected_ret: Value,
    //) -> Value {
    //    // arrange
    //    let mut env = Env::new_empty();
    //    crate::clojure_core::add_to_env(&mut env);
    //    let first_fn = env.resolve(("clojure.core", "first")).unwrap().deref().unwrap();
    //    // act
    //    let ret = first_fn.as_afn().apply(&mut env, args);
    //    // assert
    //    assert_eq!(ret.as_ref(), &expected_ret);
    //    ret.as_ref().to_owned()
    //}

    fn test_ret_eq(
        args: List,
        expected_ret: Value,
    ) -> Value {
        // arrange
        let mut env = super::Env::new_empty();
        let first_fn = super::FirstFn;

        // act
        let ret = first_fn.apply(&mut env, args);
        let ret = ret.as_ref();

        // assert
        assert_eq!(ret, &expected_ret);
        ret.to_owned()
    }

    #[test]
    fn given_nil_returns_nil() {
        test_ret_eq(
            crate::list_inner!(Value::Nil),
            Value::Nil,
        );
    }

    #[test]
    fn given_vector_returns_item_at_index_0() {
        test_ret_eq(
            crate::list_inner!(
                crate::vector!(
                    crate::integer!(6),
                    crate::integer!(7),
                    crate::integer!(8),
                ),
            ),
            crate::integer!(6),
        );
    }
}


#[cfg(test)]
mod declare_tests {
    use crate::{Env, Value, IntoValue, UnqualifiedSymbol};

    #[test]
    fn interns_unbound_var_in_ns() {
        crate::deps::tracing_subscriber::fmt()
            .finish();

        // arrange
        let mut env = crate::clojure_core::new_env();
        let current_ns_name = crate::clojure_core::NS_NAME;
        crate::set_current_ns(&mut env, &new_unqualified_symbol(current_ns_name));

        // act 1 - single
        // (declare my-var)
        let fn_ret = super::DeclareFn.apply(
            &mut env,
            crate::list_inner!(
                crate::symbol!("my-var"),
            ),
        );
        crate::assert_nil!(fn_ret);

        // assert 1
        assert!(
            crate::Resolve::resolve(&env, (current_ns_name, "my-var"))
                .is_some_and(|my_var| my_var.is_unbound())
        );

        // act 2 - multiple
        // (declare my-other-var yet-another-var)
        let fn_ret = super::DeclareFn.apply(
            &mut env,
            crate::list_inner!(
                crate::symbol!("my-other-var"),
                crate::symbol!("yet-another-var"),
            ),
        );

        // assert 2
        assert!(
            crate::Resolve::resolve(&env, (current_ns_name, "my-other-var"))
                .is_some_and(|my_other_var| my_other_var.is_unbound())
        );
        assert!(
            crate::Resolve::resolve(&env, (current_ns_name, "yet-another-var"))
                .is_some_and(|yet_another_var| yet_another_var.is_unbound())
        );

    }
}
