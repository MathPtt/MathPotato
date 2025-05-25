use uuid::Uuid;

use super::{AstContinuationNode, AstContinuationNodeApi};

pub trait AstContinuationNodeApiNew: AstContinuationNodeApi {
    fn new() -> Self;
}

impl AstContinuationNodeApiNew for AstContinuationNode {
    fn new() -> Self {
        AstContinuationNode {
            id: Uuid::nil(),
            node_type: AstNodeType::None,
        }
    }
}
