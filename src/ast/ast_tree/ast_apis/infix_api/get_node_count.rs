use crate::ast::ast_tree::MathPotatoAstTree;

use super::InfixApi;

pub trait InfixApiGetNodeCount: InfixApi {
    fn get_infix_node_count(&self) -> usize;
}

impl InfixApiGetNodeCount for MathPotatoAstTree {
    fn get_infix_node_count(&self) -> usize {
        self.infix_operation_tree.len()
    }
}
