use uuid::Uuid;

use crate::{ast::ast_tree::MathPotatoAstTree, parser::parser_error::ParseError};

use super::I32ApiCreateNodeWithValue;

impl I32ApiCreateNodeWithValue for MathPotatoAstTree {
    fn i32_api_create_node_with_value(&mut self, i32_value: i32) -> Result<Uuid, ParseError> {
        match self.i32_tree.create(i32_value) {
            Ok(id) => {
                self.node_catalog.add(id, AstNodeType::I32AstNode)
                    .unwrap_or_else(|e|
                        panic!("Failed to add the newly created {:#?} type node with id: {} to the node catalog. Details: {:#?}", 
                            AstNodeType::I32AstNode, id, e));
                Ok(id)
            }
            Err(e) => Err(ParseError::new(format!(
                "Error happened while creating {:#?} node type with value: {:#?}. Details: {:#?}.",
                AstNodeType::I32AstNode,
                i32_value,
                e
            ))),
        }
    }
}
