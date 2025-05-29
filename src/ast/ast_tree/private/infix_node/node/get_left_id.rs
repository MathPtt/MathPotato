use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetLeftId: InfixNodeApi {
    fn get_left_id(&self) -> Option<Uuid>;
}
impl InfixNodeApiGetLeftId for InfixNode {
    fn get_left_id(&self) -> Option<Uuid> {
        if self.left_id == Uuid::nil() {
            None
        } else {
            Some(self.left_id)
        }
    }
}
