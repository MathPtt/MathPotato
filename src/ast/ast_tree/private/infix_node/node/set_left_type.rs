use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetLeftType: InfixNodeApi {
    fn set_left_type(&mut self, left_type: AstNodeType);
}
impl InfixNodeApiSetLeftType for InfixNode {
    fn set_left_type(&mut self, left_type: AstNodeType) {
        self.left_type = left_type.clone();
    }
}
