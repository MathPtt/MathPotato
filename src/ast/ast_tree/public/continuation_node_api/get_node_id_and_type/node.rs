use derive_more::Display;
use uuid::Uuid;

use crate::ast::global::enums::ast_node_types_enum::AstNodeType;

pub mod from_internal;
pub mod get_id;
pub mod get_type;

#[derive(Debug, Clone, Display)]
#[display("ContinuationNodeIdAndType(id: {}, node_type: {}", id, node_type)]
pub struct GetNodeIdAndTypeResult {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait GetNodeIdAndTypeResultApi {}
impl GetNodeIdAndTypeResultApi for GetNodeIdAndTypeResult {}
