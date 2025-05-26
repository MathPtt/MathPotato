use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

#[derive(Clone, Debug)]
pub struct NodeCatalogStorage {
    catalog: HashMap<Uuid, AstNodeType>,
}
