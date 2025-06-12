use uuid::Uuid;

use crate::{
    ast::global::enums::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApi;

pub mod implementation;
pub mod result;
pub trait ContNodeApiUpdateNodeIdAndType: ContinuationNodeStorageApi {
    /// Update Continuation Node and Id
    ///
    /// It updates the continuation node (an Id and node type) in the AST.
    /// If there is no continuation node, meaning the first time continuation node has been
    /// referenced, it will create the reference. It also means that this method blindly overwrites
    /// the values.
    ///
    /// # Parameters
    /// - `id`:`Uuid` - the id of the new continuation node
    /// - `ast_node_type`:`AstNodeType` - the type of the new continuation node
    fn continuation_node_api_update_node_id_and_type(
        &mut self,
        id: Uuid,
        ast_node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;
}
