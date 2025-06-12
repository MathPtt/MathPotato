use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod new_from_id_and_type;

#[derive(Clone, Debug, Display)]
#[display(
    "UpdateContinuationNodeIdAndTypeResult(id: {}, node_type: {})",
    id,
    node_type
)]
pub struct UpdateContinuationNodeIdAndTypeResult {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait UpdateContinuationNodeIdAndTypeResultApi {}
impl UpdateContinuationNodeIdAndTypeResultApi for UpdateContinuationNodeIdAndTypeResult {}
