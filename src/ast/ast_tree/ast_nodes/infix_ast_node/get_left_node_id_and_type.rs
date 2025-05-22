use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeGetLeftNodeIdAndType: InfixAstNodeApi {
    fn get_left_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)>;
}
impl InfixAstNodeGetLeftNodeIdAndType for InfixAstNode {
    fn get_left_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }
}
