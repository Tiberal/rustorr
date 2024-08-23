use crate::bencode_parser::bencode_values::Value;
use crate::bencode_parser::values_collector::{ArrayCollector, Collector, CollectorResult, DictionaryCollector};

pub(crate) struct Parser {
    result: Vec<Value>,
    current_position: usize,
    collectors: Vec<Box<dyn Collector>>,
    encoded_string: String,
}

impl Parser {
    pub fn new(encoded_string: String) -> Self {
        Self {
            result: vec![],
            current_position: 0,
            collectors: vec![],
            encoded_string,
        }
    }

    pub fn parse(mut self) -> Vec<Value> {
        while self.current_position < self.encoded_string.len() {
            let current_slice = &self.encoded_string[self.current_position..];

            let iteration_position_offset = match current_slice.chars().next() {
                Some(char) if char.is_digit(10) => {
                    let iteration_result = decode_string(current_slice);
                    let (string, current_position_offset) = iteration_result;
                    let string_value = Value::String(string);

                    self.push_value(string_value);

                    current_position_offset
                }
                Some(char) if char == 'i' => {
                    let iteration_result = decode_int(current_slice);
                    let (number, current_position_offset) = iteration_result;
                    let number_value = Value::Number(number);

                    self.push_value(number_value);

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
                    let value = self.get_last_collector_as_value();
                    self.push_value(value);

                    1
                }
                _ => panic!("Unhandled encoded value: {:?}", current_slice.chars().next())
            };

            self.current_position += iteration_position_offset;
        }

        self.result
    }

    fn get_last_collector_as_value(&mut self) -> Value {
        let collector_result = self.collectors
            .pop()
            .map(|collector| collector.result())
            .unwrap();

        match collector_result {
            CollectorResult::Array(vec) => Value::Array(vec),
            CollectorResult::Dictionary(map) => Value::Dictionary(map),
        }
    }

    fn push_value(&mut self, value: Value) {
        if self.collectors.is_empty() {
            self.result.push(value)
        } else {
            self.collectors.last_mut().unwrap().insert(value);
        }
    }
}

fn decode_string(encoded_value: &str) -> (String, usize) {
    let colon_index = encoded_value.find(':').unwrap();
    let decoded_string_len_string = &encoded_value[..colon_index];
    let decoded_string_len = decoded_string_len_string.parse::<i64>().unwrap();

    let decoded_string_end_position = colon_index + 1 + decoded_string_len as usize;
    let string = &encoded_value[colon_index + 1..decoded_string_end_position];

    return (string.to_string(), decoded_string_end_position);
}

fn decode_int(encoded_value: &str) -> (i64, usize) {
    let decoded_number_end_position = encoded_value.find('e').unwrap();
    let number_string = &encoded_value[1..decoded_number_end_position];
    let number = number_string.parse::<i64>().unwrap();

    return (number, decoded_number_end_position + 1);
}

#[cfg(test)]
mod tests {
    use crate::bencode_parser::parser::Parser;

    #[test]
    fn dict_test() {
        let template = "d3:cow3:moo4:spam4:eggse";

        let mut parser = Parser::new(template.to_string());

        println!("template = {:?}", template);

        let actual = parser.parse();

        println!("result {:?}", actual);
    }

    #[test]
    fn list_test() {
        let template = "l4:spam4:eggse";

        let mut parser = Parser::new(template.to_string());

        println!("template = {:?}", template);

        let actual = parser.parse();

        println!("result {:?}", actual);
    }
}