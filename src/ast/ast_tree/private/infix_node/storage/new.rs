use std::collections::HashMap;

use super::{InfixNodeStorage, InfixNodeStorageApi};

pub trait InfixNodeStorageApiNew: InfixNodeStorageApi {
    fn new() -> Self;
}
impl InfixNodeStorageApiNew for InfixNodeStorage {
    fn new() -> Self {
        InfixNodeStorage {
            tree: HashMap::new(),
        }
    }
}
