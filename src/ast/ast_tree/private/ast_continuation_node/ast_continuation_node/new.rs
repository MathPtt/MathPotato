use uuid::Uuid;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiCtor: ContinuationNodeApi {
    fn new() -> Self;
}

impl ContinuationNodeApiCtor for ContinuationNode {
    fn new() -> Self {
        ContinuationNode {
            id: Uuid::nil(),
            node_type: AstNodeType::None,
        }
    }
}
