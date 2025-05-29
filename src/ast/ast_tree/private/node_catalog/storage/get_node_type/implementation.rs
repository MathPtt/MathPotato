use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::node_catalog::storage::NodeCatalogStorage,
    },
    parser::parser_error::ParseError,
};

use super::NodeCatalogInternalApiGetType;

impl NodeCatalogInternalApiGetType for NodeCatalogStorage {
    fn get_node_type(&self, node_id: Uuid) -> Result<AstNodeType, ParseError> {
        match self.catalog.get(&node_id) {
            None => Err(ParseError::new(format!(
                "There is no node in the catalog with id: {}",
                node_id
            ))),
            Some(t) => Ok(t.clone()),
        }
    }
}
