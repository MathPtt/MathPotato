use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::i32_nodes::{
            node::{new_with_value::I32AstNodeInternalApiNewWithValue, I32Node},
            storage::I32NodeStorage,
        },
    },
    parser::parser_error::ParseError,
};

use super::I32NodeStorageApiCreate;

impl I32NodeStorageApiCreate for I32NodeStorage {
    fn create(&mut self, i32_value: i32) -> Result<Uuid, ParseError> {
        let new_node = I32Node::new_with_value(i32_value).unwrap_or_else(|e| {
            panic!(
                "Error happened while creating {:#?} type node with value: {}. Error details: {:#?}.",
                AstNodeType::I32AstNode,
                i32_value,
                e
            )
        });
        let new_node_id = Uuid::new_v4();
        match self.nodes.insert(new_node_id, new_node) {
            None => Ok(new_node_id),
            Some(r) => Err(ParseError::new(format! {
                "There is an item in the HashMap with the same key. The existing item is {:#?}", r,
            })),
        }
    }
}
