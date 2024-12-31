use crate::data_structures::ElementTrait;

pub struct BoolElement {
    pub value: bool
}
impl ElementTrait for BoolElement {
    fn decode(&self) -> String {
        self.value.to_string()
    }
}