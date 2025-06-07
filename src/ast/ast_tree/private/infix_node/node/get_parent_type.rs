use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{InfixNode, InfixNodeApi};

pub trait InfixNodeApiGetParentType: InfixNodeApi {
    fn get_parent_type(&self) -> &AstNodeType;
}
impl InfixNodeApiGetParentType for InfixNode {
    fn get_parent_type(&self) -> &AstNodeType {
        &self.parent_type
    }
}
