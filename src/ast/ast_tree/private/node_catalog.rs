use storage::NodeCatalogStorage;

pub mod add;
pub mod get_node_type;
pub mod new;
pub mod storage;

pub trait NodeCatalogStorageApi {}
impl NodeCatalogStorageApi for NodeCatalogStorage {}
