use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, infix_operation_type_enum::InfixOperationTypeEnum},
    parser::parser_error::ParseError,
};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeNewWithTypeAndParentNode: InfixAstNodeApi {
    fn new_with_type_and_parent_node(
        operation_type: InfixOperationTypeEnum,
        token_literal_value: &str,
        parent_node_id: Uuid,
    ) -> Result<InfixAstNode, ParseError>;
}
impl InfixAstNodeNewWithTypeAndParentNode for InfixAstNode {
    fn new_with_type_and_parent_node(
        operation_type: InfixOperationTypeEnum,
        token_literal_value: &str,
        parent_node_id: Uuid,
    ) -> Result<InfixAstNode, ParseError> {
        Ok(InfixAstNode {
            id: Uuid::new_v4(),
            operation_type,
            left_id: Uuid::nil(),
            left_type: AstNodeType::None,
            right_id: Uuid::nil(),
            right_type: AstNodeType::None,
            parent_id: parent_node_id,
            token_literal_value,
        })
    }
}
