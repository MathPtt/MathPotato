use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{InfixAstNode, InfixAstNodeApi};

pub trait InfixAstNodeIsLeftOccupied: InfixAstNodeApi {
    fn is_left_occupied(&self) -> bool;
}
impl InfixAstNodeIsLeftOccupied for InfixAstNode {
    fn is_left_occupied(&self) -> bool {
        self.left_type != AstNodeType::None && self.left_id != Uuid::nil()
    }
}
