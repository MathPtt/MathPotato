use uuid::Uuid;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageApiSetId: ContinuationNodeStorageApi {
    fn set_id(&mut self, id: Uuid);
}
