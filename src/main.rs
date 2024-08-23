use crate::bencode_parser::parser::Parser;

mod bencode_parser;


fn main() {
    println!("Hello, world!");

    let template = "d3:cow3:moo4:spam4:eggse";
    let parser = Parser::new(template.to_string());
    let result = parser.parse();

    println!("result {:?}", result);
}