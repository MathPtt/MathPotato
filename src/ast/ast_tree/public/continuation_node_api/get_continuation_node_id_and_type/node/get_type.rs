use crate::ast::{
    ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    global::enums::ast_node_types_enum::AstNodeType,
};

use super::{GetContinuationNodeIdAndTypeResult, GetContinuationNodeIdAndTypeResultApi};

pub trait GetContinuationNodeIdAndTypeResultGetType: GetContinuationNodeIdAndTypeResultApi {
    fn get_type(&self) -> AstNodeType;
}
impl GetContinuationNodeIdAndTypeResultGetType for GetContinuationNodeIdAndTypeResult {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
