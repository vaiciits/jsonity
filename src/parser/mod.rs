use crate::data_structures::Element;
use crate::parser::parser::Parser;

mod parser;
mod string_parser;

pub fn decode_json(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut parser: Parser = Parser::new(&chars);
    let element: Element = parser.parse();

    match element {
        Element::String(str) => str.value,
    }
}