use ast_continuation_node::ContinuationNode;

pub mod ast_continuation_node;
pub mod get_id;
pub mod get_node_id_and_type;
pub mod get_type;
pub mod set_id_and_type;

pub trait ContinuationNodeStorageApi {}
impl ContinuationNodeStorageApi for ContinuationNode {}
