use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetRightId: InfixNodeApi {
    fn get_right_id(&self) -> Option<Uuid>;
}
impl InfixNodeApiGetRightId for InfixNode {
    fn get_right_id(&self) -> Option<Uuid> {
        if self.right_id == Uuid::nil() {
            None
        } else {
            Some(self.right_id)
        }
    }
}
