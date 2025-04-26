use uuid::Uuid;

use super::ast_node_types_enum::AstNodeType;
use super::infix_operation_enum::InfixOperationType;

#[derive(Debug, Clone)]
pub struct InfixOperationAstNode {
    operation_type: InfixOperationType,
    left_id: Uuid,
    left_type: AstNodeType,
    right_id: Uuid,
    right_type: AstNodeType,
}

impl InfixOperationAstNode {
    pub fn new_with_type_and_left_child_node(
        operation_type: InfixOperationType,
        left_type: AstNodeType,
        left: Uuid,
    ) -> InfixOperationAstNode {
        InfixOperationAstNode {
            operation_type,
            left_type,
            left_id: left,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
        }
    }
    pub fn get_operation_type(&self) -> InfixOperationType {
        self.operation_type.clone()
    }
    pub fn get_left_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.left_id == Uuid::nil() {
            None
        } else {
            Some((self.left_id, self.left_type.clone()))
        }
    }
    pub fn get_right_node(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }
}
