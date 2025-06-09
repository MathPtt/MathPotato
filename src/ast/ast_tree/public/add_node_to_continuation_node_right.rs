use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::AstApi;

pub mod implementation;

pub trait AddNodeToContinuationNodeRight: AstApi {
    /// Adds the designated node to the right side of the continuation node.
    /// This operation assumes that the continuation node is an `InfixNode`
    /// in this case.
    ///
    /// This method assumes that the continuation node in this case is an
    /// `InfixNode`. If the reality is something different the method
    /// returns an error.
    ///
    /// # Parameters
    /// - `id` - `Uuid` - the Id of the node to be added to the right side of
    ///   the continuation
    /// node.
    ///
    /// # Returns
    /// - `Ok(Uuid)` where the id value is the added node's id.
    /// - `Err(ParseError)` - If any error happens during execution. The
    ///   `ParseError` includes as
    /// many details as possible.
    fn add_node_to_continuation_node_right(&mut self, id: Uuid) -> Result<Uuid, ParseError>;
}
