use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    infix_operation_ast_node::InfixOperationAstNode,
    internal::ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
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
    ) -> Result<(Uuid, InfixOperationAstNode), ParseError> {
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

    pub(crate) fn update(
        &mut self,
        id: Uuid,
        node: InfixOperationAstNode,
    ) -> Result<(Uuid, InfixOperationAstNode), ParseError> {
        self.tree.get(&id).unwrap_or_else(|| {
            println!("infix tree: {:#?}", self.tree);
            panic!("There is no item to be updated with id: {}", id)
        });
        self.tree.insert(id, node.clone());
        Ok((id, node))
    }

    pub(crate) fn get_all(&self) -> Result<Vec<(Uuid, InfixOperationAstNode)>, ParseError> {
        let res: Vec<(Uuid, InfixOperationAstNode)> = self.tree.clone().into_iter().collect();
        Ok(res)
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
