use crate::{
    ast::{ast_tree::MathPotatoAstTree, infix_ast_node::InfixAstNode},
    parser::parser_error::ParseError,
};

use super::InfixApi;

pub trait InfixApiUpdateNodeById: InfixApi {
    /// Update Infix Node By Id
    ///
    /// The provided input node works as the following: the `id` value designates the
    /// `InfixAstNode` to be updated, and the other fields in the struct represent the new state.
    ///
    /// # Parameters
    /// - `node`: `InfixAstNode` - the new state of the node
    ///
    /// # Returns
    /// - `Ok(InfixAstNode)` - the updated `InfixAstNode` node.
    /// - `Err(ParseError)` - if any error happens during update.
    fn update_infix_node_by_id(&mut self, node: InfixAstNode) -> Result<InfixAstNode, ParseError>;
}
impl InfixApiUpdateNodeById for MathPotatoAstTree {
    fn update_infix_node_by_id(&mut self, node: InfixAstNode) -> Result<InfixAstNode, ParseError> {
        match self.infix_operation_tree.update(
            node.id,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Ok(r) => {
                Ok(InfixAstNode::from_internal_and_id(r.0, r.1)
                    .unwrap_or_else(|e| panic!("{:#?}", e)))
            }
            Err(e) => {
                panic!(
                    "Error while updating infix operations of AST tree. {:#?}",
                    e
                )
            }
        }
    }
}
