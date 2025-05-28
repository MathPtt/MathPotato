use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{UpdateContinuationNodeIdAndTypeResult, UpdateContinuationNodeIdAndTypeResultApi};

pub trait UpdateContinuationNodeIdAndTypeResultApiNewFromIdAndType:
    UpdateContinuationNodeIdAndTypeResultApi
{
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> Self;
}

impl UpdateContinuationNodeIdAndTypeResultApiNewFromIdAndType
    for UpdateContinuationNodeIdAndTypeResult
{
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> Self {
        UpdateContinuationNodeIdAndTypeResult { id, node_type }
    }
}
