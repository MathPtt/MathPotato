use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiSetValue: I32NodeApi {
    fn set_value(&mut self, value: i32);
}

impl I32NodeApiSetValue for I32Node {
    fn set_value(&mut self, value: i32) {
        self.value = value
    }
}
