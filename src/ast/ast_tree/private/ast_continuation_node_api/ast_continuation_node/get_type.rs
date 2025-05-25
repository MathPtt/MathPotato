use super::{AstContinuationNode, AstContinuationNodeApi};

pub trait AstContinuationNodeApiGetType: AstContinuationNodeApi {
    fn get_type(&self) -> AstNodeType;
}

impl AstContinuationNodeApiGetType for AstContinuationNode {
    fn get_type(&self) -> AstNodeType {
        self.node_type
    }
}
