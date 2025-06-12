use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;

use super::InfixNodeStorageApiDoesNodeExist;

impl InfixNodeStorageApiDoesNodeExist for InfixNodeStorage {
    fn does_node_exist(&self, id: Uuid) -> bool {
        match self.nodes.get(&id) {
            None => false,
            Some(_) => true,
        }
    }
}
