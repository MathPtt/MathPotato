use super::{infix_api::InfixApi, MathPotatoAstTree};

pub trait InfixApiNodeCount: InfixApi {
    fn infix_tree_len(&self) -> usize;
}

impl InfixApiNodeCount for MathPotatoAstTree {
    fn infix_tree_len(&self) -> usize {
        self.infix_operation_tree.len()
    }
}
