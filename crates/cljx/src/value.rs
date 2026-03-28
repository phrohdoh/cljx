use ::std::{cell::{Ref, RefMut}, fmt, rc::Rc};
use crate::prelude::*;

mod from;
pub mod optics;

pub type RcValue = Rc<Value>;

#[derive(Hash, Ord, PartialOrd, Eq)]
#[derive(Clone)]
pub enum Value {
    Nil(Option<Rc<Map>>),
    Boolean(bool, Option<Rc<Map>>),
    Integer(i64, Option<Rc<Map>>),
    Float(Float, Option<Rc<Map>>),
    String(String, Option<Rc<Map>>),
    Symbol(Symbol, Option<Rc<Map>>),
    Keyword(Keyword, Option<Rc<Map>>),
    List(List, Option<Rc<Map>>),
    Vector(Vector, Option<Rc<Map>>),
    Set(Set, Option<Rc<Map>>),
    Map(Map, Option<Rc<Map>>),
    Var(RcVar, Option<Rc<Map>>),
    Function(RcFunction, Option<Rc<Map>>),
    Handle(Handle, Option<Rc<Map>>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil(_), Self::Nil(_)) => true,
            (Self::Boolean(lhs, _), Self::Boolean(rhs, _)) => lhs == rhs,
            (Self::Integer(lhs, _), Self::Integer(rhs, _)) => lhs == rhs,
            (Self::Float(lhs, _), Self::Float(rhs, _)) => lhs == rhs,
            (Self::String(lhs, _), Self::String(rhs, _)) => lhs == rhs,
            (Self::Symbol(lhs, _), Self::Symbol(rhs, _)) => lhs == rhs,
            (Self::Keyword(lhs, _), Self::Keyword(rhs, _)) => lhs == rhs,
            (Self::List(lhs, _), Self::List(rhs, _)) => lhs == rhs,
            (Self::Vector(lhs, _), Self::Vector(rhs, _)) => lhs == rhs,
            (Self::Set(lhs, _), Self::Set(rhs, _)) => lhs == rhs,
            (Self::Map(lhs, _), Self::Map(rhs, _)) => lhs == rhs,
            (Self::Var(lhs, _), Self::Var(rhs, _)) => lhs == rhs,
            (Self::Function(lhs, _), Self::Function(rhs, _)) => lhs == rhs,
            (Self::Handle(lhs, _), Self::Handle(rhs, _)) => lhs == rhs,
            _ => false,
        }
    }
}

impl Value {
    pub fn try_as_nil(&self) -> Option<((), &Option<Rc<Map>>)> {
        if let Self::Nil(meta) = self { Some(((), meta)) } else { None }
    }

    pub fn try_as_boolean(&self) -> Option<(bool, &Option<Rc<Map>>)> {
        if let Self::Boolean(boolean, meta) = self { Some((*boolean, meta)) } else { None }
    }

    pub fn try_as_integer(&self) -> Option<(i64, &Option<Rc<Map>>)> {
        if let Self::Integer(integer, meta) = self { Some((*integer, meta)) } else { None }
    }

    pub fn try_as_float(&self) -> Option<(Float, &Option<Rc<Map>>)> {
        if let Self::Float(float, meta) = self { Some((float.clone(), meta)) } else { None }
    }

    pub fn try_as_string(&self) -> Option<(&str, &Option<Rc<Map>>)> {
        if let Self::String(string, meta) = self { Some((string.as_str(), meta)) } else { None }
    }

    pub fn try_as_symbol(&self) -> Option<(&Symbol, &Option<Rc<Map>>)> {
        if let Self::Symbol(symbol, meta) = self { Some((symbol, meta)) } else { None }
    }

    pub fn try_as_keyword(&self) -> Option<(&Keyword, &Option<Rc<Map>>)> {
        if let Self::Keyword(keyword, meta) = self { Some((keyword, meta)) } else { None }
    }

    pub fn try_as_list(&self) -> Option<(&List, &Option<Rc<Map>>)> {
        if let Self::List(list, meta) = self { Some((list, meta)) } else { None }
    }

    pub fn try_as_vector(&self) -> Option<(&Vector, &Option<Rc<Map>>)> {
        if let Self::Vector(vector, meta) = self { Some((vector, meta)) } else { None }
    }

    pub fn try_as_set(&self) -> Option<(&Set, &Option<Rc<Map>>)> {
        if let Self::Set(set, meta) = self { Some((set, meta)) } else { None }
    }

    pub fn try_as_map(&self) -> Option<(&Map, &Option<Rc<Map>>)> {
        if let Self::Map(map, meta) = self { Some((map, meta)) } else { None }
    }

    pub fn try_as_var(&self) -> Option<(&Var, &Option<Rc<Map>>)> {
        if let Self::Var(var, meta) = self { Some((var, meta)) } else { None }
    }

    pub fn try_as_function(&self) -> Option<(&Function, &Option<Rc<Map>>)> {
        if let Self::Function(function, meta) = self { Some((function, meta)) } else { None }
    }

    pub fn try_as_handle(&self) -> Option<(&Handle, &Option<Rc<Map>>)> {
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
        Self::Nil(None)
    }

    pub fn boolean(boolean: bool) -> Self {
        Self::Boolean(boolean, None)
    }

    pub fn integer(integer: i64) -> Self {
        Self::Integer(integer, None)
    }

    pub fn float(float: Float) -> Self {
        Self::Float(float, None)
    }

    pub fn string(string: String) -> Self {
        Self::String(string, None)
    }

    pub fn symbol(symbol: Symbol) -> Self {
        Self::Symbol(symbol, None)
    }

    pub fn symbol_unqualified(name: &str) -> Self {
        Self::Symbol(Symbol::new_unqualified(name), None)
    }

    pub fn symbol_qualified(namespace: &str, name: &str) -> Self {
        Self::Symbol(Symbol::new_qualified(namespace, name), None)
    }

    pub fn keyword(keyword: Keyword) -> Self {
        Self::Keyword(keyword, None)
    }

    pub fn keyword_unqualified(name: &str) -> Self {
        Self::Keyword(Keyword::new_unqualified(name), None)
    }

    pub fn keyword_qualified(namespace: &str, name: &str) -> Self {
        Self::Keyword(Keyword::new_qualified(namespace, name), None)
    }

    pub fn list(list: List) -> Self {
        Self::List(list, None)
    }

    pub fn list_from(items: Vec<RcValue>) -> Self {
        Self::List(List::from(items), None)
    }

    pub fn vector(vector: Vector) -> Self {
        Self::Vector(vector, None)
    }

    pub fn vector_from(items: Vec<RcValue>) -> Self {
        Self::Vector(Vector::from(items), None)
    }

    pub fn set(set: Set) -> Self {
        Self::Set(set, None)
    }

    pub fn set_from(items: Vec<RcValue>) -> Self {
        Self::Set(Set::new(items), None)
    }

    pub fn map(map: Map) -> Self {
        Self::Map(map, None)
    }

    pub fn map_from(pairs: Vec<(RcValue, RcValue)>) -> Self {
        Self::Map(Map::new(pairs), None)
    }

    pub fn var(var: RcVar) -> Self {
        Self::Var(var.clone(), var.meta())
    }

    pub fn function(function: RcFunction) -> Self {
        Self::Function(function, None)
    }

    pub fn handle(handle: Handle) -> Self {
        Self::Handle(handle, None)
    }
}

impl Value {
    pub fn nil_rc() -> RcValue {
        Rc::new(Self::Nil(None))
    }

    pub fn boolean_rc(boolean: bool) -> RcValue {
        Rc::new(Self::Boolean(boolean, None))
    }

    pub fn integer_rc(integer: i64) -> RcValue {
        Rc::new(Self::Integer(integer, None))
    }

    pub fn float_rc(float: Float) -> RcValue {
        Rc::new(Self::Float(float, None))
    }

    pub fn string_rc(string: String) -> RcValue {
        Rc::new(Self::String(string, None))
    }

    pub fn symbol_rc(symbol: Symbol) -> RcValue {
        Rc::new(Self::Symbol(symbol, None))
    }

    pub fn keyword_rc(keyword: Keyword) -> RcValue {
        Rc::new(Self::Keyword(keyword, None))
    }

    pub fn keyword_unqualified_rc(name: &str) -> RcValue {
        Rc::new(Self::Keyword(Keyword::new_unqualified(name), None))
    }

    pub fn keyword_qualified_rc(namespace: &str, name: &str) -> RcValue {
        Rc::new(Self::Keyword(Keyword::new_qualified(namespace, name), None))
    }

    pub fn list_rc(list: List) -> RcValue {
        Rc::new(Self::List(list, None))
    }

    pub fn vector_rc(vector: Vector) -> RcValue {
        Rc::new(Self::Vector(vector, None))
    }

    pub fn set_rc(set: Set) -> RcValue {
        Rc::new(Self::Set(set, None))
    }

    pub fn map_rc(map: Map) -> RcValue {
        Rc::new(Self::Map(map, None))
    }

    pub fn var_rc(var: RcVar) -> RcValue {
        Rc::new(Self::Var(var.clone(), var.meta()))
    }

    pub fn function_rc(function: RcFunction) -> RcValue {
        Rc::new(Self::Function(function, None))
    }

    pub fn handle_rc(handle: Handle) -> RcValue {
        Rc::new(Self::Handle(handle, None))
    }
}

impl Value {
    pub fn into_value_rc(self) -> RcValue {
        Rc::new(self)
    }
}

// List functions
impl Value {
    pub fn with_meta(&self, meta: Option<Rc<Map>>) -> Self {
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

    pub fn with_meta_rc(&self, meta: Option<Rc<Map>>) -> RcValue {
        Rc::new(self.with_meta(meta))
    }

    pub fn new_list_empty() -> Self {
        Self::List(List::new_empty(), None)
    }

    pub fn new_list_empty_rc() -> RcValue {
        Rc::new(Self::List(List::new_empty(), None))
    }

    pub fn new_list(elements: Vec<RcValue>) -> Self {
        Self::List(List::from(elements), None)
    }

    pub fn new_list_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::List(List::from(elements), None))
    }
}

// Vector functions
impl Value {
    pub fn new_vector_empty() -> Self {
        Self::Vector(Vector::new_empty(), None)
    }

    pub fn new_vector_empty_rc() -> RcValue {
        Rc::new(Self::Vector(Vector::new_empty(), None))
    }

    pub fn new_vector(elements: Vec<RcValue>) -> Self {
        Self::Vector(Vector::from(elements), None)
    }

    pub fn new_vector_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::Vector(Vector::from(elements), None))
    }
}

// Set functions
impl Value {
    pub fn new_set_empty() -> Self {
        Self::Set(Set::new_empty(), None)
    }

    pub fn new_set_empty_rc() -> RcValue {
        Rc::new(Self::Set(Set::new_empty(), None))
    }

    pub fn new_set(elements: Vec<RcValue>) -> Self {
        Self::Set(Set::new(elements), None)
    }

    pub fn new_set_rc(elements: Vec<RcValue>) -> RcValue {
        Rc::new(Self::Set(Set::new(elements), None))
    }
}

// Map functions
impl Value {
    pub fn new_map_empty() -> Self {
        Self::Map(Map::new_empty(), None)
    }

    pub fn new_map_empty_rc() -> RcValue {
        Rc::new(Self::Map(Map::new_empty(), None))
    }

    pub fn new_map(elements: Vec<(RcValue, RcValue)>) -> Self {
        Self::Map(Map::new(elements), None)
    }

    pub fn new_map_rc(elements: Vec<(RcValue, RcValue)>) -> RcValue {
        Rc::new(Self::Map(Map::new(elements), None))
    }
}


// #[derive(Debug)]
// pub enum GetFunctionError {
//     ValueIsNotFunction,
// }

#[derive(Debug)]
pub enum GetHandleError {
    IncorrectValueType,
    IncorrectHandleType,
}

impl Value {
    pub fn try_get_handle<T>(self: &'_ Value) -> Result<T, GetHandleError>
    where T: IHandle + Clone + 'static
    {
        value::optics::view_handle(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| handle.downcast_ref::<T>()
                .map(|ref_t| ref_t.to_owned())
                .ok_or(GetHandleError::IncorrectHandleType))
    }

    pub fn try_get_handle_ref<'t, T>(&'t self) -> Result<Ref<'t, T>, GetHandleError>
    where T: IHandle + 'static
    {
        value::optics::view_handle_ref(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| handle.downcast_ref::<T>()
                .ok_or(GetHandleError::IncorrectHandleType))
    }

    pub fn try_get_handle_mut<T>(self: &'_ Value) -> Result<RefMut<'_, T>, GetHandleError>
    where T: IHandle + 'static
    {
        value::optics::view_handle_ref(self)
            .ok_or(GetHandleError::IncorrectValueType)
            .and_then(|handle| handle.downcast_mut::<T>()
                .ok_or(GetHandleError::IncorrectHandleType))
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
            Self::Var(var, _meta)          => write!(f, "#var[{:p}]", RcVar::as_ptr(var).cast::<()>()),
            Self::Function(func, _meta)    => write!(f, "#fn[{addr:p} {name} {arities}]",
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

    fn create_env() -> RcEnvironment {
        let mut env_builder = Environment::builder();
        env_builder.set_current_namespace_var("clojure.core", "*ns*");
        env_builder.insert_namespace(Namespace::new_empty_rc("clojure.core"));
        env_builder.build_rc()
    }

    #[test]
    fn unqualified_keyword_equality() {
        let env = create_env();
        let k1 = read(env.clone(), ":foo").unwrap().1.unwrap();
        let k2 = read(env.clone(), " :foo").unwrap().1.unwrap();
        assert_eq!(k1, k2);
    }
}
