use storage::I32NodesApi;

pub mod create;
pub mod entity;
pub mod get_node_by_id;
pub mod get_node_count;
pub mod get_nodes;
pub mod put_all;
pub mod storage;
pub mod update;

pub trait I32AstTreeApi {}
impl I32AstTreeApi for I32NodesApi {}
