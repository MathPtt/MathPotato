use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiNewFromIdAndType: ContinuationNodeApi {
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> ContinuationNode;
}
impl ContinuationNodeApiNewFromIdAndType for ContinuationNode {
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> ContinuationNode {
        ContinuationNode { id, node_type }
    }
}
