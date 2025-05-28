use uuid::Uuid;

use crate::ast::ast_tree::{
    global::enums::ast_node_types_enum::AstNodeType,
    private::continuation_node::{
        node::{
            new::ContinuationNodeApiCtor,
            new_from_id_and_type::ContinuationNodeApiNewFromIdAndType, ContinuationNode,
        },
        storage::ContinuationNodeStorage,
    },
};

use super::ContinuationNodeStorageApiGetNode;

impl ContinuationNodeStorageApiGetNode for ContinuationNodeStorage {
    fn get_node(&self) -> Option<ContinuationNode> {
        if self.id == Uuid::nil() && self.node_type == AstNodeType::None {
            None
        } else {
            Some(ContinuationNode::new_from_id_and_type(
                self.id,
                self.node_type,
            ))
        }
    }
}
