#[cfg(test)]
mod parser_tests {
    use crate::data_structures::Element::String;
    use crate::parser::parser::Parser;

    #[test]
    fn parse_with_strings() {
        let inputs: Vec<(&str, &str)> = vec![
            ("\"\"", ""),
            ("\"foo\"", "foo"),
            ("\"foo\\\\\\\\\\\"bar\"", "foo\\\\\"bar"),
            ("  \"foo\"  ", "foo")
        ];

        for (input, expected) in inputs {
            let chars: Vec<char> = input.chars().collect::<Vec<char>>();
            let mut parser: Parser = Parser::new(&chars);
            match parser.parse() {
                String(string) => assert_eq!(string.value, expected),
                _ => assert!(false),
            }
        }
    }
}