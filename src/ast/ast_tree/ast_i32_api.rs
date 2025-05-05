use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType, i32_node::I32AstNode,
        internal::ast_tree_traits::TypedAstTreeLen,
    },
    parser::parser_error::ParseError,
};

use super::MathPotatoAstTree;

/// The I32 type related functionalities offered by the Abstract Syntax Tree.
pub trait I32Api {
    /// Returns the I32AstNode with the designated id.
    ///
    /// # Parameters
    /// - `id` - `Uuid` - the id of the requested node
    /// # Returns
    /// - `Result<I32AstNode, ParseError`
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError>;

    /// Adds a new node to the AST.
    /// By adding a new node to the AST the reference to the continuation node will be updated
    /// accordingly.
    ///
    /// # Important
    ///
    /// When there is a I32AstNode in the AST with the same key there will be no overwrite, rather
    /// error message.
    ///
    /// # Parameters
    /// - `node` - an `I32AstNode`
    ///
    /// # Returns
    /// - `Result<(Uuid, I32AstNode), ParseError>`
    fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<I32AstNode, ParseError>;

    /// Returns the length of the i32 datatype tree size.
    fn i32_tree_len(&self) -> usize;

    /// Updates the designated i32 datatype node with the provided data.
    ///
    /// # Details
    ///
    /// If there is no i32 node with the provided id the operation will panic.
    ///
    /// # Parameters
    ///
    /// - `id` - `Uuid`: The unique id of the node to be updated.
    /// - `node` - `I32AstNode`: The I32AstNode with the new data.
    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError>;
}

impl I32Api for MathPotatoAstTree {
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.get_node_by_id(id) {
            None => Err(ParseError::new(format!("No I32AstNode with id {}", id))),
            Some(internal) => Ok(I32AstNode::from_internal_and_id(internal, id)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }

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

    fn i32_tree_len(&self) -> usize {
        self.i32_tree.len()
    }

    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.update(
            id,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Err(err) => Err(err),
            Ok(res) => Ok(I32AstNode::from_internal_and_id(res.1, res.0)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }
}
