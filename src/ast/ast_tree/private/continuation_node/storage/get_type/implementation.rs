use crate::ast::ast_tree::{
    global::enums::ast_node_types_enum::AstNodeType,
    private::ast_continuation_node::{
        node::{get_type::ContinuationNodeApiGetType, ContinuationNode},
        storage::ContinuationNodeStorage,
        ContinuationNodeStorageApi,
    },
};

impl AstContinuationNodeApiGetType for ContinuationNodeStorage {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
