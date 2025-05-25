use crate::ast::ast_tree::private::i32_nodes_api::storage::I32NodesApi;

use super::I32AstTreeApiGetNodeCount;

impl I32AstTreeApiGetNodeCount for I32NodesApi {
    fn get_node_count(&self) -> usize {
        self.tree.clone().len()
    }
}
