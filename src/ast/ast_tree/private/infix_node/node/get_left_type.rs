use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::InfixNode;
use super::InfixNodeApi;

pub trait InfixNodeApiGetLeftType: InfixNodeApi {
    fn get_left_type(&self) -> &AstNodeType;
}
impl InfixNodeApiGetLeftType for InfixNode {
    fn get_left_type(&self) -> &AstNodeType {
        &self.left_type
    }
}
