use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;

use super::I32NodeStorageApi;
pub mod implementation;

pub trait I32NodeStorageApiGetNodes: I32NodeStorageApi {
    fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32Node>>;
}
