use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::AstApi;
pub mod implementation;

pub trait CreateI32NodeWithValue: AstApi {
    fn i32_api_create_node_with_value(&mut self, i32_value: i32) -> Result<Uuid, ParseError>;
}
