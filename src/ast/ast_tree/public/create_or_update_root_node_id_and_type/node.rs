use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
pub mod new_from_id_and_type;

pub struct CreateOrUpdateRootNodeIdAndTypeResult {
    id: Uuid,
    node_type: AstNodeType,
}

pub trait CreateOrUpdateRootNodeIdAndTypeResultApi {}
impl CreateOrUpdateRootNodeIdAndTypeResultApi for CreateOrUpdateRootNodeIdAndTypeResult {}
