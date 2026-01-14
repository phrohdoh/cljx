use std::{cell::{Ref, RefMut}, fmt, rc::Rc};
use crate::prelude::*;

mod from;
pub mod optics;

pub type RcValue = Rc<Value>;

#[derive(Hash, Ord, PartialOrd, PartialEq, Eq)]
#[derive(Clone)]
pub enum Value {
    Nil(RcMeta),
    Boolean(bool, RcMeta),
    Integer(i64, RcMeta),
    Float(Float, RcMeta),
    String(String, RcMeta),
    Symbol(Symbol, RcMeta),
    Keyword(Keyword, RcMeta),
    List(List, RcMeta),
    Vector(Vector, RcMeta),
    Set(Set, RcMeta),
    Map(Map, RcMeta),
    Var(RcVar, RcMeta),
    Function(RcFunction, RcMeta),
    Handle(Handle, RcMeta),
}

impl Value {
    pub fn try_as_nil(&self) -> Option<((), &RcMeta)> {
        if let Self::Nil(meta) = self { Some(((), meta)) } else { None }
    }

    pub fn try_as_boolean(&self) -> Option<(bool, &RcMeta)> {
        if let Self::Boolean(boolean, meta) = self { Some((*boolean, meta)) } else { None }
    }

    pub fn try_as_integer(&self) -> Option<(i64, &RcMeta)> {
        if let Self::Integer(integer, meta) = self { Some((*integer, meta)) } else { None }
    }

    pub fn try_as_float(&self) -> Option<(Float, &RcMeta)> {
        if let Self::Float(float, meta) = self { Some((float.clone(), meta)) } else { None }
    }

    pub fn try_as_string(&self) -> Option<(&str, &RcMeta)> {
        if let Self::String(string, meta) = self { Some((string.as_str(), meta)) } else { None }
    }

    pub fn try_as_symbol(&self) -> Option<(&Symbol, &RcMeta)> {
        if let Self::Symbol(symbol, meta) = self { Some((symbol, meta)) } else { None }
    }

    pub fn try_as_keyword(&self) -> Option<(&Keyword, &RcMeta)> {
        if let Self::Keyword(keyword, meta) = self { Some((keyword, meta)) } else { None }
    }

    pub fn try_as_list(&self) -> Option<(&List, &RcMeta)> {
        if let Self::List(list, meta) = self { Some((list, meta)) } else { None }
    }

    pub fn try_as_vector(&self) -> Option<(&Vector, &RcMeta)> {
        if let Self::Vector(vector, meta) = self { Some((vector, meta)) } else { None }
    }

    pub fn try_as_set(&self) -> Option<(&Set, &RcMeta)> {
        if let Self::Set(set, meta) = self { Some((set, meta)) } else { None }
    }

    pub fn try_as_map(&self) -> Option<(&Map, &RcMeta)> {
        if let Self::Map(map, meta) = self { Some((map, meta)) } else { None }
    }

    pub fn try_as_var(&self) -> Option<(&Var, &RcMeta)> {
        if let Self::Var(var, meta) = self { Some((var, meta)) } else { None }
    }

    pub fn try_as_function(&self) -> Option<(&Function, &RcMeta)> {
        if let Self::Function(function, meta) = self { Some((function, meta)) } else { None }
    }

    pub fn try_as_handle(&self) -> Option<(&Handle, &RcMeta)> {
        if let Self::Handle(handle, meta) = self { Some((handle, meta)) } else { None }
    }
}

impl Value {
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_, _))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_, _))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_, _))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_, _))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_, _))
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(_, _))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_, _))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, Self::Vector(_, _))
    }

    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(_, _))
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_, _))
    }

    pub fn is_var(&self) -> bool {
        matches!(self, Self::Var(_, _))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_, _))
    }

    pub fn is_handle(&self) -> bool {
        matches!(self, Self::Handle(_, _))
    }
}

impl Value {
    pub fn nil() -> Self {
        Self::Nil(Meta::new_empty_rc())
    }

    pub fn boolean(boolean: bool) -> Self {
        Self::Boolean(boolean, Meta::new_empty_rc())
    }

    pub fn integer(integer: i64) -> Self {
        Self::Integer(integer, Meta::new_empty_rc())
    }

    pub fn float(float: Float) -> Self {
        Self::Float(float, Meta::new_empty_rc())
    }

    pub fn string(string: String) -> Self {
        Self::String(string, Meta::new_empty_rc())
    }

    pub fn symbol(symbol: Symbol) -> Self {
        Self::Symbol(symbol, Meta::new_empty_rc())
    }

    pub fn symbol_unqualified(name: &str) -> Self {
        Self::Symbol(Symbol::new_unqualified(name), Meta::new_empty_rc())
    }

    pub fn symbol_qualified(namespace: &str, name: &str) -> Self {
        Self::Symbol(Symbol::new_qualified(namespace, name), Meta::new_empty_rc())
    }

    pub fn keyword(keyword: Keyword) -> Self {
        Self::Keyword(keyword, Meta::new_empty_rc())
    }

    pub fn keyword_unqualified(name: &str) -> Self {
        Self::Keyword(Keyword::new_unqualified(name), Meta::new_empty_rc())
    }

    pub fn keyword_qualified(namespace: &str, name: &str) -> Self {
        Self::Keyword(Keyword::new_qualified(namespace, name), Meta::new_empty_rc())
    }

    pub fn list(list: List) -> Self {
        Self::List(list, Meta::new_empty_rc())
    }

    pub fn list_from(items: Vec<RcValue>) -> Self {
        Self::List(List::from(items), Meta::new_empty_rc())
    }

    pub fn vector(vector: Vector) -> Self {
        Self::Vector(vector, Meta::new_empty_rc())
    }

    pub fn vector_from(items: Vec<RcValue>) -> Self {
        Self::Vector(Vector::from(items), Meta::new_empty_rc())
    }

    pub fn set(set: Set) -> Self {
        Self::Set(set, Meta::new_empty_rc())
    }

    pub fn set_from(items: Vec<RcValue>) -> Self {
        Self::Set(Set::new(items), Meta::new_empty_rc())
    }

    pub fn map(map: Map) -> Self {
        Self::Map(map, Meta::new_empty_rc())
    }

    pub fn map_from(pairs: Vec<(RcValue, RcValue)>) -> Self {
        Self::Map(Map::new(pairs), Meta::new_empty_rc())
    }

    pub fn var(var: RcVar) -> Self {
        Self::Var(var, Meta::new_empty_rc())
    }

    pub fn function(function: RcFunction) -> Self {
        Self::Function(function, Meta::new_empty_rc())
    }

    pub fn handle(handle: Handle) -> Self {
        Self::Handle(handle, Meta::new_empty_rc())
    }
}

impl Value {
    pub fn nil_rc() -> RcValue {
        Rc::new(Self::Nil(Meta::new_empty_rc()))
    }

    pub fn boolean_rc(boolean: bool) -> RcValue {
        Rc::new(Self::Boolean(boolean, Meta::new_empty_rc()))
    }

    pub fn integer_rc(integer: i64) -> RcValue {
        Rc::new(Self::Integer(integer, Meta::new_empty_rc()))
    }

    pub fn float_rc(float: Float) -> RcValue {
        Rc::new(Self::Float(float, Meta::new_empty_rc()))
    }

    pub fn string_rc(string: String) -> RcValue {
        Rc::new(Self::String(string, Meta::new_empty_rc()))
    }

    pub fn symbol_rc(symbol: Symbol) -> RcValue {
        Rc::new(Self::Symbol(symbol, Meta::new_empty_rc()))
    }

    pub fn keyword_rc(keyword: Keyword) -> RcValue {
        Rc::new(Self::Keyword(keyword, Meta::new_empty_rc()))
    }

    pub fn keyword_unqualified_rc(name: &str) -> RcValue {
        Rc::new(Self::Keyword(Keyword::new_unqualified(name), Meta::new_empty_rc()))
    }

    pub fn keyword_qualified_rc(namespace: &str, name: &str) -> RcValue {
        Rc::new(Self::Keyword(Keyword::new_qualified(namespace, name), Meta::new_empty_rc()))
    }

    pub fn list_rc(list: List) -> RcValue {
        Rc::new(Self::List(list, Meta::new_empty_rc()))
    }

    pub fn vector_rc(vector: Vector) -> RcValue {
        Rc::new(Self::Vector(vector, Meta::new_empty_rc()))
    }

    pub fn set_rc(set: Set) -> RcValue {
        Rc::new(Self::Set(set, Meta::new_empty_rc()))
    }

    pub fn map_rc(map: Map) -> RcValue {
        Rc::new(Self::Map(map, Meta::new_empty_rc()))
    }

    pub fn var_rc(var: RcVar) -> RcValue {
        Rc::new(Self::Var(var, Meta::new_empty_rc()))
    }

    pub fn function_rc(function: RcFunction) -> RcValue {
        Rc::new(Self::Function(function, Meta::new_empty_rc()))
    }

    pub fn handle_rc(handle: Handle) -> RcValue {
        Rc::new(Self::Handle(handle, Meta::new_empty_rc()))
    }
}

impl Value {
    pub fn into_value_rc(self) -> RcValue {
        Rc::new(self)
    }
}

// List functions
impl Value {
    pub fn with_meta(&self, meta: RcMeta) -> Self {
        match self {
            Value::Nil(_)                => Value::Nil(meta),
            Value::Boolean(boolean, _)   => Value::Boolean(boolean.to_owned(), meta),
            Value::Integer(integer, _)   => Value::Integer(integer.to_owned(), meta),
            Value::Float(float, _)       => Value::Float(float.to_owned(), meta),
            Value::String(string, _)     => Value::String(string.to_owned(), meta),
            Value::Symbol(symbol, _)     => Value::Symbol(symbol.to_owned(), meta),
            Value::Keyword(keyword, _)   => Value::Keyword(keyword.to_owned(), meta),
            Value::List(list, _)         => Value::List(list.to_owned(), meta),
            Value::Vector(vector, _)     => Value::Vector(vector.to_owned(), meta),
            Value::Set(set, _)           => Value::Set(set.to_owned(), meta),
            Value::Map(map, _)           => Value::Map(map.to_owned(), meta),
            Value::Var(var, _)           => Value::Var(var.to_owned(), meta),
            Value::Function(function, _) => Value::Function(function.to_owned(), meta),
            Value::Handle(handle, _)     => Value::Handle(handle.to_owned(), meta),
        }
    }

    pub fn with_meta_rc(&self, meta: RcMeta) -> RcValue {
        Rc::new(self.with_meta(meta))
    }

    pub fn new_list_empty() -> Self {
        Self::List(List::new_empty(), Meta::default().into_meta_rc())
    }

    pub fn new_list_empty_rc() -> RcValue {
        Rc::new(Self::List(List::new_empty(), Meta::default().into_meta_rc()))
    }

    pub fn new_list(elements: Vec<RcValue>) -> Self {
        Self::List(List::from(elements), Meta::default().into_meta_rc())
    }

    pub fn new_list_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::List(List::from(elements), Meta::default().into_meta_rc()))
    }
}

// Vector functions
impl Value {
    pub fn new_vector_empty() -> Self {
        Self::Vector(Vector::new_empty(), Meta::default().into_meta_rc())
    }

    pub fn new_vector_empty_rc() -> RcValue {
        Rc::new(Self::Vector(Vector::new_empty(), Meta::default().into_meta_rc()))
    }

    pub fn new_vector(elements: Vec<RcValue>) -> Self {
        Self::Vector(Vector::from(elements), Meta::default().into_meta_rc())
    }

    pub fn new_vector_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::Vector(Vector::from(elements), Meta::default().into_meta_rc()))
    }
}

// Set functions
impl Value {
    pub fn new_set_empty() -> Self {
        Self::Set(Set::new_empty(), Meta::default().into_meta_rc())
    }

    pub fn new_set_empty_rc() -> RcValue {
        Rc::new(Self::Set(Set::new_empty(), Meta::default().into_meta_rc()))
    }

    pub fn new_set(elements: Vec<RcValue>) -> Self {
        Self::Set(Set::new(elements), Meta::default().into_meta_rc())
    }

    pub fn new_set_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::Set(Set::new(elements), Meta::default().into_meta_rc()))
    }
}

// Map functions
impl Value {
    pub fn new_map_empty() -> Self {
        Self::Map(Map::new_empty(), Meta::default().into_meta_rc())
    }

    pub fn new_map_empty_rc() -> RcValue {
        Rc::new(Self::Map(Map::new_empty(), Meta::default().into_meta_rc()))
    }

    pub fn new_map(elements: Vec<(RcValue, RcValue)>) -> Self {
        Self::Map(Map::new(elements), Meta::default().into_meta_rc())
    }

    pub fn new_map_rc(elements: Vec<(RcValue, RcValue)>) -> RcValue {
        Rc::new(Self::Map(Map::new(elements), Meta::default().into_meta_rc()))
    }
}


// #[derive(Debug)]
// pub enum GetFunctionError {
//     ValueIsNotFunction,
// }

#[derive(Debug)]
pub enum GetHandleError {
    ValueIsNotHandle,
    IncorrectHandleType,
}

impl Value {
    pub fn try_get_handle<T>(self: &'_ Value) -> Result<T, GetHandleError>
    where T: IHandle + Clone + 'static
    {
        match self {
            Value::Handle(handle, _) => match handle.downcast_ref::<T>() {
                Some(ref_t) => Ok(ref_t.to_owned()),
                None => Err(GetHandleError::IncorrectHandleType),
            }
            _ => Err(GetHandleError::ValueIsNotHandle),
        }
    }

    pub fn try_get_handle_ref<'t, T>(&'t self) -> Result<Ref<'t, T>, GetHandleError>
    where T: IHandle + 'static
    {
        match self {
            Value::Handle(handle, _) => match handle.downcast_ref::<T>() {
                Some(ref_t) => Ok(ref_t),
                None => Err(GetHandleError::IncorrectHandleType),
            }
            _ => Err(GetHandleError::ValueIsNotHandle),
        }
    }

    pub fn try_get_handle_mut<T>(self: &'_ Value) -> Result<RefMut<'_, T>, GetHandleError>
    where T: IHandle + 'static
    {
        match self {
            Value::Handle(handle, _) => match handle.downcast_mut::<T>() {
                Some(ref_t) => Ok(ref_t),
                None => Err(GetHandleError::IncorrectHandleType),
            }
            _ => Err(GetHandleError::ValueIsNotHandle),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(_meta)              => write!(f, "Value::Nil"),
            Self::Boolean(boolean, _meta) => write!(f, "Value::Boolean({})", boolean),
            Self::Integer(integer, _meta) => write!(f, "Value::Integer({})", integer),
            Self::Float(float, _meta)     => write!(f, "Value::Float({:?})", float),
            Self::String(string, _meta)   => write!(f, "Value::String({:?})", string),
            Self::Symbol(symbol, _meta)   => write!(f, "Value::Symbol({:?})", symbol),
            Self::Keyword(keyword, _meta) => write!(f, "Value::Keyword({:?})", keyword),
            Self::List(list, _meta)       => write!(f, "Value::List({:?})", list),
            Self::Vector(vector, _meta)   => write!(f, "Value::Vector({:?})", vector),
            Self::Set(set, _meta)         => write!(f, "Value::Set({:?})", set),
            Self::Map(map, _meta)         => write!(f, "Value::Map({:?})", map),
            Self::Var(var, _meta)         => write!(f, "Value::Var({:p})", RcVar::as_ptr(var).cast::<()>()),
            Self::Function(func, _meta)   => write!(f, "Value::Function({:?})", func),
            Self::Handle(handle, _meta)   => write!(f, "Value::Handle({:?})", handle),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil(_meta)              => write!(f, "nil"),
            Self::Boolean(boolean, _meta) => write!(f, "{}", boolean),
            Self::Integer(integer, _meta) => write!(f, "{}", integer),
            Self::Float(float, _meta)     => write!(f, "{}", float),
            Self::String(string, _meta)   => write!(f, "\"{}\"", string),
            Self::Symbol(symbol, _meta)   => write!(f, "{}", symbol),
            Self::Keyword(keyword, _meta) => write!(f, "{}", keyword),
            Self::List(list, _meta)       => write!(f, "{}", list),
            Self::Vector(vector, _meta)   => write!(f, "{}", vector),
            Self::Set(set, _meta)         => write!(f, "{}", set),
            Self::Map(map, _meta)         => write!(f, "{}", map),
            Self::Var(var, meta)          => write!(f, "#var[{:p} {meta}]", RcVar::as_ptr(var).cast::<()>()),
            Self::Function(func, meta)    => write!(f, "#fn[{addr:p} {name} {arities} {meta}]",
                addr = RcFunction::as_ptr(func).cast::<()>(),
                name = func.name().map(|s| format!("\"{}\"", s)).unwrap_or("<unnamed>".to_owned()),
                arities = Value::vector_from(
                    func.arity_strings().into_iter().map(|s| Rc::new(Value::string(s))).collect(),
                )),
            Self::Handle(handle, _meta)   => write!(f, "{:?}", handle),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    #[ignore = "TODO: change Value's PartialEq impl to ignore metadata"]
    fn unqualified_keyword_equality() {
        let env = Environment::new_empty_rc();
        let k1 = read_one_v2(env.clone(), ":foo").unwrap().1.unwrap();
        let k2 = read_one_v2(env.clone(), " :foo").unwrap().1.unwrap();
        assert_eq!(k1, k2);
    }
}
