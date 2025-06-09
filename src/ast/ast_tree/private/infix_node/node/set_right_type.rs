use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiSetRightType: InfixNodeApi {
    fn set_right_type(&mut self, right_type: AstNodeType);
}
impl InfixNodeApiSetRightType for InfixNode {
    fn set_right_type(&mut self, right_type: AstNodeType) {
        self.right_type = right_type.clone();
    }
}
