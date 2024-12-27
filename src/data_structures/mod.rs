use crate::data_structures::object::Object;
use crate::data_structures::string_element::StringElement;

pub mod string_element;
pub mod object;

pub trait ElementTrait {
    fn decode(&self) -> String;
}

pub enum Element {
    String(StringElement),
    Object(Object),
}