use std::collections::HashMap;

use uuid::Uuid;

use super::potato_ast_node::PotatoAstNode;

pub struct PotatoAstTree {
    last_changed: Option<Uuid>,
    tree: HashMap<Uuid, PotatoAstNode>,
}

impl PotatoAstTree {
    pub fn new() -> PotatoAstTree {
        PotatoAstTree {
            tree: HashMap::new(),
            last_changed: None,
        }
    }
    pub fn merge(&mut self, t: HashMap<Uuid, PotatoAstNode>) -> Result<(), PotatoAstTreeError> {
        for item in t {
            self.tree.insert(item.0, item.1);
        }
        Ok(())
    }
    pub fn add_node(&mut self, node: PotatoAstNode) -> Result<(), PotatoAstTreeError> {
        match node {
            PotatoAstNode::IntegerStatementAstNode(a) => {
                self.tree
                    .insert(a.guid, PotatoAstNode::IntegerStatementAstNode(a));
                Ok(())
            }
            PotatoAstNode::IntegerValueExpressionAstNode(a) => {
                self.tree
                    .insert(a.guid, PotatoAstNode::IntegerValueExpressionAstNode(a));
                Ok(())
            }
            PotatoAstNode::None => Ok(()),
        }
    }
    pub fn add_node_as_last_modified(
        &mut self,
        guid: Uuid,
        node: PotatoAstNode,
    ) -> Result<(), PotatoAstTreeError> {
        self.tree.insert(guid, node);
        self.last_changed = Some(guid);
        Ok(())
    }
    pub fn clean_last_modified(&mut self) {
        self.last_changed = None;
    }
    /// Get a cloned instance from the last modified `PotatoAstNode`.
    /// The `cloned()` version is not the best solution. References should be used instead of
    /// these.
    pub fn get_last_modified_clone(&self) -> Result<PotatoAstNode, PotatoAstTreeError> {
        match self.last_changed {
            None => Err(PotatoAstTreeError::new(String::from(
                "Last changed is empty",
            ))),
            Some(s) => match self.tree.get(&s).cloned() {
                Some(r) => Ok(r),
                None => Err(PotatoAstTreeError::new(String::from(
                    "There is no PotatoAstNode with the given key",
                ))),
            },
        }
    }
    pub fn len(&self) -> usize {
        self.tree.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tree.len() == 0
    }
    /// Returns cloned underlying `HashMap<usize, PotatoAstNode>`.
    /// This method assumes that AST trees are used to parse statements and the resulting HashMap
    /// is small, so the clone cost is small.
    /// These HashMaps created during for example variable statement and expression parsing are
    /// small ones.
    pub fn get_nodes(self) -> HashMap<Uuid, PotatoAstNode> {
        self.tree.clone()
    }
    pub fn get(self, key: Uuid) -> Result<PotatoAstNode, PotatoAstTreeError> {
        match self.tree.get(&key) {
            None => Err(PotatoAstTreeError::new(format!(
                "There is no item in tree with key: {}",
                key
            ))),
            Some(r) => Ok(r.clone()),
        }
    }
}

impl Default for PotatoAstTree {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PotatoAstTreeError {
    message: String,
}

impl PotatoAstTreeError {
    pub fn new(message: String) -> PotatoAstTreeError {
        PotatoAstTreeError { message }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn merge_target_empty_source_not() {}
}
