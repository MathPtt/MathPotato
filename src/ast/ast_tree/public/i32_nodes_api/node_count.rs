use super::I32NodesApi;

pub mod implementation;
pub trait I32ApiNodeCount: I32NodesApi {
    /// Returns the length of the i32 datatype tree size.
    ///
    /// # Returns
    /// - `usize` - the count of I32AstNodes in the AST
    fn get_i32_node_count(&self) -> usize;
}
