use crate::ast::ast_tree::public::AstApi;

pub mod implementation;

pub trait GetI32NodeCount: AstApi {
    fn get_i32_node_count(&self) -> usize;
}
