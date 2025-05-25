use crate::parser::parser_error::ParseError;

use super::{ast_continuation_node::AstContinuationNode, AstContinuationNodeApi};

pub mod implementation;

pub trait AstContinuationNodeApiGetContNodeIdAndType: AstContinuationNodeApi {
    fn get_type_and_id(&self) -> Result<AstContinuationNode, ParseError>;
}
