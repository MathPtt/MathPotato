use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApi;

pub mod implementation;

pub trait InfixNodeStorageApiAddNodeIdAndTypeToLeft: InfixNodeStorageApi {
    /// Adds the designated node and type to the left of the target node.
    ///
    /// # Parameters
    /// - `target_node_id` - `Uuid`, the id of the target node
    /// - `left_node_id` - `Uuid`, the node id which is going to be the left
    ///   child of the
    /// designated target node.
    /// - `left_node_type` - `Uuid`, the node type of the node going to be the
    ///   child of the
    /// designated target node.
    ///
    /// # Returns
    /// - `Ok(Uuid)` - which is the `target_node_id` when theh operation is
    ///   successful.
    /// - `Err(ParseError)` - with the details of the error happened during
    ///   execution.
    fn add_node_id_and_type_to_left(
        &mut self,
        target_node_id: Uuid,
        left_node_id: Uuid,
        left_node_type: AstNodeType,
    ) -> Result<Uuid, ParseError>;
}
