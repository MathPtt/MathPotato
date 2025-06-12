use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::RootNode;
use super::RootNodeApi;

pub trait RootNodeApiGetNodeType: RootNodeApi {
    fn get_node_type(&self) -> AstNodeType;
}
impl RootNodeApiGetNodeType for RootNode {
    fn get_node_type(&self) -> AstNodeType {
        self.node_type.clone()
    }
}
