use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::InfixNodesApi;

pub mod implementation;

pub trait InfixApiAddInfixLeftChildToInfix: InfixNodesApi {
    /// Adds the marked infix node, `child_node_id`, to the marked infix node, `subject_node_id`, as left child.
    ///
    /// # Parameters
    /// - `subject_node_id` - `Uuid`: The node where to the operation going to executed.
    /// - `child_node_id` - `Uuid`: The node which will be added as left child to the target node.
    fn add_infix_left_child_to_infix(
        &mut self,
        subject_node_id: Uuid,
        child_node_id: Uuid,
    ) -> Result<Uuid, ParseError>;
}
