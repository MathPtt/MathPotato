use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::ContinuationNode;
use super::ContinuationNodeApi;

pub trait ContinuationNodeApiGetType: ContinuationNodeApi {
    fn get_type(&self) -> AstNodeType;
}

impl ContinuationNodeApiGetType for ContinuationNode {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
