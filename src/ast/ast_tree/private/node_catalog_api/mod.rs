use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::global::enums::ast_node_types_enum::AstNodeType;

pub mod add;
pub mod get_node_type;
pub mod new;

#[derive(Clone, Debug)]
pub struct NodeCatalogApi {
    catalog: HashMap<Uuid, AstNodeType>,
}

pub trait NodeCatalogInternalApi {}
impl NodeCatalogInternalApi for NodeCatalogApi {}
