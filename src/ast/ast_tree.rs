use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    ast_node_types_enum::AstNodeType,
    i32_ast_tree::I32AstTree,
    infix_ast_node::InfixAstNode,
    internal::{
        ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
        infix_ast_tree::InfixAstTree,
    },
};
pub mod ast_apis;
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
    infix_operation_tree: InfixAstTree,
}

impl MathPotatoAstTree {
    pub fn new() -> Self {
        MathPotatoAstTree {
            root_node_id: Uuid::nil(),
            root_node_type: AstNodeType::None,
            last_changed_node_id: Uuid::nil(),
            last_changed_node_type: AstNodeType::None,
            i32_tree: I32AstTree::new(),
            infix_operation_tree: InfixAstTree::new(),
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
}

impl Default for MathPotatoAstTree {
    fn default() -> Self {
        Self::new()
    }
}
