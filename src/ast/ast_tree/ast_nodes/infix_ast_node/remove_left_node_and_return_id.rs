use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeRemoveLeftNodeAndReturnId: InfixAstNodeApi {
    fn remove_left_node_and_return_id(&mut self) -> Result<Uuid, ParseError>;
}
impl InfixAstNodeRemoveLeftNodeAndReturnId for InfixAstNode {
    fn remove_left_node_and_return_id(&mut self) -> Result<Uuid, ParseError> {
        if self.left_id == Uuid::nil() {
            Err(ParseError::new(format!(
                "Infix node with id: {} doesn't have left node.",
                self.id
            )))
        } else {
            let left_id = self.left_id;
            self.left_id = Uuid::nil();
            Ok(left_id)
        }
    }
}
