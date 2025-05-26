use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::node_catalog::NodeCatalogStorageApi,
    },
    parser::parser_error::ParseError,
};

use super::NodeCatalogInternalApiGetType;

impl NodeCatalogInternalApiGetType for NodeCatalogStorageApi {
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
