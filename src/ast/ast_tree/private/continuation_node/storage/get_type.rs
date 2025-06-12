use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinutationNodeStorageApiGetType: ContinuationNodeStorageApi {
    fn get_type(&self) -> &AstNodeType;
}
