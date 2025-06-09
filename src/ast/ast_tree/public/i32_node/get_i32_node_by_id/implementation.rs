use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::i32_nodes::storage::get_node_by_id::I32NodeStorageApiGetNodeById;
use crate::ast::ast_tree::public::i32_node::dto::I32NodeDto;
use anyhow::Context;
use anyhow::Result;

use super::GetI32NodeById;

impl GetI32NodeById for MathPotatoAstTree {
    fn get_i32_node_by_id(&self, id: uuid::Uuid) -> Result<I32NodeDto> {
        let i32_node = self.i32_nodes.get_node_by_id(id).with_context(|| {
            format!(
                "Didn't receive {} node with id {} from storage.",
                AstNodeType::I32AstNode,
                id
            )
        })?;
        Ok(I32NodeDto::new_from_i32_node(id, i32_node))
    }
}
