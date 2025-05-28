use crate::ast::ast_tree::{
    private::continuation_node::{
        node::{get_id::ContinuationNodeApiGetId, get_type::ContinuationNodeApiGetType},
        storage::get_node::ContinuationNodeStorageApiGetNode,
    },
    MathPotatoAstTree,
};

use super::{
    node::{from_id_and_type::ContinuationNodeFromIdAndType, GetContinuationNodeIdAndTypeResult},
    ContinuationNodeStorageApiGetIdAndType,
};

impl ContinuationNodeStorageApiGetIdAndType for MathPotatoAstTree {
    fn get_continuation_node_id_and_type(&self) -> Option<GetContinuationNodeIdAndTypeResult> {
        match self.continuation_node.get_node() {
            Some(r) => Some(GetContinuationNodeIdAndTypeResult::from_id_and_type(
                r.get_id(),
                r.get_type(),
            )),
            None => None,
        }
    }
}
