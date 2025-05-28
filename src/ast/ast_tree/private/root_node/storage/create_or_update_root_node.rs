use crate::{ast::ast_tree::private::root_node::node::RootNode, parser::parser_error::ParseError};

use super::RootNodeStorageApi;

pub mod implementation;

pub trait RootNodeStorageApiCreateOrUpdate: RootNodeStorageApi {
    fn create_or_update(&mut self, node: RootNode) -> Result<RootNode, ParseError>;
}
