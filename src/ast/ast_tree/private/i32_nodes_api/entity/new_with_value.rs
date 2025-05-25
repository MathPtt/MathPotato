use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{I32AstEntity, I32AstNodeInternalApi};

pub trait I32AstNodeInternalApiNewWithValue: I32AstNodeInternalApi {
    fn new_with_value(value: i32) -> Result<I32AstEntity, ParseError>;
}
impl I32AstNodeInternalApiNewWithValue for I32AstEntity {
    fn new_with_value(value: i32) -> Result<I32AstEntity, ParseError> {
        Ok(I32AstEntity {
            value,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        })
    }
}
