use uuid::Uuid;

use crate::ast::ast_tree::private::continuation_node::storage::ContinuationNodeStorage;

use super::ContinuationNodeStorageApiSetId;

impl ContinuationNodeStorageApiSetId for ContinuationNodeStorage {
    fn set_id(&mut self, id: Uuid) {
        self.id = id
    }
}
