use uuid::Uuid;

use crate::{
    ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType,
    parser::parser_error::ParseError,
};

use super::NodeCatalogStorageApi;
pub mod implementation;

pub trait NodeCatalogApiAddNode: NodeCatalogStorageApi {
    fn add(&mut self, id: Uuid, node_type: AstNodeType) -> Result<(Uuid, AstNodeType), ParseError>;
}
