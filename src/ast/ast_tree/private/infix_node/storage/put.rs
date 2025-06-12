use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApi;

pub mod implementation;

pub trait InfixNodeStorageApiPut: InfixNodeStorageApi {
    fn put(&mut self, node: InfixNode) -> Result<Uuid, ParseError>;
}
