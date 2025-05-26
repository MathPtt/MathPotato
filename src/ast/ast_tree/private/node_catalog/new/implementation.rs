use std::collections::HashMap;

use crate::ast::ast_tree::private::node_catalog::{
    storage::NodeCatalogStorage, NodeCatalogStorageApi,
};

use super::NodeCatalogApiNew;

impl NodeCatalogApiNew for NodeCatalogStorageApi {
    fn new() -> Self {
        NodeCatalogStorage {
            catalog: HashMap::new(),
        }
    }
}
