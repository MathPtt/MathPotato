use uuid::Uuid;

use super::infix_operation_type_enum::InfixOperationTypeEnum;
use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiNew: InfixNodeApi {
    fn new(infix_operation_type: InfixOperationTypeEnum) -> Self;
}

impl InfixNodeApiNew for InfixNode {
    fn new(operation_type: InfixOperationTypeEnum) -> Self {
        InfixNode {
            operation_type,
            left_id: Uuid::nil(),
            left_type: crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType::None,
            right_id: Uuid::nil(),
            right_type: crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType::None,
        }
    }
}
