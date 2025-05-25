use super::{InfixAstTreeApi, InfixNodesApi};

pub trait InfixAstTreeApiLen: InfixAstTreeApi {
    fn len(&self) -> usize;
}
impl InfixAstTreeApiLen for InfixNodesApi {
    fn len(&self) -> usize {
        self.tree.len()
    }
}
