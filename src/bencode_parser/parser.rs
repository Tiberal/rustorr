use error_stack::{Report, Result};
use error_stack::ResultExt;

use crate::bencode_parser::bencode_values::Value;
use crate::bencode_parser::parser_error::ParserError;
use crate::bencode_parser::values_collector::{ArrayCollector, Collector, CollectorResult, DictionaryCollector};

pub(crate) struct Parser {
    result: Vec<Value>,
    current_position: usize,
    collectors: Vec<Box<dyn Collector>>,
    encoded_bytes: Vec<u8>,
}

impl Parser {
    pub fn new(encoded_bytes: Vec<u8>) -> Self {
        Self {
            result: vec![],
            current_position: 0,
            collectors: vec![],
            encoded_bytes,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Value>, ParserError> {
        while self.current_position < self.encoded_bytes.len() {
            let current_slice = &self.encoded_bytes[self.current_position..];
            let char = std::char::from_u32(current_slice[0] as u32);

            let iteration_position_offset = match char {
                Some(char) if char.is_digit(10) => {
                    let iteration_result = decode_string(current_slice)?;
                    let (string, current_position_offset) = iteration_result;
                    let string_value = Value::String(string);

                    self.push_value(string_value)?;

                    current_position_offset
                }
                Some(char) if char == 'i' => {
                    let iteration_result = decode_number(current_slice)?;
                    let (number, current_position_offset) = iteration_result;
                    let number_value = Value::Number(number);

                    self.push_value(number_value)?;

                    current_position_offset
                }
                Some(char) if char == 'd' => {
                    let collector = DictionaryCollector::new();
                    self.collectors.push(Box::new(collector));

                    1
                }
                Some(char) if char == 'l' => {
                    let collector = ArrayCollector::new();
                    self.collectors.push(Box::new(collector));

                    1
                }
                Some(char) if char == 'e' => {
                    let value = self.get_last_collector_as_value()?;
                    self.push_value(value)?;

                    1
                }
                _ => return Err(
                    Report::new(ParserError::UndefinedTokenError)
                        .attach_printable(format!("Undefined encoded value: {:?}", char))
                )
            };

            self.current_position += iteration_position_offset;
        }

        Ok(self.result)
    }

    fn get_last_collector_as_value(&mut self) -> Result<Value, ParserError> {
        let current_collector = self.collectors
            .pop()
            .ok_or(ParserError::StructParseError)
            .attach_printable("There is no collector for a struct")?;

        Ok(
            match current_collector.result() {
                CollectorResult::Array(vec) => Value::Array(vec),
                CollectorResult::Dictionary(map) => Value::Dictionary(map),
            }
        )
    }

    fn push_value(&mut self, value: Value) -> Result<(), ParserError> {
        if self.collectors.is_empty() {
            self.result.push(value);
        } else {
            let collector = self.collectors.last_mut()
                .ok_or(ParserError::StructParseError)
                .attach_printable("There is no collector for a struct")?;

            collector.insert(value);
        }
        Ok(())
    }
}

fn decode_number(encoded_value: &[u8]) -> Result<(i64, usize), ParserError> {
    let decoded_number_end_index = encoded_value.iter().position(|&x| x == b'e')
        .ok_or(ParserError::NumberParseError)
        .attach_printable("Can't find end of a number")
        .change_context(ParserError::NumberParseError)?;

    let number_byte_slice = &encoded_value[1..decoded_number_end_index];

    let number = String::from_utf8_lossy(number_byte_slice).parse::<i64>()
        .attach_printable("Can't convert number a byte slice to i64")
        .change_context(ParserError::NumberParseError)?;

    return Ok((number, decoded_number_end_index + 1));
}

fn decode_string(encoded_value: &[u8]) -> Result<(String, usize), ParserError> {
    let colon_index = encoded_value.iter().position(|&x| x == b':')
        .ok_or(ParserError::StringParseError)
        .attach_printable("Can't find start of a string")
        .change_context(ParserError::StringParseError)?;

    let decoded_string_len_slice = &encoded_value[..colon_index];
    let decoded_string_len = String::from_utf8_lossy(decoded_string_len_slice).parse::<i64>()
        .attach_printable("Can't get a string size")
        .change_context(ParserError::StringParseError)?;

    let decoded_string_end_position = colon_index + 1 + decoded_string_len as usize;
    let string_slice = &encoded_value[colon_index + 1..decoded_string_end_position];

    return Ok((String::from_utf8_lossy(string_slice).to_string(), decoded_string_end_position));
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;

    use crate::bencode_parser::bencode_values::Value;
    use crate::bencode_parser::parser::Parser;

    #[test]
    fn dict_test() {
        let template = "d3:cow3:moo4:spam4:eggse";

        let mut parser = Parser::new(Vec::from(template.as_bytes()));

        println!("template = {:?}", template);

        let actual = parser.parse();

        println!("result {:?}", actual);
    }

    #[test]
    fn list_test() {
        let template = "l4:spam4:eggse";

        let mut parser = Parser::new(Vec::from(template.as_bytes()));

        println!("template = {:?}", template);

        let actual = parser.parse();

        println!("result {:?}", actual);
    }

    #[test]
    fn u8_test() {
        let byte_data = read_non_utf8_file().unwrap();

        let parser = Parser::new(byte_data);

        let result = parser.parse().unwrap();

        for data in result {
            match data {
                Value::Dictionary(dict) => {
                    for (key, value) in dict {
                        println!("key: {:?} value: {:?}", key, value);
                    }
                }
                _ => {}
            }
        }
    }

    fn read_non_utf8_file() -> io::Result<Vec<u8>> {
        let buffer = std::fs::read("sample.torrent")
            .expect("Something went wrong reading the file");
        Ok(buffer)
    }
}