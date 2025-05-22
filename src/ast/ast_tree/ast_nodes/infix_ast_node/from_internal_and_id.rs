use uuid::Uuid;

use crate::{
    ast::internal::infix_ast_node_internal::InfixAstNodeInternal, parser::parser_error::ParseError,
};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeFromInternalAndId: InfixAstNodeApi {
    fn from_internal_and_id(
        id: Uuid,
        internal_node: InfixAstNodeInternal,
    ) -> Result<InfixAstNode, ParseError>;
}

impl InfixAstNodeFromInternalAndId for InfixAstNode {
    fn from_internal_and_id(
        id: Uuid,
        internal_node: InfixAstNodeInternal,
    ) -> Result<InfixAstNode, ParseError> {
        Ok(InfixAstNode {
            id,
            operation_type: internal_node.operation_type.clone(),
            left_id: internal_node.left_id,
            left_type: internal_node.left_type.clone(),
            right_id: internal_node.right_id,
            right_type: internal_node.right_type.clone(),
            parent_id: Uuid::nil(),
            token_literal_value: &String::from(""),
        })
    }
}
