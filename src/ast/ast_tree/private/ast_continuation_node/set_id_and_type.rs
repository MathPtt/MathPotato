use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{ast_continuation_node::ContinuationNode, AstContinuationNodeApi};

pub mod implementation;

pub trait AstContinuationInternalNodeSetIdAndType: ContinuationNodeStorageApi {
    fn set_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<ContinuationNode, ParseError>;
}
