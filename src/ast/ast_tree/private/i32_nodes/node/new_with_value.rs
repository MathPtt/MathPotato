use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::parser::parser_error::ParseError;

use super::I32Node;
use super::I32NodeApi;

pub trait I32AstNodeInternalApiNewWithValue: I32NodeApi {
    fn new_with_value(value: i32) -> Result<I32Node, ParseError>;
}
impl I32AstNodeInternalApiNewWithValue for I32Node {
    fn new_with_value(value: i32) -> Result<I32Node, ParseError> {
        Ok(I32Node {
            value,
            parent_id: Uuid::nil(),
            parent_type: AstNodeType::None,
        })
    }
}
