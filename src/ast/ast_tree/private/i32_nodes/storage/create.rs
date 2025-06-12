use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::I32NodeStorageApi;

pub mod implementation;

pub trait I32NodeStorageApiCreate: I32NodeStorageApi {
    fn create(&mut self, i32_value: i32) -> Result<Uuid, ParseError>;
}
