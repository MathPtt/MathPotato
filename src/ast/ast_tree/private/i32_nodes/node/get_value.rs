use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiGetValue: I32NodeApi {
    fn get_value(&self) -> i32;
}

impl I32NodeApiGetValue for I32Node {
    fn get_value(&self) -> i32 {
        self.value
    }
}
