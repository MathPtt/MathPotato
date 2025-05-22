use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeGetRightNodeIdAndType: InfixAstNodeApi {
    fn get_right_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)>;
}
impl InfixAstNodeGetRightNodeIdAndType for InfixAstNode {
    fn get_right_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some((self.right_id, self.right_type.clone()))
        }
    }
}
