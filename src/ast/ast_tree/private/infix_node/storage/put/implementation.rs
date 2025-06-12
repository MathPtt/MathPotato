use std::any::type_name;

use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApiPut;

impl InfixNodeStorageApiPut for InfixNodeStorage {
    fn put(&mut self, node: InfixNode) -> Result<Uuid, ParseError> {
        let id = Uuid::new_v4();
        match self.nodes.insert(id, node) {
            None => Ok(id),
            Some(_) => Err(ParseError::new(format!(
                "There is an existing {} with key: {}",
                type_name::<InfixNode>(),
                id
            ))),
        }
    }
}
