use crate::data_structures::ElementTrait;

pub struct StringElement {
    pub value: String,
}
impl ElementTrait for StringElement {
    fn decode(&self) -> String {
        self.value.clone()
    }
}