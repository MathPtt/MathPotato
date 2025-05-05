use crate::ast::internal::ast_tree_traits::TypedAstTreeLen;

use super::{i32_api::I32Api, MathPotatoAstTree};

pub trait I32ApiNodeCount: I32Api {
    /// Returns the length of the i32 datatype tree size.
    ///
    /// # Returns
    /// - `usize` - the count of I32AstNodes in the AST
    fn i32_node_count(&self) -> usize;
}

impl I32ApiNodeCount for MathPotatoAstTree {
    fn i32_node_count(&self) -> usize {
        self.i32_tree.len()
    }
}
