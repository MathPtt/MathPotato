use std::collections::HashMap;

use super::{I32NodeStorage, I32NodeStorageApi};

pub trait I32AstTreeApiNew: I32NodeStorageApi {
    fn new() -> I32NodeStorage;
}
impl I32AstTreeApiNew for I32NodeStorage {
    fn new() -> I32NodeStorage {
        I32NodeStorage {
            nodes: HashMap::new(),
        }
    }
}
