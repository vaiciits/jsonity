#[cfg(test)]
mod parser_tests {
    use crate::data_structures::string_element::StringElement;
    use crate::data_structures::Element;
    use crate::data_structures::Element::{Object, StringCase};
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

    fn parse_object(input: &str, expected_vec: Vec<(String, Element)>) {
        match parse_string(input) {
            Object(object) => {
                let expected_keys: Vec<&String> = expected_vec.iter().map(|&(ref k, _)| k).collect();
                let actual_keys: Vec<&String> = object.get_elements().iter().map(|&(ref k, _)| k).collect();
                assert_eq!(expected_keys, actual_keys);
                // TODO compare values - recursive
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parse_with_object_empty() {
        parse_object("{}", vec![]);
    }

    #[test]
    fn test_parse_with_object_having_single_property() {
        parse_object(
            "{\"foo\":\"bar\"}",
            vec![(
                "foo".to_string(),
                StringCase(StringElement {
                    value: "bar".to_string(),
                }),
            )],
        );
    }

    #[test]
    fn test_parse_with_object_having_multiple_properties_not_in_order() {
        parse_object(
            "{   \"foo\" :   \"bar\" ,   \" baz \": \"foo\\\"baz\\\"bar\"   ,\"abc\":\"def\"}",
            vec![
                (
                    "foo".to_string(),
                    StringCase(StringElement {
                        value: "bar".to_string(),
                    }),
                ),
                (
                    " baz ".to_string(),
                    StringCase(StringElement {
                        value: "foo\"baz\"bar".to_string(),
                    }),
                ),
                (
                    "abc".to_string(),
                    StringCase(StringElement {
                        value: "def".to_string(),
                    }),
                ),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "Key foo already exists.")]
    fn test_parse_with_object_having_repeating_properties() {
        parse_object(
            "{\"foo\":\"bar\",\"foo\":\"baz\"}",
            vec![(
                "foo".to_string(),
                StringCase(StringElement {
                    value: "bar".to_string(),
                }),
            )],
        );
    }

    fn compare_bool(input: &str, expected: bool) {
        let element: Element = parse_string(input);

        if let Element::Bool(ref bool_element) = element {
            assert_eq!(expected, bool_element.value);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_parse_bool_true() {
        compare_bool("true", true);
    }

    #[test]
    fn test_parse_bool_false() {
        compare_bool("false", false);
    }

    #[test]
    #[should_panic(expected = "Unexpected character for boolean value.")]
    fn test_parse_bool_incorrect_value() {
        parse_string("faire");
    }

    #[test]
    #[should_panic(expected = "Input too short for true")]
    fn test_parse_bool_too_short() {
        parse_string("tr");
    }
}
