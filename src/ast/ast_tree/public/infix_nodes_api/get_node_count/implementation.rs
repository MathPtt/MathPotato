use crate::ast::ast_tree::MathPotatoAstTree;

use super::InfixApiGetNodeCount;

impl InfixApiGetNodeCount for MathPotatoAstTree {
    fn get_infix_node_count(&self) -> usize {
        self.infix_operation_tree.len()
    }
}
