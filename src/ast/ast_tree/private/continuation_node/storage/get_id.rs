use uuid::Uuid;

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageApiGetId: ContinuationNodeStorageApi {
    fn get_id(&self) -> Uuid;
}
