use uuid::Uuid;

use crate::ast::ast_tree::private::continuation_node::storage::ContinuationNodeStorage;

use super::ContinuationNodeStorageApiGetId;

impl ContinuationNodeStorageApiGetId for ContinuationNodeStorage {
    fn get_id(&self) -> Option<uuid::Uuid> {
        if self.id == Uuid::nil() {
            None
        } else {
            Some(self.id)
        }
    }
}
