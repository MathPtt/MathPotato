use uuid::Uuid;

use super::ast_node_types_enum::MathPotatoAstNodeType;

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
