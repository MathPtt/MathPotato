use uuid::Uuid;

use super::math_potato_ast_node_types_enum::MathPotatoAstNodeType;

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstNode {
    uuid: Uuid,
    value: i32,
    parent_uuid: Uuid,
    parent_type: MathPotatoAstNodeType,
    child_uuid: Uuid,
    child_type: MathPotatoAstNodeType,
}

impl I32AstNode {
    pub fn new_with_value(value: i32) -> I32AstNode {
        I32AstNode {
            uuid: Uuid::new_v4(),
            value,
            parent_uuid: Uuid::nil(),
            parent_type: MathPotatoAstNodeType::None,
            child_uuid: Uuid::nil(),
            child_type: MathPotatoAstNodeType::None,
        }
    }
}
