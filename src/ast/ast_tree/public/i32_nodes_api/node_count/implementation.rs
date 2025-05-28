use crate::ast::ast_tree::MathPotatoAstTree;

use super::I32ApiNodeCount;

impl I32ApiNodeCount for MathPotatoAstTree {
    fn get_i32_node_count(&self) -> usize {
        self.i32_nodes_storage.len()
    }
}
