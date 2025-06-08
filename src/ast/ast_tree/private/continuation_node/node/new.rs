use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{ContinuationNode, ContinuationNodeApi};

pub trait ContinuationNodeApiCtor: ContinuationNodeApi {
    fn new() -> Self;
}

impl ContinuationNodeApiCtor for ContinuationNode {
    fn new() -> Self {
        ContinuationNode {
            id: Uuid::nil(),
            node_type: AstNodeType::None,
        }
    }
}
