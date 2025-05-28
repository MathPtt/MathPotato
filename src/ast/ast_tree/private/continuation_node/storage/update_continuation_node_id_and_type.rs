use crate::{
    ast::ast_tree::private::continuation_node::node::ContinuationNode,
    parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageApiUpdateContinuationNodeIdAndType:
    ContinuationNodeStorageApi
{
    fn update_continuation_node_id_and_type(
        &mut self,
        node: ContinuationNode,
    ) -> Result<ContinuationNode, ParseError>;
}
