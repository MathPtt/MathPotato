use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiSetType: ContinuationNodeApi {
    fn set_type(&mut self, node_type: AstNodeType) -> AstNodeType;
}

impl ContinuationNodeApiSetType for ContinuationNode {
    fn set_type(&self, node_type: AstNodeType) -> AstNodeType {
        self.node_type = node_type;
        node_type.clone()
    }
}
