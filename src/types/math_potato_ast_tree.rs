use uuid::Uuid;

use crate::parser::parser_error::MathPotatoParserError;

use super::{ast_node_types_enum::MathPotatoAstNodeType, ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeGetSize}, i32_ast_tree::{self, I32AstTree}};

#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    last_changed_node: Uuid,
    last_changed_node_type: MathPotatoAstNodeType
    i32_tree: I32AstTree,
}

impl MathPotatoAstTree {
    pub fn new() -> MathPotatoAstTree {
        MathPotatoAstTree {
            last_changed_node: Uuid::nil(),
            i32_tree: I32AstTree::new(),
            last_changed_node_type: MathPotatoAstNodeType::None
        }
    }
    pub fn merge(mut self, tree: MathPotatoAstTree) -> Result<MathPotatoAstTree, MathPotatoParserError> {
        self.last_changed_node_type = tree.last_changed_node_type;
        self.last_changed_node = tree.last_changed_node;
        if self.i32_tree.size() > tree.i32_tree.size() {
            let ours = self.i32_tree.keys();
            let theirs = tree.i32_tree.keys();
            let diff = theirs.into_iter()
                .filter(|uuid| !ours.contains(uuid))
                .collect();
            let diff_result = tree.i32_tree.get_nodes(diff).unwrap();
            self.i32_tree.put_all(diff_result);
            Ok(())
        }
    }
}
