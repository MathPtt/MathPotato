use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApi;

pub mod implementation;

pub trait InfixNodeStorageApiUpdate: InfixNodeStorageApi {
    fn update(&mut self, id: Uuid, node: InfixNode) -> Result<Uuid, ParseError>;
}
