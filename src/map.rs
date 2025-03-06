
use ::core::fmt::{self, Debug, Display};
use crate::deps::tracing;
use crate::deps::rpds::map::red_black_tree_map as rpds_map;
use crate::deps::archery::RcK;
use crate::value::Value;
use crate::vector::Vector;
pub use crate::convert::IntoMap;

pub type Entries <'m, P> = rpds_map::IterEntry<'m, Value, Value, P>;
// pub type Keys    <'m, P> = ::core::iter::Map<core::iter::Map<Entries<'m, P>, fn((&'m Value, &'m Value)) -> (&'m Value, ())>, fn((&'m Value, ())) -> &'m Value>;
// pub type Values  <'m, P> = ::core::iter::Map<core::iter::Map<Entries<'m, P>, fn((&'m Value, &'m Value)) -> ((), &'m Value)>, fn(((), &'m Value)) -> &'m Value>;
pub type Keys    <'m, P> = ::core::iter::Map<Entries<'m, P>, fn((&'m Value, &'m Value)) -> &'m Value>;
pub type Values  <'m, P> = ::core::iter::Map<Entries<'m, P>, fn((&'m Value, &'m Value)) -> &'m Value>;


pub type PersistentMap = rpds::RedBlackTreeMap<Value, Value>;


pub trait IPersistentMap {
    fn len(&self) -> usize;

    fn entries(&self) -> Entries<'_, RcK>;
    fn keys(&self) -> Keys<'_, RcK>;
    fn values(&self) -> Values<'_, RcK>;

    fn contains_key(&self, key: &Value) -> bool;

    fn map_entries<'m>(&'m self, map_fn: &'_ dyn Fn((&'m Value, &'m Value)) -> (Value, Value)) -> Map;
    fn map_keys<'m>(&'m self, map_fn: &'_ dyn Fn(&'m Value) -> Value) -> Map;
    fn map_values<'m>(&'m self, map_fn: &'_ dyn Fn(&'m Value) -> Value) -> Map;

    fn map_entries_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn((&'_ Value, &'_ Value)) -> (Value, Value),
    );

    fn map_keys_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn(&'_ Value) -> Value,
    );

    fn map_values_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn(&'_ Value) -> Value,
    );
}

impl IPersistentMap for Map {
    fn len(&self) -> usize {
        self.0.size()
    }


    fn entries<'m>(&'m self) -> Entries<'m, RcK> {
        self.0.iter_entries()
    }

    fn keys<'m>(&'m self) -> Keys<'m, RcK> {
        self.0.iter_entries().map(|(k, _)| k)
    }

    fn values<'m>(&'m self) -> Values<'m, RcK> {
        self.0.iter_entries().map(|(_, v)| v)
    }


    fn contains_key(
        &self,
        key: &Value,
    ) -> bool {
        self.0.contains_key(key)
    }


    fn map_entries<'m>(
        &'m self,
        map_fn: &'_ dyn Fn((&'m Value, &'m Value)) -> (Value, Value),
    ) -> Map {
        self.0.iter()
            .map(map_fn)
            .collect()
    }

    fn map_keys<'m>(
        &'m self,
        map_fn: &'_ dyn Fn(&'m Value) -> Value,
    ) -> Map {
        self.0.iter()
            .map(|(k, v)| (map_fn(k), v.to_owned()))
            .collect()
    }

    fn map_values<'m>(
        &'m self,
        map_fn: &'_ dyn Fn(&'m Value) -> Value,
    ) -> Map {
        self.0.iter()
            .map(|(k, v)| (k.to_owned(), map_fn(v)))
            .collect()
    }

    fn map_entries_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn((&'_ Value, &'_ Value)) -> (Value, Value),
    ) {
        let entries = self.entries()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<Vec<_>>();

        for (key, value) in entries {
            let (mut_key, mut_value) = self.0.get_entry_mut(&key).unwrap();
            (*mut_key, *mut_value) = map_fn((&key, &value));
        }
    }

    fn map_keys_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn(&'_ Value) -> Value,
    ) {
        let keys = self.keys()
            .map(|k| k.to_owned())
            .collect::<Vec<_>>();

        for key in keys {
            *self.0.get_key_mut(&key).unwrap() = map_fn(&key);
        }
    }

    fn map_values_mut<'m>(
        &'m mut self,
        map_fn: &'_ dyn Fn(&'_ Value) -> Value,
    ) {
        let keys = self.keys()
            .map(|k| k.to_owned())
            .collect::<Vec<_>>();

        for key in keys {
            *self.0.get_mut(&key).unwrap() = map_fn(&key);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//pub trait IPersistentMapMut {
//    fn map_keys_mut(&mut self, map_fn: &dyn crate::rt::AFn);
//    fn map_vals_mut(&mut self, map_fn: &dyn crate::rt::AFn);
//    fn map_entries_mut(&mut self, map_fn: &dyn crate::rt::AFn);
//    // fn map_keys_mut<F>(&mut self, map_fn: F) where F: Fn(RcValue) -> RcValue;
//    // fn map_vals_mut<F>(&mut self, map_fn: F) where F: Fn(RcValue) -> RcValue;
//    // fn map_entries_mut<F>(&mut self, map_fn: F) where F: Fn((RcValue, RcValue)) -> (RcValue, RcValue);
//
//    fn assoc_mut(&mut self, key: RcValue, val: RcValue);
//    fn dissoc_mut(&mut self, key: RcValue);
//}

////////////////////////////////////////////////////////////////////////////////


//#[nutype(
//    derive(AsRef, Clone, Deref, PartialEq, Eq, PartialOrd, Ord, Hash),
//    new_unchecked,
//)]
/// An unordered map of [Value] to [Value].
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Map(PersistentMap);

//impl ::core::ops::Deref for Map {
//    type Target = PersistentMap;
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}

////////////////////////////////////////////////////////////////////////////////

impl Map {
    pub fn new_empty() -> Self {
        Self(PersistentMap::new())
    }

    pub fn new(map: PersistentMap) -> Self {
        Self(map)
    }

    #[inline]
    // #[tracing::instrument(name = "Map::len", level = "TRACE")]
    pub fn len(&self) -> usize {
        self.0.size()
    }

    #[inline]
    // #[tracing::instrument(name = "Map::is_empty", level = "TRACE")]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[tracing::instrument(
        name = "Map::get",
        skip(self, key),
        fields(
            map.keys = %self.keys().collect::<Vector>(),
            key = %key,
            // retval,
        ),
        level = "DEBUG",
    )]
    pub fn get<'k>(&self, key: &'k Value) -> Option<&Value> {
        let ret = self.0.get(key);
        //if let Some(ret) = ret {
        //    crate::deps::tracing::Span::current().record("retval", format!("foobar-{}", ret));
        //}
        ret
    }

    #[tracing::instrument(
        name = "Map::get_or",
        skip(self, key, or),
        fields(
            map.keys = %self.keys().collect::<Vector>(),
            key = %key,
            or = %or,
            // ret,
        ),
        // ret(Display),
        level = "DEBUG",
    )]
    pub fn get_or<'m, 'k, 'o: 'm>(&'m self, key: &'k Value, or: &'o Value) -> &'m Value {
        self.0.get(key).unwrap_or(or)
        // self.get(key).unwrap_or(or)
    }

    #[tracing::instrument(
        name = "Map::get_or_nil",
        skip(self, key),
        fields(
            map.keys = %self.keys().collect::<Vector>(),
            key = %key,
            // ret,
        ),
        // ret(Display),
        level = "DEBUG",
    )]
    pub fn get_or_nil<'k>(&self, key: &'k Value) -> &Value {
        self.0.get(key).unwrap_or(&Value::Nil)
        // self.get_or(key, &Value::Nil)
    }

    #[tracing::instrument(
        name = "Map::get_or_else",
        skip(self, key, or_else),
        fields(
            map.keys = %self.keys().collect::<Vector>(),
            key = %key,
            or_else,
            // ret,
        ),
        // ret(Display),
        level = "DEBUG",
    )]
    pub fn get_or_else<'fr, 'm: 'fr, 'k, F: Fn() -> &'fr Value>(&'m self, key: &'k Value, or_else: F) -> &'fr Value {
        self.0.get(key).unwrap_or_else(|| {
            let or = or_else();
            // crate::deps::tracing::Span::current().record("or_else", format!("{}", or));
            or
        })
    }
}

////////////////////////////////////////////////////////////////////////////////

impl Display for Map {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        use crate::deps::itertools::Itertools as _;
        write!(f, "{{{}}}",
            self.0.iter()
                .map(|(k, v)| format!("{k} {v}"))
                .join(", "))
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn empty_get() {
        use crate::value::Value;
        let m = crate::map!();
        let m = m.as_map_panicing();
        let v = m.get(&Value::Nil);
        assert!(v.is_none());
    }

    #[test]
    fn multiple_entries_get() {
        fn _test() {
            let k1 = crate::keyword!("k1");
            let v1 = crate::keyword!("v1");

            let k2 = crate::keyword!("k2");
            let v2 = crate::keyword!("v2");

            let m = crate::map!(
                (k1.clone(), v1.clone()),
                (k2.clone(), v2.clone()),
            );
            let m = m.as_map_panicing();

            let o1 = m.get(&k1).expect(&format!("expect map {m} to have key {k1}"));
            assert_eq!(o1, &v1);

            let o2 = m.get(&k2).expect(&format!("expect map {m} to have key {k2}"));
            assert_eq!(o2, &v2);
        }

        let num_iters = 1; // 10, 50, 100, ...
        for _ in 1..=num_iters {
            _test();
        }
    }

    #[test]
    #[ignore = "temporarily prefer get_entry_with_nil_key"]
    fn get_with_key_from_within_self() {
        fn _test() {
            let k1 = crate::value::Value::Nil;
            let k2 = crate::keyword!("k2");

            let v1 = k2.clone();
            let v2 = crate::keyword!("v2");

            let m = crate::map!(
                (k1.clone(), v1.clone()),
                (k2.clone(), v2.clone()),
            );
            let m = m.as_map_panicing();

            let o1 = m.get(&k1).expect(&format!("expect map {m} to have key {k1}"));
            assert_eq!(o1, &v1);

            let o2 = m.get(&o1).expect(&format!("expect map {m} to have key {o1}"));
            assert_eq!(o2, &v2);
        }

        let num_iters = 1;
        for _ in 1..=num_iters {
            _test();
        }
    }

    #[test]
    fn get_entry_with_nil_key() {
        fn _test() {
            let k1 = crate::value::Value::Nil;
            let v1 = crate::keyword!("v1");

            let m = crate::map!(
                (k1.clone(), v1.clone()),
                (crate::keyword!("k2"), crate::keyword!("v2")),
            );
            let m = m.as_map_panicing();

            let o1 = m.get(&k1).expect(&format!("expect map {m} to have key {k1}"));
            assert_eq!(o1, &v1);
        }

        let num_iters = 1;
        for _ in 1..=num_iters {
            _test();
        }
    }

    #[test]
    fn map_entries_mut() {
        use crate::value::Value;
        use crate::map::{Map, IPersistentMap};
        let mut m = crate::map!(
            (crate::integer!(123),
             crate::float!(456.78)),
        );
        let m: &mut Map = m.as_map_mut_panicing();
        eprintln!("{m}");
        m.map_entries_mut(&|(_, _)| (Value::Nil, Value::Nil));
        eprintln!("{m}");
        assert_eq!(m, crate::map!((Value::Nil, Value::Nil)).as_map_panicing());
    }

    #[test]
    fn map_keys_mut() {
        use crate::value::Value;
        use crate::map::{Map, IPersistentMap};

        let mut m = crate::map!((crate::integer!(123), crate::float!(456.78)));
        let m: &mut Map = m.as_map_mut_panicing();

        m.map_keys_mut(&|_| Value::Nil);

        assert_eq!(m, crate::map!((Value::Nil, crate::float!(456.78))).as_map_panicing());
    }

    #[test]
    fn map_values_mut() {
        use crate::value::Value;
        use crate::map::{Map, IPersistentMap};

        let mut m = crate::map!((crate::integer!(123), crate::float!(456.78)));
        let m: &mut Map = m.as_map_mut_panicing();

        m.map_values_mut(&|_| Value::Nil);

        assert_eq!(m, crate::map!((crate::integer!(123), Value::Nil)).as_map_panicing());
    }


}
