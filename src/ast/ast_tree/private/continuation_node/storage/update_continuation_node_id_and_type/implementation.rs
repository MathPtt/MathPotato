use crate::{
    ast::ast_tree::private::continuation_node::{
        node::{
            ContinuationNode, get_id::ContinuationNodeApiGetId,
            get_type::ContinuationNodeApiGetType,
            new_from_id_and_type::ContinuationNodeApiNewFromIdAndType,
        },
        storage::ContinuationNodeStorage,
    },
    parser::parser_error::ParseError,
};

use super::ContinuationNodeStorageApiUpdateContinuationNodeIdAndType;

impl ContinuationNodeStorageApiUpdateContinuationNodeIdAndType for ContinuationNodeStorage {
    fn update_continuation_node_id_and_type(
        &mut self,
        node: ContinuationNode,
    ) -> Result<ContinuationNode, ParseError> {
        self.id = node.get_id();
        self.node_type = node.get_type();
        Ok(ContinuationNode::new_from_id_and_type(
            self.id,
            self.node_type.clone(),
        ))
    }
}
