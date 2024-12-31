use std::collections::HashMap;
use crate::data_structures::{Element, ElementTrait};

pub struct ObjectElement {
    elements: HashMap<String, Element>,
}
impl ObjectElement {
    pub fn new() -> ObjectElement {
        ObjectElement {
            elements: HashMap::new(),
        }
    }
    pub(crate) fn add_element(&mut self, key: String, element: Element) {
        if self.elements.contains_key(&key) {
            panic!("Key {} already exists.", key);
        }

        self.elements.insert(key, element);
    }

    pub(crate) fn get_elements(&self) -> &HashMap<String, Element> {
        &self.elements
    }
}
impl ElementTrait for ObjectElement {
    fn decode(&self) -> String {
        "decoding".to_string() // TODO implement from dictionary
    }
}