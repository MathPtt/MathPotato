use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;

use super::RootNodeApi;

pub trait RootNodeApiGetRootNodeId: RootNodeApi {
    /// Get Root Node Id
    ///
    /// Returns the Root Node id.
    ///
    /// # Returns
    /// - `Some(Uuid)` - `Option<Uuid>` the Uuid value of the Root Node
    /// - `None` - When there is no Root Node set up
    fn get_root_node_id(&self) -> Option<Uuid>;
}

impl RootNodeApiGetRootNodeId for MathPotatoAstTree {
    fn get_root_node_id(&self) -> Option<Uuid> {
        if self.root_node_id == Uuid::nil() {
            None
        } else {
            Some(self.root_node_id)
        }
    }
}
