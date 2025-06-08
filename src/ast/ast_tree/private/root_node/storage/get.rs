use crate::ast::ast_tree::private::root_node::node::RootNode;

use super::RootNodeStorageApi;

pub mod implementation;

pub trait RootNodeStorageApiGet: RootNodeStorageApi {
    fn get(&self) -> RootNode;
}
