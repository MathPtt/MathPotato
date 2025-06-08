use crate::ast::ast_tree::private::i32_nodes::storage::count::I32NodeStorageApiCount;
use crate::ast::ast_tree::private::infix_node::storage::count::InfixNodeStorageApiCount;
use crate::ast::ast_tree::MathPotatoAstTree;

use super::GetInfixNodeCount;

impl GetInfixNodeCount for MathPotatoAstTree {
    fn get_infix_node_count(&self) -> usize {
        self.infix_nodes.count()
    }
}
