use uuid::Uuid;

use crate::{
    ast::internal::infix_ast_node_internal::InfixAstNodeInternal, parser::parser_error::ParseError,
};

use super::{InfixAstTree, InfixAstTreeApi};

pub trait InfixAstTreeApiGetAll: InfixAstTreeApi {
    fn get_all(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError>;
}
impl InfixAstTreeApiGetAll for InfixAstTree {
    fn get_all(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError> {
        let res: Vec<(Uuid, InfixAstNodeInternal)> = self.tree.clone().into_iter().collect();
        Ok(res)
    }
}
