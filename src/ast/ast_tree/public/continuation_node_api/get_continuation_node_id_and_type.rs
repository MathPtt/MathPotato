use node::GetContinuationNodeIdAndTypeResult;

use super::ContinuationNodeStorageApi;

pub mod implementation;
pub mod node;

pub trait ContinuationNodeStorageApiGetIdAndType: ContinuationNodeStorageApi {
    /// Returns the Id and Type of the Continuation Node.
    fn get_continuation_node_id_and_type(&self) -> Option<GetContinuationNodeIdAndTypeResult>;
}
