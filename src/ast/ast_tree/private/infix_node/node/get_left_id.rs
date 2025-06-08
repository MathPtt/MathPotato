use uuid::Uuid;

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetLeftId: InfixNodeApi {
    fn get_left_id(&self) -> Uuid;
}
impl InfixNodeApiGetLeftId for InfixNode {
    fn get_left_id(&self) -> Uuid {
        self.left_id
    }
}
