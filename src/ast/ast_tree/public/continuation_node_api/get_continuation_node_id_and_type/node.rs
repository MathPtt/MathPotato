use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod from_id_and_type;
pub mod get_id;
pub mod get_type;

#[derive(Debug, Clone, Display)]
#[display("ContinuationNodeIdAndType(id: {}, node_type: {}", id, node_type)]
pub struct GetContinuationNodeIdAndTypeResult {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait GetContinuationNodeIdAndTypeResultApi {}
impl GetContinuationNodeIdAndTypeResultApi for GetContinuationNodeIdAndTypeResult {}
