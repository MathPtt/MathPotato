use uuid::Uuid;

use crate::ast::ast_tree::private::ast_continuation_node::storage::ContinuationNodeStorage;

use super::ContinuationNodeStorageApiGetId;

impl ContinuationNodeStorageApiGetId for ContinuationNodeStorage {
    fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}
