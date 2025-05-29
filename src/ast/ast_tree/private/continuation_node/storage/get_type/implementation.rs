use crate::ast::ast_tree::{
    global::enums::ast_node_types_enum::AstNodeType,
    private::continuation_node::storage::ContinuationNodeStorage,
};

use super::ContinutationNodeStorageApiGetType;

impl ContinutationNodeStorageApiGetType for ContinuationNodeStorage {
    fn get_type(&self) -> &AstNodeType {
        &self.node_type
    }
}
