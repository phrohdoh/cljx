use crate::value::{Value, IntoValue as _};
use crate::list::List;
use crate::rt::env::Env;
use crate::rt::RcValue;
use crate::new_unqualified_symbol;


pub const NS_NAME: &str = "clojure.edn";


crate::defn!(pub ReadStringFn, "read-string", ReadStringFn::apply);

impl ReadStringFn {
    //#[tracing::instrument(
    //    "clojure.edn/read-string"
    //    level = "TRACE",
    //    skip(self, _env),
    //    fields(args = %args),
    //)]
    fn apply(
        &self,
        _env: &mut Env,
        args: List,
    ) -> RcValue {
        crate::check_arity(
            &format!("{NS_NAME}/read-string"),
            &crate::set_inner!(crate::integer!(1)),
            &args,
        ).unwrap();
        let x = args.first();
        let Value::String(x) = x.as_ref().map(|x| x.as_ref()).unwrap() else {
            return Value::Nil.into()
        };
        match crate::read_one(x.as_str()) {
            Ok(Some(v)) => v,
            _ => Value::Nil,
        }.into()
    }
}


/// Add the `clojure.edn` namespace to an [Env].
pub fn add_to_env(env: &mut Env) {
    env.find_or_create_namespace_mut(new_unqualified_symbol(NS_NAME))
       .intern(new_unqualified_symbol("read-string") , ReadStringFn.into_value().into())
       ;

    //env.find_or_create_namespace_mut("clojure.core")
    //   .mutate_var("*ns*", |var| { var.bind(crate::symbol!(NS_NAME).into()); });
}
