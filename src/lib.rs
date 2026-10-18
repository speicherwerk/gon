pub mod gon_grammar;
pub mod gon_grammar_trait;
#[cfg(feature = "json")]
pub mod json;
pub mod parser;
pub mod value;

use parol_runtime::{ParolError, Report};
///! Parser for a simple JSON-like format that doesn't require quotes around keys and
///! allows trailing commas (but requires non at all).
pub use value::{List, Object, SpellConfig, Value};

use std::{collections::HashMap, path::Path};

use thiserror::Error;

use crate::gon_grammar::GonGrammar;

pub struct GonErrorReporter;
impl Report for GonErrorReporter {}

pub fn parse(s: &str) -> Result<Value, GonError> {
    let mut gon_grammar = &mut GonGrammar::new();
    match parser::parse(s, "", &mut gon_grammar) {
        Ok(_) => Ok(gon_grammar.value.clone().unwrap().into()),
        Err(e) => Err(GonError::ParserErr(e)),
    }
}

pub fn parse_file<P: AsRef<Path>>(src: &str, path: P) -> Result<Value, GonError> {
    let mut gon_grammar = &mut GonGrammar::new();
    match parser::parse(src, path, &mut gon_grammar) {
        Ok(_) => Ok(gon_grammar.value.clone().unwrap().into()),
        Err(e) => Err(GonError::ParserErr(e)),
    }
}

/// The map type when the `preserve_order` feature is on
#[cfg(feature = "preserve_order")]
pub type MapT = indexmap::IndexMap<String, Value>;
/// The map type when the `preserve_order` feature is off
#[cfg(not(feature = "preserve_order"))]
pub type MapT = HashMap<String, Value>;

/// Something went wrong?!
#[derive(Debug, Error)]
pub enum GonError {
    /// An error from the parser
    #[error("couldn't parse")]
    ParserErr(#[from] ParolError),
    /// Attempted to parse the empty string
    #[error("no value present")]
    IOErr(#[from] std::io::Error),
}

impl PartialEq for GonError {
    fn eq(&self, _: &Self) -> bool {
        // error = error = error
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert!(parse("").is_err());
    }

    #[test]
    fn single_value_none() {
        assert_eq!(parse("None"), Ok(Value::None));
        assert_eq!(parse("none"), Ok(Value::None));
        assert_eq!(parse("null"), Ok(Value::None));
    }

    #[test]
    fn single_value_str() {
        assert_eq!(parse("\"hello\""), Ok(Value::Str("hello".into())));
    }

    #[test]
    fn single_value_num() {
        assert_eq!(parse("3.14"), Ok(Value::Num("3.14".into())));
        assert_eq!(parse("0"), Ok(Value::Num("0".into())));
        assert_eq!(parse("-99999"), Ok(Value::Num("-99999".into())));
    }

    #[test]
    fn single_value_bool() {
        assert_eq!(parse("true"), Ok(Value::Bool(true)));
        assert_eq!(parse("false"), Ok(Value::Bool(false)));
    }

    #[test]
    fn single_value_obj() {
        assert_eq!(parse("{}"), Ok(Value::Obj(MapT::new())));
        let a = Value::Obj(HashMap::from([(
            String::from("pi"),
            Value::Num(String::from("3.14")),
        )]));
        assert_eq!(parse("{pi: 3.14}"), Ok(a));
        let b = Value::Obj(HashMap::from([(
            String::from("name"),
            Value::Str("gon".into()),
        )]));
        assert_eq!(parse("  {\n    name:\n\t\"gon\"\n\n\n\t\t}"), Ok(b));
    }
    #[test]
    fn single_value_list() {
        assert_eq!(parse("[]"), Ok(Value::List(Vec::new())));
        assert_eq!(
            parse("[2.71]"),
            Ok(Value::List(vec![Value::Num(String::from("2.71"))]))
        );
        assert_eq!(
            parse("[\n\nfalse\t,]"),
            Ok(Value::List(vec![Value::Bool(false)]))
        );
    }

    #[test]
    fn many_values() {
        let name = Value::Obj(HashMap::from([
            (String::from("first"), Value::Str("John".into())),
            (String::from("last"), Value::Str("Doe".into())),
        ]));
        let address = Value::Obj(HashMap::from([
            (String::from("street"), Value::Str("Wood Way".into())),
            (String::from("house"), Value::Num(String::from("-9_000"))),
        ]));
        let friends = Value::List(vec![
            Value::Obj(HashMap::from([(
                String::from("name"),
                Value::Str("Alice".into()),
            )])),
            Value::Obj(HashMap::from([(
                String::from("name"),
                Value::Str("Bob".into()),
            )])),
        ]);
        let obj = Value::Obj(HashMap::from([
            (String::from("id"), Value::Num(String::from("456"))),
            (String::from("name"), name),
            (String::from("address"), address),
            (String::from("alive"), Value::Bool(true)),
            (String::from("friends"), friends),
        ]));
        assert_eq!(
            parse(
                r#"{
            id: 456,
            name: {
                first: "John",
                last: "Doe",
            },
            address: {
                street: "Wood Way",
                house: -9_000,
            },
            alive: true,
            friends: [
                {name: "Alice",},
                {
                    name: "Bob"
                },
            ]
        }"#
            ),
            Ok(obj)
        );
    }
}
