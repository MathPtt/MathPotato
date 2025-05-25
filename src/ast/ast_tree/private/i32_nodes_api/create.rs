use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::I32AstTreeApi;
pub mod implementation;

pub trait I32AstTreeApiPut: I32AstTreeApi {
    fn create(&mut self, i32_value: i32) -> Result<Uuid, ParseError>;
}
