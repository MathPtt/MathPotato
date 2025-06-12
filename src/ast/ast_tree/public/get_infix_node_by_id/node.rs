use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::infix_operation_type_enum::InfixOperationTypeEnum;

pub mod new_from_id_and_infix_node;

#[derive(Debug, Clone, Display)]
#[display(
    "GetInfixNodeByIdResult(id: {}, operation_type: {}, left_id: {}, left_type: {}, right_id: {}, right_type:{}, parent_id: {}, parent_type: {})",
    id,
    operation_type,
    left_id,
    left_type,
    right_id,
    right_type,
    parent_id,
    parent_type
)]
pub struct GetInfixNodeByIdResult {
    id: Uuid,
    operation_type: InfixOperationTypeEnum,
    left_id: Uuid,
    left_type: AstNodeType,
    right_id: Uuid,
    right_type: AstNodeType,
    parent_id: Uuid,
    parent_type: AstNodeType,
}

impl GetInfixNodeByIdResult {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn operation_type(&self) -> &InfixOperationTypeEnum {
        &self.operation_type
    }

    pub fn left_id(&self) -> Uuid {
        self.left_id
    }

    pub fn left_type(&self) -> &AstNodeType {
        &self.left_type
    }

    pub fn right_id(&self) -> Uuid {
        self.right_id
    }

    pub fn parent_id(&self) -> Uuid {
        self.parent_id
    }

    pub fn parent_type(&self) -> &AstNodeType {
        &self.parent_type
    }

    pub fn set_parent_type(&mut self, parent_type: AstNodeType) {
        self.parent_type = parent_type;
    }

    pub fn set_parent_id(&mut self, parent_id: Uuid) {
        self.parent_id = parent_id;
    }

    pub fn set_right_type(&mut self, right_type: AstNodeType) {
        self.right_type = right_type;
    }

    pub fn set_right_id(&mut self, right_id: Uuid) {
        self.right_id = right_id;
    }

    pub fn set_left_type(&mut self, left_type: AstNodeType) {
        self.left_type = left_type;
    }

    pub fn set_left_id(&mut self, left_id: Uuid) {
        self.left_id = left_id;
    }

    pub fn set_operation_type(&mut self, operation_type: InfixOperationTypeEnum) {
        self.operation_type = operation_type;
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }
}

pub trait GetInfixNodeByIdResultApi {}
impl GetInfixNodeByIdResultApi for GetInfixNodeByIdResult {}
