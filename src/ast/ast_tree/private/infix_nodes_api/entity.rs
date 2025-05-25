use infix_operation_type_enum::InfixOperationTypeEnum;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod get_left_id;
pub mod get_left_type;
pub mod get_operation_type;
pub mod get_right_id;
pub mod get_right_type;
pub mod infix_operation_type_enum;
pub mod new_with_type_and_left_child_node;
pub mod set_left_id;
pub mod set_left_type;

#[derive(Debug, Clone)]
pub struct InfixAstNodeInternal {
    operation_type: InfixOperationTypeEnum,
    left_id: Uuid,
    left_type: AstNodeType,
    right_id: Uuid,
    right_type: AstNodeType,
}

pub trait InfixAstNodeInternalApi {}
impl InfixAstNodeInternalApi for InfixAstNodeInternal {}
