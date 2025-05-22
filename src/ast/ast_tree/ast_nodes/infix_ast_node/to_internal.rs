use crate::{
    ast::internal::infix_ast_node_internal::InfixAstNodeInternal, parser::parser_error::ParseError,
};

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeToInternal: InfixAstNodeApi {
    fn to_internal(&self) -> Result<InfixAstNodeInternal, ParseError>;
}

impl InfixAstNodeToInternal for InfixAstNode {
    fn to_internal(&self) -> Result<InfixAstNodeInternal, ParseError> {
        Ok(InfixAstNodeInternal {
            operation_type: self.operation_type.clone(),
            left_id: self.left_id,
            left_type: self.left_type.clone(),
            right_id: self.right_id,
            right_type: self.right_type.clone(),
        })
    }
}
