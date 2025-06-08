use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::I32Node;
use super::I32NodeApi;

pub trait I32NodeApiSetParentType: I32NodeApi {
    fn set_parent_type(&mut self, parent_type: AstNodeType);
}

impl I32NodeApiSetParentType for I32Node {
    fn set_parent_type(&mut self, parent_type: AstNodeType) {
        self.parent_type = parent_type
    }
}
