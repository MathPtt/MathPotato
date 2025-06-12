use super::NodeCatalogStorageApi;

pub mod implementation;

pub trait NodeCatalogApiNew: NodeCatalogStorageApi {
    fn new() -> Self;
}
