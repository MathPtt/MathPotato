use uuid::Uuid;

use super::GetContinuationNodeIdAndTypeResult;
use super::GetContinuationNodeIdAndTypeResultApi;

pub trait GetContinuationNodeIdAndTypeResultGetId: GetContinuationNodeIdAndTypeResultApi {
    fn get_id(&self) -> Uuid;
}
impl GetContinuationNodeIdAndTypeResultGetId for GetContinuationNodeIdAndTypeResult {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
