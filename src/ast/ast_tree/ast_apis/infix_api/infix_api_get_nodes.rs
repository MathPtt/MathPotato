use crate::{
    ast::{ast_tree::MathPotatoAstTree, infix_ast_node::InfixAstNode},
    parser::parser_error::ParseError,
};

use super::InfixApi;

pub trait InfixApiGetNodes: InfixApi {
    fn get_infix_nodes(&self) -> Result<Vec<InfixAstNode>, ParseError>;
}
impl InfixApiGetNodes for MathPotatoAstTree {
    fn get_infix_nodes(&self) -> Result<Vec<InfixAstNode>, ParseError> {
        match self.infix_operation_tree.get_all() {
            Ok(res) => {
                let result: Vec<InfixAstNode> = res
                    .into_iter()
                    .map(|internal| {
                        InfixAstNode::from_internal_and_id(internal.0, internal.1)
                            .unwrap_or_else(|e| panic!("{:#?}", e))
                    })
                    .collect();
                Ok(result)
            }
            Err(err) => Err(err),
        }
    }
}
