use super::I32AstTreeApi;
pub mod implementation;

pub trait I32AstTreeApiGetNodeCount: I32AstTreeApi {
    fn get_node_count(&self) -> usize;
}
