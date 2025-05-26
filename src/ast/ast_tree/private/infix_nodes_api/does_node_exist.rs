use uuid::Uuid;

use super::{InfixAstTreeApi, InfixNodesApi};

pub mod implementation;

pub trait InfixAstTreeApiGet: InfixAstTreeApi {
    /// Checks if there is an `InfixAstNodeInternal` in the AST tree with the provided id.
    ///
    /// # Parameters
    /// - `id` - `Uuid`: the node id.
    ///
    /// # Returns
    /// - `true` - when the node exist
    /// - `false` - when the node does not exist in the tree.
    fn does_node_exist(&self, id: Uuid) -> bool;
}
