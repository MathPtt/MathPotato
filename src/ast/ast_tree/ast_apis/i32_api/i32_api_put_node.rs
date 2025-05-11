use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree, i32_node::I32AstNode},
    parser::parser_error::ParseError,
};

use super::I32Api;

pub trait I32ApiPutNode: I32Api {
    /// Adds a new `I32AstNode` to the AST.
    ///
    /// # Remarks
    /// By adding a new node to the AST the reference to the continuation node will be updated
    /// accordingly.
    ///
    /// When there is a I32AstNode in the AST with the same key there will be no overwrite, rather
    /// error message.
    ///
    /// # Parameters
    /// - `node` - an `I32AstNode`
    ///
    /// # Returns
    /// - `OK(I32AstNode)` - the newly created `I32AstNode` node which also has the unique
    /// identifier.
    /// - `Err(ParseError)` - when recording the node is not successful due to some reasons.
    fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<I32AstNode, ParseError>;
}

impl I32ApiPutNode for MathPotatoAstTree {
    fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<I32AstNode, ParseError> {
        let uuid = Uuid::new_v4();
        match self.i32_tree.put(
            uuid,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Ok(r) => {
                self.last_changed_node_type = AstNodeType::I32AstNode;
                self.last_changed_node_id = r.0;
                Ok(I32AstNode::from_internal_and_id(r.1, uuid)
                    .unwrap_or_else(|e| panic!("{:#?}", e)))
            }
            Err(e) => Err(e),
        }
    }
}
