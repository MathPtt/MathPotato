use std::collections::HashMap;

use uuid::Uuid;

use crate::parser::parser_error::MathPotatoParserError;

use super::{
    ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeGetSize},
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
        if res.iter().count() > 0 {
            Some(res)
        } else {
            None
        }
    }
    pub fn put_all(&mut self, l: HashMap<Uuid, I32AstNode>) {
        l.into_iter().map(|i| self.tree.insert(i.0, i.1));
    }
    pub fn put(&mut self, uuid: Uuid, node: I32AstNode) -> Result<(), MathPotatoParserError> {
        match self.tree.insert(uuid, node) {
            None => Ok(()),
            Some(_) => Err(MathPotatoParserError::new(String::from(
                "There is an item in the HashMap with the same key.",
            ))),
        }
    }

    pub(crate) fn get_node_by_id(&self, id: Uuid) -> Option<I32AstNode> {
        self.tree.get(&id).cloned()
    }
}

impl TypedAstTreeGetSize for I32AstTree {
    fn size(self) -> usize {
        self.tree.iter().count()
    }
}

impl TypedAstTreeGetKeys for I32AstTree {
    fn keys(self) -> Vec<Uuid> {
        self.tree.keys().copied().collect()
    }
}
