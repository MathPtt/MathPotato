use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::NodesCatalogApi;

pub trait NodeCatalogApiGetType: NodesCatalogApi {
    fn node_catalog_api_get_node_type_by_id(
        &self,
        node_id: Uuid,
    ) -> Result<AstNodeType, ParseError>;
}
