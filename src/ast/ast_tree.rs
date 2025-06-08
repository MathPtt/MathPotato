use crate::parser::parser_error::ParseError;

use self::private::continuation_node::storage::new::ContinuationNodeStorageApiNew;
use self::private::continuation_node::storage::ContinuationNodeStorage;
use self::private::i32_nodes::storage::new::I32NodeStorageApiNew;
use self::private::i32_nodes::storage::I32NodeStorage;
use self::private::infix_node::storage::new::InfixNodeStorageApiNew;
use self::private::infix_node::storage::InfixNodeStorage;
use self::private::node_catalog::storage::new::NodeCatalogApiNew;
use self::private::node_catalog::storage::NodeCatalogStorage;
use self::private::root_node::storage::new::RootNodeStorageApiNew;
use self::private::root_node::storage::RootNodeStorage;

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
    root_node: RootNodeStorage,
    /// Represents the point where the AST processing is right now.
    /// As the parser moves ahead it lefts the pointer here to pick it up in the next round.
    continuation_node: ContinuationNodeStorage,
    /// The node tree to represent the i32 data type.
    i32_nodes: I32NodeStorage,
    /// The node tree to represent the infix operation nodes.
    infix_nodes: InfixNodeStorage,
    /// The catalog of the nodes in the AST
    node_catalog: NodeCatalogStorage,
}

impl MathPotatoAstTree {
    pub fn new() -> Self {
        MathPotatoAstTree {
            root_node: RootNodeStorage::new(),
            i32_nodes: I32NodeStorage::new(),
            infix_nodes: InfixNodeStorage::new(),
            node_catalog: NodeCatalogStorage::new(),
            continuation_node: ContinuationNodeStorage::new(),
        }
    }
    pub fn merge(&mut self, tree: MathPotatoAstTree) -> Result<(), ParseError> {
        // self.continuation_node
        //     .set_id(tree.continuation_node.get_id());
        // self.continuation_node
        //     .set_type(tree.continuation_node.get_type().clone());
        // // i32 node merge
        // let diff: Vec<Uuid> = tree
        //     .i32_nodes
        //     .get_nodes(l)
        //     .clone()
        //     .keys()
        //     .into_iter()
        //     .filter(|k| !self.i32_nodes_storage.clone().keys().contains(k))
        //     .collect();
        // let diff_result = tree.i32_nodes_storage.get_nodes(diff).unwrap();
        // self.i32_nodes_storage.put_all(diff_result);

        Ok(())
    }
}

impl Default for MathPotatoAstTree {
    fn default() -> Self {
        Self::new()
    }
}
