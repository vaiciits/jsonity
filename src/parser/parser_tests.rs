#[cfg(test)]
mod parser_tests {
    use std::collections::HashMap;
    use crate::data_structures::Element;
    use crate::data_structures::Element::{StringCase, Object};
    use crate::data_structures::string_element::StringElement;
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
            ("  \t\"foo\" \n ", "foo"),
        ];

        for (input, expected) in inputs {
            match parse_string(input) {
                StringCase(string) => assert_eq!(string.value, expected),
                _ => assert!(false),
            }
        }
    }

    macro_rules! invalid_string_cases {
        ($($name:ident: $input:expr), *) => {
            $(
                #[test]
                #[should_panic(expected = "Invalid string value.")]
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

    macro_rules! test_parse_with_object {
        ($($name:ident: $input:expr, $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    let expected: HashMap<String, Element> = HashMap::from_iter($expected);

                    match parse_string($input) {
                        Object(object) => {
                            let expected_keys: Vec<&String> = expected.keys().collect();
                            let actual_keys: Vec<&String> = object.elements.keys().collect();
                            assert_eq!(expected_keys, actual_keys);
                            // TODO compare values - recursive
                        },
                        _ => assert!(false),
                    }
                }
            )*
        };
    }

    test_parse_with_object! {
        test_parse_empty: "{}", [],
        test_parse_string: "{\"foo\":\"bar\"}", [("foo".to_string(), StringCase(StringElement { value: "bar".to_string() }))]
    }
}