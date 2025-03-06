
mod macros;

pub mod deps;
pub mod utils;

#[path = "cljx/user.rs"]
pub mod user;
#[path = "cljx/core.rs"]
pub mod cljx_core;
#[path = "clojure/core.rs"]
pub mod clojure_core;
#[path = "clojure/edn.rs"]
pub mod clojure_edn;



#[path = "unqualified_symbol.rs"]
mod unqualified_symbol;

#[path = "qualified_symbol.rs"]
mod qualified_symbol;

#[path = "symbol.rs"]
mod symbol;

pub use unqualified_symbol::{
    UnqualifiedSymbol,
    new as new_unqualified_symbol,
};

pub use qualified_symbol::{
    QualifiedSymbol,
    new as new_qualified_symbol,
};

pub use symbol::Symbol;


#[path = "unqualified_keyword.rs"]
mod unqualified_keyword;

#[path = "qualified_keyword.rs"]
mod qualified_keyword;

#[path = "keyword.rs"]
mod keyword;

pub use unqualified_keyword::{
    UnqualifiedKeyword,
    new as new_unqualified_keyword,
};

pub use qualified_keyword::{
    QualifiedKeyword,
    new as new_qualified_keyword,
};

pub use keyword::Keyword;


#[path = "list.rs"]
mod list;

#[path = "vector.rs"]
mod vector;

#[path = "set.rs"]
mod set;

#[path = "map.rs"]
mod map;

#[path = "value.rs"]
mod value;

#[path = "var.rs"]
mod var;

#[path = "rt.rs"]
mod rt;

#[path = "convert.rs"]
mod convert;

#[path = "read.rs"]
mod read;



pub use self::list::{List, PersistentList, IntoList, IPersistentList};
pub use self::vector::{Vector, PersistentVector, IntoVector, IPersistentVector};
pub use self::set::{Set, PersistentSet, IntoSet, IPersistentSet};
pub use self::map::{Map, PersistentMap, IntoMap, IPersistentMap};
pub use self::value::{Value, IntoValue};
pub use self::var::Var;

pub use self::rt::{
    RcValue,
    AFn,
    Intern,
    Resolve,
    env::Env,
    namespace::Namespace,
    // NamespaceExts,
    eval,
    apply,
    check_arity,
    set_current_ns,
    current_ns,
};
pub use self::convert::*;
pub use self::read::*;

pub trait EnvExts {}
