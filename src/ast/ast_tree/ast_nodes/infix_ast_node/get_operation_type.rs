use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeGetOperationType: InfixAstNodeApi {
    fn get_operation_type(&self) -> InfixOperationTypeEnum;
}
impl InfixAstNodeGetOperationType for InfixAstNode {
    fn get_operation_type(&self) -> InfixOperationTypeEnum {
        self.operation_type.clone()
    }
}
