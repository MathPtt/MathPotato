use uuid::Uuid;

use crate::ast::ast_tree::private::ast_continuation_node_api::ast_continuation_node::{
    get_id::AstContinuationNodeApiGetId, AstContinuationNode,
};

impl AstContinuationNodeApiGetId for AstContinuationNode {
    fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}
