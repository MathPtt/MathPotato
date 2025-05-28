use node::GetContinuationNodeIdAndTypeResult;

use super::continuation_node_api::ContinuationNodeStorageApi;

pub mod implementation;
pub mod node;

pub trait ContinuationNodeStorageApiGetIdAndType: ContinuationNodeStorageApi {
    /// Returns the `GetContinuationNodeIdAndTypeResult` struct containing the `id` and `node_type`
    /// values of the AST continuation node.
    ///
    /// # Returns
    /// - `Some(GetContinuationNodeIdAndTypeResult)` - The `id` and `node_type` is wrapped into a
    /// struct for better understanding at the consumer side.
    /// - `None` - When there is no continuation data is set up.
    fn get_continuation_node_id_and_type(&self) -> Option<GetContinuationNodeIdAndTypeResult>;
}
