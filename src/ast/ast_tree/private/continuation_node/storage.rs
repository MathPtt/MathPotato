use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod get_id;
pub mod get_node;
pub mod get_type;
pub mod update_continuation_node_id_and_type;

#[derive(Clone, Debug, Display)]
#[display("ContinuationNodeStorage(id: {}, node_type: {})", id, node_type)]
pub struct ContinuationNodeStorage {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait ContinuationNodeStorageApi {}
impl ContinuationNodeStorageApi for ContinuationNodeStorage {}
