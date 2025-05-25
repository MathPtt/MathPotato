use uuid::Uuid;

use super::AstContinuationNodeApi;
pub mod implementation;

pub trait AstContinuationInternalNodeApiGetId: AstContinuationNodeApi {
    fn get_id(&self) -> Uuid;
}
