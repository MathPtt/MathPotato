use uuid::Uuid;

use crate::{
    ast::ast_tree::private::ast_continuation_node_api::{
        ast_continuation_node::AstContinuationNode, AstContinuationNodeApi,
    },
    parser::parser_error::ParseError,
};

use super::AstContinuationNodeApiGetContNodeIdAndType;

impl AstContinuationNodeApiGetContNodeIdAndType for AstContinuationNodeApi {
    fn get_type_and_id(&self) -> Result<AstContinuationNode, ParseError> {
        if self.id == Uuid::nil() || self.node_type == AstNodeType::None {
            Err(ParseError::new(format!("Either there is no continuation node or the type is default, but something is wrong. Details: {}", self)))
        } else {
            Ok(self.clone())
        }
    }
}
