use super::AstContinuationNodeApi;

pub mod implementation;
pub trait AstContinuationInternalNodeApiGetType: AstContinuationNodeApi {
    fn get_type(&self) -> AstNodeType;
}
