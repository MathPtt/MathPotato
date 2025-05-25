use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod new_value_parent_id_and_type;
pub mod new_with_value;

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstEntity {
    pub value: i32,
    pub parent_id: Uuid,
    pub parent_type: AstNodeType,
}

pub trait I32AstNodeInternalApi {}
impl I32AstNodeInternalApi for I32AstEntity {}
