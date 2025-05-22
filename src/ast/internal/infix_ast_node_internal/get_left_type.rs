use crate::{
    ast::{ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum},
    parser::parser_error::ParseError,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalGetLeftType: InfixAstNodeInternalApi {
    fn get_left_type(&self) -> Result<AstNodeType, ParseError>;
}
impl InfixAstNodeInternalGetLeftType for InfixAstNodeInternal {
    fn get_left_type(&self) -> Result<AstNodeType, ParseError> {
        match self.left_type {
            AstNodeType::I32AstNode => Ok(AstNodeType::I32AstNode),
            AstNodeType::InfixOperationAstNode => Ok(AstNodeType::I32AstNode),
            AstNodeType::None => Err(ParseError::new(format!(
                "{:#?} type with id doesn't have left type",
                AstNodeType::InfixOperationAstNode
            ))),
        }
    }
}
