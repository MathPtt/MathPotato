use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;

use super::{CreateOrUpdateRootNodeIdAndTypeResult, CreateOrUpdateRootNodeIdAndTypeResultApi};

pub trait CreateOrUpdateRootNodeIdAndTypeResultApiNewFromIdAndType:
    CreateOrUpdateRootNodeIdAndTypeResultApi
{
    fn new_from_id_and_type(id: Uuid, node_type: AstNodeType) -> Self;
}
impl CreateOrUpdateRootNodeIdAndTypeResultApiNewFromIdAndType
    for CreateOrUpdateRootNodeIdAndTypeResult
{
    fn new_from_id_and_type(
        id: Uuid,
        node_type: AstNodeType,
    ) -> CreateOrUpdateRootNodeIdAndTypeResult {
        CreateOrUpdateRootNodeIdAndTypeResult { id, node_type }
    }
}
