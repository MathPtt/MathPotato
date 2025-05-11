use uuid::Uuid;

use crate::ast::{ast_tree::MathPotatoAstTree, infix_ast_node::InfixAstNode};

use super::InfixApi;

pub trait InfixApiGetNodeById: InfixApi {
    /// Get Infix Node By Id Method
    ///
    /// Returns the designated `InfixAstNode`.
    ///
    /// # Parameters
    /// - `id`: `Uuid` - the unique identifier of the requested node.
    ///
    /// # Returns
    /// - `Some(InFixAstNode)`: `Option<InFixAstNode>` - When there is a node with with provided
    /// Id.
    /// - `None`: `Option<InfixAstNode>` - when there is no such node.
    fn get_infix_node_by_id(&self, id: Uuid) -> Option<InfixAstNode>;
}
impl InfixApiGetNodeById for MathPotatoAstTree {
    fn get_infix_node_by_id(&self, id: Uuid) -> Option<InfixAstNode> {
        match self.infix_operation_tree.get(id) {
            Some(r) => Some(
                InfixAstNode::from_internal_and_id(id, r).unwrap_or_else(|e| panic!("{:#?}", e)),
            ),
            None => None,
        }
    }
}
