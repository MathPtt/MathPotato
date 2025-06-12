use super::AstApi;

pub mod implementation;

pub trait GetInfixNodeCount: AstApi {
    fn get_infix_node_count(&self) -> usize;
}
