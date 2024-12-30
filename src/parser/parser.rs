use crate::data_structures::object::ObjectElement;
use crate::data_structures::string_element::StringElement;
use crate::data_structures::Element;
use crate::parser::string_parser::StringParser;
use std::collections::HashMap;
use crate::data_structures::Element::Object;

pub const CHAR_SPACE: char = ' ';
pub const CHAR_TAB: char = '\t';
pub const CHAR_NEWLINE: char = '\n';

pub const CHAR_QUOTE: char = '"';
pub const CHAR_ESCAPE: char = '\\';

pub const CHAR_CURLY_LEFT: char = '{';
pub const CHAR_CURLY_RIGHT: char = '}';

pub struct Parser<'a> {
    index: usize,
    chars: &'a Vec<char>,
    length: usize,
    whitespaces: HashMap<char, ()>,
}

impl Parser<'_> {
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
            _ => panic!("Unexpected character. JSON input is invalid."),
        }
    }


    fn parse_object(&mut self) -> Element {
        let object: ObjectElement = ObjectElement::new();

        while self.chars[self.next()] != CHAR_CURLY_RIGHT {
            break;
        }

        Object(object)
    }

    fn parse_string(&mut self) -> Element {
        let (element, index): (StringElement, usize) =
            StringParser::parse(self.chars, self.index + 1);
        self.index = index;

        Element::StringCase(element)
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
