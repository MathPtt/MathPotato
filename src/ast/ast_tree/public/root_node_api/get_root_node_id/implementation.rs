use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;

use super::RootNodeApiGetRootNodeId;

impl RootNodeApiGetRootNodeId for MathPotatoAstTree {
    fn get_root_node_id(&self) -> Option<Uuid> {
        if self.root_node_id == Uuid::nil() {
            None
        } else {
            Some(self.root_node_id)
        }
    }
}
