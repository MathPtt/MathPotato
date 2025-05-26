use uuid::Uuid;

use crate::ast::ast_tree::private::ast_continuation_node::ast_continuation_node::{
    get_id::AstContinuationNodeApiGetId, ContinuationNode,
};

impl AstContinuationNodeApiGetId for ContinuationNode {
    fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}
