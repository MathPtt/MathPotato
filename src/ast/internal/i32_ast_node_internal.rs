use uuid::Uuid;

use crate::ast::{ast_node_types_enum::AstNodeType, i32_node::I32AstNode};

/// Represents an i32 value of the language.
#[derive(Clone, Debug)]
pub struct I32AstNodeInternal {
    pub value: i32,
    pub parent_id: Uuid,
    pub parent_type: AstNodeType,
}

impl I32AstNodeInternal {
    pub fn new_with_value(value: i32) -> I32AstNodeInternal {
        I32AstNodeInternal {
            value,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        }
    }
    pub fn new_value_parent_id_and_type(
        value: i32,
        parent_type: AstNodeType,
        parent_id: Uuid,
    ) -> I32AstNodeInternal {
        I32AstNodeInternal {
            value,
            parent_id,
            parent_type,
        }
    }
}
