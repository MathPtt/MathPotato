use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiSetLeftId: InfixNodeApi {
    fn set_left_id(&mut self, left_id: Uuid) -> Result<Uuid, ParseError>;
}
impl InfixNodeApiSetLeftId for InfixNode {
    fn set_left_id(&mut self, left_id: Uuid) -> Result<Uuid, ParseError> {
        self.left_id = left_id;
        Ok(left_id)
    }
}
