use uuid::Uuid;

use crate::ast::global::enums::ast_node_types_enum::AstNodeType;

use super::{GetNodeIdAndTypeResult, GetNodeIdAndTypeResultApi};

pub trait ContinuationNodeFromIdAndType: GetNodeIdAndTypeResultApi {
    fn from_id_and_type(id: Uuid, node_type: AstNodeType) -> GetNodeIdAndTypeResult;
}
impl ContinuationNodeFromIdAndType for GetNodeIdAndTypeResult {
    fn from_id_and_type(id: Uuid, node_type: AstNodeType) -> GetNodeIdAndTypeResult {
        GetNodeIdAndTypeResult { id, node_type }
    }
}
