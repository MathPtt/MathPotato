use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::private::i32_nodes::storage::count::I32NodeStorageApiCount;

use super::GetI32NodeCount;

impl GetI32NodeCount for MathPotatoAstTree {
    fn get_i32_node_count(&self) -> usize {
        self.i32_nodes.count()
    }
}
