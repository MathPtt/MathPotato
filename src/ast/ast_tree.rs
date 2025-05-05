use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    ast_node_types_enum::AstNodeType,
    i32_ast_tree::I32AstTree,
    infix_operation_ast_tree::InfixOperationAstTree,
    internal::{
        ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
        infix_operation_ast_node::InfixAstNodeInternal,
    },
};
pub mod cont_node_api_get_id_and_type;
pub mod i32_api_get_node_by_id;
pub mod i32_api_node_count;
pub mod i32_api_put_node;
pub mod i32_api_update_node;
pub mod root_node_api;
/// Represents the Abstract Syntax Tree of the Programming Language.
///
/// # Decisions
///
/// ## Tree vs HashMap
///
/// The tree is not a tree. It is a wrapper over many HashMaps. The reason is simple: managing a
/// tree in Rust a particular pain. Managing a HashMap in Rust is less pain than that. The
/// difference between representing a tree in HashMap and a tree is not that significant to eat the
/// pain of dealing with the borrow checker.
///
/// The nodes are connected via their UUID values.
///
/// ## Generics or concrete types
///
/// I chose concrete types. Dealing with generics in Rust is not as easy as it is in, for example,
/// C#. What is an interface hierarchy in C#, it is a major pain here. So, I have concrete types
/// and there are methods, with the same functionality, but for different types. This way seemed
/// way easier than dealing with generics. Maybe later I'll do the generics.
///
#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    /// A reference to the Root AST node.
    root_node_id: Uuid,
    /// The type of the root AST node.
    root_node_type: AstNodeType,
    /// A reference to the last changed AST node.
    last_changed_node_id: Uuid,
    /// The type of the last changed node.
    last_changed_node_type: AstNodeType,
    /// The node tree to represent the i32 data type.
    i32_tree: I32AstTree,
    /// The node tree to represent the infix operation nodes.
    infix_operation_tree: InfixOperationAstTree,
}

impl MathPotatoAstTree {
    pub fn new() -> Self {
        MathPotatoAstTree {
            root_node_id: Uuid::nil(),
            root_node_type: AstNodeType::None,
            last_changed_node_id: Uuid::nil(),
            last_changed_node_type: AstNodeType::None,
            i32_tree: I32AstTree::new(),
            infix_operation_tree: InfixOperationAstTree::new(),
        }
    }
    pub fn merge(&mut self, tree: MathPotatoAstTree) -> Result<(), ParseError> {
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

    pub fn put_infix_node(
        &mut self,
        inode: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        let key = Uuid::new_v4();
        match self.infix_operation_tree.put(key, inode) {
            Err(err) => Err(err),
            Ok(r) => {
                self.last_changed_node_type = AstNodeType::InfixOperationAstNode;
                self.last_changed_node_id = key;
                Ok(r)
            }
        }
    }
    pub fn infix_tree_len(&self) -> usize {
        self.infix_operation_tree.len()
    }

    pub(crate) fn get_infix_node_by_id(&self, id: Uuid) -> Option<(Uuid, InfixAstNodeInternal)> {
        self.infix_operation_tree.get(id).map(|r| (id, r))
    }

    pub(crate) fn update_infix_node_by_id(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        Ok(self
            .infix_operation_tree
            .update(id, node)
            .unwrap_or_else(|e| {
                panic!(
                    "Error while updating infix operations of AST tree. {:#?}",
                    e
                )
            }))
    }

    pub(crate) fn get_infix_nodes(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError> {
        match self.infix_operation_tree.get_all() {
            Ok(res) => Ok(res),
            Err(err) => Err(err),
        }
    }
}

impl Default for MathPotatoAstTree {
    fn default() -> Self {
        Self::new()
    }
}
