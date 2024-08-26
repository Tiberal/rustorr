use error_stack::{Report, Result};
use indexmap::IndexMap;

use crate::bencode_parser::parser_error::ParserError;

#[derive(Clone, Debug)]
pub(crate) enum Value {
    String(String),
    Number(i64),
    Array(Vec<Value>),
    Dictionary(IndexMap<String, Value>),
}

impl Value {
    pub(crate) fn as_string(&self) -> Result<String, ParserError> {
        match self {
            Value::String(value) => Ok(value.clone()),
            _ => value_extraction_error(format!("Fail to extract string: {:?}", self))
        }
    }

    pub(crate) fn as_number(&self) -> Result<i64, ParserError> {
        match self {
            Value::Number(value) => Ok(*value),
            _ => value_extraction_error(format!("Fail to extract number: {:?}", self))
        }
    }

    //todo remove dead code attribute
    #[allow(dead_code)]
    pub(crate) fn as_array(&self) -> Result<&Vec<Value>, ParserError> {
        match self {
            Value::Array(value) => Ok(value),
            _ => value_extraction_error(format!("Fail to extract array: {:?}", self))
        }
    }

    pub(crate) fn as_dictionary(&self) -> Result<&IndexMap<String, Value>, ParserError> {
        match self {
            Value::Dictionary(value) => Ok(value),
            _ => value_extraction_error(format!("Fail to extract dictionary: {:?}", self))
        }
    }
}

fn value_extraction_error<T>(printable: String) -> Result<T, ParserError> {
    Err(Report::new(ParserError::ValueDataExtractionError).attach_printable(printable))
}