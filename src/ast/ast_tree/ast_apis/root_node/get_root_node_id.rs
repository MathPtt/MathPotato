use uuid::Uuid;

use crate::ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree};

use super::RootNodeApi;

pub trait RootNodeApiGetRootNodeId: RootNodeApi {
    fn get_root_node_id(&self) -> Option<Uuid>;
}

impl RootNodeApiGetRootNodeId for MathPotatoAstTree {
    fn get_root_node_id(&self) -> Option<Uuid> {
        if self.root_node_id == Uuid::nil() {
            None
        } else {
            Some(self.root_node_id)
        }
    }
}
