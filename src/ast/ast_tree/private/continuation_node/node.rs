use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod get_id;
pub mod get_type;
pub mod new;
pub mod new_from_id_and_type;
pub mod set_id;
pub mod set_type;

#[derive(Clone, Debug, Display)]
#[display("ContinuationNode(id: {}, node_type: {}", id, node_type)]
pub struct ContinuationNode {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait ContinuationNodeApi {}
impl ContinuationNodeApi for ContinuationNode {}
