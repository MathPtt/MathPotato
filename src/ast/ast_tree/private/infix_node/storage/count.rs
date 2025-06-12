use super::InfixNodeStorageApi;

pub mod implementation;

pub trait InfixNodeStorageApiCount: InfixNodeStorageApi {
    fn count(&self) -> usize;
}
