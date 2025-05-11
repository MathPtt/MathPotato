use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree, infix_ast_node::InfixAstNode,
    },
    parser::parser_error::ParseError,
};

use super::InfixApi;

pub trait InfixApiPutNode: InfixApi {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn put_infix_node(&mut self, node: InfixAstNode) -> Result<InfixAstNode, ParseError>;
}

impl InfixApiPutNode for MathPotatoAstTree {
    fn put_infix_node(&mut self, node: InfixAstNode) -> Result<InfixAstNode, ParseError> {
        match self.infix_operation_tree.put(
            node.id,
            node.to_internal()
                .unwrap_or_else(|e| panic!("Node to internal has failed. {:#?}", e)),
        ) {
            Err(err) => Err(err),
            Ok(r) => {
                self.last_changed_node_type = AstNodeType::InfixOperationAstNode;
                self.last_changed_node_id = r.0;
                Ok(InfixAstNode::from_internal_and_id(r.0, r.1)
                    .unwrap_or_else(|e| panic!("From internal has failed.")))
            }
        }
    }
}
