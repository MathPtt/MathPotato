use storage::I32NodeStorage;

pub mod node;
pub mod storage;

pub trait I32AstTreeApi {}
impl I32AstTreeApi for I32NodeStorage {}
