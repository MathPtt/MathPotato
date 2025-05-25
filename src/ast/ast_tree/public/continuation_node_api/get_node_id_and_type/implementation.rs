use crate::ast::ast_tree::MathPotatoAstTree;

use super::{node::GetNodeIdAndTypeResult, ContNodeApiGetIdAndType};
impl ContNodeApiGetIdAndType for MathPotatoAstTree {
    fn cont_node_api_get_cont_node_id_and_type(&self) -> Option<GetNodeIdAndTypeResult> {
        match self.continuation_node.get_type_and_id() {
            Ok(r) => Some(GetNodeIdAndTypeResult::from_id_and_type(
                r.get_id(),
                r.get_type(),
            )),
            Err(_) => None,
        }
    }
}
