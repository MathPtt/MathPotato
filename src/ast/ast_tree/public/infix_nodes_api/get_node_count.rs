use super::InfixNodesApi;

pub mod implementation;

pub trait InfixApiGetNodeCount: InfixNodesApi {
    fn get_infix_node_count(&self) -> usize;
}
