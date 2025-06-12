use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod add;
pub mod get_node_type;
pub mod new;

#[derive(Clone, Debug)]
pub struct NodeCatalogStorage {
    catalog: HashMap<Uuid, AstNodeType>,
}

pub trait NodeCatalogStorageApi {}
impl NodeCatalogStorageApi for NodeCatalogStorage {}
