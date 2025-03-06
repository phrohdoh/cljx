
use ::core::{cell::RefCell, fmt};
use ::std::{any::Any, rc::Rc};
use crate::{keyword::Keyword, list::List, map::Map, rt::{namespace::Namespace, AFn}, set::Set, vector::Vector, QualifiedSymbol, Symbol, UnqualifiedKeyword, UnqualifiedSymbol, Var, QualifiedKeyword};
pub use crate::convert::IntoValue;


/// The top-level [Value] type in `cljx`. The type created and passed around at runtime.
#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
    Keyword(Keyword),
    List(List),
    Vector(Vector),
    Set(Set),
    Map(Map),
    Var(Rc<Var>),
    AFn(Rc<dyn AFn>),
    Handle(Rc<dyn Any>),
}

impl<'a> Into<Value> for &'a Value {
    fn into(self) -> Value {
        self.clone()
    }
}


impl Value {
    pub fn boolean(
        boolean: impl Into<bool>,
    ) -> Self {
        Self::Boolean(boolean.into())
    }

    pub fn boolean_true() -> Self {
        Self::Boolean(true)
    }

    pub fn boolean_false() -> Self {
        Self::Boolean(false)
    }


    pub fn integer(
        integer: impl Into<i64>,
    ) -> Self {
        Self::Integer(integer.into())
    }

    pub fn float(
        float: impl Into<f64>,
    ) -> Self {
        Self::Float(float.into())
    }


    pub fn string(
        string: impl ToString,
    ) -> Self {
        Self::String(string.to_string())
    }


    pub fn symbol(
        symbol: impl Into<Symbol>
    ) -> Self {
        Self::Symbol(symbol.into())
    }

    pub fn symbol_unqualified(
        name: UnqualifiedSymbol,
    ) -> Self {
        Self::Symbol(Symbol::from(name))
    }

    pub fn symbol_qualified(
        namespace: UnqualifiedSymbol,
        name: UnqualifiedSymbol,
    ) -> Self {
        Self::Symbol(Symbol::from((namespace, name)))
    }


    pub fn keyword(
        keyword: impl Into<Keyword>
    ) -> Self {
        Self::Keyword(keyword.into())
    }

    pub fn keyword_unqualified(
        name: UnqualifiedKeyword,
    ) -> Self {
        Self::Keyword(Keyword::from(name))
    }

    pub fn keyword_qualified(
        namespace: UnqualifiedKeyword,
        name: UnqualifiedKeyword,
    ) -> Self {
        Self::Keyword(Keyword::from((namespace, name)))
    }


    pub fn list_empty() -> Self {
        Self::List(List::new_empty())
    }

    pub fn list(
        list: impl Into<List>,
    ) -> Self {
        Self::List(list.into())
    }


    pub fn vector_empty() -> Self {
        Self::Vector(Vector::new_empty())
    }

    pub fn vector(
        vector: impl Into<Vector>,
    ) -> Self {
        Self::Vector(vector.into())
    }


    pub fn set_empty() -> Self {
        Self::Set(Set::new_empty())
    }

    pub fn set(
        set: impl Into<Set>,
    ) -> Self {
        Self::Set(set.into())
    }


    pub fn map_empty() -> Self {
        Self::Map(Map::new_empty())
    }

    //pub fn map(
    //    map: impl Into<Map>,
    //) -> Self {
    //    Self::Map(map.into())
    //}
}


impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn is_some(&self) -> bool {
        !self.is_nil()
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(..))
    }

    pub fn is_boolean_true(&self) -> bool {
        *self == Self::Boolean(true)
    }

    pub fn is_boolean_false(&self) -> bool {
        *self == Self::Boolean(false)
    }

    pub fn is_boolean_and(&self, f: impl FnOnce(bool) -> bool) -> bool {
        if let Self::Boolean(b) = self {
            f(*b)
        } else {
            false
        }
    }


    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(..))
    }

    pub fn is_integer_and(&self, f: impl FnOnce(i64) -> bool) -> bool {
        if let Self::Integer(n) = self {
            f(n.to_owned())
        } else {
            false
        }
    }


    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }

    pub fn is_float_and(&self, f: impl FnOnce(f64) -> bool) -> bool {
        if let Self::Float(n) = self {
            f(n.to_owned())
        } else {
            false
        }
    }


    pub fn is_number(&self) -> bool {
        self.is_integer() || self.is_float()
    }


    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(..))
    }

    pub fn is_symbol_unqualified(&self) -> bool {
        matches!(self, Self::Symbol(Symbol::Unqualified(..)))
    }

    pub fn is_symbol_qualified(&self) -> bool {
        matches!(self, Self::Symbol(Symbol::Qualified(..)))
    }

    pub fn is_symbol_and(&self, f: impl FnOnce(&Symbol) -> bool) -> bool {
        if let Self::Symbol(symbol) = self {
            f(symbol)
        } else {
            false
        }
    }

    pub fn is_symbol_unqualified_and(&self, f: impl FnOnce(&UnqualifiedSymbol) -> bool) -> bool {
        if let Self::Symbol(Symbol::Unqualified(symbol)) = self {
            f(symbol)
        } else {
            false
        }
    }

    pub fn is_symbol_qualified_and(&self, f: impl FnOnce(&QualifiedSymbol) -> bool) -> bool {
        if let Self::Symbol(Symbol::Qualified(symbol)) = self {
            f(symbol)
        } else {
            false
        }
    }


    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(..))
    }

    pub fn is_keyword_unqualified(&self) -> bool {
        matches!(self, Self::Keyword(Keyword::Unqualified(..)))
    }

    pub fn is_keyword_qualified(&self) -> bool {
        matches!(self, Self::Keyword(Keyword::Qualified(..)))
    }

    pub fn is_keyword_and(&self, f: impl FnOnce(&Keyword) -> bool) -> bool {
        if let Self::Keyword(keyword) = self {
            f(keyword)
        } else {
            false
        }
    }

    pub fn is_keyword_unqualified_and(&self, f: impl FnOnce(&UnqualifiedKeyword) -> bool) -> bool {
        if let Self::Keyword(Keyword::Unqualified(keyword)) = self {
            f(keyword)
        } else {
            false
        }
    }

    pub fn is_keyword_qualified_and(&self, f: impl FnOnce(&QualifiedKeyword) -> bool) -> bool {
        if let Self::Keyword(Keyword::Qualified(keyword)) = self {
            f(keyword)
        } else {
            false
        }
    }


    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(..))
    }

    pub fn is_list_and(&self, f: impl FnOnce(&List) -> bool) -> bool {
        match self {
            Self::List(list) => f(list),
            _ => false,
        }
    }

    pub fn is_list_nonempty_and(&self, f: impl FnOnce(&List) -> bool) -> bool {
        match self {
            Self::List(list) if !list.is_empty() => f(list),
            _ => false,
        }
    }


    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(..))
    }

    pub fn is_vector_and(&self, f: impl FnOnce(&Vector) -> bool) -> bool {
        match self {
            Self::Vector(vector) => f(vector),
            _ => false,
        }
    }

    pub fn is_vector_nonempty_and(&self, f: impl FnOnce(&Vector) -> bool) -> bool {
        match self {
            Self::Vector(vector) if !vector.is_empty() => f(vector),
            _ => false,
        }
    }


    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(..))
    }

    pub fn is_set_and(&self, f: impl FnOnce(&Set) -> bool) -> bool {
        match self {
            Self::Set(set) => f(set),
            _ => false,
        }
    }

    pub fn is_set_nonempty_and(&self, f: impl FnOnce(&Set) -> bool) -> bool {
        match self {
            Self::Set(set) if !set.is_empty() => f(set),
            _ => false,
        }
    }


    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(..))
    }

    pub fn is_map_and(&self, f: impl FnOnce(&Map) -> bool) -> bool {
        match self {
            Self::Map(map) => f(map),
            _ => false,
        }
    }

    pub fn is_map_nonempty_and(&self, f: impl FnOnce(&Map) -> bool) -> bool {
        match self {
            Self::Map(map) if !map.is_empty() => f(map),
            _ => false,
        }
    }


    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var(..))
    }

    pub fn is_var_and(&self, f: impl FnOnce(Rc<Var>) -> bool) -> bool {
        match self {
            Self::Var(var) => f(var.to_owned()),
            _ => false,
        }
    }
}

impl Value {
    pub fn try_as_integer(&self) -> Result<i64, &Self> {
        match self {
            Self::Integer(i) => Ok(*i),
            _ => Err(self),
        }
    }

    pub fn as_integer(&self) -> i64 {
        self.try_as_integer().unwrap()
    }
}

impl Value {
    pub fn try_as_symbol(&self) -> Result<&Symbol, &Self> {
        match self {
            Self::Symbol(symbol) => Ok(symbol),
            _ => Err(self),
        }
    }

    pub fn as_symbol_panicing(&self) -> &Symbol {
        match self {
            Self::Symbol(symbol) => symbol,
            _ => panic!("{} is not a Symbol", self),
        }
    }

    pub fn as_symbol_mut_panicing(&mut self) -> &mut Symbol {
        match self {
            Self::Symbol(symbol) => symbol,
            _ => panic!("{} is not a Symbol", self),
        }
    }
}

impl Value {
    pub fn try_as_symbol_unqualified(&self) -> Result<&UnqualifiedSymbol, &Self> {
        match self {
            Self::Symbol(Symbol::Unqualified(symbol_unqualified)) => Ok(symbol_unqualified),
            _ => Err(self),
        }
    }

    pub fn as_symbol_unqualified_panicing(&self) -> &UnqualifiedSymbol {
        match self {
            Self::Symbol(Symbol::Unqualified(symbol_unqualified)) => symbol_unqualified,
            _ => panic!("{} is not a UnqualifiedSymbol", self),
        }
    }

    pub fn as_symbol_unqualified_mut_panicing(&mut self) -> &mut UnqualifiedSymbol {
        match self {
            Self::Symbol(Symbol::Unqualified(symbol_unqualified)) => symbol_unqualified,
            _ => panic!("{} is not a UnqualifiedSymbol", self),
        }
    }
}

impl Value {
    pub fn try_as_symbol_qualified(&self) -> Result<&QualifiedSymbol, &Self> {
        match self {
            Self::Symbol(Symbol::Qualified(symbol_qualified)) => Ok(symbol_qualified),
            _ => Err(self),
        }
    }

    pub fn as_symbol_qualified_panicing(&self) -> &QualifiedSymbol {
        match self {
            Self::Symbol(Symbol::Qualified(symbol_qualified)) => symbol_qualified,
            _ => panic!("{} is not a QualifiedSymbol", self),
        }
    }

    pub fn as_symbol_qualified_mut_panicing(&mut self) -> &mut QualifiedSymbol {
        match self {
            Self::Symbol(Symbol::Qualified(symbol_qualified)) => symbol_qualified,
            _ => panic!("{} is not a QualifiedSymbol", self),
        }
    }
}

impl Value {
    pub fn try_as_afn(&self) -> Result<&dyn AFn, &Self> {
        match self {
            Self::AFn(afn) => Ok(afn.as_ref()),
            _ => Err(self),
        }
    }

    pub fn as_afn_panicing(&self) -> &dyn AFn {
        match self {
            Self::AFn(afn) => afn.as_ref(),
            _ => panic!("{} is not an AFn", self),
        }
    }
}

impl Value {
    pub fn try_as_list(&self) -> Result<&List, &Self> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(self),
        }
    }

    pub fn as_list_panicing(&self) -> &List {
        match self {
            Self::List(list) => list,
            _ => panic!("{} is not a List", self),
        }
    }

    pub fn as_list_mut_panicing(&mut self) -> &mut List {
        match self {
            Self::List(list) => list,
            _ => panic!("{} is not a List", self),
        }
    }
}

impl Value {
    pub fn try_into_list(&self) -> Result<List, &Self> {
        match self {
            Self::List(list) => Ok(list.clone()),
            _ => Err(self),
        }
    }

    pub fn into_list_panicing(&self) -> List {
        match self {
            Self::List(list) => list.clone(),
            _ => panic!("{} is not a List", self),
        }
    }
}

impl Value {
    pub fn try_as_vector(&self) -> Result<&Vector, &Self> {
        match self {
            Self::Vector(vector) => Ok(vector),
            _ => Err(self),
        }
    }

    pub fn as_vector_panicing(&self) -> &Vector {
        match self {
            Self::Vector(vector) => vector,
            _ => panic!("{} is not a Vector", self),
        }
    }

    pub fn as_vector_mut_panicing(&mut self) -> &mut Vector {
        match self {
            Self::Vector(vector) => vector,
            _ => panic!("{} is not a Vector", self),
        }
    }
}

impl Value {
    pub fn try_into_vector(&self) -> Result<Vector, &Self> {
        match self {
            Self::Vector(vector) => Ok(vector.clone()),
            _ => Err(self),
        }
    }

    pub fn into_vector_panicing(&self) -> Vector {
        match self {
            Self::Vector(vector) => vector.clone(),
            _ => panic!("{} is not a Vector", self),
        }
    }
}

impl Value {
    pub fn try_as_set(&self) -> Result<&Set, &Self> {
        match self {
            Self::Set(set) => Ok(set),
            _ => Err(self),
        }
    }

    pub fn as_set_panicing(&self) -> &Set {
        match self {
            Self::Set(set) => set,
            _ => panic!("{} is not a Set", self),
        }
    }

    pub fn as_set_mut_panicing(&mut self) -> &mut Set {
        match self {
            Self::Set(set) => set,
            _ => panic!("{} is not a Set", self),
        }
    }
}

impl Value {
    pub fn try_as_map(&self) -> Result<&Map, &Self> {
        match self {
            Self::Map(map) => Ok(map),
            _ => Err(self),
        }
    }

    pub fn as_map_panicing(&self) -> &Map {
        match self {
            Self::Map(map) => map,
            _ => panic!("{} is not a Map", self),
        }
    }

    pub fn as_map_mut_panicing(&mut self) -> &mut Map {
        match self {
            Self::Map(map) => map,
            _ => panic!("{} is not a Map", self),
        }
    }
}

impl Value {
    pub fn try_into_map(&self) -> Result<Map, &Self> {
        match self {
            Self::Map(map) => Ok(map.to_owned()),
            _ => Err(self),
        }
    }

    pub fn into_map_panicing(&self) -> Map {
        match self {
            Self::Map(map) => map.to_owned(),
            _ => panic!("{} is not a Map", self),
        }
    }
}


impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::Nil
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<i64> for Value {
    fn from(integer: i64) -> Self {
        Self::Integer(integer)
    }
}

impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<Symbol> for Value {
    fn from(symbol: Symbol) -> Self {
        Self::Symbol(symbol)
    }
}

impl From<Keyword> for Value {
    fn from(keyword: Keyword) -> Self {
        Self::Keyword(keyword)
    }
}

////////////////////////////////////////////////////////////////////////////////

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil              => write!(f, "nil"),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::Float(float)     => write!(f, "{}", float),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::String(string)   => write!(f, "\"{}\"", string),
            Self::Symbol(symbol)   => write!(f, "{}", symbol),
            Self::Keyword(keyword) => write!(f, "{}", keyword),
            Self::List(list)       => write!(f, "{}", list),
            Self::Vector(vector)   => write!(f, "{}", vector),
            Self::Set(set)         => write!(f, "{}", set),
            Self::Map(map)         => write!(f, "{}", map),


            Self::Var(var)         => {
                match var.deref() {
                    Some(rc_value) => write!(f, "#var[{} \"{:#p}\"]", *rc_value, *var),
                    None           => write!(f, "#unbound-var[\"{:#p}\"]", *var),
                }
            },

            Self::AFn(afn)         => {
                match afn.name() {
                    Some(fn_name) => write!(f, "#fn[{} \"{:#p}\"]", crate::string!(fn_name), *afn),
                    None          => write!(f, "#unnamed-fn[\"{:#p}\"]", *afn),
                }
            },

            Self::Handle(rc_any)   => {
                match rc_any.downcast_ref::<RefCell<Namespace>>().and_then(|ns| ns.try_borrow().ok()) {
                    Some(ns) => {
                        write!(f, "#namespace[{} \"{:#p}\"]", crate::string!(ns.name().name()), *rc_any)
                    },
                    None => write!(f, "#handle[\"{:#p}\"]", *rc_any),
                }
            },


            //Self::Var(var)         => {
            //    match var.deref() {
            //        Some(value) => write!(f, "#object[BoundVar {}]", value),
            //        None        => write!(f, "#object[UnboundVar \"{:#p}\"]", var.as_ref()),
            //    }
            //},

            //Self::AFn(afn)         => {
            //    match afn.name() {
            //        Some(fn_name) => write!(f, "#object[Function {} \"{:#p}\"]", crate::string!(fn_name), afn.as_ref()),
            //        None          => write!(f, "#object[Function \"{:#p}\"]", afn.as_ref()),
            //    }
            //},

            //Self::Handle(rc_any)   => {
            //    match rc_any.downcast_ref::<RefCell<Namespace>>().and_then(|ns| ns.try_borrow().ok()) {
            //        Some(ns) => {
            //            write!(f, "#object[Namespace {} \"{:#p}\"]", crate::string!(ns.name()), rc_any.as_ref())
            //        },
            //        None => write!(f, "#handle[\"{:#p}\"]", rc_any.as_ref()),
            //    }
            //},


        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Nil         => write!(f, "Value::Nil"),
            Self::Integer (x) => f.debug_tuple("Value::Integer").field(x).finish(),
            Self::Float   (x) => f.debug_tuple("Value::Float").field(x).finish(),
            Self::Boolean (x) => f.debug_tuple("Value::Boolean").field(x).finish(),
            Self::String  (x) => f.debug_tuple("Value::String").field(x).finish(),
            Self::Symbol  (x) => f.debug_tuple("Value::Symbol").field(x).finish(),
            Self::Keyword (x) => f.debug_tuple("Value::Keyword").field(x).finish(),
            Self::List    (x) => f.debug_tuple("Value::List").field(x).finish(),
            Self::Vector  (x) => f.debug_tuple("Value::Vector").field(x).finish(),
            Self::Set     (x) => f.debug_tuple("Value::Set").field(x).finish(),
            Self::Map     (x) => f.debug_tuple("Value::Map").field(x).finish(),
            Self::Var     (x) => f.debug_tuple("Value::Var").field(x).finish(),
          //Self::AFn     (x) => f.debug_tuple("Value::AFn").field(x).finish(),
            Self::AFn     (x) => write!(f, "Value::AFn({:#p})", x.as_ref()),
            Self::Handle  (x) => write!(f, "Value::Handle({:#p})", x.as_ref()),
        }
    }
}


mod value_impls {
    mod eq {
        use crate::value::Value;
        impl Eq for Value {}
        impl PartialEq for Value {
            fn eq(&self, other: &Self) -> bool {
                use std::rc::Rc;
                match (self, other) {
                    ( Self::Nil,          Self::Nil          ) => true,
                    ( Self::Boolean (_1), Self::Boolean (_2) ) => _1 == _2,
                    ( Self::Integer (_1), Self::Integer (_2) ) => _1 == _2,
                    ( Self::Float   (_1), Self::Float   (_2) ) => _1 == _2,
                    ( Self::String  (_1), Self::String  (_2) ) => _1 == _2,
                    ( Self::Symbol  (_1), Self::Symbol  (_2) ) => _1 == _2,
                    ( Self::Keyword (_1), Self::Keyword (_2) ) => _1 == _2,
                    ( Self::List    (_1), Self::List    (_2) ) => _1 == _2,
                    ( Self::Vector  (_1), Self::Vector  (_2) ) => _1 == _2,
                    ( Self::Set     (_1), Self::Set     (_2) ) => _1 == _2,
                    ( Self::Map     (_1), Self::Map     (_2) ) => _1 == _2,
                    ( Self::Var     (_1), Self::Var     (_2) ) => Rc::ptr_eq(_1, _2),
                    ( Self::AFn     (_1), Self::AFn     (_2) ) => Rc::ptr_eq(_1, _2),
                    _ => false,
                }
            }
        }
    }

    mod ord {
        use ::core::cmp::Ordering;
        use crate::value::Value;
        impl Ord for Value {
            fn cmp(&self, other: &Self) -> Ordering {
                match (self, other) {
                    ( Self::Nil,          Self::Nil          ) => Ordering::Equal,
                    ( Self::Boolean (_1), Self::Boolean (_2) ) => _1.cmp(_2),
                    ( Self::Integer (_1), Self::Integer (_2) ) => _1.cmp(_2),
                    ( Self::Float   (_1), Self::Float   (_2) ) => if _1 < _2 { Ordering::Less } else if _1 > _2 { Ordering::Greater } else { Ordering::Equal },
                    ( Self::String  (_1), Self::String  (_2) ) => _1.cmp(_2),
                    ( Self::Symbol  (_1), Self::Symbol  (_2) ) => _1.cmp(_2),
                    ( Self::Keyword (_1), Self::Keyword (_2) ) => _1.cmp(_2),
                    ( Self::List    (_1), Self::List    (_2) ) => _1.cmp(_2),
                    ( Self::Vector  (_1), Self::Vector  (_2) ) => _1.cmp(_2),
                    ( Self::Set     (_1), Self::Set     (_2) ) => _1.cmp(_2),
                    ( Self::Map     (_1), Self::Map     (_2) ) => _1.cmp(_2),
                    ( Self::Var     (_1), Self::Var     (_2) ) => _1.cmp(_2),
                    _ => if ::core::mem::discriminant(self) == ::core::mem::discriminant(other) { Ordering::Equal } else { Ordering::Less },
                }
            }
        }
        impl PartialOrd for Value {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
    }

    mod hash {
        use ::core::hash::{Hash, Hasher};
        use crate::value::Value;
        impl Hash for Value {
            fn hash<H: Hasher>(&self, _state: &mut H) {
                // core::mem::discriminant(self).hash(state);
                match self {
                    Self::Nil         => todo!(),
                    Self::Boolean (_) => todo!(),
                    Self::Integer (_) => todo!(),
                    Self::Float   (_) => todo!(),
                    Self::String  (_) => todo!(),
                    Self::Symbol  (_) => todo!(),
                    Self::Keyword (_) => todo!(),
                    Self::List    (_) => todo!(),
                    Self::Vector  (_) => todo!(),
                    Self::Set     (_) => todo!(),
                    Self::Map     (_) => todo!(),
                    Self::Var     (_) => todo!(),
                    Self::AFn     (_) => todo!(),
                    Self::Handle  (_) => todo!(),
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    #[test]
    fn nil() {
        assert_eq!(crate::nil!(), crate::nil!());
        assert_ne!(crate::nil!(), crate::boolean!(true));
        assert_ne!(crate::nil!(), crate::boolean!(false));
    }

    #[test]
    fn boolean() {
        assert_eq!(crate::boolean!(true), crate::boolean!(true));
        assert_eq!(crate::boolean!(false), crate::boolean!(false));
        assert_ne!(crate::boolean!(true), crate::boolean!(false));
    }

    #[test]
    fn integer() {
        assert_eq!(crate::integer!(123), crate::integer!(123));
        assert_ne!(crate::integer!(123), crate::integer!(456));
    }

    #[test]
    fn float() {
        assert_eq!(crate::float!(123.0), crate::float!(123.0));
        assert_ne!(crate::float!(123.0), crate::float!(456.0));
    }

    #[test]
    fn number() {
        // assert_eq!(crate::integer!(123), crate::float!(123.0));
        assert_ne!(crate::integer!(123), crate::float!(123.0));
        assert_ne!(crate::integer!(123), crate::float!(456.0));
    }

    #[test]
    fn string() {
        assert_eq!(crate::string!("abc"), crate::string!("abc"));
        assert_ne!(crate::string!("abc"), crate::string!("xyz"));
    }

    #[test]
    fn symbol_unqualified() {
        assert_eq!(crate::symbol!("abc"), crate::symbol!("abc"));
        assert_ne!(crate::symbol!("abc"), crate::symbol!("xyz"));
    }

    #[test]
    fn symbol_qualified() {
        assert_eq!(crate::symbol!("abc", "xyz"), crate::symbol!("abc", "xyz"));
        assert_ne!(crate::symbol!("abc", "xyz"), crate::symbol!("xyz", "xyz"));
    }

    #[test]
    fn keyword_unqualified() {
        assert_eq!(crate::keyword!("abc"), crate::keyword!("abc"));
        assert_ne!(crate::keyword!("abc"), crate::keyword!("xyz"));
    }

    #[test]
    fn keyword_qualified() {
        assert_eq!(crate::keyword!("abc", "xyz"), crate::keyword!("abc", "xyz"));
        assert_ne!(crate::keyword!("abc", "xyz"), crate::keyword!("xyz", "xyz"));
    }

}
