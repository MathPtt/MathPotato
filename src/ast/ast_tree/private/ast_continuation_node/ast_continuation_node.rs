use derive_more::Display;
use uuid::Uuid;

pub mod get_id;
pub mod get_type;
pub mod new;
pub mod set_id;
pub mod set_type;

#[derive(Clone, Debug, Display)]
#[display("AstContinuationNode(id: {}, node_type: {}", id, node_type)]
pub struct ContinuationNode {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait ContinuationNodeApi {}
impl ContinuationNodeApi for ContinuationNode {}
