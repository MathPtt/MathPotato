use uuid::Uuid;

use crate::{
    ast::global::enums::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError,
};

use super::{I32AstNodeInternal, I32AstNodeInternalApi};

pub trait I32AstNodeInternalApiNewValueParentIdAndType: I32AstNodeInternalApi {
    fn new_value_parent_id_and_type(
        value: i32,
        parent_type: AstNodeType,
        parent_id: Uuid,
    ) -> Result<I32AstNodeInternal, ParseError>;
}
impl I32AstNodeInternalApiNewValueParentIdAndType for I32AstEntity {
    fn new_value_parent_id_and_type(
        value: i32,
        parent_type: AstNodeType,
        parent_id: Uuid,
    ) -> Result<I32AstNodeInternal, ParseError> {
        Ok(I32AstNodeInternal {
            value,
            parent_id,
            parent_type,
        })
    }
}
