use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::MathPotatoAstTree;

pub trait ContinuationNodeApi {
    fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)>;
}

impl ContinuationNodeApi for MathPotatoAstTree {
    fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)> {
        if self.last_changed_node_id == Uuid::nil() {
            None
        } else {
            Some((
                self.last_changed_node_id,
                self.last_changed_node_type.clone(),
            ))
        }
    }
}
