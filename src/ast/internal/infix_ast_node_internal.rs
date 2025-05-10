use crate::ast::ast_node_types_enum::AstNodeType;
use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;
use crate::parser::parser_error::ParseError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InfixAstNodeInternal {
    pub operation_type: InfixOperationTypeEnum,
    pub left_id: Uuid,
    pub left_type: AstNodeType,
    pub right_id: Uuid,
    pub right_type: AstNodeType,
}

impl InfixAstNodeInternal {
    pub fn new_with_type_and_left_child_node(
        operation_type: InfixOperationTypeEnum,
        left_type: AstNodeType,
        left: Uuid,
    ) -> InfixAstNodeInternal {
        InfixAstNodeInternal {
            operation_type,
            left_type,
            left_id: left,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
        }
    }
    pub fn get_operation_type(&self) -> InfixOperationTypeEnum {
        self.operation_type.clone()
    }

    pub fn get_left_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.left_id == Uuid::nil() {
            None
        } else {
            Some((self.left_id, self.left_type.clone()))
        }
    }

    pub fn get_right_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }

    pub fn get_right_node(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }

    pub(crate) fn is_left_occupied(&self) -> bool {
        self.left_type != AstNodeType::None && self.left_id != Uuid::nil()
    }

    pub(crate) fn is_right_occupied(&self) -> bool {
        self.right_type != AstNodeType::None && self.right_id != Uuid::nil()
    }
}
