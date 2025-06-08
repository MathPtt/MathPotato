use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;

use super::InfixNodeStorageApiCount;

impl InfixNodeStorageApiCount for InfixNodeStorage {
    fn count(&self) -> usize {
        self.nodes.len()
    }
}
