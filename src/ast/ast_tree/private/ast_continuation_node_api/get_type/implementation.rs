use crate::ast::ast_tree::private::ast_continuation_node_api::ast_continuation_node::{
    get_type::AstContinuationNodeApiGetType, AstContinuationNode,
};

impl AstContinuationNodeApiGetType for AstContinuationNode {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
