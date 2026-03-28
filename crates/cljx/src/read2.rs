use ::std::rc::Rc;
use tracing as log;
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, one_of, none_of},
    combinator::{map, opt, recognize, value, cut},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, preceded, separated_pair, tuple}
};
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnomalyMap(Map);

impl AnomalyMap {
    pub fn new(
        category: KeywordUnqualified,
        message: String,
    ) -> Self {
        Self(Map::new(vec![
            (Value::keyword_qualified_rc("cljx.anomalies", "category"),
             Value::keyword_rc(Keyword::Unqualified(category))),
            (Value::keyword_qualified_rc("cljx.anomalies", "message"),
             Value::string_rc(message)),
        ]))
    }

    fn new_empty() -> Self {
        Self(Map::new_empty())
    }

    pub fn set_category(&mut self, category: KeywordUnqualified) -> &mut Self {
        self.0.insert(
            Value::keyword_qualified_rc("cljx.anomalies", "category"),
            Value::keyword_rc(Keyword::Unqualified(category)),
        );
        self
    }

    pub fn set_message(&mut self, message: String) -> &mut Self {
        self.0.insert(
            Value::keyword_qualified_rc("cljx.anomalies", "message"),
            Value::string_rc(message),
        );
        self
    }

    pub fn get_category(&self) -> KeywordUnqualified {
        self.0.get(&Value::keyword_qualified_rc("cljx.anomalies", "category"))
              .as_ref()
              .map(RcValue::as_ref)
              .and_then(value::optics::view_keyword)
              .and_then(keyword::optics::view_unqualified)
              .expect("anomaly is missing :cljx.anomalies/category")
    }

    pub fn get_message(&self) -> String {
        self.0.get(&Value::keyword_qualified_rc("cljx.anomalies", "message"))
              .as_ref()
              .map(RcValue::as_ref)
              .and_then(value::optics::view_string)
              .expect("anomaly is missing :cljx.anomalies/message")
    }

    pub fn insert(&mut self, key: RcValue, value: RcValue) -> &mut Self {
        self.0.insert(key, value);
        self
    }

    pub fn merge_in(&mut self, other: &Map) -> &mut Self {
        other.iter().for_each(|(k, v)| {
            self.0.insert(k.to_owned(), v.to_owned());
        });
        self
    }

    pub fn inner(&self) -> &Map {
        &self.0
    }

    pub fn into_inner(self) -> Map {
        self.0
    }
}

impl From<Map> for AnomalyMap {
    fn from(map: Map) -> Self {
        Self(map)
    }
}

pub fn read(
    env: RcEnvironment,
    input: &str,
) -> Result<
    (&str, Option<RcValue>),
    AnomalyMap,
> {
    // let resolve_fn = |_: &_, env: _, symbol: &_| try_resolve(env, symbol).ok();
    let reader = Reader::new(
        // resolve_fn,
    );
    reader.try_read(env, input)
}

pub trait TryRead {
    // type Error;
    fn try_read         <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), AnomalyMap>;
    // fn try_read_nil     <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_boolean <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_number  <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_string  <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_symbol  <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_keyword <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_list    <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_vector  <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_set     <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
    // fn try_read_map     <'r, 'o, 'i: 'o>(&'r self, env: RcEnvironment, input: &'i str) -> Result<(&'o str, Option<RcValue>), nom::Err<nom::error::Error<&'i str>>>;
}

pub struct Reader {
    // resolve_fn: Box<dyn for<'this, 'sym> Fn(&'this Self, RcEnvironment, &'sym Symbol) -> Option<RcVar>>,
}

impl Reader {
    pub fn new(
        // resolve_fn: impl for<'this, 'sym> Fn(&'this Self, RcEnvironment, &'sym Symbol) -> Option<RcVar> + 'static,
    ) -> Self {
        Self {
            // resolve_fn: Box::new(resolve_fn),
        }
    }

    // fn resolve(
    //     &self,
    //     env: RcEnvironment,
    //     sym: &Symbol,
    // ) -> Option<RcVar> {
    //     (self.resolve_fn)(self, env, sym)
    // }

    fn try_read_any<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let parser = alt((
            // TODO: https://clojure.org/guides/reader_conditionals
            |i| self.try_read_nil     (env.clone(), i),
            |i| self.try_read_boolean (env.clone(), i),
            |i| self.try_read_number  (env.clone(), i),
            |i| self.try_read_string  (env.clone(), i),
            |i| self.try_read_list    (env.clone(), i),
            |i| self.try_read_vector  (env.clone(), i),
            |i| self.try_read_set     (env.clone(), i),
            |i| self.try_read_map     (env.clone(), i),
            |i| self.try_read_keyword (env.clone(), i),
            |i| self.try_read_symbol  (env.clone(), i),
        ));
        let mut parser = preceded(ws0, parser);
        parser(input)
    }

    fn try_read_nil<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let mut parser = value(Some(Value::nil_rc()), tag("nil"));
        parser(input)
    }

    fn try_read_boolean<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        alt((
            value(Some(Value::boolean_rc(true)), tag("true")),
            value(Some(Value::boolean_rc(false)), tag("false")),
        ))(input)
    }

    fn try_read_number<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let number_parser = recognize(tuple((
            opt(char('-')),
            many1(one_of("0123456789")),
            opt(tuple((char('.'), many1(one_of("0123456789"))))),
        )));
        let mut parser = map(number_parser, |s: &str| {
            if s.contains('.') {
                let float: f64 = s.parse().unwrap();
                Rc::new(Value::float(float.into()))
            } else {
                Rc::new(Value::integer(s.parse().unwrap()))
            }
        });
        let (remaining, value) = parser(input)?;
        Ok((remaining, Some(value)))
    }

    fn try_read_string<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let backslash_escape = alt((
            map(tag("\\\""), |_| '"'),
            map(tag("\\\\"), |_| '\\'),
            map(tag("\\n"), |_| '\n'),
            map(tag("\\t"), |_| '\t'),
            map(tag("\\r"), |_| '\r'),
        ));
        
        let string_char = alt((
            backslash_escape,
            none_of("\"\\"),
        ));
        
        let mut parser = delimited(
            char('"'),
            map(many0(string_char), |chars| {
                Rc::new(Value::string(chars.into_iter().collect()))
            }),
            char('"'),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, Some(value)))
    }

    fn try_read_list<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let mut parser = delimited(
            char('('),
            map(
                cut(separated_list0(
                    ws1,
                    |i| self.try_read_any(env.clone(), i),
                )),
                |elements| elements.into_iter()
                                   .filter_map(std::convert::identity)
                                   .collect::<Vec<RcValue>>(),
            ),
            preceded(ws0, char(')')),
        );
        let (remaining, elements) = parser(input)?;

        if let Some(head) = elements.first().map(RcValue::as_ref)
                                    .and_then(value::optics::view_symbol) {
            match head {
                Symbol::Qualified(head) => {
                    log::warn!("Resolving qualified symbol: {}", head);
                    if let Some(var) = env.try_get_namespace(head.namespace())
                                          .and_then(|ns| ns.try_get_var(head.name()).ok()) {
                        let is_macro = var.get_meta(&Value::keyword_unqualified_rc("macro")).as_ref().map(RcValue::as_ref)
                            .and_then(value::optics::view_boolean)
                            .unwrap_or(false);
                        if is_macro {
                            if let Some(macro_func) = var.as_ref()
                                .deref().as_ref().map(RcValue::as_ref)
                                .and_then(value::optics::view_function) {
                                    let macro_ret = macro_func.invoke(env.clone(), elements.into_iter().skip(1).collect());
                                    return Ok((remaining, Some(macro_ret)));
                                }
                        }
                    }
                },
                Symbol::Unqualified(head) => {
                    log::warn!("Resolving unqualified symbol: {}", head);
                    let current_ns = env.try_get_namespace("clojure.core")
                        .and_then(|ns| ns.try_get_handle::<RcNamespace>("*ns*").ok())
                        .expect("current namespace not set, expected #'clojure.core/*ns* to be bound to a Value::Handle of Rc<Namespace>");
                    if let Some(var) = current_ns.try_get_var(head.name()).ok() {
                        let is_macro = var.get_meta(&Value::keyword_unqualified_rc("macro")).as_ref().map(RcValue::as_ref)
                            .and_then(value::optics::view_boolean)
                            .unwrap_or(false);
                        if is_macro {
                            if let Some(macro_func) = var.as_ref()
                                .deref().as_ref().map(RcValue::as_ref)
                                .and_then(value::optics::view_function) {
                                    let macro_ret = macro_func.invoke(env.clone(), elements.into_iter().skip(1).collect());
                                    return Ok((remaining, Some(macro_ret)));
                                }
                        }
                    }
                },
            }
        }

        Ok((remaining, Some(Value::list_rc(List::from(elements)))))
    }

    fn try_read_vector<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let mut parser = delimited(
            char('['),
            map(
                cut(separated_list0(
                    ws1,
                    |i| self.try_read_any(env.clone(), i),
                )),
                |elements| elements.into_iter()
                                   .filter_map(std::convert::identity)
                                   .collect::<Vec<RcValue>>(),
            ),
            preceded(ws0, char(']')),
        );
        let (remaining, elements) = parser(input)?;
        Ok((remaining, Some(Value::vector_rc(Vector::from(elements)))))
    }

    fn try_read_set<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let mut parser = preceded(
            tag("#{"),
            delimited(
                ws0,
                map(
                cut(separated_list0(
                    ws1,
                    |i| self.try_read_any(env.clone(), i),
                )),
                |elements| elements.into_iter()
                                   .filter_map(std::convert::identity)
                                   .collect::<Vec<RcValue>>(),
            ),
                preceded(ws0, char('}')),
            ),
        );
        let (remaining, elements) = parser(input)?;
        Ok((remaining, Some(Value::set_rc(Set::new(elements)))))
    }

    fn try_read_map<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let mut parser = delimited(
            char('{'),
            map(
                separated_list0(
                    ws1,
                    separated_pair(
                        |i| self.try_read_any(env.clone(), i),
                        ws1,
                        |i| self.try_read_any(env.clone(), i),
                    ),
                ),
                // |pairs| Rc::new(Value::map_from(pairs)),
                |pairs| pairs.into_iter()
                            //  .filter_map(std::convert::identity)
                            .filter_map(|(k, v)| match (k, v) {
                                (Some(k), Some(v)) => Some((k, v)),
                                _ => None
                            })
                             .collect::<Vec<(RcValue, RcValue)>>(),
            ),
            preceded(ws0, char('}')),
        );
        let (remaining, entries) = parser(input)?;
        Ok((remaining, Some(Value::map_rc(Map::new(entries)))))
    }

    fn try_read_keyword<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        // Charset without : (prefix) and / (namespace separator)
        let keyword_charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+-=~<>.";
        let build_keyword_chars = || recognize(many1(one_of(keyword_charset)));

        // Consume the initial `:` or `::`
        let (input, prefix) = alt((tag("::"), tag(":")))(input)?;

        // Check for special case: :/ should be the keyword "/"
        // But ::/ is NOT allowed (:: requires qualified form)
        let (input, special_slash) = opt(tag("/"))(input)?;
        if special_slash.is_some() {
            if prefix == "::" {
                // :: requires qualification, :/ is not qualified, so reject
                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)));
            }
            return Ok((input, Some(Value::keyword_rc(Keyword::new_unqualified("/")))));
        }

        // Parse the first identifier (either namespace or the whole keyword)
        let (input, first_part) = build_keyword_chars()(input)?;

        // Check for qualified keyword (namespace/name)
        let (input_after_slash, is_qualified) = opt(char('/'))(input)?;
        let (final_input, keyword) = if is_qualified.is_some() {
            let (input_, second_part) = build_keyword_chars()(input_after_slash)?;
            (input_, Keyword::new_qualified(first_part, second_part))
        } else {
            // If prefix is ::, then keyword MUST be qualified
            if prefix == "::" {
                return Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Verify)));
            }
            (input, Keyword::new_unqualified(first_part))
        };

        Ok((final_input, Some(Value::keyword_rc(keyword))))
    }

    fn try_read_symbol<'r, 'o, 'i: 'o>(
        &'r self,
        _env: RcEnvironment,
        input: &'i str,
    ) -> Result<
        (&'o str, Option<RcValue>),
        nom::Err<nom::error::Error<&'i str>>,
    > {
        let symbol_charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+-=~<>.";
        let build_symbol_chars = || recognize(many1(one_of(symbol_charset)));

        // Try to parse a qualified symbol (namespace/name), falling back to unqualified
        let qualified_parser = map(
            tuple((
                build_symbol_chars(),
                char('/'),
                build_symbol_chars(),
            )),
            |(namespace, _, name): (&str, char, &str)| {
                Value::symbol_rc(Symbol::new_qualified(namespace, name))
            },
        );

        let unqualified_parser = map(build_symbol_chars(), |s: &str| {
            Value::symbol_rc(Symbol::new_unqualified(s))
        });

        // Also handle "/" as a special unqualified symbol for division
        let slash_parser = map(tag("/"), |_| {
            Value::symbol_rc(Symbol::new_unqualified("/"))
        });

        let mut parser = alt((qualified_parser, unqualified_parser, slash_parser));
        let (remaining, value) = parser(input)?;
        Ok((remaining, Some(value)))
    }
}

fn ws0(input: &str) -> IResult<&str, ()> { value((), many0(one_of(", \t\r\n")))(input) }
fn ws1(input: &str) -> IResult<&str, ()> { value((), many1(one_of(", \t\r\n")))(input) }

impl TryRead for Reader {
    // type Error = nom::Err<nom::error::Error<&'e str>>;
    // type Error = nom::Err<nom::error::Error<String>>;
    // type Error = ();

    fn try_read<'r, 'o, 'i: 'o>(
        &'r self,
        env: RcEnvironment,
        input: &'i str,
    ) -> Result<(&'o str, Option<RcValue>), AnomalyMap> {
        self.try_read_any(env, input)
            .map_err(move |e| {
                let mut anomaly = AnomalyMap::new_empty();
                match e {
                    nom::Err::Incomplete(nom::Needed::Unknown) => {
                        anomaly.set_category(KeywordUnqualified::new("incomplete-input"))
                               .set_message("reader needs an additional unknown number of bytes of input".to_owned());
                    },
                    nom::Err::Incomplete(nom::Needed::Size(size)) => {
                        anomaly.set_category(KeywordUnqualified::new("incomplete-input"))
                               .set_message(format!("reader needs an additional {size} byte(s) of input"));
                    },
                    nom::Err::Error(e) | nom::Err::Failure(e) => {
                        anomaly.set_category(KeywordUnqualified::new("erroneous-input"))
                               .set_message(match e.input.trim() {
                                "(" => "unclosed list".to_owned(),
                                "[" => "unclosed vector".to_owned(),
                                "#{" => "unclosed set".to_owned(),
                                "{" => "unclosed map".to_owned(),
                                ")" => "unopened list".to_owned(),
                                "]" => "unopened vector".to_owned(),
                                "}" => "unopened set or map".to_owned(),
                                _input => format!("{e:?}"),
                            });
                    },
                }
                anomaly
            }
        )
    }

    // fn try_read_nil<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_nil")
    // }

    // fn try_read_boolean<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_boolean")
    // }

    // fn try_read_number<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_number")
    // }

    // fn try_read_string<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_string")
    // }

    // fn try_read_symbol<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_symbol")
    // }

    // fn try_read_keyword<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_keyword")
    // }

    // fn try_read_list<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_list")
    // }

    // fn try_read_vector<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_vector")
    // }

    // fn try_read_set<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_set")
    // }

    // fn try_read_map<'i>(
    //     &self,
    //     env: RcEnvironment,
    //     input: &'i str,
    // ) -> Result<(&'i str, RcValue), Self::Error> {
    //     todo!("try_read_map")
    // }
}
