use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::InfixAstTreeApi;
pub mod implementation;

pub trait InfixAstTreeApiAddLeftChildNodeToNode: InfixAstTreeApi {
    /// Adds the `child_node_id` to the `subject_node_id` as child node to the left side.
    ///
    /// # Parameters
    /// - `target_node_id` - `Uuid`: the node where the operation will be executed, a.k.a. parent
    /// node.
    /// - `child_node_id` - `Uuid`: the node going to be a child node.
    ///
    /// # Returns
    /// - `Ok((Uuid, Uuid))` - when the oeration is executed successfully.
    /// - `Err(ParseError)` - when an error happened in the execution.
    fn add_left_child_node_to_node(
        &mut self,
        target_node_id: Uuid,
        child_node_id: Uuid,
    ) -> Result<(Uuid, Uuid), ParseError>;
}
