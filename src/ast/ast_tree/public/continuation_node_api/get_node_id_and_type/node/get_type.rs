use crate::ast::global::enums::ast_node_types_enum::AstNodeType;

use super::{GetNodeIdAndTypeResult, GetNodeIdAndTypeResultApi};

pub trait ContinuationNodeApiGetType: GetNodeIdAndTypeResultApi {
    fn get_type(&self) -> AstNodeType;
}
impl ContinuationNodeApiGetType for GetNodeIdAndTypeResult {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
