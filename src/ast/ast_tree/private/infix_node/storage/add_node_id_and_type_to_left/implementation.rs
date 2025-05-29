use std::any::type_name;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::infix_node::{
            node::{
                set_left_id::InfixNodeApiSetLeftId, set_left_type::InfixNodeApiSetLeftType,
                InfixNode,
            },
            storage::InfixNodeStorage,
        },
    },
    parser::parser_error::ParseError,
};

use super::InfixNodeStorageApiAddNodeIdAndTypeToLeft;

impl InfixNodeStorageApiAddNodeIdAndTypeToLeft for InfixNodeStorage {
    fn add_node_id_and_type_to_left(
        &mut self,
        target_node_id: uuid::Uuid,
        left_node_id: uuid::Uuid,
        left_node_type: crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    ) -> Result<uuid::Uuid, crate::parser::parser_error::ParseError> {
        match self.tree.get(&target_node_id) {
            None => Err(ParseError::new(format!(
                "There is no {} node with id: {}.",
                AstNodeType::InfixOperationAstNode,
                target_node_id
            ))),
            Some(n) => {
                let node = n.clone();
                node.set_left_id(left_node_id).unwrap_or_else(|e| {
                    return Err(ParseError::new(format!(
                        "Failed while setting up {}.set_left_id(). Details: {}.",
                        type_name::<InfixNode>(),
                        e
                    )));
                });
                node.set_left_type(&left_node_type).unwrap_or_else(|e| {
                    return Err(ParseError::new(format!(
                        "Failed while setting up {}.set_left_type(). Details: {}.",
                        type_name::<InfixNode>(),
                        e
                    )));
                });
                self.tree.update(target_node_id, node).unwrap_or_else(|e| {
                    Err(ParseError::new(format!(
                        "Error happened while updating {} node with input: {}. Details: {}",
                        n, node, e
                    )))
                });
                Ok(target_node_id)
            }
        }
    }
}
