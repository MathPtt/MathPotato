use uuid::Uuid;

use super::{AstContinuationNode, AstContinuationNodeApi};

pub trait AstContinuationNodeApiGetId: AstContinuationNodeApi {
    fn get_id(&self) -> Uuid;
}

impl AstContinuationNodeApiGetId for AstContinuationNode {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
