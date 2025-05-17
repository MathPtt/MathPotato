use crate::ast::{ast_tree::MathPotatoAstTree, internal::ast_tree_traits::TypedAstTreeLen};

use super::I32Api;

pub trait I32ApiNodeCount: I32Api {
    /// Returns the length of the i32 datatype tree size.
    ///
    /// # Returns
    /// - `usize` - the count of I32AstNodes in the AST
    fn get_i32_node_count(&self) -> usize;
}

impl I32ApiNodeCount for MathPotatoAstTree {
    fn get_i32_node_count(&self) -> usize {
        self.i32_tree.len()
    }
}
