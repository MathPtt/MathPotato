use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::continuation_node::node::ContinuationNode,
    },
    parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApi;

pub mod implementation;

pub trait ContinuationNodeStorageSetIdAndType: ContinuationNodeStorageApi {
    fn set_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<ContinuationNode, ParseError>;
}
