use uuid::Uuid;

use crate::ast::{
    ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum,
};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeNewWithTypeAndLeftChildNode: InfixAstNodeApi {
    fn new_with_type_and_left_child_node(
        infix_operation_type: InfixOperationTypeEnum,
        left_node_type: AstNodeType,
        left_node_id: Uuid,
    ) -> InfixAstNode;
}
impl InfixAstNodeNewWithTypeAndLeftChildNode for InfixAstNode {
    fn new_with_type_and_left_child_node(
        infix_operation_type: InfixOperationTypeEnum,
        left_node_type: AstNodeType,
        left_node_id: Uuid,
    ) -> InfixAstNode {
        InfixAstNode {
            id: Uuid::new_v4(),
            left_id: left_node_id,
            left_type: left_node_type,
            right_type: AstNodeType::None,
            right_id: Uuid::nil(),
            operation_type: infix_operation_type,
            parent_id: Uuid::nil(),
            token_literal_value: &String::from(""),
        }
    }
}
