use uuid::Uuid;

use super::{AstContinuationNode, AstContinuationNodeApi};

pub trait AstContinuationNodeApiSetId: AstContinuationNodeApi {
    fn set_id(&mut self, id: Uuid) -> Uuid;
}

impl AstContinuationNodeApiSetId for AstContinuationNode {
    fn set_id(&mut self, id: Uuid) -> Uuid {
        self.id = id;
        id
    }
}
