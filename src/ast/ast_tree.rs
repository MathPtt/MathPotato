use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{
    ast_node_types_enum::AstNodeType,
    i32_node::I32AstNode,
    infix_operation_ast_node::InfixOperationAstNode,
    infix_operation_ast_tree::InfixOperationAstTree,
    internal::{
        ast_tree_traits::{TypedAstTreeGetKeys, TypedAstTreeLen},
        i32_ast_tree::I32AstTree,
    },
};

#[derive(Clone, Debug)]
pub struct MathPotatoAstTree {
    root_node_id: Uuid,
    root_node_type: AstNodeType,
    last_changed_node_id: Uuid,
    last_changed_node_type: AstNodeType,
    i32_tree: I32AstTree,
    infix_operation_tree: InfixOperationAstTree,
}

/// The I32 type related functionalities offered by the Abstract Syntax Tree.
pub trait I32 {
    /// Returns the I32AstNode with the designated id.
    ///
    /// # Parameters
    /// - `id` - `Uuid` - the id of the requested node
    /// # Returns
    /// - `Result<I32AstNode, ParseError`
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError>;

    /// Adds a new node to the AST.
    /// By adding a new node to the AST the reference to the continuation node will be updated
    /// accordingly.
    ///
    /// # Important
    ///
    /// When there is a I32AstNode in the AST with the same key there will be no overwrite, rather
    /// error message.
    ///
    /// # Parameters
    /// - `node` - an I32AstNode
    ///
    /// # Returns
    /// - `Result<(Uuid, I32AstNode), ParseError>`
    fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<I32AstNode, ParseError>;
    fn i32_tree_len(&self) -> usize;
    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError>;
}

impl I32 for MathPotatoAstTree {
    fn get_i32_node_by_id(&self, id: Uuid) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.get_node_by_id(id) {
            None => Err(ParseError::new(format!("No I32AstNode with id {}", id))),
            Some(internal) => Ok(I32AstNode::from_internal_and_id(internal, id)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }

    fn put_i32_ast_node(&mut self, node: I32AstNode) -> Result<I32AstNode, ParseError> {
        let uuid = Uuid::new_v4();
        match self.i32_tree.put(
            uuid,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Ok(r) => {
                self.last_changed_node_type = AstNodeType::I32AstNode;
                self.last_changed_node_id = r.0;
                Ok(I32AstNode::from_internal_and_id(r.1, uuid)
                    .unwrap_or_else(|e| panic!("{:#?}", e)))
            }
            Err(e) => Err(e),
        }
    }

    fn i32_tree_len(&self) -> usize {
        self.i32_tree.len()
    }

    fn update_i32_node(&mut self, id: Uuid, node: I32AstNode) -> Result<I32AstNode, ParseError> {
        match self.i32_tree.update(
            id,
            node.to_internal().unwrap_or_else(|e| panic!("{:#?}", e)),
        ) {
            Err(err) => Err(err),
            Ok(res) => Ok(I32AstNode::from_internal_and_id(res.1, res.0)
                .unwrap_or_else(|e| panic!("{:#?}", e))),
        }
    }
}

/// AST Root node related methods
pub trait RootNode {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;

    /// Returns the root node by id.
    fn get_root_node_infix(&self, id: Uuid) -> Result<(Uuid, InfixOperationAstNode), ParseError>;

    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;

    fn get_root_node_id(&self) -> Option<Uuid>;

    fn get_root_node_type(&self) -> AstNodeType;
}

impl RootNode for MathPotatoAstTree {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        if self.root_node_id != Uuid::nil() && self.root_node_type != AstNodeType::None {
            Err(ParseError::new(format!(
                "Root node type is not {:#?} and id is not {}.",
                AstNodeType::None,
                Uuid::nil()
            )))
        } else {
            self.root_node_id = id;
            self.root_node_type = node_type.clone();
            Ok((id, node_type))
        }
    }

    /// Returns the root node by id.
    fn get_root_node_infix(&self, id: Uuid) -> Result<(Uuid, InfixOperationAstNode), ParseError> {
        match self.infix_operation_tree.get(id) {
            Some(r) => Ok((id, r)),
            None => Err(ParseError::new(format!(
                "There is no InfixOperationAstNode type root node with id {}",
                id
            ))),
        }
    }

    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        self.root_node_id = id;
        self.root_node_type = node_type.clone();
        Ok((id, node_type))
    }

    fn get_root_node_id(&self) -> Option<Uuid> {
        if self.root_node_id == Uuid::nil() {
            None
        } else {
            Some(self.root_node_id)
        }
    }

    fn get_root_node_type(&self) -> AstNodeType {
        self.root_node_type.clone()
    }
}

pub trait ContinuationNode {
    fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)>;
}

impl ContinuationNode for MathPotatoAstTree {
    fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.last_changed_node_id == Uuid::nil() {
            None
        } else {
            Some((
                self.last_changed_node_id,
                self.last_changed_node_type.clone(),
            ))
        }
    }
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
        inode: InfixOperationAstNode,
    ) -> Result<(Uuid, InfixOperationAstNode), ParseError> {
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

    pub(crate) fn get_infix_node_by_id(&self, id: Uuid) -> Option<(Uuid, InfixOperationAstNode)> {
        self.infix_operation_tree.get(id).map(|r| (id, r))
    }

    pub(crate) fn update_infix_node_by_id(
        &mut self,
        id: Uuid,
        node: InfixOperationAstNode,
    ) -> Result<(Uuid, InfixOperationAstNode), ParseError> {
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

    pub(crate) fn get_infix_nodes(&self) -> Result<Vec<(Uuid, InfixOperationAstNode)>, ParseError> {
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
