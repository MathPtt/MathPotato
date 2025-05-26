use uuid::Uuid;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiSetId: ContinuationNodeApi {
    fn set_id(&mut self, id: Uuid) -> Uuid;
}

impl ContinuationNodeApiSetId for ContinuationNode {
    fn set_id(&mut self, id: Uuid) -> Uuid {
        self.id = id;
        id
    }
}
