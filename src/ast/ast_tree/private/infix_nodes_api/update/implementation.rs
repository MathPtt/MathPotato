use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        private::infix_nodes_api::entity::InfixAstNodeInternal,
        public::infix_nodes_api::InfixNodesApi,
    },
    parser::parser_error::ParseError,
};

use super::InfixAstTreeApiUpdate;

impl InfixAstTreeApiUpdate for InfixNodesApi {
    fn update(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        match self.does_node_exist(id) {
            false => Err(ParseError::new(format!(
                "There is no {:#?} type node with id: {}.",
                type_name::<AstNodeType>(),
                id
            ))),
            true => {
                self.tree.insert(id, node.clone());
            }
        }
    }
}
