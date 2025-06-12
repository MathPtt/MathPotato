use crate::ast::ast_tree::private::root_node::node::RootNode;
use crate::ast::ast_tree::private::root_node::node::new_from_id_and_type::RootNodeApiNewFromIdAndType;
use crate::ast::ast_tree::private::root_node::storage::RootNodeStorage;

use super::RootNodeStorageApiGet;

impl RootNodeStorageApiGet for RootNodeStorage {
    fn get(&self) -> RootNode {
        RootNode::new_from_id_and_type(self.id, self.node_type.clone())
    }
}
