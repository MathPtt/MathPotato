use node::CreateOrUpdateRootNodeIdAndTypeResult;
use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::AstApi;

pub mod implementation;
pub mod node;

pub trait CreateOrUpdateRootNodeIdAndType: AstApi {
    fn create_or_update_root_node_id_and_type(
        &self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<CreateOrUpdateRootNodeIdAndTypeResult, ParseError>;
}
