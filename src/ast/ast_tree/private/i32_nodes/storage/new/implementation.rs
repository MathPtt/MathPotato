use std::collections::HashMap;

use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;

use super::I32NodeStorageApiNew;

impl I32NodeStorageApiNew for I32NodeStorage {
    fn new() -> I32NodeStorage {
        I32NodeStorage {
            nodes: HashMap::new(),
        }
    }
}
