use uuid::Uuid;

use crate::ast::ast_tree::public::infix_nodes_api::InfixNodesApi;

use super::InfixAstTreeApiGet;

impl InfixAstTreeApiGet for InfixNodesApi {
    fn does_node_exist(&self, id: Uuid) -> bool {
        match self.tree.get(&id) {
            None => false,
            Some(_) => true,
        }
    }
}
