use storage::NodeCatalogStorage;

pub mod storage;

pub trait NodeCatalogStorageApi {}
impl NodeCatalogStorageApi for NodeCatalogStorage {}
