use crate::{
    ast::ast_tree::private::continuation_node::node::ContinuationNode,
    parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageApiGetContNodeIdAndType: ContinuationNodeStorageApi {
    fn get_type_and_id(&self) -> Result<ContinuationNode, ParseError>;
}
