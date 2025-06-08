use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::i32_nodes::node::get_parent_id::I32NodeApiGetParentId;
use crate::ast::ast_tree::private::i32_nodes::node::get_parent_type::I32NodeApiGetParentType;
use crate::ast::ast_tree::private::i32_nodes::node::get_value::I32NodeApiGetValue;
use crate::ast::ast_tree::private::i32_nodes::node::I32Node;

#[derive(Clone, Debug, Display)]
#[display(
    "I32NodeDto(value: {}, parent_id: {}, parent_type: {})",
    value,
    parent_id,
    parent_type
)]
pub struct I32NodeDto {
    value: i32,
    parent_id: Uuid,
    parent_type: AstNodeType,
}

impl I32NodeDto {
    pub fn new(value: i32, parent_id: Uuid, parent_type: AstNodeType) -> Self {
        Self {
            value,
            parent_id,
            parent_type,
        }
    }
    pub fn new_from_i32_node(i32_node: I32Node) -> Self {
        Self {
            value: i32_node.get_value(),
            parent_id: i32_node.get_parent_id(),
            parent_type: i32_node.get_parent_type(),
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }

    pub fn parent_id(&self) -> Uuid {
        self.parent_id
    }

    pub fn set_parent_id(&mut self, parent_id: Uuid) {
        self.parent_id = parent_id;
    }

    pub fn parent_type(&self) -> &AstNodeType {
        &self.parent_type
    }

    pub fn set_parent_type(&mut self, parent_type: AstNodeType) {
        self.parent_type = parent_type;
    }
}
