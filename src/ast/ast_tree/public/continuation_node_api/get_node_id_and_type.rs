use node::GetNodeIdAndTypeResult;

use super::ContinuationNodeApi;

pub mod implementation;
pub mod node;

pub trait ContNodeApiGetIdAndType: ContinuationNodeApi {
    /// Returns the Id and Type of the Continuation Node.
    fn cont_node_api_get_cont_node_id_and_type(&self) -> Option<GetNodeIdAndTypeResult>;
}
