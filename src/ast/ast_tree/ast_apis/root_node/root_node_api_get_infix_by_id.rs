use uuid::Uuid;

use crate::{
    ast::{
        ast_tree::{infix_get_by_id::InfixApiGetNodeById, MathPotatoAstTree},
        infix_ast_node::InfixAstNode,
    },
    parser::parser_error::ParseError,
};

use super::root_node_api::RootNodeApi;

pub trait RootNodeApiGetInfixNodeById: RootNodeApi {
    /// Returns the root node when the root node type is Infix node.
    /// This method is an alias over the InfixApiGetNodeById.
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
