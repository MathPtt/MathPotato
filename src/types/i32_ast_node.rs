use uuid::Uuid;

use super::ast_node_types_enum::AstNodeType;

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstNode {
    pub value: i32,
    pub parent_id: Uuid,
    pub parent_type: AstNodeType,
    pub child_id: Uuid,
    pub child_type: AstNodeType,
}

impl I32AstNode {
    pub fn new_with_value(value: i32) -> I32AstNode {
        I32AstNode {
            value,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
            child_id: Uuid::nil(),
            child_type: AstNodeType::None,
        }
    }
    pub fn new_value_parent_id_and_type(
        value: i32,
        parent_type: AstNodeType,
        parent_id: Uuid,
    ) -> I32AstNode {
        I32AstNode {
            value,
            parent_id,
            parent_type,
            child_id: Uuid::nil(),
            child_type: AstNodeType::None,
        }
    }
}
