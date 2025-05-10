use crate::{ast::infix_ast_node::InfixAstNode, parser::parser_error::ParseError};

use super::{infix_api::InfixApi, MathPotatoAstTree};

pub trait InfixApiGetNodes: InfixApi {
    fn get_infix_nodes(&self) -> Result<Vec<InfixAstNode>, ParseError>;
}
impl InfixApiGetNodes for MathPotatoAstTree {
    fn get_infix_nodes(&self) -> Result<Vec<InfixAstNode>, ParseError> {
        match self.infix_operation_tree.get_all() {
            Ok(res) => {
                res.iter().collect();
            }
            Err(err) => Err(err),
        }
    }
}
