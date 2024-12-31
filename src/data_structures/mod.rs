use crate::data_structures::object::ObjectElement;
use crate::data_structures::string_element::StringElement;

pub mod string_element;
pub mod object;

pub trait ElementTrait {
    fn decode(&self) -> String;
}

pub enum Element {
    StringCase(StringElement),
    Object(ObjectElement),
}
impl Element {
    pub(crate) fn get_value_from_string_element(element: Element) -> String {
        if let Element::StringCase(string_element) = element {
            string_element.value
        } else {
            unreachable!();
        }
    }
}