use uuid::Uuid;

use crate::{
    ast::internal::infix_ast_node_internal::InfixAstNodeInternal, parser::parser_error::ParseError,
};

use super::{InfixAstTree, InfixAstTreeApi};

pub trait InfixAstTreeApiPut: InfixAstTreeApi {
    fn put(
        &mut self,
        key: Uuid,
        value: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError>;
}
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
