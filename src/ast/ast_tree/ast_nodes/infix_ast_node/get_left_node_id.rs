use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeGetLeftNodeId: InfixAstNodeApi {
    fn get_left_node_id(&self) -> Option<Uuid>;
}
impl InfixAstNodeGetLeftNodeId for InfixAstNode {
    fn get_left_node_id(&self) -> Option<Uuid> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some(self.right_id)
        }
    }
}
