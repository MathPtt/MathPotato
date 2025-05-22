use uuid::Uuid;

use crate::ast::{
    ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum,
};
pub mod add_i32node_to_the_right;
pub mod check_if_left_empty_right_occupied;
pub mod from_internal_and_id;
pub mod get_left_node_id;
pub mod get_left_node_id_and_type;
pub mod get_operation_type;
pub mod get_right_node;
pub mod get_right_node_id_and_type;
pub mod is_left_occupied;
pub mod is_right_occupied;
pub mod new_with_type_and_left_child_node;
pub mod new_with_type_and_parent_node;
pub mod remove_left_node_and_return_id;
pub mod to_internal;

#[derive(Debug, Clone)]
pub struct InfixAstNode {
    pub id: Uuid,
    pub operation_type: InfixOperationTypeEnum,
    pub left_id: Uuid,
    pub left_type: AstNodeType,
    pub right_id: Uuid,
    pub right_type: AstNodeType,
    pub parent_id: Uuid,
    pub token_literal_value: &str,
}

pub trait InfixAstNodeApi {}
impl InfixAstNodeApi for InfixAstNode {}
