use uuid::Uuid;

use crate::{ast::i32_node::I32AstNode, parser::parser_error::ParseError};

use super::{i32_api::I32Api, MathPotatoAstTree};

pub trait I32ApiUpdateNode: I32Api {
    /// Updates the designated `I32AstNode` node with the provided data.
    ///
    /// # Remarks
    ///
    /// The node must exist in the AST to be updated. There is no "if doesn't exist, update it". At
    /// this point I think this would lead to results I don't want.
    ///
    /// # Parameters
    ///
    /// - `id` - `Uuid`: The unique id of the node to be updated.
    /// - `node` - `I32AstNode`: The I32AstNode with the new data.
    ///
    /// # Returns
    /// - `Ok(I32AstNode)` - the updated node
    /// - `Err(ParseError)` - if any error happens during updating the node
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
