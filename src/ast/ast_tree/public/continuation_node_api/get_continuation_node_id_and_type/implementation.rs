use crate::ast::ast_tree::MathPotatoAstTree;

use super::node::{GetContinuationNodeIdAndTypeResult, GetContinuationNodeIdAndTypeResultApi};

impl GetContinuationNodeIdAndTypeResultApi for MathPotatoAstTree {
    fn get_continuation_node_id_and_type(&self) -> Option<GetContinuationNodeIdAndTypeResult> {
        match self.continuation_node {
            Ok(r) => Some(GetNodeIdAndTypeResult::from_id_and_type(
                r.get_id(),
                r.get_type(),
            )),
            Err(_) => None,
        }
    }
}
