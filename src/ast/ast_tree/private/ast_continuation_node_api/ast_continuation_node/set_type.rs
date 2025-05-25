use super::{AstContinuationNode, AstContinuationNodeApi};

pub trait AstContinuationNodeApiSetType: AstContinuationNodeApi {
    fn set_type(&mut self, node_type: AstNodeType) -> AstNodeType;
}

impl AstContinuationNodeApiSetType for AstContinuationNode {
    fn set_type(&self, node_type: AstNodeType) -> AstNodeType {
        self.node_type = node_type;
        node_type.clone()
    }
}
