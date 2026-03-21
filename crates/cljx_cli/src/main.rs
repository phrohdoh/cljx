use std::{env, io::{self}, rc::Rc};
use cljx::prelude::*;

use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing::{self as log, Level};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};


fn init_tracer_provider() -> Result<SdkTracerProvider, Box<dyn std::error::Error + Send + Sync + 'static>> {
    // Set up the OTLP exporter to send traces via gRPC
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()?;

    // Create a tracer provider with a batch span processor
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(Resource::builder()
            .with_service_name("cljx")
            .build())
        .build();

    // Set the global tracer provider
    global::set_tracer_provider(tracer_provider.clone());

    // Set up the tracing subscriber with OpenTelemetry layer
    let telemetry_layer = OpenTelemetryLayer::new(tracer_provider.tracer("cljx"));

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        //.with(tracing_subscriber::fmt::layer())
        .with(telemetry_layer)
        .init();

    Ok(tracer_provider)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer_provider = init_tracer_provider().unwrap();
    use futures::FutureExt as _;
    let run_result = std::panic::AssertUnwindSafe(run()).catch_unwind().await;
    tracer_provider.shutdown()?;
    match run_result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(panic_err) => {
            eprintln!("Panic occurred: {:?}", panic_err);
            std::process::exit(1);
        }
    }
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    log::info!("START RUN");
    let mut args = env::args();
    let bin_call = args.next().unwrap().to_owned();
    log::info!("START ARGS: {:?} {:?}", bin_call, args);
    let args = args.collect::<Vec<_>>();
    if let Some(first) = args.first() {
        match first.as_ref() {
            "--help" | "-h" | "help" => {
                usage(&bin_call);
                log::info!("END RUN");
                return Ok(());
            },
            "repl" => {
                demo_repl(
                    &bin_call,
                    args.iter()
                        .map(ToOwned::to_owned)
                        .skip(1)
                        .collect::<Vec<_>>(),
                );
            },
            "eval-string" => {
                eval_string(
                    &bin_call,
                    args.iter()
                        .map(ToOwned::to_owned)
                        .skip(1)
                        .collect::<Vec<_>>(),
                )
            },
            "eval-file" => {
                eval_file(
                    &bin_call,
                    args.iter()
                        .map(ToOwned::to_owned)
                        .skip(1)
                        .collect::<Vec<_>>(),
                )
            },
            "read-string" => {
                read_string(
                    &bin_call,
                    args.iter()
                        .map(ToOwned::to_owned)
                        .skip(1)
                        .collect::<Vec<_>>(),
                )
            },
            "optics" => {
                demo_optics(
                    &bin_call,
                    args.iter()
                        .map(ToOwned::to_owned)
                        .skip(1)
                        .collect::<Vec<_>>(),
                )
            },
            _ => todo!("{:?}", first),
        }
    }
    log::info!("END RUN");
    Ok(())
}

#[derive(Debug, Clone)]
pub enum ResolveError {
    NoSuchNamespace(SymbolUnqualified),
    NoSuchVar(SymbolQualified),
    UnboundVar(SymbolQualified),
    UnknownCurrentNamespace,
}

impl From<GetVarError> for ResolveError {
    fn from(get_var_err: GetVarError) -> Self {
        match get_var_err {
            GetVarError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
        }
    }
}

impl From<GetValueError> for ResolveError {
    fn from(get_value_err: GetValueError) -> Self {
        match get_value_err {
            GetValueError::NoSuchVar(var_sym) => Self::NoSuchVar(var_sym),
            GetValueError::UnboundVar(var_sym) => Self::UnboundVar(var_sym),
        }
    }
}

#[tracing::instrument(ret, fields(env, symbol), level = "info")]
fn try_resolve(
    env: RcEnvironment,
    symbol: &Symbol
) -> Result<RcVar, ResolveError> {
    match symbol {
        Symbol::Qualified(sym) => {
            log::warn!("Resolving qualified symbol: {}", sym);
            env.try_get_namespace(sym.namespace())
               .ok_or_else(|| ResolveError::NoSuchNamespace(SymbolUnqualified::new(sym.namespace())))?
               .try_get_var(sym.name())
               .map_err(ResolveError::from)
        },
        Symbol::Unqualified(sym) => {
            log::warn!("Resolving unqualified symbol: {}", sym);
            env.try_get_namespace("clojure.core")
               .ok_or_else(|| ResolveError::NoSuchNamespace(SymbolUnqualified::new("clojure.core")))?
               .try_get_handle::<RcNamespace>("*ns*")
               .map_err(|_| ResolveError::UnknownCurrentNamespace)?
               .try_get_var(sym.name())
               .map_err(ResolveError::from)
        },
    }
}

#[tracing::instrument(ret, fields(env, symbol), level = "info")]
fn resolve_or_panic(
    env: RcEnvironment,
    symbol: &Symbol
) -> RcVar {
    try_resolve(env, symbol)
        .expect(&format!("could not resolve: #'{}", symbol))
}

#[tracing::instrument(ret, fields(env, v), level = "info")]
fn eval(
    env: RcEnvironment,
    v: RcValue,
) -> RcValue {
    let clojure_core = env.get_namespace_or_panic("clojure.core");
    let eval_func = clojure_core.get_function_or_panic("eval");
    let apply_func = clojure_core.get_function_or_panic("apply");
    match v.as_ref() {
        Value::Nil(_) => v,
        Value::Symbol(symbol, _) => resolve_or_panic(env.clone(), symbol).deref().expect(&format!("attempted to deref unbound Var: #'{}", symbol)),
        Value::Keyword(_, _) => v,
        Value::Boolean(_, _) => v,
        Value::Integer(_, _) => v,
        Value::Float(_, _) => v,
        Value::String(_, _) => v,
        Value::List(list, _) => {
            if list.is_empty() { return v; }
            let args: Vec<RcValue> = list.iter().skip(1).map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect();
            let v = eval_func.invoke(env.clone(), vec![list.get_first().unwrap().to_owned()]);
            apply_func.invoke(env.clone(), {
                let mut apply_args = args;
                apply_args.insert(0, v);
                apply_args
            })
        },
        Value::Vector(vector, _) => Value::new_vector_rc(vector.iter().map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect()),
        Value::Set(set, _) => Value::new_set_rc(set.iter().map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect()),
        Value::Map(map, _) => Value::new_map_rc(map.iter().map(|(k, v)| (eval_func.invoke(env.clone(), vec![k.to_owned()]), eval_func.invoke(env.clone(), vec![v.to_owned()]))).collect()),
        Value::Var(var, _) => var.deref().expect("attempted to deref unbound Var"),
        Value::Function(_, _) => v,
        Value::Handle(_, _) => v,
    }
}

#[tracing::instrument(ret, fields(env, f, args), level = "info")]
fn apply(
    env: RcEnvironment,
    f: RcValue,
    args: Vec<RcValue>,
) -> RcValue {
    match f.as_ref() {
        Value::Function(func, _) => func.invoke(env.clone(), args),
        Value::Handle(handle, _) => {
            if let Some(func) = handle.downcast_ref::<Function>() {
                func.invoke(env.clone(), args)
            } else {
                f
            }
        }
        // TODO: properly handle other variants
        _ => {
            eprintln!(
                "Warning: apply called on non-function value: {:?}",
                f
            );
            f
        }
    }
}

fn usage(bin_call: &str) {
    println!("Usage of {bin_call}:");
    println!("{bin_call} --help");
    println!("{bin_call} repl");
    println!("{bin_call} eval-string '(+ 1 2)'");
    println!("{bin_call} eval-file /path/to/file.cljx");
}

fn usage_repl(bin_call: &str) {
    println!("Usage: {bin_call} repl - enter one command per line");
}

// #[tracing::instrument(ret, level = "info")]
fn create_env() -> RcEnvironment {
    let env = Environment::new_empty_rc();

    let clojure_core = env.create_namespace("clojure.core");
    clojure_core.bind_value("*ns*", Value::handle(Handle::new(clojure_core.clone())));

    // (defn clojure.core/+ [& xs])
    // (clojure.core/+ a)
    // (clojure.core/+ a b)
    // (clojure.core/+ a b c ,,,)
    clojure_core.build_and_bind_function(
        "+",
        vec![(
            FunctionArity::AtLeast(0),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let any_arg_is_float = args.iter().map(Rc::as_ref).any(Value::is_float);
                if any_arg_is_float {
                    let mut x = 0f64;
                    for arg in args {
                        let arg_integer = value::optics::view_integer(arg.as_ref());
                        let arg_float   = value::optics::view_float(arg.as_ref());
                        match (arg_integer, arg_float) {
                            (Some(int), None) => x += int as f64,
                            (None, Some(float)) => x += float.as_f64(),
                            (Some(_), Some(_)) => unreachable!("value cannot be both integer and float"),
                            (None, None) => panic!("clojure.core/+ only supports integer and float arguments, but got: {:?}", arg),
                        }
                    }
                    Rc::new(Value::float(x.into()))
                } else {
                    let mut x = 0i64;
                    for arg in args {
                        let arg_integer = value::optics::view_integer(arg.as_ref()).unwrap();
                        x += arg_integer;
                    }
                    Rc::new(Value::integer(x))
                }
            }),
        )],
    );

    // (defn clojure.core/- [x & xs])
    // (clojure.core/- a)
    // (clojure.core/- a b)
    // (clojure.core/- a b c ,,,)
    clojure_core.build_and_bind_function(
        "-",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, _args: Vec<RcValue>| {
                todo!()
                // let any_arg_is_float = args.iter().map(Rc::as_ref).any(Value::is_float);
                // if any_arg_is_float {
                //     let mut x = 0f64;
                //     for arg in args {
                //         let arg_integer = value::optics::view_integer(arg.as_ref());
                //         let arg_float   = value::optics::view_float(arg.as_ref());
                //         match (arg_integer, arg_float) {
                //             (Some(int), None) => x -= int as f64,
                //             (None, Some(float)) => x -= float.as_f64(),
                //             (Some(_), Some(_)) => unreachable!("value cannot be both integer and float"),
                //             (None, None) => panic!("clojure.core/+ only supports integer and float arguments, but got: {:?}", arg),
                //         }
                //     }
                //     Rc::new(Value::float(x.into()))
                // } else {
                //     let mut x = 0i64;
                //     for arg in args {
                //         let arg_integer = value::optics::view_integer(arg.as_ref()).unwrap();
                //         x += arg_integer;
                //     }
                //     Rc::new(Value::integer(x))
                // }
            }),
        ), (
            FunctionArity::AtLeast(2),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let any_arg_is_float = args.iter().map(Rc::as_ref).any(Value::is_float);
                if any_arg_is_float {
                    let mut x = 0f64;
                    for arg in args {
                        let arg_integer = value::optics::view_integer(arg.as_ref());
                        let arg_float   = value::optics::view_float(arg.as_ref());
                        match (arg_integer, arg_float) {
                            (Some(int), None) => x -= int as f64,
                            (None, Some(float)) => x -= float.as_f64(),
                            (Some(_), Some(_)) => unreachable!("value cannot be both integer and float"),
                            (None, None) => panic!("clojure.core/- only supports integer and float arguments, but got: {:?}", arg),
                        }
                    }
                    Rc::new(Value::float(x.into()))
                } else {
                    let mut x = value::optics::view_integer(args[0].as_ref()).unwrap();
                    for arg in args.iter().skip(1) {
                        let arg_integer = value::optics::view_integer(arg.as_ref()).unwrap();
                        x -= arg_integer;
                    }
                    Rc::new(Value::integer(x))
                }
            }),
        )],
    );

    // (defn clojure.core/map [f coll])
    // (clojure.core/map f coll)
    clojure_core.build_and_bind_function(
        "map",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| {
                let f = args[0].clone();
                let coll = args[1].clone();
                match coll.as_ref() {
                    Value::List(list, _) => Value::new_list_rc(list.iter().map(|element| apply(env.clone(), f.clone(), vec![element.to_owned()])).collect()),
                    Value::Vector(vector, _) => Value::new_vector_rc(vector.iter().map(|element| apply(env.clone(), f.clone(), vec![element.to_owned()])).collect()),
                    Value::Set(set, _) => Value::new_set_rc(set.iter().map(|element| apply(env.clone(), f.clone(), vec![element.to_owned()])).collect()),
                    Value::Map(map, _) => Value::new_vector_rc(map.iter().map(|(k, v)| {
                        let new_kv = apply(env.clone(), f.clone(), vec![
                            Rc::new(Value::vector_from(vec![
                                k.to_owned(),
                                v.to_owned(),
                            ])),
                        ]);
                        new_kv
                    })
                    .collect::<Vec<_>>()
                ),
                    _ => panic!("map expects a list, vector, set, or map as the second argument, but got: {:?}", coll),
                }
            }),
        )],
    );

    // (defn clojure.core/prn [v & vs])
    // (clojure.core/prn)
    // (clojure.core/prn x)
    // (clojure.core/prn x y)
    // (clojure.core/prn x y z ,,,)
    clojure_core.build_and_bind_function(
        "prn",
        vec![(
            FunctionArity::AtLeast(0),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| {
                let ns = env.get_namespace_or_panic("clojure.core");
                let out = ns.get_value_or_panic("*out*");

                // Extract the WriteHandle's inner Rc in the minimal scope
                let out = out.try_get_handle_ref::<WriteHandle>()
                    .expect(&format!("*out* must be a WriteHandle, but was: {:?}", out))
                    .inner();

                // Now use the Rc without any borrows on the Handle
                let mut out = out.borrow_mut();
                let mut first = true;
                for arg in args.iter() {
                    if !first {
                        write!(out, " ").unwrap();
                    }
                    first = false;
                    write!(out, "{arg}").unwrap();
                }
                writeln!(out).unwrap();

                Value::nil().into()
            }),
        )],
    );

    // (clojure.core/symbol name)
    // (clojure.core/symbol ns_name name)
    clojure_core.build_and_bind_function(
        "symbol",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let name = match args[0].as_ref() {
                    Value::String(name, _) => name.as_ref(),
                    _ => panic!("symbol name must be a string, got {:?}", args[0]),
                };
                Rc::new(Value::symbol_unqualified(name))
            }),
        ), (
            FunctionArity::Exactly(2),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let ns_name = match args[0].as_ref() {
                    Value::String(ns_name, _) => ns_name.as_ref(),
                    _ => panic!("symbol namespace must be a string, got {:?}", args[0]),
                };
                let name = match args[1].as_ref() {
                    Value::String(name, _) => name.as_ref(),
                    _ => panic!("symbol name must be a string, got {:?}", args[1]),
                };
                Rc::new(Value::symbol_qualified(ns_name, name))
            }),
        )],
    );

    // (defn clojure.core/resolve [symbol])
    // (clojure.core/resolve symbol)
    clojure_core.build_and_bind_function(
        "resolve",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| {
                let symbol = match args[0].as_ref() {
                    Value::Symbol(symbol, _) => symbol,
                    _ => panic!("resolve expects a symbol argument, but got: {:?}", args[0]),
                };
                let var = try_resolve(env, symbol).expect(&format!("unable to resolve: {}", symbol));
                Rc::new(Value::var(var))
            }),
        )],
    );

    // (clojure.core/deref var)
    clojure_core.build_and_bind_function(
        "deref",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<Rc<Value>>| {
                let derefee = args.first().unwrap().to_owned();
                match derefee.as_ref() {
                    Value::Var(var, _) => var.deref().expect("attempted to deref unbound Var").clone(),
                    _ => derefee,
                }
            }),
        )],
    );

    // (clojure.core/eval value)
    clojure_core.build_and_bind_function(
        "eval",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| eval(env, args[0].clone())),
        )],
    );

    // (clojure.core/apply f)
    // (clojure.core/apply f args)
    clojure_core.build_and_bind_function(
        "apply",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| apply(env, args[0].clone(), args[1..].to_vec())),
        )],
    );

    clojure_core.build_and_bind_function(
        "list",
        vec![(
            FunctionArity::AtLeast(0),
            Box::new(|_: RcEnvironment, args: Vec<RcValue>| Value::new_list_rc(args)),
        )],
    );

    // TODO: clojure.core/declare macro
    /*
    clojure_core.build_and_bind_function(
        "declare",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnvironment, decl_args: Vec<RcValue>| {
                // unmap all
                let unmap_func = env.get_function_value(("clojure.core", "unmap"));
                let apply_func = env.get_function_rc(("clojure.core", "apply"));

                apply_func.invoke(env.clone(), {
                    let mut apply_args = decl_args.clone();
                    apply_args.insert(0, unmap_func);
                    apply_args
                });

                for name in decl_args.iter() {
                    let name = match name.as_ref() {
                        // Value::String(name, _) => name.to_owned(),
                        Value::Symbol(Symbol::Unqualified(sym), _) => sym.name().to_owned(),
                        _ => break,
                    };
                    // if let Some(var) = env.get_var_in_self_or_ancestors(&SymbolUnqualified::new(name)) {
                    //     if var.is_bound() {
                    //         var.unbind();
                    //     }
                    //     env.remove(&name);
                    // }
                    env.insert(SymbolUnqualified::new(name.as_str()), Var::new_unbound());
                }

                Value::nil().into()
            }),
        )],
    );
    */

    // (clojure.core/all-ns)
    clojure_core.build_and_bind_function(
        "all-ns",
        vec![(
            FunctionArity::Exactly(0),
            Box::new(|env: RcEnvironment, _args: Vec<RcValue>| {
                Value::new_list_rc(
                    env.all_namespaces()
                        .into_iter()
                        .map(|ns| Rc::new(Value::handle(Handle::new(ns))))
                        .collect()
                )
            }),
        )],
    );

    // (clojure.core/ns-map ns-name-symbol)
    // (clojure.core/ns-map (symbol "clojure.core"))
    clojure_core.build_and_bind_function(
        "ns-map",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns_sym = value::optics::view_symbol(args.first().expect("ns-map requires at least one argument: the namespace to map").as_ref())
                    .expect("ns-map first argument must be a symbol naming the namespace to map");
                let ns = env.get_namespace_or_panic(ns_sym.name());
                Rc::new(Value::map_from(
                    ns.entries().into_iter()
                      .map(|(sym_name, var)| (
                        Rc::new(Value::symbol_unqualified(&sym_name)),
                        Rc::new(Value::var(var)),
                    )).collect::<Vec<(_, _)>>()
                ))
            }),
        )],
    );

    // (clojure.core/ns-map-2 (symbol "clojure.core"))
    clojure_core.build_and_bind_function(
        "ns-map-2",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns_name_symbol = args.first()
                    .expect("ns-map requires at least one argument: an unqualified symbol naming the namespace to introspect");
                let ns_name_symbol = value::optics::view_symbol(ns_name_symbol.as_ref())
                    .expect("ns-map's argument must be an unqualified symbol naming the namespace to introspect");
                let ns_name = ns_name_symbol.name();
                let ns = env.get_namespace_or_panic(ns_name);
                Rc::new(Value::map_from(
                    ns.entries().into_iter()
                      .map(|(sym_name, var)| (
                        Rc::new(Value::symbol_qualified(ns_name, &sym_name)),
                        match var.deref() {
                            Some(value) => value.clone(),
                            None => Value::var(var.clone()).into(),
                        }
                    )).collect::<Vec<(_, _)>>()
                ))
            }),
        )],
    );

    // (clojure.core/in-ns sym)
    clojure_core.build_and_bind_function(
        "in-ns",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let sym = value::optics::view_symbol(args[0].as_ref())
                    .expect("in-ns argument must be a symbol");
                let ns = env.create_namespace(sym.name());
                env.get_namespace_or_panic("clojure.core")
                   .bind_value("*ns*", Value::handle(Handle::new(ns.clone())));
                Rc::new(Value::handle(Handle::new(ns)))
            }),
        )],
    );

    // (clojure.core/create-ns sym)
    clojure_core.build_and_bind_function(
        "create-ns",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let sym = value::optics::view_symbol(args[0].as_ref())
                    .expect("create-ns argument must be a symbol");
                let ns = env.create_namespace(sym.name());
                Rc::new(Value::handle(Handle::new(ns)))
            }),
        )],
    );

    // (clojure.core/find-ns sym)
    clojure_core.build_and_bind_function(
        "find-ns",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let sym = value::optics::view_symbol(args[0].as_ref())
                    .expect("find-ns argument must be a symbol");
                match env.try_get_namespace(sym.name()) {
                    Some(ns) => Rc::new(Value::handle(Handle::new(ns))),
                    None => Value::nil_rc(),
                }
            }),
        )],
    );

    // (clojure.core/ns-name ns)
    clojure_core.build_and_bind_function(
        "ns-name",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-name argument must be a namespace handle");
                Rc::new(Value::symbol_unqualified(ns.name_str()))
            }),
        )],
    );

    // (clojure.core/ns-publics ns)
    clojure_core.build_and_bind_function(
        "ns-publics",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-publics argument must be a namespace handle");
                let refers = ns.refers();
                Rc::new(Value::map_from(
                    ns.entries().into_iter()
                        .filter(|(name, _)| !refers.contains_key(&SymbolUnqualified::new(name.as_str())))
                        .map(|(sym_name, var)| (
                            Rc::new(Value::symbol_unqualified(&sym_name)),
                            Rc::new(Value::var(var)),
                        ))
                        .collect::<Vec<_>>(),
                ))
            }),
        )],
    );

    // (clojure.core/ns-imports ns)
    clojure_core.build_and_bind_function(
        "ns-imports",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-imports argument must be a namespace handle");
                Rc::new(Value::map_from(
                    ns.imports().into_iter()
                        .map(|(sym, fqn)| (
                            Rc::new(Value::symbol_unqualified(sym.name())),
                            Rc::new(Value::string(fqn)),
                        ))
                        .collect::<Vec<_>>(),
                ))
            }),
        )],
    );

    // (clojure.core/ns-aliases ns)
    clojure_core.build_and_bind_function(
        "ns-aliases",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-aliases argument must be a namespace handle");
                Rc::new(Value::map_from(
                    ns.aliases().into_iter()
                        .map(|(sym, alias_ns)| (
                            Rc::new(Value::symbol_unqualified(sym.name())),
                            Rc::new(Value::handle(Handle::new(alias_ns))),
                        ))
                        .collect::<Vec<_>>(),
                ))
            }),
        )],
    );

    // (clojure.core/ns-refers ns)
    clojure_core.build_and_bind_function(
        "ns-refers",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-refers argument must be a namespace handle");
                Rc::new(Value::map_from(
                    ns.refers().into_iter()
                        .map(|(sym, var)| (
                            Rc::new(Value::symbol_unqualified(sym.name())),
                            Rc::new(Value::var(var)),
                        ))
                        .collect::<Vec<_>>(),
                ))
            }),
        )],
    );

    // (clojure.core/ns-resolve ns sym)
    clojure_core.build_and_bind_function(
        "ns-resolve",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-resolve first argument must be a namespace handle");
                let sym = value::optics::view_symbol(args[1].as_ref())
                    .expect("ns-resolve second argument must be a symbol");
                match &sym {
                    Symbol::Unqualified(s) => match ns.try_get_var(s.name()) {
                        Ok(var) => Rc::new(Value::var(var)),
                        Err(_) => Value::nil_rc(),
                    },
                    Symbol::Qualified(s) => {
                        match env.try_get_namespace(s.namespace())
                            .and_then(|lookup_ns| lookup_ns.try_get_var(s.name()).ok())
                        {
                            Some(var) => Rc::new(Value::var(var)),
                            None => Value::nil_rc(),
                        }
                    },
                }
            }),
        )],
    );

    // (clojure.core/resolve sym)
    clojure_core.build_and_bind_function(
        "resolve",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let current_ns = env.get_namespace_or_panic("clojure.core")
                    .try_get_handle::<RcNamespace>("*ns*")
                    .expect("*ns* must be a namespace handle");
                let sym = value::optics::view_symbol(args[0].as_ref())
                    .expect("resolve argument must be a symbol");
                match &sym {
                    Symbol::Unqualified(s) => match current_ns.try_get_var(s.name()) {
                        Ok(var) => Rc::new(Value::var(var)),
                        Err(_) => Value::nil_rc(),
                    },
                    Symbol::Qualified(s) => {
                        match env.try_get_namespace(s.namespace())
                            .and_then(|lookup_ns| lookup_ns.try_get_var(s.name()).ok())
                        {
                            Some(var) => Rc::new(Value::var(var)),
                            None => Value::nil_rc(),
                        }
                    },
                }
            }),
        )],
    );

    // (clojure.core/remove-ns sym)
    clojure_core.build_and_bind_function(
        "remove-ns",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let sym = value::optics::view_symbol(args[0].as_ref())
                    .expect("remove-ns argument must be a symbol");
                env.remove_namespace(sym.name());
                Value::nil_rc()
            }),
        )],
    );

    // (clojure.core/ns-unalias ns alias)
    clojure_core.build_and_bind_function(
        "ns-unalias",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-unalias first argument must be a namespace handle");
                let alias = value::optics::view_symbol(args[1].as_ref())
                    .expect("ns-unalias second argument must be a symbol");
                ns.remove_alias(alias.name());
                Value::nil_rc()
            }),
        )],
    );

    // (clojure.core/ns-unmap ns sym)
    clojure_core.build_and_bind_function(
        "ns-unmap",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                let ns = args[0].try_get_handle::<RcNamespace>()
                    .expect("ns-unmap first argument must be a namespace handle");
                let sym = value::optics::view_symbol(args[1].as_ref())
                    .expect("ns-unmap second argument must be a symbol");
                ns.remove_var(sym.name());
                Value::nil_rc()
            }),
        )],
    );

    // (clojure.core/intern ns sym)
    // (clojure.core/intern ns sym val)
    clojure_core.build_and_bind_function(
        "intern",
        vec![
            (
                FunctionArity::Exactly(2),
                Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                    let ns = match args[0].try_get_handle::<RcNamespace>() {
                        Ok(ns) => ns,
                        Err(_) => {
                            let sym = value::optics::view_symbol(args[0].as_ref())
                                .expect("intern first arg must be a namespace handle or symbol");
                            env.get_namespace_or_panic(sym.name())
                        }
                    };
                    let var_sym = value::optics::view_symbol(args[1].as_ref())
                        .expect("intern second argument must be a symbol");
                    let var = match ns.try_get_var(var_sym.name()) {
                        Ok(existing) => existing,
                        Err(_) => {
                            let new_var = Rc::new(Var::new_unbound());
                            ns.insert_var(var_sym.name(), new_var.clone());
                            new_var
                        }
                    };
                    Rc::new(Value::var(var))
                }),
            ),
            (
                FunctionArity::Exactly(3),
                Box::new(|env: RcEnvironment, args: Vec<RcValue>| -> RcValue {
                    let ns = match args[0].try_get_handle::<RcNamespace>() {
                        Ok(ns) => ns,
                        Err(_) => {
                            let sym = value::optics::view_symbol(args[0].as_ref())
                                .expect("intern first arg must be a namespace handle or symbol");
                            env.get_namespace_or_panic(sym.name())
                        }
                    };
                    let var_sym = value::optics::view_symbol(args[1].as_ref())
                        .expect("intern second argument must be a symbol");
                    let var = match ns.try_get_var(var_sym.name()) {
                        Ok(existing) => existing,
                        Err(_) => {
                            let new_var = Rc::new(Var::new_unbound());
                            ns.insert_var(var_sym.name(), new_var.clone());
                            new_var
                        }
                    };
                    var.bind(args[2].clone());
                    Rc::new(Value::var(var))
                }),
            ),
        ],
    );

    // (clojure.core/meta obj)
    clojure_core.build_and_bind_function(
        "meta",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env, args| {
                let obj = args.first().unwrap();
                value::optics::view_meta(obj)
                    .map(Value::map_rc)
                    .unwrap_or_else(Value::nil_rc)
            }),
        )],
    );

    // (clojure.core/with-meta obj meta)
    clojure_core.build_and_bind_function(
        "with-meta",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|_env, args| {
                let mut args = args.into_iter();
                let value = args.next().unwrap();
                let meta = args.next().unwrap();
                let meta = value::optics::view_map(meta.as_ref()).expect("with-meta meta argument must be a map");
                value.with_meta_rc(Some(Rc::new(meta)))
            }),
        )],
    );

    // (clojure.core/get m k)
    // (clojure.core/get m k d)
    clojure_core.build_and_bind_function(
        "get",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|env: RcEnvironment, args: Vec<RcValue>| {
                let mut args = args.into_iter();
                let m = args.next().unwrap();
                if m.is_nil() { return m; }
                let k = args.next().unwrap();
                let d = Value::nil_rc();
                // (clojure.core/get m k nil)
                env.get_namespace_or_panic("clojure.core")
                   .get_function_or_panic("get")
                   .invoke(env.clone(), vec![m, k, d])
            }),
        ), (
            FunctionArity::Exactly(3),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let mut args = args.into_iter();
                let m = args.next().unwrap();
                if m.is_nil() { return m; }
                let k = args.next().unwrap();
                let d = args.next().unwrap();
                match m.as_ref() {
                    Value::Nil(_) => m,
                    Value::Vector(vector, _) => {
                        let expect_message = format!("clojure.core/get vector branch: key is not an integer >= 0: {}", k.as_ref());
                        let k = value::optics::view_integer(k.as_ref()).expect(&expect_message) as usize;
                        vector.get_nth_or(k, d)
                    },
                    Value::Map(map, _) => map.get_or(&k, d),
                    _ => Value::nil_rc(),
                }
            }),
        )],
    );

    // (clojure.core/get-in m ks)
    // (clojure.core/get-in m ks d)
    clojure_core.build_and_bind_function(
        "get-in",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|env, args| {
                let mut args = args.into_iter();
                let m = args.next().unwrap();
                if m.is_nil() { return m; }
                let ks = args.next().unwrap();
                let ks = value::optics::view_vector_ref(ks.as_ref()).unwrap();
                if ks.len() == 0 { return m; }
                let ks = ks.iter().into_iter().map(RcValue::clone);
                let get_fn = env.get_namespace_or_panic("clojure.core").get_function_or_panic("get");
                let mut v = m;
                for k in ks {
                    v = get_fn.invoke(env.clone(), vec![v, k]);
                    if v.is_nil() { return v; }
                }
                v
            })
        )],
    );

    // (clojure.core/assoc m k v & kvs)
    clojure_core.build_and_bind_function(
        "assoc",
        vec![(
            FunctionArity::AtLeast(3),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let m = args[0].to_owned();
                let m = match m.as_ref() {
                    Value::Nil(_) => Rc::new(Value::new_map_empty()),
                    Value::Map(m, meta) => Rc::new(Value::Map(m.clone(), meta.clone())),
                    _ => panic!("assoc expects a map or nil as the first argument"),
                    // TODO: support Vector
                };
                // Validate even number of key-value pairs
                if (args.len() - 1) % 2 != 0 {
                    panic!("assoc expects an even number of key-value arguments");
                }
                match m.as_ref() {
                    Value::Map(map, meta) => {
                        let mut new_map = map.clone();
                        // Apply all key-value pairs
                        for i in (1..args.len()).step_by(2) {
                            let k = args[i].to_owned();
                            let v = args[i + 1].to_owned();
                            new_map.insert(k, v);
                        }
                        Rc::new(Value::Map(
                            new_map,
                            meta.clone(),
                        ))
                    },
                    _ => m,
                }
            }),
        )],
    );

    // (clojure.core/assoc-in m ks v)
    clojure_core.build_and_bind_function(
        "assoc-in",
        vec![(
            FunctionArity::Exactly(3),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let m = args[0].to_owned();
                let ks_arg = args[1].to_owned();
                let v = args[2].to_owned();

                // Extract keys vector and panic if not a vector
                let ks_vec = value::optics::view_vector_ref(ks_arg.as_ref())
                    .expect("assoc-in expects a vector of keys");

                // Convert to Vec for easier indexing
                let ks: Vec<RcValue> = ks_vec.iter().cloned().collect();

                // If keys is empty at the top level, assoc with nil
                if ks.is_empty() {
                    let nil_key = Value::nil_rc();
                    match m.as_ref() {
                        Value::Nil(_) => {
                            let mut map = Map::new_empty();
                            map.insert(nil_key, v);
                            Rc::new(Value::Map(map, None))
                        }
                        Value::Map(map, meta) => {
                            let mut new_map = map.clone();
                            new_map.insert(nil_key, v);
                            Rc::new(Value::Map(new_map, meta.clone()))
                        }
                        Value::Vector(_vec, _) => {
                            // For vectors, nil is not a valid integer key
                            let err_msg = format!("Key must be integer");
                            let _ = value::optics::view_integer(nil_key.as_ref())
                                .expect(&err_msg);
                            unreachable!()
                        }
                        _ => m,
                    }
                } else {
                    // Define recursive helper for processing non-empty key paths
                    fn assoc_in_recursive(
                        m: RcValue,
                        ks: &[RcValue],
                        v: RcValue,
                    ) -> RcValue {
                        if ks.is_empty() {
                            // Base case: no more keys, return the value
                            return v;
                        }
                        // Recursive case: descend with first key, recurse with rest
                        let first_key = ks[0].clone();
                        let rest_keys = &ks[1..];

                        match m.as_ref() {
                            Value::Nil(_) => {
                                // Nil is treated as empty map when descending
                                let empty_map = Rc::new(Value::new_map_empty());
                                let nested = assoc_in_recursive(empty_map, rest_keys, v);
                                let mut result_map = Map::new_empty();
                                result_map.insert(first_key, nested);
                                Rc::new(Value::Map(result_map, None))
                            }
                            Value::Map(map, meta) => {
                                let current = map.get_or_nil(&first_key);
                                let nested = assoc_in_recursive(current, rest_keys, v);
                                let mut new_map = map.clone();
                                new_map.insert(first_key, nested);
                                Rc::new(Value::Map(new_map, meta.clone()))
                            }
                            Value::Vector(vec, meta) => {
                                let err_msg = format!("Key must be integer: {}", first_key.as_ref());
                                let idx = value::optics::view_integer(first_key.as_ref())
                                    .expect(&err_msg) as usize;

                                // Panic if index is out of bounds (matching Babashka behavior)
                                if idx >= vec.len() {
                                    panic!("java.lang.IndexOutOfBoundsException: {}", idx);
                                }

                                let current = vec.get_nth_or_nil(idx);
                                let nested = assoc_in_recursive(current, rest_keys, v);

                                // Build a new vector with the updated value at index
                                let mut vec_items: Vec<RcValue> = vec.iter().cloned().collect();
                                vec_items[idx] = nested;
                                Rc::new(Value::Vector(Vector::from(vec_items), meta.clone()))
                            }
                            _ => m,
                        }
                    }

                    assoc_in_recursive(m, &ks, v)
                }
            }),
        )],
    );

    // (clojure.core/dissoc m k & ks)
    clojure_core.build_and_bind_function(
        "dissoc",
        vec![(
            FunctionArity::AtLeast(2),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let m = args[0].to_owned();
                match m.as_ref() {
                    Value::Map(map, meta) => {
                        let mut new_map = map.clone();
                        // Remove all specified keys
                        for i in 1..args.len() {
                            let k = &args[i];
                            new_map.remove(k);
                        }
                        Rc::new(Value::Map(
                            new_map,
                            meta.clone(),
                        ))
                    },
                    _ => m,
                }
            }),
        )],
    );

    // (clojure.core/keys m)
    clojure_core.build_and_bind_function(
        "keys",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let Value::Map(m, _) = args[0].as_ref() else { unimplemented!() };
                List::new_value_rc(m.keys())
            }),
        )],
    );

    // (clojure.core/vals m)
    clojure_core.build_and_bind_function(
        "vals",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                let Value::Map(m, _) = args[0].as_ref() else { unimplemented!() };
                List::new_value_rc(m.values())
            }),
        )],
    );

    // (clojure.core/first coll)
    clojure_core.build_and_bind_function(
        "first",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _) => list.get_first().unwrap_or_else(Value::nil_rc),
                    Value::Vector(vec, _) => vec.get_first().unwrap_or_else(Value::nil_rc),
                    _ => Value::nil_rc(),
                }
            }),
        )],
    );

    // (clojure.core/second coll)
    clojure_core.build_and_bind_function(
        "second",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _) => list.get_second().unwrap_or_else(Value::nil_rc),
                    Value::Vector(vec, _) => vec.get_second().unwrap_or_else(Value::nil_rc),
                    _ => Value::nil_rc(),
                }
            }),
        )],
    );

    // (clojure.core/last coll)
    clojure_core.build_and_bind_function(
        "last",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnvironment, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _) => list.get_last().unwrap_or_else(Value::nil_rc),
                    Value::Vector(vec, _) => vec.get_last().unwrap_or_else(Value::nil_rc),
                    _ => Value::nil_rc(),
                }
            }),
        )],
    );

    bind_stdioe(
        clojure_core.as_ref(),
        "*in*", || BufReadHandle::new(io::BufReader::new(io::stdin())),
        "*out*", || WriteHandle::new(io::stdout()),
        "*err*", || WriteHandle::new(io::stderr()),
    );

    env
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
fn eval_string(bin_call: &str, args: Vec<String>) {
    let string = args.first().expect(&format!("{} eval-string requires an argument", bin_call));
    let env = create_env();
    let read_output = read_one_v2(env.clone(), string.as_str()).expect("failed to read string");
    let read_value = read_output.1.expect("no value read from string");
    let evaled = eval(env.clone(), read_value);
    println!("{evaled}");
}

fn calculate_offset(outer: &str, inner: &str) -> Option<usize> {
    let outer_start = outer.as_ptr() as usize;
    let inner_start = inner.as_ptr() as usize;
    let outer_end = outer_start + outer.len();

    // Ensure the inner slice is actually contained within the outer slice's memory range
    if inner_start >= outer_start && inner_start <= outer_end {
        // The offset is the difference in memory addresses
        Some(inner_start - outer_start)
    } else {
        None // The slices are not from the same allocation or not nested correctly
    }
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
fn eval_file(
    bin_call: &str,
    args: Vec<String>,
) {
    let file_path = args.first().expect(&format!("{} eval-file requires a file path argument", bin_call));
    let file_contents = std::fs::read_to_string(file_path).expect(&format!("failed to read file: {}", file_path));
    let env = create_env();

    let mut remaining = file_contents.as_str();
    let mut last_result = Value::nil_rc();

    loop {
        remaining = remaining.trim_start();
        if remaining.is_empty() {
            break;
        }

        match read_one_v2(env.clone(), remaining) {
            Ok((next_remaining, Some(value))) => {
                last_result = eval(env.clone(), value);
                remaining = next_remaining;
            }
            Ok((_, None)) => {
                break;
            }
            Err(err) => {
                // eprintln!("Error reading file at position: {}", remaining.len());
                match calculate_offset(&file_contents, remaining) {
                    Some(offset) => eprintln!("Error reading file at byte offset: {}", offset),
                    None => eprintln!("Error reading file: remaining slice is not within original file contents"),
                }
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    // println!("{}", last_result);
    env.get_namespace_or_panic("clojure.core")
       .get_function_or_panic("prn")
       .invoke(env.clone(), vec![last_result]);
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
fn read_string(
    bin_call: &str,
    args: Vec<String>,
) {
    let string = args.first().expect(&format!("{} read-string requires an argument", bin_call));
    let env = create_env();
    let read_output = read_one_v2(env.clone(), string.as_str()).expect("failed to read string");
    let read_value = read_output.1.expect("no value read from string");
    println!("{read_value}");
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
fn demo_optics(
    _bin_call: &str,
    _args: Vec<String>,
) {
    let env = create_env();
    let clojure_core = env.get_namespace_or_panic("clojure.core");
    for (test_input, test_expr, expected) in vec![
        ("[:foo :bar [:baz [:qux]]]", "(get-in *test-input* [0])",     ":foo"),
        ("[:foo :bar [:baz [:qux]]]", "(get-in *test-input* [2])",     "[:baz [:qux]]"),
        ("[:foo :bar [:baz [:qux]]]", "(get-in *test-input* [2 0])",   ":baz"),
        ("[:foo :bar [:baz [:qux]]]", "(get-in *test-input* [2 1])",   "[:qux]"),
        ("[:foo :bar [:baz [:qux]]]", "(get-in *test-input* [2 1 0])", ":qux"),
    ] {
        let test_input = read_one_v2(env.clone(), test_input).unwrap().1.unwrap();
        let test_expr = read_one_v2(env.clone(), test_expr).unwrap().1.unwrap();
        let expected = read_one_v2(env.clone(), expected).unwrap().1.unwrap();
        clojure_core.bind_value_rc("*test-input*", test_input.clone());
        let ret = clojure_core.get_function_or_panic("eval")
            .invoke(env.clone(), vec![test_expr.clone()]);
        use value::optics::meta_ref;

        println!();
        println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
        println!();

        println!("*test-input* ;; => {test_input}");
        match meta_ref(&test_input).as_ref() {
            Some(meta) => println!("(meta *test-input*) ;; => {meta}"),
            None       => println!("(meta *test-input*) ;; => nil"),
        }

        // match input.as_ref() {
        //     Value::List(input, _) => {
        //         for (idx, value) in input.iter().enumerate() {
        //             println!("(get-in *test-input* [{idx}]) ;; => {value}");
        //             match meta_ref(&value).as_ref() {
        //                 Some(meta) => println!("(meta (get-in *test-input* [{idx}])) ;; => {meta}"),
        //                 None       => println!("(meta (get-in *test-input* [{idx}])) ;; => nil"),
        //             }
        //         }
        //     },
        //     Value::Vector(input, _) => {
        //         for (idx, value) in input.iter().enumerate() {
        //             println!("(get-in *test-input* [{idx}]) ;; => {value}");
        //             match meta_ref(&value).as_ref() {
        //                 Some(meta) => println!("(meta (get-in *test-input* [{idx}])) ;; => {meta}"),
        //                 None       => println!("(meta (get-in *test-input* [{idx}])) ;; => nil"),
        //             }
        //         }
        //     },
        //     _ => {},
        // }

        println!();

        println!("*test-expr* ;; => {test_expr}");
        match meta_ref(&test_expr).as_ref() {
            Some(meta) => println!("(meta *test-expr*) ;; => {meta}"),
            None       => println!("(meta *test-expr*) ;; => nil"),
        }

        println!();

        println!("expected ;; => {expected}");
        match meta_ref(&expected).as_ref() {
            Some(meta) => println!("(meta expected) ;; => {meta}"),
            None       => println!("(meta expected) ;; => nil"),
        }

        println!();

        println!("ret ;; => {ret}");
        match meta_ref(&ret).as_ref() {
            Some(meta) => println!("(meta ret) ;; => {meta}"),
            None       => println!("(meta ret) ;; => nil"),
        }

        println!();
        println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
        println!();

        // println!("(let [**est-input* {input}] {test-source}) ;; => (assert= {ret} {expected})");

        // println!("");

        assert_eq!(ret, expected);
    }

    // use value::optics::view_vector_ref;
    // use vector::partials::{get_nth as vector_get_nth};
    // use value::optics::view_list_ref;
    // use list::partials::{get_nth as list_get_nth};
    // let input = args.first().map(AsRef::as_ref).unwrap_or("[:foo :bar (:baz [:qux])]");
    // let input = read_one_v2(Environment::new_empty_rc(), input).unwrap().1.unwrap();
    // let input_0     = view_vector_ref(input.as_ref()).and_then(vector_get_nth(0)).unwrap_or_else(Value::nil_rc);
    // let input_1     = view_vector_ref(input.as_ref()).and_then(vector_get_nth(1)).unwrap_or_else(Value::nil_rc);
    // let input_2     = view_vector_ref(input.as_ref()).and_then(vector_get_nth(2)).unwrap_or_else(Value::nil_rc);
    // let input_3     = view_vector_ref(input.as_ref()).and_then(vector_get_nth(3)).unwrap_or_else(Value::nil_rc);
    // let input_2_0   = view_list_ref(input_2.as_ref()).and_then(list_get_nth(0)).unwrap_or_else(Value::nil_rc);
    // let input_2_1   = view_list_ref(input_2.as_ref()).and_then(list_get_nth(1)).unwrap_or_else(Value::nil_rc);
    // let input_2_1_0 = view_vector_ref(input_2_1.as_ref()).and_then(vector_get_nth(0)).unwrap_or_else(Value::nil_rc);
    // println!("----------------------------------------------------------");
    // println!("input                  ;; => {}", input);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [0])     ;; => {}", input_0);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [1])     ;; => {}", input_1);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [2])     ;; => {}", input_2);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [3])     ;; => {}", input_3);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [2 0])   ;; => {}", input_2_0);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [2 1])   ;; => {}", input_2_1);
    // println!("----------------------------------------------------------");
    // println!("(get-in input [2 1 0]) ;; => {}", input_2_1_0);
    // println!("----------------------------------------------------------");
}

fn bind_stdioe(
    ns: &Namespace,
    in_name: &str,  // *in*
    get_in_handle: impl Fn() -> BufReadHandle,
    out_name: &str, // *out*
    get_out_handle: impl Fn() -> WriteHandle,
    err_name: &str, // *err*
    get_err_handle: impl Fn() -> WriteHandle,
) {
    log::info!("Creating stdin, stdout, and stderr handles.");
    ns.bind_value(in_name, Value::handle(Handle::new(get_in_handle())));
    ns.bind_value(out_name, Value::handle(Handle::new(get_out_handle())));
    ns.bind_value(err_name, Value::handle(Handle::new(get_err_handle())));
    log::info!("Created stdin, stdout, and stderr handles.");
}

#[tracing::instrument(ret, fields(bin_call, args), level = "info")]
fn demo_repl(bin_call: &str, args: Vec<String>) {
    let mut args = args;
    let mut display_startup_messages = true;
    if let Some(first) = args.first() {
        if first == "--help" || first == "-h" || first == "help" {
            usage_repl(&bin_call);
            return;
        } else if first == "--no-startup-messages" {
            display_startup_messages = false;
            args = args.into_iter().skip(1).collect();
        }
        // else {
        //     unimplemented!("unknown argument: {}", first)
        // }
    }

    fn print_welcome_message() {
        println!("Welcome to the CLJX REPL!");
    }

    fn print_help_message() {
        println!("help:");
        println!("  type (help) to print this help message");
        println!("  type (exit) to exit the REPL");
        println!("  type (nsmap) to print a map of vars");
    }

    let env = create_env();
    let clojure_core = env.get_namespace_or_panic("clojure.core");

    clojure_core.bind_value(
        "*command-line-args*",
        List::new_value(args.into_iter().map(Value::string_rc).collect()),
    );

    clojure_core.build_and_bind_function(
        "help",
        vec![(
        FunctionArity::Exactly(0),
        Box::new(|_scope, _args| {
            print_help_message();
            Value::nil_rc()
        }),
    )]);

    if display_startup_messages {
        print_welcome_message();
        print_help_message();
    }

    repl(env);
}

#[tracing::instrument(ret, fields(env), level = "info")]
fn repl(env: RcEnvironment) {
    env.create_namespace("cljx.repl");

    use std::io::Write as _;
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let _stderr = std::io::stderr();

    let mut pending = PendingInput::new_empty();

    write!(stdout, "> ").unwrap();
    loop {
        stdout.flush().unwrap();
        let mut line = String::new();
        stdin.read_line(&mut line).expect("failed to read line");
        if line.is_empty() { write!(stdout, "> ").unwrap(); continue; }

        pending.push(line.as_str());
        if contains_unclosed_delimiter(line.as_str()) { write!(stdout, "> ").unwrap(); continue; }

        let accumulated_input = pending.accumulated_input().to_owned();
        match read_one_v2(env.clone(), accumulated_input.as_str()) {
            Ok(read_output) => {
                let rest_input = read_output.0.to_owned();

                pending.clear();
                pending.push(rest_input.as_str());

                if let Some(value) = read_output.1 {
                    let evaled =
                    env.get_namespace_or_panic("clojure.core")
                       .get_function_or_panic("eval")
                       .invoke(env.clone(), vec![value])
                       ;
                    //let clojure_core = env.get_namespace_or_panic("clojure.core");
                    //let eval_func = clojure_core.get_function_or_panic("eval");
                    //let evaled = eval_func.invoke(env.clone(), vec![value]);
                    writeln!(stdout, "{}", evaled).unwrap();
                }
            },
            Err(err) => {
                writeln!(stdout, "Read error: {:?}", err).unwrap();
                pending.clear();
            }
        }

        if env.try_get_namespace("cljx.repl")
              .and_then(|ns| ns.try_get_value("quit?").ok())
              .map(|v| matches!(v.as_ref(), Value::Boolean(true, _)))
              .unwrap_or(false)
        {
            break;
        }
    }
}

struct PendingInput {
    buf: String,
}

impl PendingInput {
    pub fn new_empty() -> Self {
        Self { buf: String::new() }
    }

    //pub fn new(buf: String) -> Self {
    //    Self { buf }
    //}

    pub fn push(&mut self, line: &str) {
        self.buf += line;
    }

    pub fn clear(&mut self) {
        self.buf = String::new();
    }

    //pub fn is_empty(&self) -> bool {
    //    self.buf.is_empty()
    //}

    pub fn accumulated_input(&self) -> &str {
        self.buf.as_str()
    }
}

fn contains_unclosed_delimiter(s: &str) -> bool {
    let mut paren = 0;
    let mut bracket = 0;
    let mut brace = 0;

    let mut in_string = false;
    let mut escape = false;

    for ch in s.chars() {
        if escape {
            escape = false;
            continue
        }

        if in_string {
            if ch == '\\' {
                escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '(' => paren += 1,
            ')' => paren -= 1,
            '[' => bracket += 1,
            ']' => bracket -= 1,
            '{' => brace += 1,
            '}' => brace -= 1,
            _ => {},
        }
    }

    paren > 0 || bracket > 0 || brace > 0
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use super::*;

    fn ensure_trailing_linefeed<'s>(s: &'s str) -> Cow<'s, str> {
        if s.ends_with("\n") {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(s.to_owned() + "\n")
        }
    }

    #[test]
    fn multi_line_repl_input() {
        // arrange
        let env = Environment::new_empty_rc();
        let mut pending = PendingInput::new_empty();

        let lines = vec![
            "(prn",
            ":hi)",
            "[:a :b]"
        ];
        let mut lines = lines.iter();

        // act
        let mut rest_input = None;
        let mut value = None;
        while let Some(line) = lines.next() {
            pending.push(ensure_trailing_linefeed(line).to_string().as_str());
            if contains_unclosed_delimiter(line) {
                continue;
            }
            let accumulated_input = pending.accumulated_input().to_owned();
            let read_output = read_one_v2(env.clone(), accumulated_input.as_str()).expect("read error");
            let rest_input_1 = read_output.0.to_owned();
            value = read_output.1;
            pending.clear();
            pending.push(rest_input_1.as_str());
            rest_input = Some(rest_input_1);
            break;
        }

        // assert
        assert_eq!(pending.accumulated_input(), "\n");
        assert!(rest_input.is_some());
        assert!(!rest_input.unwrap().is_empty());
        assert!(value.is_some());
        let value = value.unwrap();
        assert!(value.is_list());
    }

    #[test]
    fn try_resolve_qualified_symbol() {
        // arrange
        let env = {
            let env = Environment::new_empty();
            //let ns = env.create_namespace("cljx.cli.tests");
            //env.create_namespace("clojure.core")
            //   .bind_handle("*ns*", Handle::new(ns));
            Rc::new(env)
        };
        let symbol = Symbol::new_qualified("cljx.cli.tests", "my-var");
        // act
        let resolved = try_resolve(env, &symbol);
        // assert
        assert!(resolved.is_err());
    }

    #[test]
    fn try_resolve_unqualified_symbol() {
        // arrange
        let env = {
            let env = Environment::new_empty();
            let ns = env.create_namespace("cljx.cli.tests");
            env.create_namespace("clojure.core")
               .bind_handle("*ns*", Handle::new(ns));
            Rc::new(env)
        };
        let symbol = Symbol::new_unqualified("my-var");
        // act
        let resolved = try_resolve(env, &symbol);
        // assert
        assert!(resolved.is_err());
    }

    #[test]
    fn assoc_add_single_key() {
        // arrange
        let env = create_env();
        let input = "(assoc {:a 1} :b 2)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(1))));
                assert_eq!(m.get(&b_key), Some(Rc::new(Value::integer(2))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_update_existing_key() {
        // arrange
        let env = create_env();
        let input = "(assoc {:a 1 :b 2} :a 10)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(10))));
                assert_eq!(m.get(&b_key), Some(Rc::new(Value::integer(2))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_multiple_kvpairs() {
        // arrange
        let env = create_env();
        let input = "(assoc {:a 1} :b 2 :c 3 :d 4)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                let c_key = Value::keyword_unqualified_rc("c");
                let d_key = Value::keyword_unqualified_rc("d");
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(1))));
                assert_eq!(m.get(&b_key), Some(Rc::new(Value::integer(2))));
                assert_eq!(m.get(&c_key), Some(Rc::new(Value::integer(3))));
                assert_eq!(m.get(&d_key), Some(Rc::new(Value::integer(4))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_mixed_add_and_update() {
        // arrange
        let env = create_env();
        let input = "(assoc {:a 1 :b 2} :a 100 :c 3)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                let c_key = Value::keyword_unqualified_rc("c");
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(100))));
                assert_eq!(m.get(&b_key), Some(Rc::new(Value::integer(2))));
                assert_eq!(m.get(&c_key), Some(Rc::new(Value::integer(3))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_empty_map() {
        // arrange
        let env = create_env();
        let input = "(assoc {} :a 1)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                assert_eq!(m.len(), 1);
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(1))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    #[should_panic(expected = "assoc expects an even number of key-value arguments")]
    fn assoc_odd_arguments() {
        // arrange
        let env = create_env();
        let input = "(assoc {:a 1} :b 2 :c)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act & assert (should panic)
        let _result = eval(env, value);
    }

    #[test]
    fn dissoc_removes_single_key() {
        // arrange
        let env = create_env();
        let input = "(dissoc {:a 1 :b 2} :a)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                assert_eq!(m.len(), 1);
                assert!(!m.contains_key(&a_key));
                assert_eq!(m.get(&b_key), Some(Rc::new(Value::integer(2))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn dissoc_removes_multiple_keys() {
        // arrange
        let env = create_env();
        let input = "(dissoc {:a 1 :b 2 :c 3} :a :c)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let b_key = Value::keyword_unqualified_rc("b");
                let c_key = Value::keyword_unqualified_rc("c");
                assert_eq!(m.len(), 1);
                assert!(!m.contains_key(&a_key));
                assert!(m.contains_key(&b_key));
                assert!(!m.contains_key(&c_key));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn dissoc_on_non_existent_key() {
        // arrange
        let env = create_env();
        let input = "(dissoc {:a 1} :missing)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                assert_eq!(m.len(), 1);
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(1))));
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn dissoc_empty_map() {
        // arrange
        let env = create_env();
        let input = "(dissoc {} :a)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                assert_eq!(m.len(), 0);
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn dissoc_all_keys() {
        // arrange
        let env = create_env();
        let input = "(dissoc {:a 1 :b 2} :a :b)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        // act
        let result = eval(env, value);

        // assert
        match result.as_ref() {
            Value::Map(m, _) => {
                assert_eq!(m.len(), 0);
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_empty_map_nested() {
        let env = create_env();
        let input = "(assoc-in {} [:a :b :c] 42)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let a_val = m.get_or_nil(&a_key);
                match a_val.as_ref() {
                    Value::Map(ab, _) => {
                        let b_key = Value::keyword_unqualified_rc("b");
                        let b_val = ab.get_or_nil(&b_key);
                        match b_val.as_ref() {
                            Value::Map(abc, _) => {
                                let c_key = Value::keyword_unqualified_rc("c");
                                assert_eq!(abc.get(&c_key), Some(Rc::new(Value::integer(42))));
                            }
                            other => panic!("Expected map at [:a :b], got: {:?}", other),
                        }
                    }
                    other => panic!("Expected map at [:a], got: {:?}", other),
                }
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_existing_map() {
        let env = create_env();
        let input = "(assoc-in {:x {:y 1}} [:x :z] 2)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Map(m, _) => {
                let x_key = Value::keyword_unqualified_rc("x");
                let x_val = m.get_or_nil(&x_key);
                match x_val.as_ref() {
                    Value::Map(inner, _) => {
                        let y_key = Value::keyword_unqualified_rc("y");
                        let z_key = Value::keyword_unqualified_rc("z");
                        assert_eq!(inner.get(&y_key), Some(Rc::new(Value::integer(1))));
                        assert_eq!(inner.get(&z_key), Some(Rc::new(Value::integer(2))));
                    }
                    other => panic!("Expected map at [:x], got: {:?}", other),
                }
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_vector() {
        let env = create_env();
        let input = "(assoc-in [1 2 3] [1] 99)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Vector(vec, _) => {
                assert_eq!(vec.get_nth(0), Some(Rc::new(Value::integer(1))));
                assert_eq!(vec.get_nth(1), Some(Rc::new(Value::integer(99))));
                assert_eq!(vec.get_nth(2), Some(Rc::new(Value::integer(3))));
            }
            other => panic!("Expected vector, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_nil_input() {
        let env = create_env();
        let input = "(assoc-in nil [:a :b] 1)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let a_val = m.get_or_nil(&a_key);
                match a_val.as_ref() {
                    Value::Map(inner, _) => {
                        let b_key = Value::keyword_unqualified_rc("b");
                        assert_eq!(inner.get(&b_key), Some(Rc::new(Value::integer(1))));
                    }
                    other => panic!("Expected map at [:a], got: {:?}", other),
                }
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_mixed_map_vector() {
        let env = create_env();
        let input = "(assoc-in {:names [\"foo\" :bar {:qux :zap}]} [:names 2 :whirlwind] 99)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Map(m, _) => {
                let names_key = Value::keyword_unqualified_rc("names");
                let names_val = m.get_or_nil(&names_key);
                match names_val.as_ref() {
                    Value::Vector(vec, _) => {
                        let element_2 = vec.get_nth(2);
                        match element_2 {
                            Some(v) => {
                                match v.as_ref() {
                                    Value::Map(inner, _) => {
                                        let whirlwind_key = Value::keyword_unqualified_rc("whirlwind");
                                        assert_eq!(inner.get(&whirlwind_key), Some(Rc::new(Value::integer(99))));
                                    }
                                    other => panic!("Expected map at index 2, got: {:?}", other),
                                }
                            }
                            None => panic!("Expected element at index 2, got None"),
                        }
                    }
                    other => panic!("Expected vector at [:names], got: {:?}", other),
                }
            }
            other => panic!("Expected map, got: {:?}", other),
        }
    }

    #[test]
    fn assoc_in_empty_path_on_map() {
        let env = create_env();
        let input = "(assoc-in {:a 1} [] 99)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let result = eval(env, value);

        match result.as_ref() {
            Value::Map(m, _) => {
                let a_key = Value::keyword_unqualified_rc("a");
                let nil_key = Value::nil_rc();
                assert_eq!(m.get(&a_key), Some(Rc::new(Value::integer(1))));
                assert_eq!(m.get(&nil_key), Some(Rc::new(Value::integer(99))));
            }
            other => panic!("Expected map with nil key, got: {:?}", other),
        }
    }

    #[test]
    #[should_panic(expected = "Key must be integer")]
    fn assoc_in_empty_path_on_vector() {
        let env = create_env();
        let input = "(assoc-in [:a 1] [] 99)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let _result = eval(env, value);
    }

    #[test]
    #[should_panic(expected = "java.lang.IndexOutOfBoundsException")]
    fn assoc_in_vector_out_of_bounds() {
        let env = create_env();
        let input = "(assoc-in [:a 1] [8] 99)";
        let read_output = read_one_v2(env.clone(), input).expect("failed to read");
        let value = read_output.1.expect("no value read");

        let _result = eval(env, value);
    }

    // create-ns tests
    #[test]
    fn create_ns_creates_new_namespace() {
        let env = create_env();

        // Call create-ns by constructing the symbol and calling through apply
        let sym = Rc::new(Value::symbol_unqualified("my.test.ns"));
        let create_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("create-ns");
        let result = create_ns_func.invoke(env.clone(), vec![sym]);

        // Verify result is a Handle
        assert!(result.is_handle());

        // Verify find-ns can find it
        assert!(env.has_namespace("my.test.ns"));
    }

    #[test]
    fn create_ns_idempotent() {
        let env = create_env();
        let ns = env.create_namespace("idempotent.test");
        ns.insert_var("existing-var", Var::new_bound(Value::integer(99)));

        // Call create-ns
        let sym = Rc::new(Value::symbol_unqualified("idempotent.test"));
        let create_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("create-ns");
        let _result = create_ns_func.invoke(env.clone(), vec![sym]);

        // Verify the var still exists
        let ns_again = env.get_namespace_or_panic("idempotent.test");
        assert!(ns_again.contains_var("existing-var"));
    }

    // in-ns tests
    #[test]
    fn in_ns_creates_new_namespace() {
        let env = create_env();

        let sym = Rc::new(Value::symbol_unqualified("new.ns"));
        let in_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("in-ns");
        let _result = in_ns_func.invoke(env.clone(), vec![sym]);

        assert!(env.has_namespace("new.ns"));
    }

    #[test]
    fn in_ns_updates_star_ns_star() {
        let env = create_env();

        let sym = Rc::new(Value::symbol_unqualified("some.ns"));
        let in_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("in-ns");
        let _result = in_ns_func.invoke(env.clone(), vec![sym]);

        // Get *ns* from clojure.core
        let current_ns = env.get_namespace_or_panic("clojure.core")
            .try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should be a namespace handle");

        assert_eq!(current_ns.name_str(), "some.ns");
    }

    #[test]
    fn in_ns_returns_handle() {
        let env = create_env();

        let sym = Rc::new(Value::symbol_unqualified("handle.test"));
        let in_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("in-ns");
        let result = in_ns_func.invoke(env.clone(), vec![sym]);

        assert!(result.is_handle());
    }

    #[test]
    fn in_ns_does_not_overwrite_existing() {
        let env = create_env();
        let ns = env.create_namespace("preserve.ns");
        ns.insert_var("old-var", Var::new_bound(Value::integer(42)));

        let sym = Rc::new(Value::symbol_unqualified("preserve.ns"));
        let in_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("in-ns");
        let _result = in_ns_func.invoke(env.clone(), vec![sym]);

        let ns_after = env.get_namespace_or_panic("preserve.ns");
        assert!(ns_after.contains_var("old-var"));
    }

    // find-ns tests
    #[test]
    fn find_ns_returns_handle_for_existing() {
        let env = create_env();

        let sym = Rc::new(Value::symbol_unqualified("clojure.core"));
        let find_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("find-ns");
        let result = find_ns_func.invoke(env.clone(), vec![sym]);

        assert!(result.is_handle());
    }

    #[test]
    fn find_ns_returns_nil_for_missing() {
        let env = create_env();

        let sym = Rc::new(Value::symbol_unqualified("nonexistent"));
        let find_ns_func = env.get_namespace_or_panic("clojure.core")
            .get_function_or_panic("find-ns");
        let result = find_ns_func.invoke(env.clone(), vec![sym]);

        assert!(result.is_nil());
    }

    // ns-name tests
    #[test]
    fn ns_name_returns_correct_symbol() {
        let env = create_env();

        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let find_ns_func = clojure_core.get_function_or_panic("find-ns");
        let ns_handle = find_ns_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("clojure.core"))]);

        let ns_name_func = clojure_core.get_function_or_panic("ns-name");
        let result = ns_name_func.invoke(env.clone(), vec![ns_handle]);

        let sym = value::optics::view_symbol(result.as_ref())
            .expect("ns-name should return a symbol");
        assert_eq!(sym.name(), "clojure.core");
    }

    #[test]
    fn ns_name_after_in_ns() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let in_ns_func = clojure_core.get_function_or_panic("in-ns");
        let _in_result = in_ns_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("foo.bar"))]);

        let ns_name_func = clojure_core.get_function_or_panic("ns-name");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_name_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let sym = value::optics::view_symbol(result.as_ref())
            .expect("ns-name should return a symbol");
        assert_eq!(sym.name(), "foo.bar");
    }

    // ns-publics tests
    #[test]
    fn ns_publics_contains_interned_var() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        // Intern a variable
        let intern_func = clojure_core.get_function_or_panic("intern");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let _intern_result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns.clone()))),
            Rc::new(Value::symbol_unqualified("testpub")),
            Rc::new(Value::integer(1)),
        ]);

        // Get ns-publics
        let ns_publics_func = clojure_core.get_function_or_panic("ns-publics");
        let result = ns_publics_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-publics should return a map");
        let key = Rc::new(Value::symbol_unqualified("testpub"));
        assert!(map.get(&key).is_some());
    }

    #[test]
    fn ns_publics_excludes_refers() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let referred_var = Rc::new(Var::new_bound(Value::integer(1)));
        clojure_core.add_refer("referred-var", referred_var);

        let ns_publics_func = clojure_core.get_function_or_panic("ns-publics");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_publics_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-publics should return a map");
        let key = Rc::new(Value::symbol_unqualified("referred-var"));
        assert!(map.get(&key).is_none());
    }

    // ns-imports tests
    #[test]
    fn ns_imports_empty_by_default() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_imports_func = clojure_core.get_function_or_panic("ns-imports");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_imports_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-imports should return a map");
        assert!(map.is_empty());
    }

    #[test]
    fn ns_imports_returns_added_import() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        clojure_core.add_import("MyClass", "com.example.MyClass".to_string());

        let ns_imports_func = clojure_core.get_function_or_panic("ns-imports");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_imports_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-imports should return a map");
        let key = Rc::new(Value::symbol_unqualified("MyClass"));
        let val = map.get(&key).expect("MyClass key should exist");
        let fqn = value::optics::view_string(val.as_ref())
            .expect("value should be a string");
        assert_eq!(fqn, "com.example.MyClass");
    }

    // ns-aliases tests
    #[test]
    fn ns_aliases_empty_by_default() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_aliases_func = clojure_core.get_function_or_panic("ns-aliases");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_aliases_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-aliases should return a map");
        assert!(map.is_empty());
    }

    #[test]
    fn ns_aliases_returns_added_alias() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let other_ns = env.create_namespace("other.ns");
        clojure_core.add_alias("foo", other_ns);

        let ns_aliases_func = clojure_core.get_function_or_panic("ns-aliases");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_aliases_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-aliases should return a map");
        let key = Rc::new(Value::symbol_unqualified("foo"));
        assert!(map.get(&key).is_some());
    }

    // ns-refers tests
    #[test]
    fn ns_refers_empty_by_default() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_refers_func = clojure_core.get_function_or_panic("ns-refers");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_refers_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-refers should return a map");
        assert!(map.is_empty());
    }

    #[test]
    fn ns_refers_returns_added_refer() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let var = Rc::new(Var::new_bound(Value::integer(1)));
        clojure_core.add_refer("bar", var);

        let ns_refers_func = clojure_core.get_function_or_panic("ns-refers");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_refers_func.invoke(env.clone(), vec![Rc::new(Value::handle(Handle::new(current_ns)))]);

        let map = value::optics::view_map(result.as_ref())
            .expect("ns-refers should return a map");
        let key = Rc::new(Value::symbol_unqualified("bar"));
        assert!(map.get(&key).is_some());
    }

    // ns-resolve tests
    #[test]
    fn ns_resolve_unqualified_present() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        // Intern a var
        let intern_func = clojure_core.get_function_or_panic("intern");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let _intern_result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns.clone()))),
            Rc::new(Value::symbol_unqualified("myvar")),
            Rc::new(Value::integer(42)),
        ]);

        // Resolve it
        let ns_resolve_func = clojure_core.get_function_or_panic("ns-resolve");
        let result = ns_resolve_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("myvar")),
        ]);

        assert!(result.is_var());
        match result.as_ref() {
            Value::Var(var, _) => {
                let deref = var.deref().expect("var should be bound");
                let n = value::optics::view_integer(deref.as_ref())
                    .expect("should be an integer");
                assert_eq!(n, 42);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn ns_resolve_unqualified_absent() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_resolve_func = clojure_core.get_function_or_panic("ns-resolve");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_resolve_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("no-such-var")),
        ]);

        assert!(result.is_nil());
    }

    #[test]
    fn ns_resolve_qualified_symbol() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let qvar = Rc::new(Var::new_bound(Value::integer(7)));
        clojure_core.insert_var("qvar", qvar);

        let ns_resolve_func = clojure_core.get_function_or_panic("ns-resolve");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_resolve_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_qualified("clojure.core", "qvar")),
        ]);

        assert!(result.is_var());
    }

    // resolve tests
    #[test]
    fn resolve_present_symbol() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let resolve_func = clojure_core.get_function_or_panic("resolve");
        let result = resolve_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("prn"))]);

        assert!(result.is_var());
    }

    #[test]
    fn resolve_absent_symbol() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let resolve_func = clojure_core.get_function_or_panic("resolve");
        let result = resolve_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("no-such-binding"))]);

        assert!(result.is_nil());
    }

    #[test]
    fn resolve_after_in_ns() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let test_ns = env.create_namespace("test.resolve.ns");
        let local_var = Rc::new(Var::new_bound(Value::integer(99)));
        test_ns.insert_var("ns-local-var", local_var);

        let in_ns_func = clojure_core.get_function_or_panic("in-ns");
        let _in_result = in_ns_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("test.resolve.ns"))]);

        let resolve_func = clojure_core.get_function_or_panic("resolve");
        let result = resolve_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("ns-local-var"))]);

        assert!(result.is_var());
    }

    // remove-ns tests
    #[test]
    fn remove_ns_removes_namespace() {
        let env = create_env();
        env.create_namespace("to.remove");
        assert!(env.has_namespace("to.remove"));

        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let remove_ns_func = clojure_core.get_function_or_panic("remove-ns");
        let _result = remove_ns_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("to.remove"))]);

        assert!(!env.has_namespace("to.remove"));
    }

    #[test]
    fn remove_ns_nonexistent_is_noop() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let remove_ns_func = clojure_core.get_function_or_panic("remove-ns");
        let result = remove_ns_func.invoke(env.clone(), vec![Rc::new(Value::symbol_unqualified("never.existed"))]);

        // Should return nil and not panic
        assert!(result.is_nil());
    }

    // ns-unalias tests
    #[test]
    fn ns_unalias_removes_alias() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let alias_ns = env.create_namespace("alias.target");
        clojure_core.add_alias("myalias", alias_ns);

        let ns_unalias_func = clojure_core.get_function_or_panic("ns-unalias");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let _result = ns_unalias_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("myalias")),
        ]);

        let aliases = clojure_core.aliases();
        assert!(aliases.is_empty());
    }

    #[test]
    fn ns_unalias_nonexistent_is_noop() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_unalias_func = clojure_core.get_function_or_panic("ns-unalias");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_unalias_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("ghost")),
        ]);

        assert!(result.is_nil());
    }

    // ns-unmap tests
    #[test]
    fn ns_unmap_removes_var() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        // Intern a var
        let intern_func = clojure_core.get_function_or_panic("intern");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let _intern_result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns.clone()))),
            Rc::new(Value::symbol_unqualified("to-unmap")),
            Rc::new(Value::integer(5)),
        ]);

        // Unmap it
        let ns_unmap_func = clojure_core.get_function_or_panic("ns-unmap");
        let _unmap_result = ns_unmap_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("to-unmap")),
        ]);

        // Verify it's gone
        assert!(!clojure_core.contains_var("to-unmap"));
    }

    #[test]
    fn ns_unmap_nonexistent_is_noop() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let ns_unmap_func = clojure_core.get_function_or_panic("ns-unmap");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = ns_unmap_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("ghost")),
        ]);

        assert!(result.is_nil());
    }

    // intern tests
    #[test]
    fn intern_2_arity_creates_unbound_var() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let intern_func = clojure_core.get_function_or_panic("intern");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("fresh-var")),
        ]);

        assert!(result.is_var());
        match result.as_ref() {
            Value::Var(var, _) => {
                assert!(var.is_unbound());
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn intern_3_arity_binds_value() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let intern_func = clojure_core.get_function_or_panic("intern");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");
        let result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("bound-var")),
            Rc::new(Value::integer(99)),
        ]);

        assert!(result.is_var());
        match result.as_ref() {
            Value::Var(var, _) => {
                let deref = var.deref().expect("var should be bound");
                let n = value::optics::view_integer(deref.as_ref())
                    .expect("should be an integer");
                assert_eq!(n, 99);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn intern_2_arity_returns_existing_var() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");

        let intern_func = clojure_core.get_function_or_panic("intern");
        let result1 = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns.clone()))),
            Rc::new(Value::symbol_unqualified("same-name")),
        ]);
        let result2 = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("same-name")),
        ]);

        match (result1.as_ref(), result2.as_ref()) {
            (Value::Var(var1, _), Value::Var(var2, _)) => {
                assert!(Rc::ptr_eq(var1, var2));
            }
            _ => panic!("Expected both to be Vars"),
        }
    }

    #[test]
    fn intern_3_arity_rebinds_existing_var() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");
        let current_ns = clojure_core.try_get_handle::<RcNamespace>("*ns*")
            .expect("*ns* should exist");

        let intern_func = clojure_core.get_function_or_panic("intern");
        let _result1 = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns.clone()))),
            Rc::new(Value::symbol_unqualified("rebind-var")),
            Rc::new(Value::integer(1)),
        ]);
        let result2 = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::handle(Handle::new(current_ns))),
            Rc::new(Value::symbol_unqualified("rebind-var")),
            Rc::new(Value::integer(2)),
        ]);

        match result2.as_ref() {
            Value::Var(var, _) => {
                let deref = var.deref().expect("var should be bound");
                let n = value::optics::view_integer(deref.as_ref())
                    .expect("should be an integer");
                assert_eq!(n, 2);
            }
            _ => panic!("Expected Var"),
        }
    }

    #[test]
    fn intern_accepts_symbol_as_first_arg() {
        let env = create_env();
        let clojure_core = env.get_namespace_or_panic("clojure.core");

        let intern_func = clojure_core.get_function_or_panic("intern");
        let result = intern_func.invoke(env.clone(), vec![
            Rc::new(Value::symbol_unqualified("clojure.core")),
            Rc::new(Value::symbol_unqualified("sym-via-sym")),
        ]);

        assert!(result.is_var());
    }
}
