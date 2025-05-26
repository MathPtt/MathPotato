use uuid::Uuid;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiGetId: ContinuationNodeApi {
    fn get_id(&self) -> Uuid;
}

impl ContinuationNodeApiGetId for ContinuationNode {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
