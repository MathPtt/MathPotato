use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinutationNodeStorageApiSetType: ContinuationNodeStorageApi {
    fn set_type(&mut self, node_type: AstNodeType);
}
