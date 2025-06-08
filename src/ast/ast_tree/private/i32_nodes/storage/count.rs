use super::I32NodeStorageApi;

pub mod implementation;

pub trait I32NodeStorageApiCount: I32NodeStorageApi {
    fn count(&self) -> usize;
}
