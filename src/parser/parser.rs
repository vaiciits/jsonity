use crate::data_structures::object::ObjectElement;
use crate::data_structures::string_element::StringElement;
use crate::data_structures::Element;
use crate::data_structures::Element::{Bool, Object, StringCase};
use crate::parser::string_parser::StringParser;
use std::collections::HashMap;
use crate::data_structures::bool_element::BoolElement;

pub const CHAR_SPACE: char = ' ';
pub const CHAR_TAB: char = '\t';
pub const CHAR_NEWLINE: char = '\n';

pub const CHAR_QUOTE: char = '"';
pub const CHAR_ESCAPE: char = '\\';

pub const CHAR_CURLY_LEFT: char = '{';
pub const CHAR_CURLY_RIGHT: char = '}';
pub const CHAR_COLON: char = ':';
pub const CHAR_COMMA: char = ',';

pub const CHAR_F: char = 'f';
pub const CHAR_T: char = 't';

pub struct Parser<'a> {
    index: usize,
    chars: &'a Vec<char>,
    length: usize,
    whitespaces: HashMap<char, ()>,
}

impl Parser<'_> {
    fn increase_by_one(&mut self) {
        self.index += 1;

        if self.index >= self.length {
            panic!("Input too short.");
        }
    }

    pub fn new(chars: &Vec<char>) -> Parser {
        Parser {
            index: 0,
            chars,
            length: chars.len(),
            whitespaces: HashMap::new(),
        }
    }

    pub(crate) fn parse(&mut self) -> Element {
        match self.chars[self.next()] {
            CHAR_QUOTE => self.parse_string(),
            CHAR_CURLY_LEFT => self.parse_object(),
            CHAR_F | CHAR_T => self.parse_bool(),
            _ => panic!("Unexpected character. JSON input is invalid."),
        }
    }

    fn parse_bool(&mut self) -> Element {
        let target: bool = match self.chars[self.index] {
            CHAR_F => false,
            CHAR_T => true,
            _ => panic!("Unexpected character for boolean value."),
        };
        let looking_for: String = target.to_string();
        let length: usize = looking_for.len();

        if self.index + length - 1 >= self.length {
            panic!("Input too short for {}.", looking_for);
        }

        let value: String = self.chars[self.index..self.index + length].iter().collect();
        if value != looking_for {
            panic!("Unexpected character for boolean value.");
        }

        self.index += length;

        Bool(BoolElement { value: target})
    }

    fn parse_object(&mut self) -> Element {
        let mut object: ObjectElement = ObjectElement::new();
        self.increase_by_one();

        while self.chars[self.next()] != CHAR_CURLY_RIGHT {
            let key_element: Element = self.parse_string();
            let key: String = Element::get_value_from_string_element(key_element);
            self.increase_by_one();

            if self.chars[self.next()] != CHAR_COLON {
                let char_string: String = self.chars.iter().collect();
                let error_message: String = format!(
                    "Colon was expected at place: {}, {}",
                    self.index, char_string
                );
                panic!("{}", error_message);
            }

            self.increase_by_one();
            let value: Element = self.parse();
            object.add_element(key, value);
            self.increase_by_one();

            if self.chars[self.next()] != CHAR_COMMA {
                break;
            }

            self.increase_by_one();
        }

        Object(object)
    }

    fn parse_string(&mut self) -> Element {
        let (element, index): (StringElement, usize) =
            StringParser::parse(self.chars, self.index + 1);
        self.index += index + 1;

        StringCase(element)
    }

    fn next(&mut self) -> usize {
        while self.index < self.length
            && self.get_whitespaces().contains_key(&self.chars[self.index])
        {
            self.index += 1;
        }

        self.index
    }

    fn get_whitespaces(&mut self) -> HashMap<char, ()> {
        if self.whitespaces.len() == 0 {
            self.whitespaces.insert(CHAR_SPACE, ());
            self.whitespaces.insert(CHAR_NEWLINE, ());
            self.whitespaces.insert(CHAR_TAB, ());
        }

        self.whitespaces.clone()
    }
}
