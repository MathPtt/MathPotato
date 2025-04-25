use uuid::Uuid;

use super::math_potato_infix_operation_enum::InfixOperationType;
use super::math_potato_ast_node_types_enum::AstNodeType;

pub struct InfixOperationAstNode {
    operation_type: InfixOperationType,
    left: Uuid,
    left_type: AstNodeType
    right: Uuid,
    right_type: AstNodeType
}

impl InfixOperationAstNode {
    pub fn new_with_type_and_left_child_node(operation_type: InfixOperationType, left_type: AstNodeType, left: Uuid) -> InfixOperationAstNode {
        InfixOperationAstNode{
            operation_type,
            left_type,
            left,
            right:Uuid::nil(),
            right_type: AstNodeType::None 
        }
    }
    pub fn get_operation_type(&self) -> InfixOperationType {
        self.operation_type.clone()
    }
}
