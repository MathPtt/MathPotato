use uuid::Uuid;

use crate::{
    ast::ast_tree::private::infix_node::node::InfixNode, parser::parser_error::ParseError,
};

use super::InfixNodeStorageApi;

pub mod implementation;

pub trait InfixNodeStorageApiGetNodeById: InfixNodeStorageApi {
    fn get_node_by_id(&self, id: Uuid) -> Result<InfixNode, ParseError>;
}
