use uuid::Uuid;

use crate::ast::ast_node_types_enum::AstNodeType;

use super::{cont_node_api::ContNodeApi, MathPotatoAstTree};

pub trait ContNodeApiGetIdAndType: ContNodeApi {
    /// Returns the Id and Type of the Continuation Node.
    fn get_continuation_node_id_and_type(&self) -> Option<(Uuid, AstNodeType)>;
}

impl ContNodeApiGetIdAndType for MathPotatoAstTree {
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
