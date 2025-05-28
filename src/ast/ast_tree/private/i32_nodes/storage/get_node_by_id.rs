use uuid::Uuid;

use crate::ast::ast_tree::public::i32_nodes_api::I32NodeStorageApi;

use super::entity::I32AstEntity;

pub mod implementation;

pub trait I32AstTreeApiGetNodeById: I32NodeStorageApi {
    fn get_node_by_id(&self, id: Uuid) -> Option<I32AstEntity>;
}
