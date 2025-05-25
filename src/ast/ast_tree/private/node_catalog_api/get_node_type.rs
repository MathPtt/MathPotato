use uuid::Uuid;

use crate::{
    ast::global::enums::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError,
};

use super::{NodeCatalogApi, NodeCatalogInternalApi};

pub trait NodeCatalogInternalApiGetType: NodeCatalogInternalApi {
    fn get_type(&self, node_id: Uuid) -> Result<AstNodeType, ParseError>;
}

impl NodeCatalogInternalApiGetType for NodeCatalogApi {
    fn get_type(&self, node_id: Uuid) -> Result<AstNodeType, ParseError> {
        match self.catalog.get(&node_id) {
            None => Err(ParseError::new(format!(
                "There is no node in the catalog with id: {}",
                node_id
            ))),
            Some(t) => Ok(t.clone()),
        }
    }
}
