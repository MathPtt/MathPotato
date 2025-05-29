use node::InfixNode;

pub mod node;
pub mod storage;

pub trait InfixNodeStorageApi {}
impl InfixNodeStorageApi for InfixNode {}
