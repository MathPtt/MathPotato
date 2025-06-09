use uuid::Uuid;

use super::dto::I32NodeDto;
use crate::ast::ast_tree::public::AstApi;
use anyhow::Result;

pub mod implementation;

pub trait GetI32NodeById: AstApi {
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32NodeDto>;
}
