use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;
use crate::parser::parser_error::ParseError;

use super::I32NodeStorageApi;
pub mod implementation;

pub trait I32NodeStorageApiUpdate: I32NodeStorageApi {
    fn update(&mut self, id: Uuid, node: I32Node) -> Result<Uuid, ParseError>;
}
