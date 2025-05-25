use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::public::i32_nodes_api::I32NodesApi;

use super::entity::I32AstEntity;
pub mod implementation;

pub trait I32AstTreeApiGetNodes: I32NodesApi {
    fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32AstEntity>>;
}
