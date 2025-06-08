use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod create_or_update_root_node;
pub mod get;
pub mod new;

#[derive(Clone, Debug, Display)]
#[display("RootNodeStorage(id: {}, node_type: {})", id, node_type)]
pub struct RootNodeStorage {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait RootNodeStorageApi {}
impl RootNodeStorageApi for RootNodeStorage {}
