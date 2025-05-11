use uuid::Uuid;

use crate::{
    ast::{ast_tree::MathPotatoAstTree, i32_node::I32AstNode},
    parser::parser_error::ParseError,
};

use super::I32Api;

pub trait I32ApiGetNodeById: I32Api {
    /// Returns the I32AstNode with the designated id.
    ///
    /// # Parameters
    /// - `id` - `Uuid` - the id of the requested node
    ///
    /// # Returns
    /// - `Ok(I32AstNode)` - the designated `I32AstNode`
    /// - `Err(ParseError)` - if there is no `I32AstNode` in the AST.
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError>;
}

impl I32ApiGetNodeById for MathPotatoAstTree {
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.get_node_by_id(id) {
            None => Err(ParseError::new(format!("No I32AstNode with id {}", id))),
            Some(internal) => Ok(I32AstNode::from_internal_and_id(internal, id)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }
}
