use uuid::Uuid;

use crate::{
    ast::ast_tree::private::infix_nodes_api::entity::InfixAstNodeInternal,
    parser::parser_error::ParseError,
};

use super::InfixAstTreeApiPut;

impl InfixAstTreeApiPut for InfixAstTree {
    fn put(
        &mut self,
        key: Uuid,
        value: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        match self.tree.insert(key, value) {
            None => Ok((key, self.tree.get(&key).unwrap().clone())),
            Some(_) => panic!(
                "There is an existing InfixOperationAstNode with key: {}",
                key
            ),
        }
    }
}
