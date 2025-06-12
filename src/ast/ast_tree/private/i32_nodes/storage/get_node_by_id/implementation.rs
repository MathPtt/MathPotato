use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::i32_nodes::node::I32Node;
use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;

use super::I32NodeStorageApiGetNodeById;

impl I32NodeStorageApiGetNodeById for I32NodeStorage {
    fn get_node_by_id(&self, id: Uuid) -> Result<I32Node> {
        let result = self
            .nodes
            .get(&id)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "There is no {} with id {} in the storage.",
                    AstNodeType::I32AstNode,
                    id
                )
            })
            .with_context(|| {
                format!(
                    "There is no {} with id {} in the storage.",
                    AstNodeType::I32AstNode,
                    id
                )
            })?;
        Ok(*result)
    }
}
