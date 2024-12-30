use std::collections::HashMap;
use crate::data_structures::{Element, ElementTrait};

pub struct ObjectElement {
    pub elements: HashMap<String, Element>,
}
impl ObjectElement {
    pub fn new() -> ObjectElement {
        ObjectElement {
            elements: HashMap::new(),
        }
    }
}
impl ElementTrait for ObjectElement {
    fn decode(&self) -> String {
        "decoding".to_string() // TODO implement from dictionary
    }
}