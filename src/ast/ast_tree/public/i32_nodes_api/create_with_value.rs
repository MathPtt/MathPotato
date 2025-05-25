use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::I32NodesApi;
pub mod implementation;

pub trait I32ApiCreateNodeWithValue: I32NodesApi {
    fn i32_api_create_node_with_value(&mut self, i32_value: i32) -> Result<Uuid, ParseError>;
}
