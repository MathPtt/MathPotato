use crate::ast::ast_tree::private::ast_continuation_node::ast_continuation_node::{
    get_type::AstContinuationNodeApiGetType, ContinuationNode,
};

impl AstContinuationNodeApiGetType for ContinuationNode {
    fn get_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
