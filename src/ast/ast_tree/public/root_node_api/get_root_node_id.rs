use uuid::Uuid;

use super::RootNodeApi;

pub mod implementation;
pub trait RootNodeApiGetRootNodeId: RootNodeApi {
    /// Get Root Node Id
    ///
    /// Returns the Root Node id.
    ///
    /// # Returns
    /// - `Some(Uuid)` - `Option<Uuid>` the Uuid value of the Root Node
    /// - `None` - When there is no Root Node set up
    fn get_root_node_id(&self) -> Option<Uuid>;
}
