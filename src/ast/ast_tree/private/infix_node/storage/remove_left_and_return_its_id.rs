use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApi;
pub mod implementation;

pub trait InfixNodeStorageApiRemoveLeftAndReturnItsId: InfixNodeStorageApi {
    fn remove_left_and_return_its_id(&mut self, node_id: Uuid) -> Result<Uuid, ParseError>;
}
