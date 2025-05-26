use public::i32_nodes_api::I32NodesApi;
use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    global::enums::ast_node_types_enum::AstNodeType,
    private::apis::{
        ast_continuation_node_api::{new::AstContinuationInternalNodeNew, AstContinuationNodeApi},
        i32_nodes_api::{new::I32AstTreeApiNew, I32NodesApi},
        infix_nodes_api::{new::InfixAstTreeApiNew, InfixNodesApi},
        node_catalog_api::{new::NodeCatalogApiNew, NodeCatalogApi},
    },
};

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
pub mod global;
pub mod private;
pub mod public;

#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    /// A reference to the Root AST node.
    root_node_id: Uuid,
    /// The type of the root AST node.
    root_node_type: AstNodeType,
    /// Represents the point where the AST processing is right now.
    /// As the parser moves ahead it lefts the pointer here to pick it up in the next round.
    continuation_node: AstContinuationNodeSt
    /// The node tree to represent the i32 data type.
    i32_tree: I32NodesApi,
    /// The node tree to represent the infix operation nodes.
    infix_operation_tree: InfixNodesApi,
    /// The catalog of the nodes in the AST
    node_catalog: NodeCatalogApi,
}

impl MathPotatoAstTree {
    pub fn new() -> Self {
        MathPotatoAstTree {
            root_node_id: Uuid::nil(),
            root_node_type: AstNodeType::None,
            i32_tree: I32NodesApi::new(),
            infix_operation_tree: InfixNodesApi::new(),
            node_catalog: NodeCatalogApi::new(),
            continuation_node: AstContinuationNodeApi::new(),
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
