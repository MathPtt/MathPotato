use derive_more::Display;
use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

pub mod get_parent_id;
pub mod get_parent_type;
pub mod get_value;
pub mod new_with_value;
pub mod set_parent_id;
pub mod set_parent_type;
pub mod set_value;

/// Represents an i32 value of the language.
#[derive(Clone, Copy, Debug, Display)]
#[display(
    "I32Node(value: {}, parent_id: {}, parent_type: {})",
    value,
    parent_id,
    parent_type
)]
pub struct I32Node {
    value: i32,
    parent_id: Uuid,
    parent_type: AstNodeType,
}

pub trait I32NodeApi {}
impl I32NodeApi for I32Node {}
