use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub(crate) enum ParserError {
    NumberParseError,
    StringParseError,
    StructParseError,
    UndefinedTokenError,
    ValueDataExtractionError,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParserError::NumberParseError => "Number parse error",
            ParserError::StringParseError => "String parse error",
            ParserError::StructParseError => "Struct parse error",
            ParserError::UndefinedTokenError => "Undefined token error",
            ParserError::ValueDataExtractionError => "Value data extraction error",
        };
        write!(f, "{msg}")
    }
}

impl Context for ParserError {}