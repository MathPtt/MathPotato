use uuid::Uuid;

use super::AstContinuationNodeApi;
pub mod implementation;

pub trait AstContinuationInternalNodeApiGetId: ContinuationNodeStorageApi {
    fn get_id(&self) -> Uuid;
}
