use crate::data_structures::string_element::StringElement;

pub mod string_element;

pub enum Element {
    String(StringElement),
}