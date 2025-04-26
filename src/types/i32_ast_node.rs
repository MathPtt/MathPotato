use uuid::Uuid;

use super::ast_node_types_enum::AstNodeType;

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstNode {
    pub value: i32,
    pub parent_uuid: Uuid,
    pub parent_type: AstNodeType,
    pub child_uuid: Uuid,
    pub child_type: AstNodeType,
}

impl I32AstNode {
    pub fn new_with_value(value: i32) -> I32AstNode {
        I32AstNode {
            value,
            parent_uuid: Uuid::nil(),
            parent_type: AstNodeType::None,
            child_uuid: Uuid::nil(),
            child_type: AstNodeType::None,
        }
    }
}
