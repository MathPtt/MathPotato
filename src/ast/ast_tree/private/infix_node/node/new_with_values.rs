use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{infix_operation_type_enum::InfixOperationTypeEnum, InfixNode, InfixNodeApi};

pub trait InfixNodeApiNewWithValues: InfixNodeApi {
    fn new_with_values(
        operation_type: InfixOperationTypeEnum,
        left_id: Uuid,
        left_type: AstNodeType,
        right_id: Uuid,
        right_type: AstNodeType,
        parent_id: Uuid,
        paren_type: AstNodeType,
    ) -> Self;
}

impl InfixNodeApiNewWithValues for InfixNode {
    fn new_with_values(
        operation_type: InfixOperationTypeEnum,
        left_id: Uuid,
        left_type: AstNodeType,
        right_id: Uuid,
        right_type: AstNodeType,
        parent_id: Uuid,
        parent_type: AstNodeType,
    ) -> Self {
        InfixNode {
            operation_type,
            left_id,
            left_type,
            right_id,
            right_type,
            parent_id,
            parent_type,
        }
    }
}
