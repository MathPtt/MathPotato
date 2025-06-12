use uuid::Uuid;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiGetParentId: InfixNodeApi {
    fn get_parent_id(&self) -> Uuid;
}
impl InfixNodeApiGetParentId for InfixNode {
    fn get_parent_id(&self) -> Uuid {
        self.parent_id
    }
}
