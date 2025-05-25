use std::collections::HashMap;

use super::{NodeCatalogApi, NodeCatalogInternalApi};

pub trait NodeCatalogApiNew: NodeCatalogInternalApi {
    fn new() -> Self;
}
impl NodeCatalogApiNew for NodeCatalogApi {
    fn new() -> Self {
        NodeCatalogApi {
            catalog: HashMap::new(),
        }
    }
}
