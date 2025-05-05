use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType, i32_node::I32AstNode,
        internal::ast_tree_traits::TypedAstTreeLen,
    },
    parser::parser_error::ParseError,
};

use super::MathPotatoAstTree;

/// The I32 type related functionalities offered by the Abstract Syntax Tree.
pub trait I32ApiUpdateNode {
    /// Updates the designated i32 datatype node with the provided data.
    ///
    /// # Details
    ///
    /// If there is no i32 node with the provided id the operation will panic.
    ///
    /// # Parameters
    ///
    /// - `id` - `Uuid`: The unique id of the node to be updated.
    /// - `node` - `I32AstNode`: The I32AstNode with the new data.
    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError>;
}

impl I32ApiUpdateNode for MathPotatoAstTree {
    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.update(
            id,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Err(err) => Err(err),
            Ok(res) => Ok(I32AstNode::from_internal_and_id(res.1, res.0)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }
}
