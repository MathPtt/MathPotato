use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;
use super::infix_operation_type_enum::InfixOperationTypeEnum;

pub trait InfixAstNodeInternalNewWithTypeAndLeftChildNode: InfixNodeApi {
    fn new_with_type_and_left_child_node(
        operation_type: InfixOperationTypeEnum,
        left_type: AstNodeType,
        left: Uuid,
    ) -> InfixNode;
}
impl InfixAstNodeInternalNewWithTypeAndLeftChildNode for InfixNode {
    fn new_with_type_and_left_child_node(
        operation_type: InfixOperationTypeEnum,
        left_type: AstNodeType,
        left: Uuid,
    ) -> Self {
        InfixNode {
            operation_type,
            left_type,
            left_id: left,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        }
    }
}
