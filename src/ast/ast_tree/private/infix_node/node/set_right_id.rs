use uuid::Uuid;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetRightId: InfixNodeApi {
    fn set_right_id(&mut self, right_id: Uuid);
}
impl InfixNodeApiSetRightId for InfixNode {
    fn set_right_id(&mut self, right_id: Uuid) {
        self.right_id = right_id;
    }
}
