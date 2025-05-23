use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::internal::{
    ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
    infix_operation_ast_node::InfixAstNodeInternal,
};

#[derive(Clone, Debug)]
pub struct InfixOperationAstTree {
    tree: HashMap<Uuid, InfixAstNodeInternal>,
}

impl InfixOperationAstTree {
    pub fn put(
        &mut self,
        key: Uuid,
        value: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
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

    pub(crate) fn get(&self, id: Uuid) -> Option<InfixAstNodeInternal> {
        self.tree.get(&id).cloned()
    }

    pub(crate) fn update(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        self.tree.get(&id).unwrap_or_else(|| {
            println!("infix tree: {:#?}", self.tree);
            panic!("There is no item to be updated with id: {}", id)
        });
        self.tree.insert(id, node.clone());
        Ok((id, node))
    }

    pub(crate) fn get_all(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError> {
        let res: Vec<(Uuid, InfixAstNodeInternal)> = self.tree.clone().into_iter().collect();
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
