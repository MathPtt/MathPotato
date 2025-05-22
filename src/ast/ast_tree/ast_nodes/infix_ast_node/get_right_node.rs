use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeGetRightNode: InfixAstNodeApi {
    fn get_right_node(&self) -> Option<(Uuid, AstNodeType)>;
}
impl InfixAstNodeGetRightNode for InfixAstNode {
    fn get_right_node(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }
}
