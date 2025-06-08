use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::storage::ContinuationNodeStorage;

use super::ContinutationNodeStorageApiSetType;

impl ContinutationNodeStorageApiSetType for ContinuationNodeStorage {
    fn set_type(&mut self, node_type: AstNodeType) {
        self.node_type = node_type;
    }
}
