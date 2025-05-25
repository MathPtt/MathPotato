use crate::parser::parser_error::ParseError;

use super::{
    infix_operation_type_enum::InfixOperationTypeEnum, InfixAstNodeInternal,
    InfixAstNodeInternalApi,
};

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
