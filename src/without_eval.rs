
use crate::read::{ReadAllResult, ReadInput, ReadOneResult};

/// `read` from `input` once
pub fn read_one<'e, 'i: 'e>(input: ReadInput<'i>) -> ReadOneResult<'e> {
    let (_input, output) = parse::try_parse(input)?;
    Ok(output)
}

/// `read` from `input` until it has been exhausted or an error is encountered
pub fn read_many<'e, 'i: 'e>(input: ReadInput<'i>) -> ReadAllResult<'e> {
    let mut input = input.trim();
    let mut values = vec![];

    loop {
        if input.is_empty() {
            break;
        }
        match parse::try_parse(input) {
            Ok((rest_input, opt_value)) => {
                input = rest_input;
                if let Some(value) = opt_value {
                    values.push(value);
                }
            },
            Err(_err) => {
                break;
            },
        }
    }

    Ok(values)
}

mod parse {
    use core::convert::identity;
    // use std::rc::Rc;
    use nom::{sequence::separated_pair, Parser};

    use crate::deps::nom::{
        // Parser as _,
        IResult,
        branch::alt,
        //bytes::complete::{tag, is_a, take_until, take_while},
        //combinator::{value, recognize, opt, map, eof},
        //character::complete::{one_of, char},
        //multi::{many0, many1, separated_list0},
        //sequence::{preceded, terminated, delimited, separated_pair, tuple},
        bytes::complete::{tag, is_a, take_until, //take_till
            },
        combinator::{value, recognize, opt, eof},
        character::complete::{one_of, char},
        multi::{many0, many1, separated_list0},
        sequence::{preceded, terminated, delimited},
    };
    use crate::deps::justerror::Error;
    use crate::value::Value;
    use crate::list::PersistentList;
    use crate::vector::PersistentVector;
    use crate::set::PersistentSet;
    use crate::map::{Map, PersistentMap};
    // use crate::symbol::Symbol;
    // use crate::keyword::Keyword;

    pub type ParseInput<'i> = &'i str;
    pub type ParseOutput = Option<Value>;

    // TODO: consider using this type as the E of ParserResult so
    //       that our logging can ignore backtracks and retain errors
    //enum ParseDivergence<I> {
    //    Backtrack(),
    //    Error(ParseError<I>),
    //}

    #[derive(PartialEq)]
    #[Error]
    pub enum ParseError<I> {
        MissingDiscardForm,
        Other(nom::error::Error<I>),
    }

    impl<I> nom::error::ParseError<I> for ParseError<I> {
        fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
            Self::Other(nom::error::Error::new(input, kind))
        }
    
        fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
            other
        }
    }

    pub type ParseResult<'i, O = ParseOutput> = IResult<ParseInput<'i>, O, ParseError<ParseInput<'i>>>;

    impl<I> From<nom::error::Error<I>> for ParseError<I> {
        fn from(nom_err: nom::error::Error<I>) -> Self {
            Self::Other(nom_err)
        }
    }

    fn wrap_log<L, I, O, E: crate::deps::nom::error::ParseError<I>>(
        mut parser: impl Parser<I, Output = O, Error = E>,
        label: L,
        log_before: impl Fn(&L),
        log_ok: impl Fn(&L, &O),
        log_err: impl Fn(&L, &crate::deps::nom::Err<E>),
    ) -> impl Parser<I, Output = O, Error = E> {
        move |input| {
            log_before(&label);
            match parser.parse(input) {
                Ok(ok) => {
                    let (rest_input, output) = ok;
                    log_ok(&label, &output);
                    Ok((rest_input, output))
                },
                Err(err) => {
                    log_err(&label, &err);
                    Err(err)
                },
            }
        }
    }

    fn wrap_log_minimal<L, I, O, E: crate::deps::nom::error::ParseError<I>>(
        parser: impl Parser<I, Output = O, Error = E>,
        label: L,
    ) -> impl Parser<I, Output = O, Error = E>
        where L: ::core::fmt::Display,
              E: ::core::fmt::Debug,
    {
        let log_before = |label: &L| { tracing::debug!("{label} start"); };
        let log_ok = |label: &L, _output: &O| { tracing::debug!("{label} success"); };
        let log_err = |label: &L, err: &crate::deps::nom::Err<E>| {
            match err {
                crate::deps::nom::Err::Error(_err) => { tracing::debug!("{label} backtrack"); },
                crate::deps::nom::Err::Failure(err) => { tracing::error!("{label} error: {err:?}"); },
                _ => unreachable!(),
            }
        };
        wrap_log(
            parser,
            label,
            log_before,
            log_ok,
            log_err,
        )
    }

    //fn wrap_xf<L, I, O, E: crate::deps::nom::error::ParseError<I>, O2, E2: crate::deps::nom::error::ParseError<I>,>(
    //    mut parser: impl Parser<I, Output = O, Error = E>,
    //    xf_input: impl Fn(I) -> I,
    //    xf_ret: impl Fn(Result<(I, O), E>) -> Result<(I, O2), E2>,
    //) -> impl Parser<I, Output = O2, Error = E2> {
    //    move |input| { xf_ret(parser.parse(xf_input(input))) }
    //}

    #[tracing::instrument(level = "INFO", target = "try_parse", skip_all)]
    pub fn try_parse(mut input: ParseInput) -> ParseResult {
        // let mut input = input.trim();
        if input.is_empty() {
            return Ok(("", None));
        }

        // tracing::trace!("attempting to consume comment(s) and/or discard(s)");
        while let Ok((rest_input, _)) = alt((
            wrap_log_minimal(ws1, "ws"),
            wrap_log_minimal(try_parse_comment, stringify!(try_parse_comment)),
            wrap_log_minimal(try_parse_discard, stringify!(try_parse_discard)),
        )).parse(input) {
            // tracing::trace!("consumed comment or discard");
            input = rest_input;
        }
        // tracing::trace!("finished attempting to consume comment(s) and/or discard(s)");

        let parser = preceded(ws0, alt((
            wrap_log_minimal(try_parse_list, stringify!(try_parse_list)),
            wrap_log_minimal(try_parse_vector, stringify!(try_parse_vector)),
            wrap_log_minimal(try_parse_set, stringify!(try_parse_set)),
            wrap_log_minimal(try_parse_map, stringify!(try_parse_map)),
            wrap_log_minimal(try_parse_quote, stringify!(try_parse_quote)),
            wrap_log_minimal(try_parse_nil, stringify!(try_parse_nil)),
            wrap_log_minimal(try_parse_boolean, stringify!(try_parse_boolean)),
            wrap_log_minimal(try_parse_number, stringify!(try_parse_number)),
            wrap_log_minimal(try_parse_string, stringify!(try_parse_string)),
            wrap_log_minimal(try_parse_keyword, stringify!(try_parse_keyword)),
            wrap_log_minimal(try_parse_symbol, stringify!(try_parse_symbol)),
        )));

        let mut parser =
            // parser
            wrap_log_minimal( parser, stringify!(try_parse) )
            ;
        parser.parse(input)
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_comment", skip_all)]
    fn try_parse_comment(input: ParseInput) -> ParseResult<()> {
        let prefix = preceded(ws0, char(';'));
        let until_eol = alt((
            take_until("\r"),
            take_until("\n"),
            eof,
        ));
        let comment = terminated(until_eol, ws0);
        let mut parser = preceded(prefix, comment).map(|_| ());
        parser.parse(input)
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_discard", skip_all)]
    fn try_parse_discard(input: ParseInput) -> ParseResult<()> {
        let mut discard_dispatch_parser = wrap_log_minimal(
            many1(delimited(ws0, tag("#_"), ws0)),
            "discard dispatch",
        );
        let (mut input, discards) = discard_dispatch_parser.parse(input)?;

        let mut comment_parser = wrap_log_minimal(
            try_parse_comment,
            stringify!(try_parse_comment),
        );

        let mut discard_form_parser = wrap_log_minimal(
            try_parse,
            "discard form",
        );

        for _ in discards {
            while let Some(rest_input) = comment_parser
                .parse(input)
                .ok()
                .map(|(rest_input, _)| { rest_input }) {
                    input = rest_input;
                }

            let (rest_input, _) = match discard_form_parser.parse(input) {
                Ok((rest_input, Some(_discarded))) => {
                    // tracing::info!("consumed discard form");
                    (rest_input, ())
                },
                Ok((_, None)) => {
                    tracing::error!("discard dispatch without subsequent discard form");
                    return Err(nom::Err::Failure(ParseError::MissingDiscardForm));
                },
                Err(err) => {
                    // tracing::trace!("failed to consume form to discard");
                    match &err {
                        crate::deps::nom::Err::Error(_err) => { tracing::info!("backtrack"); },
                        crate::deps::nom::Err::Failure(err) => { tracing::info!("error: {err:?}"); },
                        _ => unreachable!(),
                    }
                    return Err(err);
                },
            };

            input = rest_input;
        }
        tracing::trace!("consumed discard");

        return Ok((input, ()));
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_quote", skip_all)]
    fn try_parse_quote(input: ParseInput) -> ParseResult {
        let (input, _quote) = tag("'").parse(input)?;
        let (input, parsed) = try_parse.parse(input)?;
        match parsed {
            Some(parsed) => return Ok((input, Some(crate::list!(crate::symbol!("quote"), parsed)))),
            None => panic!("(quote ,,,) must be given exactly 1 argument"),
        }
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_nil", skip_all)]
    fn try_parse_nil(input: ParseInput) -> ParseResult {
        let mut parser = value((), tag("nil"));
        let (input, _) = parser.parse(input)?;
        Ok((input, Value::Nil.into()))
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_boolean", skip_all)]
    fn try_parse_boolean(input: ParseInput) -> ParseResult {
        let mut parser = alt((
            value(Value::from(true), tag("true")),
            value(Value::from(false), tag("false")),
        ));
        let (input, output) = parser.parse(input)?;
        Ok((input, output.into()))
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_number", skip_all)]
    fn try_parse_number(input: ParseInput) -> ParseResult {
        let mut parser = recognize((
            opt(tag("-")),
            one_of("0123456789"),
            many0(one_of("0123456789_")),
            opt((
                tag("."),
                one_of("0123456789"),
                many0(one_of("0123456789_")),
            )),
        ));
        let (rest_input, num_text) = parser.parse(input)?;
        let num_text = num_text.chars().filter(|&c| c != '_').collect::<String>();
        let val = if num_text.contains(".") {
            let num = num_text.parse::<f64>().expect("expect parse float");
            crate::float!(num)
        } else {
            let num = num_text.parse::<i64>().expect("expect parse integer");
            crate::integer!(num)
        };
        Ok((rest_input, val.into()))
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_string", skip_all)]
    fn try_parse_string(input: ParseInput) -> ParseResult {
        let mut parser = delimited(char('"'), take_until("\""), char('"'));
        let (input, string) = parser.parse(input)?;
        Ok((input, Some(crate::string!(string))))
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_keyword", skip_all)]
    fn try_parse_keyword(input: ParseInput) -> ParseResult {
        // TODO: do not discard the :: or :
        let mut parser = preceded(alt((tag("::"), tag(":"))), try_parse_simple_symbol);
        let (input, part1) = parser.parse(input)
            //.inspect_err(|_| tracing::trace!(target: "cljx::read::try_parse_keyword", "failed to parse keyword"))
            ?;
        let (input, part2) = opt(preceded(char('/'), try_parse_simple_symbol)).parse(input)
            //.inspect_err(|_| tracing::trace!(target: "cljx::read::try_parse_keyword", "failed to parse keyword"))
            ?;
        let keyword = match part2 {
            Some(part2) => crate::keyword!(part1.name(), part2.name()),
            None        => crate::keyword!(part1.name()),
        };
        Ok((input, keyword.into()))
            //.inspect(|_| { tracing::trace!(target: "cljx::read::try_parse_keyword", "succeeded parsing keyword"); })
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_symbol", skip_all)]
    fn try_parse_symbol(input: ParseInput) -> ParseResult {
        let (input, part1) = try_parse_simple_symbol(input)
            //.inspect_err(|_| tracing::trace!(target: "cljx::read::try_parse_symbol", "failed to parse symbol"))
            ?;
        let (input, part2) = opt(preceded(char('/'), try_parse_simple_symbol)).parse(input)
            //.inspect_err(|_| tracing::trace!(target: "cljx::read::try_parse_symbol", "failed to parse symbol"))
            ?;
        let symbol = match part2 {
            Some(part2) => crate::symbol!(part1.name(), part2.name()),
            None        => crate::symbol!(part1.name()),
        };
        Ok((input, symbol.into()))
            //.inspect(|_| { tracing::trace!(target: "cljx::read::try_parse_symbol", "succeeded parsing symbol"); })
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_simple_symbol", skip_all)]
    fn try_parse_simple_symbol(input: ParseInput) -> ParseResult<crate::UnqualifiedSymbol> {
        is_a(".~!@$%^&*_-+=|<>?:|'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
            .parse(input)
            .map(|(input, name)| (input, crate::new_unqualified_symbol(name)))
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_list", skip_all)]
    fn try_parse_list(input: ParseInput) -> ParseResult {
        let mut parser = {
            let open = terminated(char('('), ws0);
            let item = separated_list0(ws1, try_parse);
            let close = preceded(ws0, char(')'));
            //let open = wrapped_minimal(open, "list-open");
            //let item = wrapped_minimal(item, "list-item");
            //let close = wrapped_minimal(close, "list-close");
            delimited(
                open,
                item,
                close,
            )
        };
        let (input, values) = parser.parse(input)?;
        let values: PersistentList = values.into_iter().filter_map(identity).collect();
        Ok((input, Some(Value::list(values))))
            //.inspect(|(_, opt_value)| {
            //    if let Some(value) = opt_value {
            //        tracing::debug!(target: "cljx::read::try_parse_list", "succeeded parsing list: {value}");
            //    }
            //})
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_vector", skip_all)]
    fn try_parse_vector(input: ParseInput) -> ParseResult {
        let mut parser = {
            let open = terminated(char('['), ws0);
            let item = separated_list0(ws1, try_parse);
            let close = preceded(ws0, char(']'));
            //let open = wrapped_minimal(open, "vector-open");
            // let item = wrapped_minimal(item, "vector-item");
            //let close = wrapped_minimal(close, "vector-close");
            delimited(
                open,
                item,
                close,
            )
        };
        let (input, values) = parser.parse(input)?;
        let values: PersistentVector = values.into_iter().filter_map(identity).collect();
        Ok((input, Some(Value::vector(values))))
            //.inspect(|(_, opt_value)| {
            //    if let Some(value) = opt_value {
            //        tracing::debug!(target: "cljx::read::try_parse_vector", "succeeded parsing vector: {value}");
            //    }
            //})
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_set", skip_all)]
    fn try_parse_set(input: ParseInput) -> ParseResult {
        let mut parser = delimited(
            terminated(tag("#{"), ws0),
            separated_list0(ws1, try_parse),
            preceded(ws0, char('}')),
        );
        let (input, values) = parser.parse(input)?;
        let values: PersistentSet = values.into_iter().filter_map(identity).collect();
        Ok((input, Some(Value::set(values))))
            //.inspect(|_| { tracing::trace!(target: "cljx::read::try_parse_set", "succeeded parsing set"); })
    }

    #[tracing::instrument(level = "INFO", target = "try_parse_map", skip_all)]
    fn try_parse_map(input: ParseInput) -> ParseResult {
        // TODO: should be able to successfully read "{ #_ :k }" as an empty map
        let mut parser = delimited(
            terminated(char('{'), ws0),
            separated_list0(ws1, separated_pair(try_parse, ws1, try_parse)),
            preceded(ws0, char('}')),
        );
        let (input, values) = parser.parse(input)
            //.inspect(|_| { tracing::trace!(target: "cljx::read::try_parse_map", "parsed map"); })
            //.inspect_err(|_| { tracing::trace!(target: "cljx::read::try_parse_map", "failed parsing map"); })
        ?;
        let values: PersistentMap = values.into_iter().filter_map(|(k, v)| {
            match (k, v) {
                (Some(k), Some(v)) => Some((k, v)),
                _ => None,
            }
        }).collect();
        Ok((input, Some(Value::Map(Map::new(values)))))
            //.inspect(|_| { tracing::trace!(target: "cljx::read::try_parse_map", "succeeded parsing map"); })
    }

    //#[tracing::instrument(level = "INFO", target = "ws0", skip_all)]
    fn ws0(input: ParseInput) -> ParseResult<()> {
        let mut parser = value((), many0(one_of(", \t\r\n")));
        parser.parse(input)
    }

    //#[tracing::instrument(level = "INFO", target = "ws1", skip_all)]
    fn ws1(input: ParseInput) -> ParseResult<()> {
        let mut parser = value((), many1(one_of(", \t\r\n")));
        parser.parse(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::value::Value;
    use crate::read::without_eval as cljx_read;

    #[test]
    fn discard_and_comment() {
        // a ;-comment between a discard and the discardee is effectively
        // ignored by the discard logic, such that :value-discarded is the
        // discardee instead of 'a or " a comment" etc
        assert_eq!(
            read_one("[#_ ; a comment
                :value-discarded
                :value-read
            ]"),
            crate::vector!(crate::keyword!("value-read")),
        );

        // a discard after a ;-comment is consumed as part of the to-end-of-line
        // comment like any other text, it is not special
        assert_eq!(
            read_one("[#_ ; a comment #_
                :value-discarded
                :value-read
            ]"),
            crate::vector!(crate::keyword!("value-read")),
        );
    }

    fn read_one(input: &str) -> Value {
        cljx_read::read_one(input)
            .unwrap()
            .unwrap()
    }

    fn read_many(input: &str) -> Vec<Value> {
        cljx_read::read_many(input)
            .unwrap()
    }
}
