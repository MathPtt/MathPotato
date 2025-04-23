use uuid::Uuid;

use crate::parser::parser_error::MathPotatoParserError;

use super::{
    ast_tree_traits::TypedAstTreeGetKeys, i32_ast_node::I32AstNode, i32_ast_tree::I32AstTree,
    math_potato_ast_node_types_enum::MathPotatoAstNodeType,
};

#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    last_changed_node_id: Uuid,
    last_changed_node_type: MathPotatoAstNodeType,
    i32_tree: I32AstTree,
}

impl MathPotatoAstTree {
    pub fn new() -> MathPotatoAstTree {
        MathPotatoAstTree {
            last_changed_node_id: Uuid::nil(),
            i32_tree: I32AstTree::new(),
            last_changed_node_type: MathPotatoAstNodeType::None,
        }
    }
    pub fn merge(&mut self, tree: MathPotatoAstTree) -> Result<(), MathPotatoParserError> {
        self.last_changed_node_type = tree.last_changed_node_type;
        self.last_changed_node_id = tree.last_changed_node_id;

        // i32 node merge
        let diff: Vec<Uuid> = tree
            .i32_tree
            .clone()
            .keys()
            .into_iter()
            .filter(|k| !self.i32_tree.clone().keys().contains(k))
            .collect();
        let diff_result = tree.i32_tree.get_nodes(diff).unwrap();
        self.i32_tree.put_all(diff_result);

        Ok(())
    }
    pub fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, MathPotatoAstNodeType)> {
        if self.last_changed_node_id == Uuid::nil() {
            None
        } else {
            Some((
                self.last_changed_node_id,
                self.last_changed_node_type.clone(),
            ))
        }
    }
    pub fn get_i32_node(&self, id: Uuid) -> Option<I32AstNode> {
        self.i32_tree.get_node_by_id(id)
    }

    pub(crate) fn put_i32_ast_node(
        &mut self,
        node: I32AstNode,
    ) -> Result<(), MathPotatoParserError> {
        match self.i32_tree.put(Uuid::new_v4(), node) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
