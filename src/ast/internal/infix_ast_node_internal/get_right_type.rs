use crate::{
    ast::{ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum},
    parser::parser_error::ParseError,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalGetRightType: InfixAstNodeInternalApi {
    fn get_right_type(&self) -> Result<AstNodeType, ParseError>;
}
impl InfixAstNodeInternalGetRightType for InfixAstNodeInternal {
    fn get_right_type(&self) -> Result<AstNodeType, ParseError> {
        match self.left_type {
            AstNodeType::I32AstNode => Ok(AstNodeType::I32AstNode),
            AstNodeType::InfixOperationAstNode => Ok(AstNodeType::I32AstNode),
            AstNodeType::None => Err(ParseError::new(format!(
                "{:#?} type with id doesn't have right type",
                AstNodeType::InfixOperationAstNode
            ))),
        }
    }
}
