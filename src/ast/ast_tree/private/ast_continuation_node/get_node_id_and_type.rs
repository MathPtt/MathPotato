use crate::parser::parser_error::ParseError;

use super::{ast_continuation_node::ContinuationNode, AstContinuationNodeApi};

pub mod implementation;

pub trait AstContinuationNodeApiGetContNodeIdAndType: ContinuationNodeStorageApi {
    fn get_type_and_id(&self) -> Result<ContinuationNode, ParseError>;
}
