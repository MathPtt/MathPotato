use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::GetContinuationNodeIdAndTypeResult;
use super::GetContinuationNodeIdAndTypeResultApi;

pub trait ContinuationNodeFromIdAndType: GetContinuationNodeIdAndTypeResultApi {
    fn from_id_and_type(id: Uuid, node_type: AstNodeType) -> GetContinuationNodeIdAndTypeResult;
}

impl ContinuationNodeFromIdAndType for GetContinuationNodeIdAndTypeResult {
    fn from_id_and_type(id: Uuid, node_type: AstNodeType) -> GetContinuationNodeIdAndTypeResult {
        GetContinuationNodeIdAndTypeResult { id, node_type }
    }
}
