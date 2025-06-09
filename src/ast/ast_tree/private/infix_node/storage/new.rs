use std::collections::HashMap;

use super::InfixNodeStorage;
use super::InfixNodeStorageApi;

pub trait InfixNodeStorageApiNew: InfixNodeStorageApi {
    fn new() -> Self;
}
impl InfixNodeStorageApiNew for InfixNodeStorage {
    fn new() -> Self {
        InfixNodeStorage {
            nodes: HashMap::new(),
        }
    }
}
