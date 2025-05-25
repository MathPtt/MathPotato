use uuid::Uuid;

use crate::ast::global::enums::ast_node_types_enum::AstNodeType;

pub struct UpdateNodeIdAndTypeResult {
    id: Uuid,
    node_type: AstNodeType,
}
