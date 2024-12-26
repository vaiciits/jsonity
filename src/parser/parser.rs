pub const CHAR_QUOTE: char = '"';

pub struct Parser<'a> {
    index: usize,
    chars: &'a Vec<char>,
}

impl Parser<'_> {
    pub fn new(chars: &Vec<char>) -> Parser {
        Parser {
            index: 0,
            chars,
        }
    }

    pub(crate) fn parse(&self) {

    }
}