use crate::ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree};

use super::RootNodeApi;

pub trait RootNodeApiGetRootNodeType: RootNodeApi {
    /// Get Root Node Type method
    ///
    /// Returns the root node type.
    ///
    /// # Returns
    /// - `AstNodeType` - representing the actual type of the Root Node.
    fn get_root_node_type(&self) -> AstNodeType;
}

impl RootNodeApiGetRootNodeType for MathPotatoAstTree {
    fn get_root_node_type(&self) -> AstNodeType {
        self.root_node_type.clone()
    }
}
