use uuid::Uuid;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApisetParentId: InfixNodeApi {
    fn set_parent_id(&mut self, id: Uuid) -> Uuid;
}
impl InfixNodeApisetParentId for InfixNode {
    fn set_parent_id(&mut self, id: Uuid) -> Uuid {
        self.parent_id = id;
        id
    }
}
