use crate::ast::ast_tree::public::infix_nodes_api::InfixNodesApi;

use super::InfixAstTreeApiLen;

impl InfixAstTreeApiLen for InfixNodesApi {
    fn len(&self) -> usize {
        self.tree.len()
    }
}
