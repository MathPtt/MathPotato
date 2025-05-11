use crate::ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree};

use super::RootNodeApi;

pub trait RootNodeApiGetRootNodeType: RootNodeApi {
    fn get_root_node_type(&self) -> AstNodeType;
}

impl RootNodeApiGetRootNodeType for MathPotatoAstTree {
    fn get_root_node_type(&self) -> AstNodeType {
        self.root_node_type.clone()
    }
}
