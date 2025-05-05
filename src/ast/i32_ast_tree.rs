use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::internal::{
    ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
    i32_ast_node_internal::I32AstNodeInternal,
};

#[derive(Clone, Debug)]
pub struct I32AstTree {
    pub tree: HashMap<Uuid, I32AstNodeInternal>,
}

impl I32AstTree {
    pub fn new() -> I32AstTree {
        I32AstTree {
            tree: HashMap::new(),
        }
    }
    pub fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32AstNodeInternal>> {
        let res: HashMap<Uuid, I32AstNodeInternal> = l
            .into_iter()
            .filter_map(|uuid| self.tree.get(&uuid).cloned().map(|res| (uuid, res.clone())))
            .collect();

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }

    pub fn put_all(&mut self, l: HashMap<Uuid, I32AstNodeInternal>) {
        let _ = l.into_iter().map(|i| self.tree.insert(i.0, i.1));
    }

    pub fn put(
        &mut self,
        uuid: Uuid,
        node: I32AstNodeInternal,
    ) -> Result<(Uuid, I32AstNodeInternal), ParseError> {
        match self.tree.insert(uuid, node.clone()) {
            None => Ok((uuid, node)),
            Some(_) => Err(ParseError::new(String::from(
                "There is an item in the HashMap with the same key.",
            ))),
        }
    }

    pub fn get_node_by_id(&self, id: Uuid) -> Option<I32AstNodeInternal> {
        self.tree.get(&id).cloned()
    }

    pub fn get_tree_count(&self) -> usize {
        self.tree.clone().len()
    }

    pub(crate) fn update(
        &mut self,
        id: Uuid,
        node: I32AstNodeInternal,
    ) -> Result<(Uuid, I32AstNodeInternal), ParseError> {
        match self.tree.get(&id) {
            None => Err(ParseError::new(format!(
                "There is no I32AstNode with id {}.",
                id
            ))),
            Some(_) => match self.tree.insert(id, node) {
                None => panic!(
                    "This case should not happen as the previous operation would have caught it."
                ),
                Some(r) => Ok((id, r)),
            },
        }
    }
}

impl TypedAstTreeLen for I32AstTree {
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl TypedAstTreeGetKeys for I32AstTree {
    fn keys(self) -> Vec<Uuid> {
        self.tree.keys().copied().collect()
    }
}
