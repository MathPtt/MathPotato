use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    ast_node_types_enum::AstNodeType, internal::i32_ast_node_internal::I32AstNodeInternal,
};

#[derive(Debug)]
pub struct I32AstNode {
    pub id: Uuid,
    pub value: i32,
    pub parent_id: Uuid,
    pub parent_type: AstNodeType,
}

impl I32AstNode {
    pub fn new_with_value(value: i32) -> I32AstNode {
        I32AstNode {
            id: Uuid::nil(),
            value,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        }
    }
    pub fn new_value_parent_id_and_type(
        value: i32,
        parent_type: AstNodeType,
        parent_id: Uuid,
    ) -> I32AstNode {
        I32AstNode {
            id: Uuid::nil(),
            value,
            parent_id,
            parent_type,
        }
    }

    pub fn from_internal_and_id(
        node: I32AstNodeInternal,
        id: Uuid,
    ) -> Result<I32AstNode, ParseError> {
        Ok(I32AstNode {
            id,
            value: node.value,
            parent_id: node.parent_id,
            parent_type: node.parent_type,
        })
    }
    pub fn to_internal(&self) -> Result<I32AstNodeInternal, ParseError> {
        Ok(I32AstNodeInternal {
            value: self.value,
            parent_id: self.parent_id,
            parent_type: self.parent_type.clone(),
        })
    }
}
