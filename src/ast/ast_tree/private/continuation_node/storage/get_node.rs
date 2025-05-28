use crate::ast::ast_tree::private::continuation_node::node::ContinuationNode;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageApiGetNode: ContinuationNodeStorageApi {
    fn get_node(&self) -> Option<ContinuationNode>;
}
