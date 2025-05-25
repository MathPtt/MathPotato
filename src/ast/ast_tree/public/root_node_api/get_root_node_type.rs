use super::RootNodeApi;
pub mod implementation;

pub trait RootNodeApiGetRootNodeType: RootNodeApi {
    /// Get Root Node Type method
    ///
    /// Returns the root node type.
    ///
    /// # Returns
    /// - `AstNodeType` - representing the actual type of the Root Node.
    fn get_root_node_type(&self) -> AstNodeType;
}
