use uuid::Uuid;

use crate::{ast::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalGetRightId: InfixAstNodeInternalApi {
    fn get_right_id(&self) -> Result<Uuid, ParseError>;
}
impl InfixAstNodeInternalGetRightId for InfixAstNodeInternal {
    fn get_right_id(&self) -> Result<Uuid, ParseError> {
        if self.right_id == Uuid::nil() {
            Err(ParseError::new(format!(
                "There is no right if for {:#?} with id:",
                AstNodeType::InfixOperationAstNode
            )))
        } else {
            Ok(self.right_id)
        }
    }
}
