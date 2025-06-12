use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiGetParentType: I32NodeApi {
    fn get_parent_type(&self) -> AstNodeType;
}

impl I32NodeApiGetParentType for I32Node {
    fn get_parent_type(&self) -> AstNodeType {
        self.parent_type.clone()
    }
}
