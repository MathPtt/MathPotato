use uuid::Uuid;

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetRightId: InfixNodeApi {
    fn get_right_id(&self) -> Uuid;
}
impl InfixNodeApiGetRightId for InfixNode {
    fn get_right_id(&self) -> Uuid {
        self.right_id
    }
}
