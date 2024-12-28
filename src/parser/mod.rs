use crate::data_structures::{Element, ElementTrait};
use crate::parser::parser::Parser;

mod parser;
mod string_parser;
mod object_parser;
mod parser_tests;

pub fn decode_json(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut parser: Parser = Parser::new(&chars);
    let element: Element = parser.parse();

    match element {
        Element::String(string_element) => string_element.decode(),
        Element::Object(object) => object.decode(),
    }
}