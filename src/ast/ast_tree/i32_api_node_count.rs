use crate::ast::internal::ast_tree_traits::TypedAstTreeLen;

use super::MathPotatoAstTree;

pub trait I32ApiNodeCount {
    /// Returns the length of the i32 datatype tree size.
    fn i32_node_count(&self) -> usize;
}

impl I32ApiNodeCount for MathPotatoAstTree {
    fn i32_node_count(&self) -> usize {
        self.i32_tree.len()
    }
}
