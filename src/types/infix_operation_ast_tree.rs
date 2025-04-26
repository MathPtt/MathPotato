use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParserError;

use super::{
    ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
    infix_operation_ast_node::InfixOperationAstNode,
};

#[derive(Clone, Debug)]
pub struct InfixOperationAstTree {
    tree: HashMap<Uuid, InfixOperationAstNode>,
}

impl InfixOperationAstTree {
    pub fn put(
        &mut self,
        key: Uuid,
        value: InfixOperationAstNode,
    ) -> Result<(Uuid, InfixOperationAstNode), ParserError> {
        match self.tree.insert(key, value) {
            None => Ok((key, self.tree.get(&key).unwrap().clone())),
            Some(_) => panic!(
                "There is an existing InfixOperationAstNode with key: {}",
                key
            ),
        }
    }

    pub(crate) fn new() -> Self {
        Self {
            tree: HashMap::new(),
        }
    }

    pub(crate) fn get(&self, id: Uuid) -> Option<InfixOperationAstNode> {
        self.tree.get(&id).cloned()
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
