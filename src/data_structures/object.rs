use crate::data_structures::{Element, ElementTrait};

pub struct ObjectElement {
    elements: Vec<(String, Element)>,
}
impl ObjectElement {
    pub fn new() -> ObjectElement {
        ObjectElement {
            elements: Vec::new(),
        }
    }
    pub(crate) fn add_element(&mut self, key: String, element: Element) {
        if self.contains_key(&key) {
            panic!("Key {} already exists.", key);
        }

        self.elements.push((key, element));
    }

    fn contains_key(&self, key: &String) -> bool {
        for (key_value, _element) in &self.elements {
            if key_value == key {
                return true;
            }
        }

        false
    }

    pub(crate) fn get_elements(&self) -> &Vec<(String, Element)> {
        &self.elements
    }
}
impl ElementTrait for ObjectElement {
    fn decode(&self) -> String {
        "decoding".to_string() // TODO implement from dictionary
    }
}