use uuid::Uuid;

use super::math_potato_ast_node_types_enum::MathPotatoAstNodeType;

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstNode {
    pub uuid: Uuid,
    pub value: i32,
    pub parent_uuid: Uuid,
    pub parent_type: MathPotatoAstNodeType,
    pub child_uuid: Uuid,
    pub child_type: MathPotatoAstNodeType,
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
