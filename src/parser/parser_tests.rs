#[cfg(test)]
mod parser_tests {
    use crate::data_structures::Element;
    use crate::data_structures::Element::String;
    use crate::parser::parser::Parser;

    #[test]
    fn parse_with_strings() {
        let inputs: Vec<(&str, &str)> = vec![
            ("\"\"", ""),
            ("\"foo\"", "foo"),
            ("\"foo\\\\\\\\\\\"bar\"", "foo\\\\\"bar"),
            ("  \"foo\"  ", "foo"),
        ];

        for (input, expected) in inputs {
            match parse_string(input) {
                String(string) => assert_eq!(string.value, expected),
                _ => assert!(false),
            }
        }
    }

    fn parse_string(input: &str) -> Element {
        Parser::new(&input.chars().collect()).parse()
    }

    #[test]
    #[should_panic]
    fn parse_with_invalid_string_with_escaped_quote_as_last() {
        parse_string("\"foo\\\"");
    }

    #[test]
    #[should_panic]
    fn parse_with_invalid_string_with_unclosed_quote() {
        parse_string("\"foo\\bar");
    }
}