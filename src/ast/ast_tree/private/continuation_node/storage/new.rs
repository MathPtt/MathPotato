use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::ContinuationNodeStorage;
use super::ContinuationNodeStorageApi;

pub trait ContinuationNodeStorageApiNew: ContinuationNodeStorageApi {
    fn new() -> Self;
}
impl ContinuationNodeStorageApiNew for ContinuationNodeStorage {
    fn new() -> Self {
        ContinuationNodeStorage {
            id: Uuid::nil(),
            node_type: AstNodeType::None,
        }
    }
}
