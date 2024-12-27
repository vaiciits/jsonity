use std::collections::HashMap;
use crate::data_structures::{Element, ElementTrait};

pub struct Object {
    pub elements: HashMap<String, Element>,
}
impl Object {
    pub fn new() -> Object {
        Object {
            elements: HashMap::new(),
        }
    }
}
impl ElementTrait for Object {
    fn decode(&self) -> String {
        "decoding".to_string() // TODO implement from dictionary
    }
}