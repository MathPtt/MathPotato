use uuid::Uuid;

use crate::parser::parser_error::ParserError;

use super::{
    i32_ast_node::I32AstNode,
    i32_ast_tree::I32AstTree,
    infix_operation_ast_node::InfixOperationAstNode,
    infix_operation_ast_tree::InfixOperationAstTree,
    math_potato_ast_node_types_enum::AstNodeType,
    math_potato_ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
    math_potato_infix_operation_enum::InfixOperationType,
};

#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    last_changed_node_id: Uuid,
    last_changed_node_type: AstNodeType,
    i32_tree: I32AstTree,
    infix_operation_tree: InfixOperationAstTree,
}

impl MathPotatoAstTree {
    pub fn new() -> MathPotatoAstTree {
        MathPotatoAstTree {
            last_changed_node_id: Uuid::nil(),
            i32_tree: I32AstTree::new(),
            last_changed_node_type: AstNodeType::None,
        }
    }
    pub fn merge(&mut self, tree: MathPotatoAstTree) -> Result<(), ParserError> {
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
    pub fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
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

    pub fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<(), ParserError> {
        let uuid = Uuid::new_v4();
        self.last_changed_node_id = uuid;
        match self.i32_tree.put(uuid, node) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    pub fn i32_tree_len(&self) -> usize {
        self.i32_tree.len()
    }

    pub fn put_infix_node(&self, inode: InfixOperationAstNode) -> _ {
        let key = Uuid::new_v4();
        self.infix_operation_tree.put(key, inode);
        self.last_changed_node_type = AstNodeType::InfixOperationAstNode;
        self.last_changed_node_id = key;
    }
}
