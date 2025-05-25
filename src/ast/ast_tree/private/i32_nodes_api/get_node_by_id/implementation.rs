use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes_api::{entity::I32AstEntity, storage::I32NodesApi};

use super::I32AstTreeApiGetNodeById;

impl I32AstTreeApiGetNodeById for I32NodesApi {
    fn get_node_by_id(&self, id: Uuid) -> Option<I32AstEntity> {
        self.tree.get(&id).cloned()
    }
}
