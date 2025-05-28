use node::UpdateContinuationNodeIdAndTypeResult;
use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::AstApi;

pub mod implementation;
pub mod node;

pub trait UpdateContinuationNodeIdAndType: AstApi {
    fn update_continuation_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<UpdateContinuationNodeIdAndTypeResult, ParseError>;
}
