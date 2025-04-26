use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::ParserError;

use super::{
    ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
    i32_ast_node::I32AstNode,
};

#[derive(Clone, Debug)]
pub struct I32AstTree {
    tree: HashMap<Uuid, I32AstNode>,
}

impl I32AstTree {
    pub fn new() -> I32AstTree {
        I32AstTree {
            tree: HashMap::new(),
        }
    }
    pub fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32AstNode>> {
        let res: HashMap<Uuid, I32AstNode> = l
            .into_iter()
            .filter_map(|uuid| self.tree.get(&uuid).cloned().map(|res| (uuid, res.clone())))
            .collect();

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }
    pub fn put_all(&mut self, l: HashMap<Uuid, I32AstNode>) {
        l.into_iter().map(|i| self.tree.insert(i.0, i.1));
    }
    pub fn put(&mut self, uuid: Uuid, node: I32AstNode) -> Result<(Uuid, I32AstNode), ParserError> {
        match self.tree.insert(uuid, node.clone()) {
            None => Ok((uuid, node)),
            Some(_) => Err(ParserError::new(String::from(
                "There is an item in the HashMap with the same key.",
            ))),
        }
    }

    pub fn get_node_by_id(&self, id: Uuid) -> Option<I32AstNode> {
        self.tree.get(&id).cloned()
    }

    pub fn get_tree_count(&self) -> usize {
        self.tree.clone().len()
    }

    pub(crate) fn overwrite(
        &mut self,
        id: Uuid,
        node: I32AstNode,
    ) -> Result<(Uuid, I32AstNode), ParserError> {
        match self.tree.get(&id) {
            None => Err(ParserError::new(format!(
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
