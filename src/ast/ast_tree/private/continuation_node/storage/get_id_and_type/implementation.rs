use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::ast_continuation_node::{
            node::{ContinuationNode, ContinuationNodeApi},
            storage::ContinuationNodeStorage,
        },
    },
    parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApiGetContNodeIdAndType;

impl ContinuationNodeStorageApiGetContNodeIdAndType for ContinuationNodeStorage {
    fn get_type_and_id(&self) -> Result<ContinuationNode, ParseError> {
        if self.id == Uuid::nil() || self.node_type == AstNodeType::None {
            Err(ParseError::new(format!("Either there is no continuation node or the type is default, but something is wrong. Details: {}", self)))
        } else {
            Ok(self.clone())
        }
    }
}
