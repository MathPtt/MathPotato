use anyhow::Result;
use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;

use super::I32NodeStorageApi;

pub mod implementation;

pub trait I32NodeStorageApiGetNodeById: I32NodeStorageApi {
    fn get_node_by_id(&self, id: Uuid) -> Result<I32Node>;
}
