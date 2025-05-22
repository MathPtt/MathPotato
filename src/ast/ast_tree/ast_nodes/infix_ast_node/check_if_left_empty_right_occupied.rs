use uuid::Uuid;

use crate::{ast::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeCheckIfLeftEmptyRightOccupied: InfixAstNodeApi {
    fn check_if_left_empty_right_occupied(&self) -> Result<(), ParseError>;
}
impl InfixAstNodeCheckIfLeftEmptyRightOccupied for InfixAstNode {
    fn check_if_left_empty_right_occupied(&self) -> Result<(), ParseError> {
        if self.left_id != Uuid::nil()
            && self.left_type != AstNodeType::None
            && self.right_id != Uuid::nil()
            && self.right_type != AstNodeType::None
        {
            Err(ParseError::new(String::from("InfixOperationAstNode is in an impossible state. It has its left side empty, but right side occupied.")))
        } else {
            Ok(())
        }
    }
}
