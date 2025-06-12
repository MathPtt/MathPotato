use uuid::Uuid;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetLeftId: InfixNodeApi {
    fn set_left_id(&mut self, left_id: Uuid);
}
impl InfixNodeApiSetLeftId for InfixNode {
    fn set_left_id(&mut self, left_id: Uuid) {
        self.left_id = left_id;
    }
}
