use uuid::Uuid;

use crate::ast::infix_ast_node::InfixAstNode;

use super::{infix_api::InfixApi, MathPotatoAstTree};

pub trait InfixApiGetNodeById: InfixApi {
    fn get_infix_node_by_id(&self, id: Uuid) -> Option<InfixAstNode>;
}
impl InfixApiGetNodeById for MathPotatoAstTree {
    fn get_infix_node_by_id(&self, id: Uuid) -> Option<InfixAstNode> {
        match self.infix_operation_tree.get(id) {
            Some(r) => Some(
                InfixAstNode::from_internal_and_id(id, r).unwrap_or_else(|e| panic!("{:#?}", e)),
            ),
            None => None,
        }
    }
}
