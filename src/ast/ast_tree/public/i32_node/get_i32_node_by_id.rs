use uuid::Uuid;

use crate::ast::ast_tree::public::AstApi;
use crate::parser::parser_error::ParseError;

use super::dto::I32NodeDto;

pub mod implementation;

pub trait GetI32NodeById: AstApi {
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32NodeDto, ParseError>;
}
