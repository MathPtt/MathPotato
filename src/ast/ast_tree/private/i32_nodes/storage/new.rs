use std::collections::HashMap;

use super::{I32NodeStorage, I32NodeStorageApi};
pub mod implementation;

pub trait I32NodeStorageApiNew: I32NodeStorageApi {
    fn new() -> I32NodeStorage;
}
