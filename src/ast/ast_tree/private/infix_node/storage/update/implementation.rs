use std::any::type_name;

use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::infix_node::{
            node::InfixNode,
            storage::{does_node_exist::InfixNodeStorageApiDoesNodeExist, InfixNodeStorage},
        },
    },
    parser::parser_error::ParseError,
};

use super::InfixNodeStorageApiUpdate;

impl InfixNodeStorageApiUpdate for InfixNodeStorage {
    fn update(&mut self, id: Uuid, node: InfixNode) -> Result<Uuid, ParseError> {
        match self.does_node_exist(id) {
            false => Err(ParseError::new(format!(
                "There is no {:#?} type node with id: {}.",
                type_name::<AstNodeType>(),
                id
            ))),
            true => {
                self.tree.insert(id, node.clone());
            }
        }
    }
}
