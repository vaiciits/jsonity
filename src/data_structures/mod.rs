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