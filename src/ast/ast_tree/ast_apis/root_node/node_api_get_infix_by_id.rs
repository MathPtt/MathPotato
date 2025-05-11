use uuid::Uuid;

use crate::ast::{
    ast_tree::{ast_apis::infix_api::get_by_id::InfixApiGetNodeById, MathPotatoAstTree},
    infix_ast_node::InfixAstNode,
};
use crate::parser::parser_error::ParseError;

use super::RootNodeApi;

pub trait RootNodeApiGetInfixNodeById: RootNodeApi {
    /// Get Root Node Infix By Id
    ///
    /// Returns the root node when the root node type is Infix type node.
    ///
    /// # Note
    /// This method is an alias over the InfixApiGetNodeById.
    ///
    /// # Returns
    /// - `Ok(InfixAstNode)` - The Root Node.
    /// - `Err(ParseError)` - The details of the error happened related to this operation.
    fn get_root_node_infix_by_id(&self, id: Uuid) -> Result<InfixAstNode, ParseError>;
}

impl RootNodeApiGetInfixNodeById for MathPotatoAstTree {
    /// Returns the root node by id.
    fn get_root_node_infix_by_id(&self, id: Uuid) -> Result<InfixAstNode, ParseError> {
        match self.get_infix_node_by_id(id) {
            Some(r) => Ok(r),
            None => Err(ParseError::new(format!(
                "There is no InfixOperationAstNode type root node with id {}",
                id
            ))),
        }
    }
}
