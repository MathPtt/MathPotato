use storage::ContinuationNodeStorage;

pub mod node;
pub mod storage;

pub trait ContinuationNodeStorageApi {}
impl ContinuationNodeStorageApi for ContinuationNodeStorage {}
