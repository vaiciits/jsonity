use crate::data_structures::string_element::StringElement;
use crate::parser::parser::{CHAR_ESCAPE, CHAR_QUOTE};

pub struct StringParser {}

impl StringParser {
    pub(crate) fn parse(chars: &Vec<char>, mut index: usize) -> (StringElement, usize) {
        let mut escaped: bool = false;
        let mut result: Vec<char> = Vec::new();
        let length: usize = chars[index..].len();

        for (i, &char) in chars[index..].iter().enumerate() {
            if i + 1 == length {
                if escaped || char != CHAR_QUOTE {
                    panic!("Invalid string value.");
                }
            }

            index = i;

            if !escaped && char == CHAR_ESCAPE {
                escaped = true;
                continue;
            }

            if (char == CHAR_QUOTE) && !escaped {
                break;
            }

            escaped = false;
            result.push(char);
        }

        (
            StringElement {
                value: result.iter().collect(),
            },
            index,
        )
    }
}
