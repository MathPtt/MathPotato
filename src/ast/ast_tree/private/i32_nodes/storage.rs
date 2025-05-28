use std::collections::HashMap;

use uuid::Uuid;

use super::node::I32Node;

pub mod create;
pub mod new;

#[derive(Clone, Debug)]
pub struct I32NodeStorage {
    nodes: HashMap<Uuid, I32Node>,
}

pub trait I32NodeStorageApi {}
impl I32NodeStorageApi for I32NodeStorage {}
