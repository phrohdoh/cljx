
pub use into_value::IntoValue;
pub use into_symbol::IntoSymbol;
pub use into_keyword::IntoKeyword;
pub use into_list::IntoList;
pub use into_vector::IntoVector;
pub use into_set::IntoSet;
pub use into_map::IntoMap;


mod into_value {
    use crate::value::Value;
    use crate::list::List;
    use crate::vector::Vector;
    use crate::set::Set;
    use crate::map::Map;

    pub trait IntoValue {
        fn into_value(self) -> Value;
    }

    impl<T> IntoValue for T where T: Into<Value> {
        fn into_value(self) -> Value {
            self.into()
        }
    }

    impl From<List> for Value {
        fn from(list: List) -> Self {
            Value::List(list)
        }
    }

    impl From<Vector> for Value {
        fn from(vector: Vector) -> Self {
            Value::Vector(vector)
        }
    }

    impl From<Set> for Value {
        fn from(set: Set) -> Self {
            Value::Set(set)
        }
    }

    impl From<Map> for Value {
        fn from(map: Map) -> Self {
            Value::Map(map)
        }
    }

    impl<'a> From<&'a List> for Value {
        fn from(list: &'a List) -> Self {
            Self::List(list.to_owned())
        }
    }

    impl<'a> From<&'a Vector> for Value {
        fn from(vector: &'a Vector) -> Self {
            Self::Vector(vector.to_owned())
        }
    }

    impl<'a> From<&'a Set> for Value {
        fn from(set: &'a Set) -> Self {
            Self::Set(set.to_owned())
        }
    }

    impl<'a> From<&'a Map> for Value {
        fn from(map: &'a Map) -> Self {
            Self::Map(map.to_owned())
        }
    }
}

mod into_symbol {
    use crate::symbol::Symbol;
    use crate::keyword::Keyword;

    pub trait IntoSymbol {
        fn into_symbol(self) -> Symbol;
    }

    impl<T> IntoSymbol for T where T: Into<Symbol> {
        fn into_symbol(self) -> Symbol {
            self.into()
        }
    }

    impl From<Keyword> for Symbol {
        fn from(kw: Keyword) -> Symbol {
            match kw {
                Keyword::Unqualified(kw) => Symbol::Unqualified(kw.into()),
                Keyword::Qualified(kw) => Symbol::Qualified(kw.into()),
            }
        }
    }
}

mod into_keyword {
    use crate::keyword::Keyword;

    pub trait IntoKeyword {
        fn into_keyword(self) -> Keyword;
    }

    impl<T> IntoKeyword for T where T: Into<Keyword> {
        fn into_keyword(self) -> Keyword {
            self.into()
        }
    }
}

mod into_list {
    use crate::convert::IntoValue;
    use crate::list::{List, PersistentList};

    pub trait IntoList {
        fn into_list(self) -> List;
    }

    impl<T> FromIterator<T> for List where T: IntoValue {
        #[tracing::instrument(
            name = "List::from_iter::<T:IntoValue>",
            skip(iter),
            level = "DEBUG",
        )]
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            PersistentList::from_iter(iter.into_iter().map(IntoValue::into_value)).into()
        }
    }
}

mod into_vector {
    use crate::convert::IntoValue;
    use crate::vector::{Vector, PersistentVector};

    pub trait IntoVector {
        fn into_vector(self) -> Vector;
    }

    impl<T> FromIterator<T> for Vector where T: IntoValue {
        #[tracing::instrument(
            name = "Vector::from_iter::<T:IntoValue>",
            skip(iter),
            level = "DEBUG",
        )]
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            PersistentVector::from_iter(iter.into_iter().map(IntoValue::into_value)).into()
        }
    }
}

mod into_set {
    use crate::convert::IntoValue;
    use crate::set::{Set, PersistentSet};

    pub trait IntoSet {
        fn into_set(self) -> Set;
    }

    impl<T> FromIterator<T> for Set where T: IntoValue {
        #[tracing::instrument(
            name = "Set::from_iter::<T:IntoValue>",
            skip(iter),
            level = "DEBUG",
        )]
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            PersistentSet::from_iter(iter.into_iter().map(IntoValue::into_value)).into()
        }
    }
}

mod into_map {
    use crate::convert::IntoValue;
    use crate::map::{Map, PersistentMap};

    pub trait IntoMap {
        fn into_map(self) -> Map;
    }

    impl IntoMap for PersistentMap {
        fn into_map(self) -> Map {
            Map::new(self)
        }
    }

    impl<K, V> FromIterator<(K, V)> for Map where K: IntoValue, V: IntoValue {
        #[tracing::instrument(
            name = "Map::from_iter<(K:IntoValue,V:IntoValue)>",
            skip(iter),
            level = "TRACE",
        )]
        fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
            let mut m = PersistentMap::new();
            iter.into_iter()
                .map(|(k, v)| (k.into_value(), v.into_value()))
                .for_each(|(k, v)| { m.insert_mut(k, v); });
            Map::new(m)
        }
    }
}
