use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{RootNode, RootNodeApi};

pub trait RootNodeApiNewFromIdAndType: RootNodeApi {
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> Self;
}
impl RootNodeApiNewFromIdAndType for RootNode {
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> Self {
        RootNode { id, node_type }
    }
}
