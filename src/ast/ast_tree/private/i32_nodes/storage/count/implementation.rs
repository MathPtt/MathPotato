use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;

use super::I32NodeStorageApiCount;

impl I32NodeStorageApiCount for I32NodeStorage {
    fn count(&self) -> usize {
        self.nodes.len()
    }
}
