
mod deps {
    pub use cljx;
}

use std::io::Write as _;

use crate::deps::cljx;
use crate::deps::cljx::deps::tracing;
use crate::deps::cljx::deps::tracing_subscriber::{self, filter::EnvFilter};
use crate::deps::cljx::Env;
use crate::deps::cljx::RcValue;

// use crate::rt::namespace_v2::NamespaceExts;

fn create_env_filter() -> EnvFilter {
    EnvFilter::from_env("LOG_LEVEL")
        //.add_directive("cljx=info".parse().unwrap())
}

fn init_logging() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(create_env_filter())
        .with_writer(std::io::stderr)
        // .with_file(true)
        // .with_line_number(true)
        .with_target(false)
        .without_time()
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}


fn create_env() -> Env {
    let mut env = cljx::clojure_core::new_env();
    cljx::clojure_edn::add_to_env(&mut env);
    // cljx::cljx_core::add_to_env(&mut env);
    cljx::user::add_to_env(&mut env);

    // set back to clojure.core in case something else changed it
    cljx::set_current_ns(&mut env, cljx::new_unqualified_symbol(cljx::clojure_core::NS_NAME));

    env
}

fn repl(env: &mut Env) {
    use cljx::Resolve as _;
    let read_string_fn = env.resolve(("clojure.edn", "read-string")).unwrap().deref().unwrap();
    let eval_fn = env.resolve(("clojure.core", "eval")).unwrap().deref().unwrap();
    let prn_fn = env.resolve(("clojure.core", "prn")).unwrap().deref().unwrap();
    // let _ns_var = env.resolve(("clojure.core", "*ns*")).unwrap();

    let read_string_fn = read_string_fn.as_afn_panicing();
    let eval_fn = eval_fn.as_afn_panicing();
    let prn_fn = prn_fn.as_afn_panicing();

    print!("{}=> ", {
        let current_ns = cljx::current_ns(env);
        let current_ns = current_ns.borrow();
        current_ns.name().to_owned()
    });
    ::std::io::stdout().flush().unwrap();

    ::std::io::stdin().lines().filter_map(Result::ok).for_each(|line| {
        let read_ret = read_string_fn.apply(env, cljx::list_inner!(cljx::string!(line)));
        let read_ret = read_ret.as_ref();
        if read_ret == &cljx::keyword!("repl", "quit") || read_ret == &cljx::keyword!("repl", "exit") {
            prn_fn.apply(env, cljx::list_inner!(cljx::symbol!("Bye!")));
            ::std::process::exit(0);
        }
        let eval_ret = eval_fn.apply(env, cljx::list_inner!(read_ret.to_owned()));
        prn_fn.apply(env, cljx::list_inner!(eval_ret.as_ref().to_owned()));

        print!("{}=> ", {
            let current_ns = cljx::current_ns(env);
            let current_ns = current_ns.borrow();
            current_ns.name().to_owned()
        });
        ::std::io::stdout().flush().unwrap();
    });
}

fn run() {
    use ::std::path::Path;

    let mut args = ::std::env::args().skip(1);
    match args.next() {
        Some(cmd) => {
            match cmd.as_str() {
                "-e" => {
                    let to_eval = args.next().unwrap();
                    handle_eval(&to_eval);
                },
                "-ep" => {
                    let to_eval = args.next().unwrap();
                    handle_eval(&format!("(clojure.core/prn {to_eval})"));
                },
                "repl" => {
                    handle_repl();
                },
                file_path => {
                    handle_file_path(Path::new(file_path));
                },
            }
        },
        None => {
            eprintln!("error: must provide subcommand or filepath");
            ::std::process::exit(1);
        },
    }

    fn handle_repl() {
        let mut env = create_env();
        repl(&mut env);
    }

    fn handle_eval(str_to_eval: &str) {
        let mut env = create_env();

        let str_to_read_then_eval = cljx::string!(str_to_eval);

        use cljx::IntoValue as _;
        let read_string_fn = cljx::clojure_edn::ReadStringFn.into_value();
        let eval_fn = cljx::clojure_core::EvalFn.into_value();

        let read_string_fn = read_string_fn.as_afn_panicing();
        let eval_fn = eval_fn.as_afn_panicing();

        let read_ret = read_string_fn.apply(&mut env, cljx::list_inner!(str_to_read_then_eval));
        let read_ret = read_ret.as_ref();

        let eval_args = cljx::list_inner!(read_ret.to_owned());
        eval_fn.apply(&mut env, eval_args);
    }

    fn handle_file_path(file_path: &Path) {
        let mut env = create_env();
        let str_to_read = ::std::fs::read_to_string(file_path).unwrap();
        cljx::read_many(&str_to_read).unwrap()
            .into_iter()
            .map(RcValue::from)
            .for_each(|value| { cljx::eval(&mut env, value); });
    }
}

fn main() {
    init_logging();
    run();
}
