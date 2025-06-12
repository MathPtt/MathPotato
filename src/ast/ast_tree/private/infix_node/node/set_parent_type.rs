use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetParentType: InfixNodeApi {
    fn set_parent_type(&mut self, node_type: AstNodeType) -> AstNodeType;
}
impl InfixNodeApiSetParentType for InfixNode {
    fn set_parent_type(&mut self, node_type: AstNodeType) -> AstNodeType {
        self.parent_type = node_type.clone();
        node_type
    }
}
