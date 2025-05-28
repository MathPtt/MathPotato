use node::GetContinuationNodeIdAndTypeResult;

use super::AstApi;

pub mod implementation;
pub mod node;

pub trait ContinuationNodeStorageApiGetIdAndType: AstApi {
    /// Returns the `GetContinuationNodeIdAndTypeResult` struct containing the `id` and `node_type`
    /// values of the AST continuation node.
    ///
    /// # Returns
    /// - `Some(GetContinuationNodeIdAndTypeResult)` - The `id` and `node_type` is wrapped into a
    /// struct for better understanding at the consumer side.
    /// - `None` - When there is no continuation data is set up.
    fn get_continuation_node_id_and_type(&self) -> Option<GetContinuationNodeIdAndTypeResult>;
}
