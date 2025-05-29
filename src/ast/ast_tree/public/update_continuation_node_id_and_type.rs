use node::UpdateContinuationNodeIdAndTypeResult;
use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::AstApi;

pub mod implementation;
pub mod node;

pub trait UpdateContinuationNodeIdAndType: AstApi {
    /// Updates the continuation node with the provided id and type.
    ///
    /// # Parameters
    /// - `id` - `Uuid`: the new id of the continuation node.
    /// - `node_type` - `AstNodeType`: the new type of the continuation node.
    ///
    /// # Returns
    /// - `Ok(UpdateContinuationNodeIdAndType)` - struct including the new and type.
    /// - `Err(ParseError)` - when any error happens during exectuion. The `ParseError` includes
    /// all available information about the details of the error.
    fn update_continuation_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<UpdateContinuationNodeIdAndTypeResult, ParseError>;
}
