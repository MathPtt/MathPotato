use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeIsRightOccupied: InfixAstNodeApi {
    fn is_right_occupied(&self) -> bool;
}
impl InfixAstNodeIsRightOccupied for InfixAstNode {
    fn is_right_occupied(&self) -> bool {
        self.right_type != AstNodeType::None && self.right_id != Uuid::nil()
    }
}
