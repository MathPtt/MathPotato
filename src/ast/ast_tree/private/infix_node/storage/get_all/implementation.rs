use uuid::Uuid;

use crate::{
    ast::ast_tree::private::infix_nodes_api::entity::InfixAstNodeInternal,
    parser::parser_error::ParseError,
};

use super::InfixAstTreeApiGetAll;

impl InfixAstTreeApiGetAll for InfixAstTree {
    fn get_all(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError> {
        let res: Vec<(Uuid, InfixAstNodeInternal)> = self.tree.clone().into_iter().collect();
        Ok(res)
    }
}
