use uuid::Uuid;

use crate::ast::ast_tree::{
    global::enums::ast_node_types_enum::AstNodeType, private::root_node::storage::RootNodeStorage,
};

use super::RootNodeStorageApiNew;

impl RootNodeStorageApiNew for RootNodeStorage {
    fn new() -> Self {
        RootNodeStorage {
            id: Uuid::nil(),
            node_type: AstNodeType::None,
        }
    }
}
