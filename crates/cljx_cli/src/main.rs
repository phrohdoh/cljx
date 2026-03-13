use core::fmt;
use std::{cell::{RefCell, RefMut}, env, io::{self, Stdin}, rc::Rc};
use cljx::{BufReadHandle, Environment, Function, FunctionArity, GetValueError, GetVarError, Handle, IFunction as _, IHandle, List, Map, Namespace, RcEnvironment as RcEnv, RcNamespace, RcValue, RcVar, Symbol, SymbolQualified, SymbolUnqualified, Value, WriteHandle, list_optics, read_one, read_one_v2, value_optics};
// use rustyline::{DefaultEditor, error::ReadlineError};

use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use tracing::{self as log, trace, debug, info, warn, error, span, Level};
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
        if first == "--help" || first == "-h" || first == "help" {
            usage(&bin_call);
            log::info!("END RUN");
            return Ok(());
        } else if first == "repl" {
            // demo_repl_v1(&bin_call, args.into_iter().skip(1).collect::<Vec<_>>());
            log::info!("ante-repl");
            demo_repl(&bin_call, args.into_iter().skip(1).collect::<Vec<_>>());
            log::info!("post-repl");
        } else if first == "eval-string" {
            eval_string(&bin_call, args.into_iter().skip(1).collect::<Vec<_>>());
        } else if first == "eval-file" {
            eval_file(&bin_call, args.into_iter().skip(1).collect::<Vec<_>>());
        } else if first == "optics" {
            demo_optics(&bin_call, args.into_iter().skip(1).collect::<Vec<_>>());
        } else {
            todo!("{:?}", first)
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
    env: RcEnv,
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
    env: RcEnv,
    symbol: &Symbol
) -> RcVar {
    try_resolve(env, symbol)
        .expect(&format!("could not resolve: #'{}", symbol))
}

#[tracing::instrument(ret, fields(env, v), level = "info")]
fn eval(
    env: RcEnv,
    v: RcValue,
) -> RcValue {
    let clojure_core = env.get_namespace_or_panic("clojure.core");
    let eval_func = clojure_core.get_function_or_panic("eval");
    let apply_func = clojure_core.get_function_or_panic("apply");
    match v.as_ref() {
        Value::Nil(_meta) => v.clone(),
        Value::Symbol(symbol, _meta) => resolve_or_panic(env.clone(), symbol).deref().expect(&format!("attempted to deref unbound Var: #'{}", symbol)),
        Value::Keyword(_, _meta) => v.clone(),
        Value::Boolean(_, _meta) => v.clone(),
        Value::Integer(_, _meta) => v.clone(),
        Value::Float(_, _meta) => v.clone(),
        Value::String(_, _meta) => v.clone(),
        Value::List(list, _meta) => {
            if list.is_empty() { return v; }
            let args: Vec<RcValue> = list.iter().skip(1).map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect();
            let v = eval_func.invoke(env.clone(), vec![list.first().unwrap().to_owned()]);
            apply_func.invoke(env.clone(), {
                let mut apply_args = args;
                apply_args.insert(0, v);
                apply_args
            })
        },
        Value::Vector(vector, _meta) => Value::new_vector_rc(vector.iter().map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect()),
        Value::Set(set, _meta) => Value::new_set_rc(set.iter().map(|value| eval_func.invoke(env.clone(), vec![value.to_owned()])).collect()),
        Value::Map(map, _meta) => Value::new_map_rc(map.iter().map(|(k, v)| (eval_func.invoke(env.clone(), vec![k.to_owned()]), eval_func.invoke(env.clone(), vec![v.to_owned()]))).collect()),
        Value::Var(var, _meta) => var.deref().expect("attempted to deref unbound Var"),
        Value::Function(_, _meta) => v.clone(),
        Value::Handle(_, _meta) => v.clone(),
    }
}

#[tracing::instrument(ret, fields(env, f, args), level = "info")]
fn apply(
    env: RcEnv,
    f: RcValue,
    args: Vec<RcValue>,
) -> RcValue {
    match f.as_ref() {
        Value::Function(func, _meta) => func.invoke(env.clone(), args),
        Value::Handle(handle, _meta) => {
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
fn create_env() -> RcEnv {
    let env = Environment::new_empty_rc();

    let clojure_core = env.create_namespace("clojure.core");
    clojure_core.bind_value("*ns*", Value::handle(Handle::new(clojure_core.clone())));

    // (clojure.core/map f coll)
    clojure_core.build_and_bind_function(
        "map",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|env: RcEnv, args: Vec<RcValue>| {
                let f = args[0].clone();
                let coll = args[1].clone();
                match coll.as_ref() {
                    Value::List(list, _meta) => Value::new_list_rc(list.iter().map(|item| apply(env.clone(), f.clone(), vec![item.to_owned()])).collect()),
                    Value::Vector(vector, _meta) => Value::new_vector_rc(vector.iter().map(|item| apply(env.clone(), f.clone(), vec![item.to_owned()])).collect()),
                    Value::Set(set, _meta) => Value::new_set_rc(set.iter().map(|item| apply(env.clone(), f.clone(), vec![item.to_owned()])).collect()),
                    Value::Map(map, _meta) => Value::new_vector_rc(map.iter().map(|(k, v)| {
                        let new_kv = apply(env.clone(), f.clone(), vec![
                            Rc::new(Value::vector_from(vec![
                                k.to_owned(),
                                v.to_owned(),
                            ])),
                        ]);
                        new_kv
                        //let (new_kv, _) = new_kv.try_as_vector().expect(&format!("map function must return a vector of [k v] pairs when mapping over a map, but got: {:?}", new_kv));
                        //let new_k = new_kv[0].to_owned();
                        //let new_v = new_kv[1].to_owned();
                        //(new_k, new_v)
                    })
                    .collect::<Vec<_>>()
                    //.collect::<Vec<(_, _)>>()
                ),
                    _ => panic!("map expects a list, vector, set, or map as the second argument, but got: {:?}", coll),
                }
            }),
        )],
    );

    // (clojure.core/prn)
    // (clojure.core/prn v & vs)
    clojure_core.build_and_bind_function(
        "prn",
        vec![(
            FunctionArity::AtLeast(0),
            Box::new(|env: RcEnv, args: Vec<RcValue>| {
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
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                let name = match args[0].as_ref() {
                    Value::String(name, _) => name,
                    _ => panic!("symbol name must be a string, got {:?}", args[0]),
                };
                Rc::new(Value::symbol_unqualified(name))
            }),
        ), (
            FunctionArity::Exactly(2),
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                let ns_name = match args[0].as_ref() {
                    Value::String(ns_name, _) => ns_name,
                    _ => panic!("symbol namespace must be a string, got {:?}", args[0]),
                };
                let name = match args[1].as_ref() {
                    Value::String(name, _) => name,
                    _ => panic!("symbol name must be a string, got {:?}", args[1]),
                };
                Rc::new(Value::symbol_qualified(ns_name, name))
            }),
        )],
    );

    // (clojure.core/resolve symbol)
    clojure_core.build_and_bind_function(
        "resolve",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|env: RcEnv, args: Vec<RcValue>| {
                let symbol = match args[0].as_ref() {
                    Value::Symbol(symbol, _meta) => symbol,
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
            Box::new(|_env: RcEnv, args: Vec<Rc<Value>>| {
                let derefee = args.first().unwrap().to_owned();
                match derefee.as_ref() {
                    Value::Var(var, _meta) => var.deref().expect("attempted to deref unbound Var").clone(),
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
            Box::new(|env: RcEnv, args: Vec<RcValue>| eval(env, args[0].clone())),
        )],
    );

    // (clojure.core/apply f)
    // (clojure.core/apply f args)
    clojure_core.build_and_bind_function(
        "apply",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnv, args: Vec<RcValue>| apply(env, args[0].clone(), args[1..].to_vec())),
        )],
    );

    clojure_core.build_and_bind_function(
        "list",
        vec![(
            FunctionArity::AtLeast(0),
            Box::new(|_: RcEnv, args: Vec<RcValue>| Value::new_list_rc(args)),
        )],
    );

    // TODO: clojure.core/declare macro
    /*
    clojure_core.build_and_bind_function(
        "declare",
        vec![(
            FunctionArity::AtLeast(1),
            Box::new(|env: RcEnv, decl_args: Vec<RcValue>| {
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
            Box::new(|env: RcEnv, _args: Vec<RcValue>| {
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
            Box::new(|env: RcEnv, args: Vec<RcValue>| -> RcValue {
                let (ns_sym, _) = args.first().expect("ns-map requires at least one argument: the namespace to map")
                    .try_as_symbol().expect("ns-map first argument must be a symbol naming the namespace to map");
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
            Box::new(|env: RcEnv, args: Vec<RcValue>| -> RcValue {
                let (ns_sym, _) = args.first().expect("ns-map requires at least one argument: the namespace to map")
                    .try_as_symbol().expect("ns-map first argument must be a symbol naming the namespace to map");
                let ns = env.get_namespace_or_panic(ns_sym.name());
                Rc::new(Value::map_from(
                    ns.entries().into_iter()
                      .map(|(sym_name, var)| (
                        Rc::new(Value::symbol_qualified(ns_sym.name(), &sym_name)),
                        match var.deref() {
                            Some(value) => value.clone(),
                            None => Value::var(var.clone()).into(),
                        }
                    )).collect::<Vec<(_, _)>>()
                ))
            }),
        )],
    );

    // (clojure.core/get m k)
    // (clojure.core/get m k d)
    clojure_core.build_and_bind_function(
        "get",
        vec![(
            FunctionArity::Exactly(2),
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                let Value::Map(m, _) = args[0].as_ref() else { unimplemented!() };
                let k = &args[1];
                m.get(k).unwrap_or(Value::nil_rc())
            }),
        ), (
            FunctionArity::Exactly(3),
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                let Value::Map(m, _) = args[0].as_ref() else { unimplemented!() };
                let k = &args[1];
                m.get(k).unwrap_or(args[2].clone())
            }),
        )],
    );

    // (clojure.core/keys m)
    clojure_core.build_and_bind_function(
        "keys",
        vec![(
            FunctionArity::Exactly(1),
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
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
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
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
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _meta) => list.first().cloned().unwrap_or(Value::nil_rc()),
                    Value::Vector(vec, _meta) => vec.first().cloned().unwrap_or(Value::nil_rc()),
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
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _meta) => list.second().cloned().unwrap_or(Value::nil_rc()),
                    Value::Vector(vec, _meta) => vec.second().cloned().unwrap_or(Value::nil_rc()),
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
            Box::new(|_env: RcEnv, args: Vec<RcValue>| {
                match args[0].as_ref() {
                    Value::List(list, _meta) => list.last().cloned().unwrap_or(Value::nil_rc()),
                    Value::Vector(vec, _meta) => vec.last().cloned().unwrap_or(Value::nil_rc()),
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
fn eval_file(bin_call: &str, args: Vec<String>) {
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
fn demo_optics(
    _bin_call: &str,
    _args: Vec<String>,
) {
    let env = create_env();
    let (_, value) = read_one_v2(env, "(:foo :bar (:baz (:qux)))").unwrap();
    let value = value.unwrap();
    let first_value = value_optics::view_list_ref(value.as_ref())
        .and_then(list_optics::view_first)
        .unwrap_or_else(|| Rc::new(Value::nil()));
    println!("{}", first_value);
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
fn repl(env: RcEnv) {
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

    pub fn new(buf: String) -> Self {
        Self { buf }
    }

    pub fn push(&mut self, line: &str) {
        self.buf += line;
    }

    pub fn clear(&mut self) {
        self.buf = String::new();
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

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
}
