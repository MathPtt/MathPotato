use uuid::Uuid;

use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiGetParentId: I32NodeApi {
    fn get_parent_id(&self) -> Uuid;
}

impl I32NodeApiGetParentId for I32Node {
    fn get_parent_id(&self) -> Uuid {
        self.parent_id
    }
}
