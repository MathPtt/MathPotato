use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;
use super::infix_operation_type_enum::InfixOperationTypeEnum;

pub trait InfixNodeApiNew: InfixNodeApi {
    fn new(infix_operation_type: InfixOperationTypeEnum) -> Self;
}

impl InfixNodeApiNew for InfixNode {
    fn new(operation_type: InfixOperationTypeEnum) -> Self {
        InfixNode {
            operation_type,
            left_id: Uuid::nil(),
            left_type: AstNodeType::None,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        }
    }
}
