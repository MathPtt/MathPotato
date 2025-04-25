use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParserError;

use super::{infix_operation_ast_node::InfixOperationAstNode, math_potato_ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen}};

#[derive(Clone, Debug)]
pub struct InfixOperationAstTree {
    tree: HashMap<Uuid, InfixOperationAstNode
}
impl InfixOperationAstTree {
    pub fn put(&self, key: Uuid, value: InfixOperationAstNode) -> Result<InfixOperationAstNode, ParserError> {
        match self.tree.insert(key, value) {
            None => {
                Ok(self.tree.get(&key))
            },
            Some(_) => panic!("There is an existing InfixOperationAstNode with key: {}", key)
        }
    }
}
impl TypedAstTreeLen for InfixOperationAstTree {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl TypedAstTreeGetKeys for InfixOperationAstTree {
    fn keys(self) -> Vec<Uuid> {
        self.tree.keys().copied().collect()
    }
}
