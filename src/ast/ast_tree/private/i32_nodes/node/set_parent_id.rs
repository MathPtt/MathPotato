use uuid::Uuid;

use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiSetParentId: I32NodeApi {
    fn set_parent_id(&mut self, id: Uuid);
}

impl I32NodeApiSetParentId for I32Node {
    fn set_parent_id(&mut self, id: Uuid) {
        self.parent_id = id
    }
}
