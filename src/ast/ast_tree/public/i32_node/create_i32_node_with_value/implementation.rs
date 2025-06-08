use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::i32_nodes::storage::create::I32NodeStorageApiCreate;
use crate::{ast::ast_tree::MathPotatoAstTree, parser::parser_error::ParseError};

use super::CreateI32NodeWithValue;

impl CreateI32NodeWithValue for MathPotatoAstTree {
    fn create_i32_node_with_value(&mut self, i32_value: i32) -> Result<Uuid, ParseError> {
        match self.i32_nodes.create(i32_value) {
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
