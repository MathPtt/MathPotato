use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::parser::parser_error::ParseError;

use super::NodeCatalogStorageApi;

pub mod implementation;

pub trait NodeCatalogInternalApiGetType: NodeCatalogStorageApi {
    fn get_node_type(&self, node_id: Uuid) -> Result<AstNodeType, ParseError>;
}
