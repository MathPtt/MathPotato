use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;
use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;

use super::I32NodeStorageApiGetNodeById;

impl I32NodeStorageApiGetNodeById for I32NodeStorage {
    fn get_node_by_id(&self, id: Uuid) -> Option<I32Node> {
        self.nodes.get(&id).cloned()
    }
}
