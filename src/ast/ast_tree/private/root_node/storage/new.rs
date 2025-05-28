use super::RootNodeStorageApi;

pub mod implementation;

pub trait RootNodeStorageApiNew: RootNodeStorageApi {
    fn new() -> Self;
}
