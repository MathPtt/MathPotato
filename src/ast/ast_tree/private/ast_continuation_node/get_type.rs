use super::AstContinuationNodeApi;

pub mod implementation;
pub trait AstContinuationInternalNodeApiGetType: ContinuationNodeStorageApi {
    fn get_type(&self) -> AstNodeType;
}
