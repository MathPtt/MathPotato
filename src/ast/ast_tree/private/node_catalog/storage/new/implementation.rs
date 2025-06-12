use std::collections::HashMap;

use crate::ast::ast_tree::private::node_catalog::storage::NodeCatalogStorage;

use super::NodeCatalogApiNew;

impl NodeCatalogApiNew for NodeCatalogStorage {
    fn new() -> Self {
        NodeCatalogStorage {
            catalog: HashMap::new(),
        }
    }
}
