use uuid::Uuid;

use super::RootNode;
use super::RootNodeApi;

pub trait RootNodeApiGetId: RootNodeApi {
    fn get_id(&self) -> Uuid;
}

impl RootNodeApiGetId for RootNode {
    fn get_id(&self) -> Uuid {
        self.id
    }
}
