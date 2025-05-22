use crate::{
    ast::infix_operation_type_enum::InfixOperationTypeEnum, parser::parser_error::ParseError,
};

use super::{InfixAstNodeInternal, InfixAstNodeInternalApi};

pub trait InfixAstNodeInternalGetOperationType: InfixAstNodeInternalApi {
    fn get_infix_operation_type(&self) -> Result<InfixOperationTypeEnum, ParseError>;
}
impl InfixAstNodeInternalGetOperationType for InfixAstNodeInternal {
    fn get_infix_operation_type(&self) -> Result<InfixOperationTypeEnum, ParseError> {
        match self.operation_type {
            InfixOperationTypeEnum::Addition => Ok(InfixOperationTypeEnum::Addition),
            InfixOperationTypeEnum::Multiplication => Ok(InfixOperationTypeEnum::Multiplication),
        }
    }
}
