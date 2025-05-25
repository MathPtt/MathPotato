use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::RootNodeApi;
pub mod implementation;

pub trait RootNodeApiUpdateRootNodeIdAndType: RootNodeApi {
    fn root_node_api_update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;
}
