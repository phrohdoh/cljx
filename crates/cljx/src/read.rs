use tracing::{info};
use crate::{RcEnvironment as RcEnv, RcValue, Value};
use std::rc::Rc;

/*
pub type Result<'error, T> = ::core::result::Result<T, self::Error<'error>>;
pub type Error<'error> = Box<dyn ::core::error::Error + 'error>;

pub type ReadInput<'input> = &'input str;
pub type ReadOutput = Rc<Value>;
pub type ReadResult<'error> = Result<'error, ReadOutput>;

pub fn read_one(input: ReadInput) -> ReadResult {
    let (_input, v) = parse::try_parse_one(input)?;
    Ok(v)
}
*/

/// Position information for a parsed value: line and column (both 1-indexed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePosition {
    pub line: usize,
    pub col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

/// Constants for position metadata keys
pub mod position_keys {
    use crate::Keyword;

    pub fn line() -> Keyword {
        Keyword::Unqualified(crate::KeywordUnqualified::new("line"))
    }

    pub fn col() -> Keyword {
        Keyword::Unqualified(crate::KeywordUnqualified::new("col"))
    }

    pub fn end_line() -> Keyword {
        Keyword::Unqualified(crate::KeywordUnqualified::new("end-line"))
    }

    pub fn end_col() -> Keyword {
        Keyword::Unqualified(crate::KeywordUnqualified::new("end-col"))
    }
}

/// Computes line and column (both 1-indexed) for a given slice offset in a buffer.
/// `buffer_start` is a pointer to the beginning of the full input buffer.
/// `slice_start` is a pointer to the start of the slice we want to compute position for.
/// Returns (line, col, end_line, end_col) where line and col correspond to the slice start.
pub fn compute_line_col_for_slice(buffer: &str, slice_start: &str) -> (usize, usize) {
    // Compute offset: distance from buffer start to slice start
    let offset = buffer.len() - slice_start.len();

    let mut line = 1;
    let mut col = 1;

    for (_i, ch) in buffer[..offset].chars().enumerate() {
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    (line, col)
}

/// Computes the end position (line, col) for a consumed token given:
/// - `original_input`: the full input buffer
/// - `remaining_input`: the input remaining after parsing
/// This gives us the position where the parsed token ends.
pub fn compute_consumed_range(buffer: &str, original_input: &str, remaining_input: &str) -> SourcePosition {
    let (start_line, start_col) = compute_line_col_for_slice(buffer, original_input);
    let (end_line, end_col) = compute_line_col_for_slice(buffer, remaining_input);

    SourcePosition {
        line: start_line,
        col: start_col,
        end_line,
        end_col,
    }
}

/// Attaches position metadata to a Value by inserting position keys into its Meta map.
fn attach_position_to_value(value: RcValue, pos: SourcePosition) -> RcValue {
    let meta_ref = match value.as_ref() {
        Value::Nil(m) => m,
        Value::Boolean(_, m) => m,
        Value::Integer(_, m) => m,
        Value::Float(_, m) => m,
        Value::String(_, m) => m,
        Value::Symbol(_, m) => m,
        Value::Keyword(_, m) => m,
        Value::List(_, m) => m,
        Value::Vector(_, m) => m,
        Value::Set(_, m) => m,
        Value::Map(_, m) => m,
        Value::Var(_, m) => m,
        Value::Function(_, m) => m,
        Value::Handle(_, m) => m,
    };

    // Insert position keys into the meta map
    let mut new_meta = meta_ref.clone();

    let line_key = Value::keyword(position_keys::line()).into_value_rc();
    let col_key = Value::keyword(position_keys::col()).into_value_rc();
    let end_line_key = Value::keyword(position_keys::end_line()).into_value_rc();
    let end_col_key = Value::keyword(position_keys::end_col()).into_value_rc();

    let line_val = Value::integer(pos.line as i64).into_value_rc();
    let col_val = Value::integer(pos.col as i64).into_value_rc();
    let end_line_val = Value::integer(pos.end_line as i64).into_value_rc();
    let end_col_val = Value::integer(pos.end_col as i64).into_value_rc();

    new_meta = crate::Meta::insert(new_meta, line_key, line_val);
    new_meta = crate::Meta::insert(new_meta, col_key, col_val);
    new_meta = crate::Meta::insert(new_meta, end_line_key, end_line_val);
    new_meta = crate::Meta::insert(new_meta, end_col_key, end_col_val);

    // Unwrap and reconstruct the value with the new meta, or clone if necessary
    let value_with_new_meta = Rc::try_unwrap(value)
        .unwrap_or_else(|v| (*v).clone())
        .with_meta(new_meta);

    Rc::new(value_with_new_meta)
}

#[derive(Clone, Debug)]
pub struct IncompleteRead<'a> {
    pub input: &'a str,
}

#[derive(Clone, Debug)]
pub struct CompleteRead<'a> {
    pub input: &'a str,
    pub rest_input: &'a str,
    pub value: RcValue,
}

#[derive(Clone, Debug)]
pub enum ReadOutput<'input> {
    Incomplete(IncompleteRead<'input>),
    Complete(CompleteRead<'input>),
    EndOfInput,
}

impl<'input> ReadOutput<'input> {
    pub fn is_end_of_input(&self) -> bool {
        matches!(self, Self::EndOfInput)
    }

    pub fn try_into_end_of_input_read(self) -> Option<()> {
        if let Self::EndOfInput = self { Some(()) } else { None }
    }

    pub fn into_end_of_input_read_or_panic(self) -> () {
        let expect_msg = format!("Expected EndOfInput variant but got: {:?}", self);
        self.try_into_end_of_input_read()
            .expect(&expect_msg)
    }


    pub fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete(_))
    }

    pub fn try_into_incomplete_read(self) -> Option<IncompleteRead<'input>> {
        if let Self::Incomplete(x) = self { Some(x) } else { None }
    }

    pub fn into_incomplete_read_or_panic(self) -> IncompleteRead<'input> {
        let expect_msg = format!("Expected Incomplete variant but got: {:?}", self);
        self.try_into_incomplete_read()
            .expect(&expect_msg)
    }


    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete(_))
    }

    pub fn try_into_complete_read(self) -> Option<CompleteRead<'input>> {
        if let Self::Complete(x) = self { Some(x) } else { None }
    }

    pub fn into_complete_read_or_panic(self) -> CompleteRead<'input> {
        let expect_msg = format!("Expected Complete variant but got: {:?}", self);
        self.try_into_complete_read()
            .expect(&expect_msg)
    }


    pub fn incomplete(input: &'input str) -> Self {
        Self::Incomplete(IncompleteRead { input })
    }

    pub fn complete(input: &'input str, rest_input: &'input str, value: RcValue) -> Self {
        Self::Complete(CompleteRead { input, rest_input, value })
    }
}


#[derive(Debug)]
pub struct UnexpectedEndOfInputError<'input>(&'input str);

impl<'input> UnexpectedEndOfInputError<'input> {
    pub fn input(&self) -> &str {
        self.0
    }
}

#[derive(Debug)]
pub struct TypeErasedError<'input>(&'input str, Box<dyn std::error::Error + 'input>);

impl<'input> TypeErasedError<'input> {
    pub fn input(&self) -> &str {
        self.0
    }
    pub fn error(&self) -> &(dyn std::error::Error + 'input) {
        &*self.1
    }
}


#[derive(Debug)]
pub enum ReadError<'input> {
    UnexpectedEndOfInput(UnexpectedEndOfInputError<'input>),
    TypeErased(TypeErasedError<'input>)
}

impl<'input> ReadError<'input> {
    pub fn is_unexpected_end_of_input(&self) -> bool {
        matches!(self, Self::UnexpectedEndOfInput(_))
    }

    pub fn try_into_unexpected_end_of_input(self) -> Result<UnexpectedEndOfInputError<'input>, Self> {
        if let Self::UnexpectedEndOfInput(x) = self { Ok(x) } else { Err(self) }
    }

    pub fn into_unexpected_end_of_input_or_panic(self) -> UnexpectedEndOfInputError<'input> {
        let expect_msg = format!("Expected UnexpectedEndOfInput variant but got: {:?}", self);
        self.try_into_unexpected_end_of_input().expect(&expect_msg)
    }


    pub fn is_type_erased(&self) -> bool {
        matches!(self, Self::TypeErased(_))
    }

    pub fn try_into_type_erased(self) -> Result<TypeErasedError<'input>, Self> {
        if let Self::TypeErased(x) = self { Ok(x) } else { Err(self) }
    }

    pub fn into_type_erased_or_panic(self) -> TypeErasedError<'input> {
        let expect_msg = format!("Expected TypeErased variant but got: {:?}", self);
        self.try_into_type_erased().expect(&expect_msg)
    }


    pub fn unexpected_end_of_input(input: &'input str) -> Self {
        Self::UnexpectedEndOfInput(UnexpectedEndOfInputError(input))
    }

    pub fn type_erased(input: &'input str, error: Box<dyn std::error::Error + 'input>) -> Self {
        Self::TypeErased(TypeErasedError(input, error))
    }
}


/// Reads a single value from the input string, returning the value and the remaining unparsed input.
/// Being unable to read a value (e.g., due to end of input) is considered an error.
/// Use-sites can convert that to `Option<Value>` as desired.
pub fn read_one<'input>(env: RcEnv, input: &'input str) -> Result<ReadOutput<'input>, ReadError<'input>> {
    if input.trim().is_empty() {
        return Ok(ReadOutput::EndOfInput);
    }
    match parse::try_parse_one(env, input) {
        Ok((rest_input, value))                                                                                  => Ok(ReadOutput::complete(input, rest_input, value)),
        Err(nom::Err::Incomplete(nom::Needed::Unknown))                                                          => Ok(ReadOutput::incomplete(input)),
        Err(nom::Err::Error(nom::error::Error { input, code: nom::error::ErrorKind::IsA } )) if input.is_empty() => Err(ReadError::unexpected_end_of_input("")), 
        Err(nom::Err::Error(err))                                                                                => Err(ReadError::type_erased(input, Box::new(err))),
        Err(err)                                                                                                 => Err(ReadError::type_erased(input, Box::new(err))),
    }
}

#[tracing::instrument(ret, level = "info")]
pub fn read_one_v2<'input>(
    env: RcEnv,
    input: &'input str,
) -> Result<(&'input str, Option<RcValue>), ReadError<'input>> {
    match read_one_v2_inner(env, input) {
        Ok(ReadOutput::EndOfInput) => {
            info!("end-of-input");
            Ok(("", None))
        },
        Ok(ReadOutput::Complete(CompleteRead { input: _, rest_input, value })) => {
            info!("complete read");
            Ok((rest_input, Some(value)))
        },
        Ok(ReadOutput::Incomplete(IncompleteRead { input })) => {
            info!("incomplete read");
            Ok((input, None))
        },
        Err(err) => {
            info!("read error");
            Err(err)
        },
    }
}

#[tracing::instrument(fields(input), ret, level = "info")]
fn read_one_v2_inner<'input>(
    env: RcEnv,
    input: &'input str,
) -> Result<ReadOutput<'input>, ReadError<'input>> {
    use nom::{Err as NomErr, error::{Error as NomError, ErrorKind as NomErrKind}};

    // Handle empty/whitespace-only input early
    if input.trim().is_empty() {
        return Ok(ReadOutput::EndOfInput);
    }

    // Save the original input for position computation
    let original_input = input;
    let buffer = input;

    let result = match parse::try_parse_one(env, input) {
        Ok((rest_input, value)) => {
            info!("complete read");
            // Compute position info based on the input pointers
            let pos = compute_consumed_range(buffer, original_input, rest_input);
            // Attach position to the value
            let value_with_pos = attach_position_to_value(value, pos);
            Ok(ReadOutput::complete(original_input, rest_input, value_with_pos))
        },
        Err(err @ NomErr::Error(NomError {
            input,
            code:
                | NomErrKind::IsA
             // | NomErrKind::OneOf
                ,
        } )) => {
            let input = input.trim();
            if input.is_empty() {
                info!("end-of-input");
                return Ok(ReadOutput::EndOfInput);
            }
            let first_char = input.chars().next();
            match first_char {
                Some('('|'['|'{'|'#') => {
                    info!("unexpected end-of-input");
                    Err(ReadError::unexpected_end_of_input(input))
                },
                _ => {
                    info!("other error");
                    Err(ReadError::type_erased(input, Box::new(err)))
                },
            }
        }, 
        Err(err) => {
            // todo!("unhandled error: {:?}", err);
            Err(ReadError::type_erased(input, Box::new(err)))
        },
    };

    result
}


// #[cfg(test)]
// mod v2_tests {
//     use crate::{Keyword, KeywordUnqualified, Symbol, SymbolUnqualified, Environment};
//     use super::*;
// 
//     #[test]
//     fn empty() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         assert!(opt_value.is_none());
//     }
// 
//     #[test]
//     fn nil() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "nil";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read a value");
//         assert!(value.is_nil(), "value expected to be Value::Nil, instead was {:?}", value);
//     }
// 
//     #[test]
//     fn symbol() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "assoc";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read a value");
//         assert!(value.is_symbol(), "value expected to be Value::Symbol, instead was {:?}", value);
//         let (symbol, _meta) = value.try_as_symbol().unwrap();
//         assert!(symbol.is_unqualified(), "symbol expected to be Symbol::Unqualified, instead was {:?}", symbol);
//         let name = symbol.as_unqualified_symbol().unwrap().name();
//         assert_eq!(name, "assoc");
//     }
// 
//     #[test]
//     fn keyword() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = ":assoc";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read a value");
//         assert!(value.is_keyword(), "value expected to be Value::Keyword, instead was {:?}", value);
//         let (keyword, _meta) = value.try_as_keyword().unwrap();
//         assert!(keyword.is_unqualified(), "keyword expected to be Keyword::Unqualified, instead was {:?}", keyword);
//         let name = keyword.as_unqualified_keyword().unwrap().name();
//         assert_eq!(name, "assoc");
//     }
// 
//     #[test]
//     fn multi_line_list() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "(prn\n:hi)";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read multi-line list value");
//         assert!(value.is_list(), "value expected to be Value::List, instead was {:?}", value);
//         let (list, _meta) = value.try_as_list().unwrap();
//         assert!(!list.is_empty());
//         let mut list_iter = list.iter();
//         assert!(list_iter.next().is_some_and(|v| **v == Value::symbol(Symbol::Unqualified(SymbolUnqualified::new("prn")))));
//         assert!(list_iter.next().is_some_and(|v| **v == Value::keyword(Keyword::Unqualified(KeywordUnqualified::new("hi")))));
//     }
// 
//     #[test]
//     fn multi_line_vector() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "[prn\n:hi]";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read multi-line vector value");
//         assert!(value.is_vector(), "value expected to be Value::Vector, instead was {:?}", value);
//         let (vector, _meta) = value.try_as_vector().unwrap();
//         assert!(!vector.is_empty());
//         let mut vector_iter = vector.iter();
//         assert!(vector_iter.next().is_some_and(|v| **v == Value::symbol_unqualified("prn")));
//         assert!(vector_iter.next().is_some_and(|v| **v == Value::keyword_unqualified("hi")));
//     }
// 
//     #[test]
//     fn multi_line_vector_with_sequential_newlines() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "[prn\n\n:hi]";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read multi-line vector value");
//         assert!(value.is_vector(), "value expected to be Value::Vector, instead was {:?}", value);
//         let (vector, _meta) = value.try_as_vector().unwrap();
//         assert!(!vector.is_empty());
//         let mut vector_iter = vector.iter();
//         assert!(vector_iter.next().is_some_and(|v| **v == Value::symbol_unqualified("prn")));
//         assert!(vector_iter.next().is_some_and(|v| **v == Value::keyword_unqualified("hi")));
//     }
// 
//     #[test]
//     fn multi_line_set() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "#{prn\n:hi}";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read multi-line set value");
//         assert!(value.is_set(), "value expected to be Value::Set, instead was {:?}", value);
//         let (set, _meta) = value.try_as_set().unwrap();
//         assert!(!set.is_empty());
//         assert!(set.contains(&Value::symbol_unqualified("prn").into_value_rc()));
//         assert!(set.contains(&Value::keyword_unqualified("hi").into_value_rc()));
//     }
// 
//     #[test]
//     fn multi_line_map() {
//         // arrange
//         let env = Environment::new_empty_rc();
//         let input = "{prn\n:hi}";
//         // act
//         let read_result = read_one_v2(env, input);
//         // assert
//         let (rest_input, opt_value) = read_result.expect("successful read");
//         assert!(rest_input.is_empty());
//         let value = opt_value.expect("to read multi-line map value");
//         assert!(value.is_map(), "value expected to be Value::Map, instead was {:?}", value);
//         let (map, _meta) = value.try_as_map().unwrap();
//         assert!(!map.is_empty());
//         assert!(map.contains_key(&Value::symbol_unqualified("prn").into_value_rc()));
//     }
// }

#[cfg(test)]
mod v2_tests_inner {
    use crate::Environment;
    use super::*;

    /*
    #[test]
    fn empty() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = " \n ";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_output = read_result.expect("successful read");
        let end_of_input = read_output.try_into_end_of_input_read().expect("end-of-input");
        assert_eq!(end_of_input, ());
    }

    #[test]
    fn nil() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "nil";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_output = read_result.expect("successful read");
        let complete_read = read_output.try_into_complete_read().expect("complete read");
        assert!(complete_read.value.is_nil());
        assert!(complete_read.rest_input.is_empty());
    }

    #[test]
    fn multi_line_list() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "(prn\n:hi)";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_output = read_result.expect("successful read");
        let complete_read = read_output.try_into_complete_read().expect("complete read");
        assert!(complete_read.value.is_list());
        assert!(complete_read.rest_input.is_empty());
    }
    */

    #[test]
    fn position_tracking_on_symbol() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "hello";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_output = read_result.expect("successful read");
        let complete_read = read_output.try_into_complete_read().expect("complete read");
        let value = &complete_read.value;

        // Extract meta from symbol
        let (_, meta) = value.try_as_symbol().expect("should be a symbol");

        // Check position keys exist
        //assert!(meta.is_some(), "meta should be present");
        //let meta_ref = meta.unwrap();
        let meta_ref = meta;

        // Verify position information is in the meta
        // The input "hello" starts at line 1, col 1 and ends at line 1, col 6
        let line_key = Value::keyword(position_keys::line()).into_value_rc();
        let col_key = Value::keyword(position_keys::col()).into_value_rc();
        let end_line_key = Value::keyword(position_keys::end_line()).into_value_rc();
        let end_col_key = Value::keyword(position_keys::end_col()).into_value_rc();

        let line_val = crate::Meta::get(&meta_ref, &line_key);
        let col_val = crate::Meta::get(&meta_ref, &col_key);
        let end_line_val = crate::Meta::get(&meta_ref, &end_line_key);
        let end_col_val = crate::Meta::get(&meta_ref, &end_col_key);

        assert!(line_val.is_some(), "line key should be in meta");
        assert!(col_val.is_some(), "col key should be in meta");
        assert!(end_line_val.is_some(), "end_line key should be in meta");
        assert!(end_col_val.is_some(), "end_col key should be in meta");
    }

    #[test]
    fn position_tracking_on_list() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "(+ 1 2)";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_output = read_result.expect("successful read");
        let complete_read = read_output.try_into_complete_read().expect("complete read");
        let value = &complete_read.value;

        // Extract meta from list
        let (_, meta) = value.try_as_list().expect("should be a list");

        // Check position keys exist
        // assert!(meta.is_some(), "meta should be present on list");
        // let meta_ref = meta.unwrap();
        let meta_ref = meta;

        // Verify position information is in the meta
        let line_key = Value::keyword(position_keys::line()).into_value_rc();
        let line_val = crate::Meta::get(&meta_ref, &line_key);
        assert!(line_val.is_some(), "line key should be in meta");
    }


    /*
    #[test]
    fn incomplete_list() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "(prn";
        // act
        let read_result = read_one_v2_inner(env, input);
        // assert
        let read_error = read_result.expect_err("expect unsuccessful read");
        let eoi_error = read_error.try_into_unexpected_end_of_input().expect("unexpected end-of-input (reading list)");
        assert_eq!(eoi_error.input(), "(prn");
        //let complete_read = read_output.try_into_incomplete_read().expect("incomplete read");
        //assert!(complete_read.value.is_list());
        //assert!(complete_read.rest_input.is_empty());
    }

    // #[test]
    // fn read_form_split_across_multiple_lines() {
    //     // arrange
    //     let input = "(prn\n:ok)";
    //     // act
    //     let read_result = super::read_one_v2_inner(env, input);
    //     // assert
    //     assert!(read_result.is_ok());
    //     let read_output = read_result.unwrap();
    //     let (rest_input, value) = read_output.unwrap();
    //     assert!(value.is_some());
    //     let value = value.unwrap();
    //     assert!(value.is_list(), "{:?}", value);
    // }
    */
}

pub mod parse {
    use std::rc::Rc;

    use crate::{Keyword, List, RcEnvironment as RcEnv, RcValue, Symbol, Value};
    use nom::{
        IResult,
        Parser,
        branch::alt,
        bytes::complete::{is_a, tag, take_until},
        character::complete::{char, one_of, none_of},
        combinator::{map, opt, recognize, value, cut},
        multi::{many0, many1, separated_list0},
        sequence::{delimited, preceded, separated_pair, terminated, tuple}
    };

    // Type alias for cleaner signatures
    type ParseResult<'a> = IResult<&'a str, RcValue>;

    // Whitespace parsers don't need environment
    pub fn ws0(input: &str) -> IResult<&str, ()> {
        value((), many0(one_of(", \t\r\n")))(input)
    }

    pub fn ws1(input: &str) -> IResult<&str, ()> {
        value((), many1(one_of(", \t\r\n")))(input)
    }

    // ========== BUILDER FUNCTIONS ==========

    pub fn build_try_parse_one<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_one(env.clone(), input)
    }

    pub fn build_try_parse_nil<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_nil(env.clone(), input)
    }

    pub fn build_try_parse_boolean<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_boolean(env.clone(), input)
    }

    pub fn build_try_parse_number<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_number(env.clone(), input)
    }

    pub fn build_try_parse_string<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_string(env.clone(), input)
    }

    pub fn build_try_parse_symbol<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_symbol(env.clone(), input)
    }

    pub fn build_try_parse_keyword<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_keyword(env.clone(), input)
    }

    pub fn build_try_parse_list<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_list(env.clone(), input)
    }

    pub fn build_try_parse_vector<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_vector(env.clone(), input)
    }

    pub fn build_try_parse_set<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_set(env.clone(), input)
    }

    pub fn build_try_parse_map<'o, 'i: 'o>(env: RcEnv) -> impl Fn(&'i str) -> ParseResult<'o> {
        move |input: &str| try_parse_map(env.clone(), input)
    }

    // ========== PARSER FUNCTIONS ==========

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_one(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let parser = alt((
            |i| try_parse_nil(env.clone(), i),
            |i| try_parse_boolean(env.clone(), i),
            |i| try_parse_number(env.clone(), i),
            |i| try_parse_string(env.clone(), i),
            |i| try_parse_list(env.clone(), i),
            |i| try_parse_vector(env.clone(), i),
            |i| try_parse_set(env.clone(), i),
            |i| try_parse_map(env.clone(), i),
            |i| try_parse_keyword(env.clone(), i),
            |i| try_parse_symbol(env.clone(), i),
        ));
        preceded(ws0, parser)(input)
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_nil(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let (remaining, _) = tag("nil")(input)?;
        let value = Rc::new(Value::nil());
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_boolean(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let mut parser = alt((
            map(tag("true"), |_| Rc::new(Value::boolean(true))),
            map(tag("false"), |_| Rc::new(Value::boolean(false))),
        ));
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_number(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
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
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_string(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
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
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_symbol(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
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
                Rc::new(Value::symbol(Symbol::new_qualified(namespace, name)))
            },
        );

        let unqualified_parser = map(build_symbol_chars(), |s: &str| {
            Rc::new(Value::symbol(Symbol::new_unqualified(s)))
        });

        // Also handle "/" as a special unqualified symbol for division
        let slash_parser = map(tag("/"), |_| {
            Rc::new(Value::symbol(Symbol::new_unqualified("/")))
        });

        let mut parser = alt((qualified_parser, unqualified_parser, slash_parser));
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_keyword(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
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
            return Ok((input, Rc::new(Value::keyword(Keyword::new_unqualified("/")))));
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

        Ok((final_input, Rc::new(Value::keyword(keyword))))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_list(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('('),
            map(
                cut(separated_list0(
                    ws1,
                    build_try_parse_one(env.clone()),
                )),
                |items| Rc::new(Value::list_from(items)),
            ),
            preceded(ws0, char(')')),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_vector(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('['),
            map(
                separated_list0(ws1, build_try_parse_one(env.clone())),
                |items| Rc::new(Value::vector_from(items)),
            ),
            preceded(ws0, char(']')),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_set(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let mut parser = preceded(
            tag("#{"),
            delimited(
                ws0,
                map(
                    separated_list0(ws1, build_try_parse_one(env.clone())),
                    |items| Rc::new(Value::set_from(items)),
                ),
                preceded(ws0, char('}')),
            ),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    #[tracing::instrument(fields(input), ret, level = "info")]
    pub fn try_parse_map(env: RcEnv, input: &'_ str) -> ParseResult<'_> {
        let mut parser = delimited(
            char('{'),
            map(
                separated_list0(
                    ws1,
                    separated_pair(
                        build_try_parse_one(env.clone()),
                        ws1,
                        build_try_parse_one(env.clone()),
                    ),
                ),
                |pairs| Rc::new(Value::map_from(pairs)),
            ),
            preceded(ws0, char('}')),
        );
        let (remaining, value) = parser(input)?;
        Ok((remaining, value))
    }

    // ========== TOP-LEVEL API ==========

    // #[tracing::instrument(fields(input), ret, level = "info")]
    // pub fn parse(env: RcEnv, input: &str) -> Result<RcValue, String> {
    //     match try_parse_one(env, input) {
    //         Ok(("", value)) => Ok(value),
    //         Ok((remaining, _)) => Err(format!("Unexpected trailing input: {}", remaining)),
    //         Err(e) => Err(format!("Parse error: {:?}", e)),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Environment;

    #[test]
    fn symbol_unqualified() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "hello";
        // act
        let result = parse::try_parse_symbol(env, input);
        // assert
        let (remaining, value) = result.expect("should parse unqualified symbol");
        assert!(remaining.is_empty());
        let (symbol, _meta) = value.try_as_symbol().expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "hello");
        assert_eq!(symbol.namespace(), None);
    }

    #[test]
    fn symbol_qualified() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "foo/bar";
        // act
        let result = parse::try_parse_symbol(env, input);
        // assert
        let (remaining, value) = result.expect("should parse qualified symbol");
        assert!(remaining.is_empty());
        let (symbol, _meta) = value.try_as_symbol().expect("should be a symbol");
        assert!(symbol.is_qualified(), "symbol should be qualified");
        assert_eq!(symbol.name(), "bar");
        assert_eq!(symbol.namespace(), Some("foo"));
    }

    #[test]
    fn symbol_slash() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "/";
        // act
        let result = parse::try_parse_symbol(env, input);
        // assert
        let (remaining, value) = result.expect("should parse slash as symbol");
        assert!(remaining.is_empty());
        let (symbol, _meta) = value.try_as_symbol().expect("should be a symbol");
        assert!(symbol.is_unqualified(), "/ should be an unqualified symbol");
        assert_eq!(symbol.name(), "/");
        assert_eq!(symbol.namespace(), None);
    }

    #[test]
    fn keyword_unqualified() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = ":hello";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        let (remaining, value) = result.expect("should parse unqualified keyword");
        assert!(remaining.is_empty());
        let (keyword, _meta) = value.try_as_keyword().expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "hello");
        assert_eq!(keyword.namespace(), None);
    }

    #[test]
    fn keyword_qualified() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = ":foo/bar";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        let (remaining, value) = result.expect("should parse qualified keyword");
        assert!(remaining.is_empty());
        let (keyword, _meta) = value.try_as_keyword().expect("should be a keyword");
        assert!(keyword.is_qualified(), "keyword should be qualified");
        assert_eq!(keyword.name(), "bar");
        assert_eq!(keyword.namespace(), Some("foo"));
    }

    #[test]
    fn keyword_slash() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = ":/";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        let (remaining, value) = result.expect("should parse slash as keyword");
        assert!(remaining.is_empty());
        let (keyword, _meta) = value.try_as_keyword().expect("should be a keyword");
        assert!(keyword.is_unqualified(), ":/ should be an unqualified keyword");
        assert_eq!(keyword.name(), "/");
        assert_eq!(keyword.namespace(), None);
    }

    #[test]
    fn keyword_double_colon_slash_rejected() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "::/";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        assert!(result.is_err(), "::/ should be rejected because :: requires qualified form");
    }

    #[test]
    fn keyword_slash_with_following_identifier_rejected() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = ":/foo";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert - either error or has remaining input (unreadable form)
        match result {
            Ok((remaining, _)) => {
                assert!(!remaining.is_empty(), ":/foo is unreadable, should have remaining input or error");
            }
            Err(_) => {
                // Also acceptable - rejection is fine
            }
        }
    }

    #[test]
    fn keyword_double_colon_slash_with_following_identifier_rejected() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "::/foo";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert - should error
        assert!(result.is_err(), "::/foo should be rejected as unreadable");
    }

    #[test]
    fn keyword_qualified_missing_name_rejected() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = ":foo/";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        assert!(result.is_err(), ":foo/ is unreadable, missing name after /");
    }

    #[test]
    fn keyword_double_colon_qualified_missing_name_rejected() {
        // arrange
        let env = Environment::new_empty_rc();
        let input = "::foo/";
        // act
        let result = parse::try_parse_keyword(env, input);
        // assert
        assert!(result.is_err(), "::foo/ is unreadable, missing name after /");
    }

    // Empty list tests
    #[test]
    fn empty_list() {
        let env = Environment::new_empty_rc();
        let input = "()";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space() {
        let env = Environment::new_empty_rc();
        let input = "( )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_comma() {
        let env = Environment::new_empty_rc();
        let input = "(,)";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "(, )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with comma and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space_comma() {
        let env = Environment::new_empty_rc();
        let input = "( ,)";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with space and comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    #[test]
    fn empty_list_with_space_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "( , )";
        let result = parse::try_parse_list(env, input);
        let (remaining, value) = result.expect("should parse empty list with space, comma, and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_list().expect("should be a list");
        assert!(items.is_empty(), "list should be empty");
    }

    // Empty vector tests
    #[test]
    fn empty_vector() {
        let env = Environment::new_empty_rc();
        let input = "[]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space() {
        let env = Environment::new_empty_rc();
        let input = "[ ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_comma() {
        let env = Environment::new_empty_rc();
        let input = "[,]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "[, ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with comma and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space_comma() {
        let env = Environment::new_empty_rc();
        let input = "[ ,]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with space and comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    #[test]
    fn empty_vector_with_space_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "[ , ]";
        let result = parse::try_parse_vector(env, input);
        let (remaining, value) = result.expect("should parse empty vector with space, comma, and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_vector().expect("should be a vector");
        assert!(items.is_empty(), "vector should be empty");
    }

    // Empty set tests
    #[test]
    fn empty_set() {
        let env = Environment::new_empty_rc();
        let input = "#{}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space() {
        let env = Environment::new_empty_rc();
        let input = "#{ }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_comma() {
        let env = Environment::new_empty_rc();
        let input = "#{,}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "#{, }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with comma and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space_comma() {
        let env = Environment::new_empty_rc();
        let input = "#{ ,}";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with space and comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    #[test]
    fn empty_set_with_space_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "#{ , }";
        let result = parse::try_parse_set(env, input);
        let (remaining, value) = result.expect("should parse empty set with space, comma, and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_set().expect("should be a set");
        assert!(items.is_empty(), "set should be empty");
    }

    // Empty map tests
    #[test]
    fn empty_map() {
        let env = Environment::new_empty_rc();
        let input = "{}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space() {
        let env = Environment::new_empty_rc();
        let input = "{ }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_comma() {
        let env = Environment::new_empty_rc();
        let input = "{,}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "{, }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with comma and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space_comma() {
        let env = Environment::new_empty_rc();
        let input = "{ ,}";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with space and comma");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    #[test]
    fn empty_map_with_space_comma_space() {
        let env = Environment::new_empty_rc();
        let input = "{ , }";
        let result = parse::try_parse_map(env, input);
        let (remaining, value) = result.expect("should parse empty map with space, comma, and space");
        assert!(remaining.is_empty());
        let (items, _meta) = value.try_as_map().expect("should be a map");
        assert!(items.is_empty(), "map should be empty");
    }

    // Tests for parsing keywords followed by collections
    #[test]
    fn keyword_followed_by_list() {
        let env = Environment::new_empty_rc();
        let input = ":foo(bar)";
        let result = parse::try_parse_keyword(env.clone(), input);
        let (remaining, value) = result.expect("should parse keyword");
        assert_eq!(remaining, "(bar)");
        let (keyword, _meta) = value.try_as_keyword().expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "foo");

        // Parse the remaining list
        let list_result = parse::try_parse_list(env, remaining);
        let (list_remaining, list_value) = list_result.expect("should parse list");
        assert!(list_remaining.is_empty());
        let (items, _meta) = list_value.try_as_list().expect("should be a list");
        assert_eq!(items.len(), 1, "list should contain one item");
        let item = items.first().unwrap();
        let (symbol, _meta) = item.try_as_symbol().expect("item should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn keyword_followed_by_vector() {
        let env = Environment::new_empty_rc();
        let input = ":foo[bar]";
        let result = parse::try_parse_keyword(env.clone(), input);
        let (remaining, value) = result.expect("should parse keyword");
        assert_eq!(remaining, "[bar]");
        let (keyword, _meta) = value.try_as_keyword().expect("should be a keyword");
        assert!(keyword.is_unqualified(), "keyword should be unqualified");
        assert_eq!(keyword.name(), "foo");

        // Parse the remaining vector
        let vector_result = parse::try_parse_vector(env, remaining);
        let (vector_remaining, vector_value) = vector_result.expect("should parse vector");
        assert!(vector_remaining.is_empty());
        let (items, _meta) = vector_value.try_as_vector().expect("should be a vector");
        assert_eq!(items.len(), 1, "vector should contain one item");
        let item = items.first().unwrap();
        let (symbol, _meta) = item.try_as_symbol().expect("item should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn symbol_followed_by_vector() {
        let env = Environment::new_empty_rc();
        let input = "foo[bar]";
        let result = parse::try_parse_symbol(env.clone(), input);
        let (remaining, value) = result.expect("should parse symbol");
        assert_eq!(remaining, "[bar]");
        let (symbol, _meta) = value.try_as_symbol().expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "foo");

        // Parse the remaining vector
        let vector_result = parse::try_parse_vector(env, remaining);
        let (vector_remaining, vector_value) = vector_result.expect("should parse vector");
        assert!(vector_remaining.is_empty());
        let (items, _meta) = vector_value.try_as_vector().expect("should be a vector");
        assert_eq!(items.len(), 1, "vector should contain one item");
        let item = items.first().unwrap();
        let (symbol, _meta) = item.try_as_symbol().expect("item should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }

    #[test]
    fn symbol_followed_by_list() {
        let env = Environment::new_empty_rc();
        let input = "foo(bar)";
        let result = parse::try_parse_symbol(env.clone(), input);
        let (remaining, value) = result.expect("should parse symbol");
        assert_eq!(remaining, "(bar)");
        let (symbol, _meta) = value.try_as_symbol().expect("should be a symbol");
        assert!(symbol.is_unqualified(), "symbol should be unqualified");
        assert_eq!(symbol.name(), "foo");

        // Parse the remaining list
        let list_result = parse::try_parse_list(env, remaining);
        let (list_remaining, list_value) = list_result.expect("should parse list");
        assert!(list_remaining.is_empty());
        let (items, _meta) = list_value.try_as_list().expect("should be a list");
        assert_eq!(items.len(), 1, "list should contain one item");
        let item = items.first().unwrap();
        let (symbol, _meta) = item.try_as_symbol().expect("item should be a symbol");
        assert_eq!(symbol.name(), "bar");
    }
}