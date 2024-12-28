#[cfg(test)]
mod parser_tests {
    use crate::data_structures::Element;
    use crate::data_structures::Element::String;
    use crate::parser::parser::Parser;

    fn parse_string(input: &str) -> Element {
        Parser::new(&input.chars().collect()).parse()
    }

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

    macro_rules! invalid_string_cases {
        ($($name:ident: $input:expr), *) => {
            $(
                #[test]
                #[should_panic]
                fn $name() {
                    parse_string($input);
                }
            )*
        };
    }

    invalid_string_cases! {
        parse_with_invalid_string_having_escaped_quote_as_last: "\"foo\\\"",
        parse_with_invalid_string_having_unclosed_quotation: "\"foo\\bar"
    }
}