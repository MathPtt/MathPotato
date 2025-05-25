use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalSetLeftId: InfixAstNodeInternalApi {
    fn set_left_id(&mut self, left_id: Uuid) -> Result<Uuid, ParseError>;
}
impl InfixAstNodeInternalSetLeftId for InfixAstNodeInternal {
    fn set_left_id(&mut self, left_id: Uuid) -> Result<Uuid, ParseError> {
        self.left_id = left_id;
        Ok(left_id)
    }
}
