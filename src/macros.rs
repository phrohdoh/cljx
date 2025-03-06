
#[macro_export]
macro_rules! nil {
    () => {{ $crate::Value::Nil }};
}

#[macro_export]
macro_rules! boolean {
    ($b: expr $(,)?) => {{ $crate::Value::Boolean(::core::convert::Into::<_>::into($b)) }};
}

#[macro_export]
macro_rules! integer {
    ($i: expr $(,)?) => {{ $crate::Value::Integer(::core::convert::Into::<_>::into($i)) }};
}

#[macro_export]
macro_rules! float {
    ($f: expr $(,)?) => {{ $crate::Value::Float(::core::convert::Into::<_>::into($f)) }};
}

#[macro_export]
macro_rules! string {
    ($s: expr $(,)?) => {{ $crate::Value::String(::core::convert::Into::<_>::into($s)) }};
}

#[macro_export]
macro_rules! keyword {
    ($ns: expr, $n: expr $(,)?) => {{
        $crate::Value::keyword_qualified(
            $crate::UnqualifiedKeyword::from($ns),
            $crate::UnqualifiedKeyword::from($n),
        )
    }};
    ($n: expr $(,)?) => {{
        $crate::Value::keyword_unqualified(
            $crate::UnqualifiedKeyword::from($n),
        )
    }};
}

#[macro_export]
macro_rules! symbol {
    ($ns: expr, $n: expr $(,)?) => {{
        $crate::Value::symbol_qualified(
            $crate::UnqualifiedSymbol::from($ns),
            $crate::UnqualifiedSymbol::from($n),
        )
    }};
    ($n: expr $(,)?) => {{
        $crate::Value::symbol_unqualified(
            $crate::UnqualifiedSymbol::from($n),
        )
    }};
}


#[macro_export]
macro_rules! into_value {
    ($v: expr $(,)?) => {{
        // $crate::deps::tracing::debug!("cljx::into_value!(...)");
        let value: $crate::Value = ::core::convert::From::<_>::from($v);
        value
    }};
}


#[macro_export]
macro_rules! list_inner {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::list_inner!(...)");
        let iter = ::core::iter::IntoIterator::into_iter([$($v,)*]);
        let persistent_list = $crate::PersistentList::from_iter(iter);
        $crate::List::new(persistent_list)
    }};
}

#[macro_export]
macro_rules! list {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::list!(...)");
        let list = $crate::list_inner!($($v,)*);
        $crate::Value::List(list)
    }};
}

#[macro_export]
macro_rules! into_list {
    ($v: expr $(,)?) => {{
        // $crate::deps::tracing::debug!("cljx::into_list!(...)");
        let list: $crate::List = ::core::convert::From::<_>::from($v);
        list
    }};
}



#[macro_export]
macro_rules! vector_inner {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::vector_inner!(...)");
        let iter = ::core::iter::IntoIterator::into_iter([$($v,)*]);
        let persistent_vector = $crate::PersistentVector::from_iter(iter);
        $crate::Vector::new(persistent_vector)
    }};
}

#[macro_export]
macro_rules! vector {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::vector!(...)");
        let vector = $crate::vector_inner!($($v,)*);
        $crate::Value::Vector(vector)
    }};
}

#[macro_export]
macro_rules! into_vector {
    ($v: expr $(,)?) => {{
        // $crate::deps::tracing::debug!("cljx::into_vector!(...)");
        let vector: $crate::Vector = ::core::convert::From::<_>::from($v);
        vector
    }};
}


#[macro_export]
macro_rules! set_inner {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::set_inner!(...)");
        let iter = ::core::iter::IntoIterator::into_iter([$($v,)*]);
        let persistent_set = $crate::set::PersistentSet::from_iter(iter);
        $crate::set::Set::new(persistent_set)
    }};
}

#[macro_export]
macro_rules! set {
    {$($v: expr $(,)?)*} => {{
        // $crate::deps::tracing::debug!("cljx::set!(...)");
        let set = $crate::set_inner!($($v,)*);
        $crate::Value::Set(set)
    }};
}

#[macro_export]
macro_rules! into_set {
    ($v: expr $(,)?) => {{
        // $crate::deps::tracing::debug!("cljx::into_set!(...)");
        let set: $crate::Set = ::core::convert::From::<_>::from($v);
        set
    }};
}


#[macro_export]
macro_rules! map_inner {
    {$(($k: expr, $v: expr $(,)?)),* $(,)?} => {{
        // $crate::deps::tracing::debug!("cljx::map_inner!(...)");
        let iter = ::core::iter::IntoIterator::into_iter([$(($k, $v),)*]);
        let persistent_map = $crate::map::PersistentMap::from_iter(iter);
        $crate::map::Map::new(persistent_map)
    }};
}

#[macro_export]
macro_rules! map {
    {$(($k: expr, $v: expr $(,)?)),* $(,)?} => {{
        // $crate::deps::tracing::debug!("cljx::map!(...)");
        let map = $crate::map_inner!($(($k, $v),)*);
        $crate::Value::Map(map)
    }};
}

#[macro_export]
macro_rules! into_map {
    ($v: expr $(,)?) => {{
        // $crate::deps::tracing::debug!("cljx::into_map!(...)");
        let map: $crate::Map = ::core::convert::From::<_>::from($v);
        map
    }};
}

#[macro_export]
macro_rules! defn {
    (priv $ty_name:ident ,
     $name:expr ,
     $f:expr ,
     $doc:expr $(,)?) => {
        #[doc = $doc]
        ///
        /// Generated via the [`defn`] macro.
        ///
        /// [`defn`]: crate::defn
        struct $ty_name;

        #[doc(hidden)]
        impl $crate::rt::AFn for $ty_name {
            fn name(&self) -> Option<String> { Some(format!("{}", $name)) }
            fn apply(
                &self,
                env: &mut $crate::Env,
                args: $crate::List,
            ) -> $crate::rt::RcValue {
                $f(self, env, args)
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Value {
            fn from(afn: $ty_name) -> $crate::Value {
                $crate::Value::AFn(::std::rc::Rc::new(afn))
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Var {
            fn from(afn: $ty_name) -> $crate::Var {
                $crate::Var::new_bound($crate::RcValue::from($crate::Value::from(afn)))
            }
        }
    };
    ($viz:vis $ty_name:ident ,
     $name:expr ,
     $f:expr ,
     $doc:expr $(,)?) => {
        #[doc = $doc]
        ///
        /// Generated via the [`defn`] macro.
        ///
        /// [`defn`]: crate::defn#macro
        $viz struct $ty_name;

        #[doc(hidden)]
        impl $crate::rt::AFn for $ty_name {
            fn name(&self) -> Option<String> { Some(format!("{}", $name)) }
            fn apply(
                &self,
                env: &mut $crate::Env,
                args: $crate::List,
            ) -> $crate::rt::RcValue {
                $f(self, env, args)
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Value {
            fn from(afn: $ty_name) -> $crate::Value {
                $crate::Value::AFn(::std::rc::Rc::new(afn))
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Var {
            fn from(afn: $ty_name) -> $crate::Var {
                $crate::Var::new_bound($crate::RcValue::from($crate::Value::from(afn)))
            }
        }
    };
    (priv $ty_name:ident ,
     $name:expr ,
     $f:expr $(,)?) => {
        ///
        /// Generated via the [`defn`] macro.
        ///
        /// [`defn`]: crate::defn
        struct $ty_name;

        #[doc(hidden)]
        impl $crate::rt::AFn for $ty_name {
            fn name(&self) -> Option<String> { Some(format!("{}", $name)) }
            fn apply(
                &self,
                env: &mut $crate::Env,
                args: $crate::List,
            ) -> $crate::rt::RcValue {
                $f(self, env, args)
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Value {
            fn from(afn: $ty_name) -> $crate::Value {
                $crate::Value::AFn(::std::rc::Rc::new(afn))
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Var {
            fn from(afn: $ty_name) -> $crate::Var {
                $crate::Var::new_bound($crate::RcValue::from($crate::Value::from(afn)))
            }
        }
    };
    ($viz:vis $ty_name:ident ,
     $name:expr ,
     $f:expr $(,)?) => {
        ///
        /// Generated via the [`defn`] macro.
        ///
        /// [`defn`]: crate::defn
        $viz struct $ty_name;

        #[doc(hidden)]
        impl $crate::rt::AFn for $ty_name {
            fn name(&self) -> Option<String> { Some(format!("{}", $name)) }
            fn apply(
                &self,
                env: &mut $crate::Env,
                args: $crate::List,
            ) -> $crate::rt::RcValue {
                $f(self, env, args)
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Value {
            fn from(afn: $ty_name) -> $crate::Value {
                $crate::Value::AFn(::std::rc::Rc::new(afn))
            }
        }

        #[doc(hidden)]
        impl ::core::convert::From::<$ty_name> for $crate::Var {
            fn from(afn: $ty_name) -> $crate::Var {
                $crate::Var::new_bound($crate::RcValue::from($crate::Value::from(afn)))
            }
        }
    };
}

#[macro_export]
macro_rules! assert_nil {
    ($v:expr) => {
        assert!($v.is_nil())
    }
}
