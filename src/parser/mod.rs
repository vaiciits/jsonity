use crate::parser::parser::Parser;

mod parser;

pub fn decode_json(input: String) -> String {
    let chars: Vec<char> = input.chars().collect();
    let parser: Parser = Parser::new(&chars);
    parser.parse();

    input
}