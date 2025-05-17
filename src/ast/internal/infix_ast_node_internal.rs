use crate::ast::ast_node_types_enum::AstNodeType;
use crate::ast::infix_operation_type_enum::InfixOperationTypeEnum;
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
}
