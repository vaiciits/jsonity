use crate::data_structures::object::Object;

pub struct ObjectParser {}
impl ObjectParser {
    pub(crate) fn parse(chars: &Vec<char>, mut index: usize) -> (Object, usize) {
        // TODO implement

        (Object::new(), index)
    }
}
