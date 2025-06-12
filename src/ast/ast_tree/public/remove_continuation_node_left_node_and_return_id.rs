use uuid::Uuid;

use crate::ast::ast_tree::public::AstApi;
use crate::parser::parser_error::ParseError;

pub mod implementation;

pub trait RemoveContinuationNodeLeftNodeAndReturnId: AstApi {
    /// Removes the connection between the actual continuation node and the node
    /// attached to its left node.
    ///
    /// The left node remains in the tree as it will be picked up later.
    fn remove_continuation_node_left_node_and_return_id(&mut self) -> Result<Uuid, ParseError>;
}
