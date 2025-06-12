impl NodeCatalogApiGetType for MathPotatoAstTree {
    fn node_catalog_api_get_node_type_by_id(
        &self,
        node_id: Uuid,
    ) -> Result<AstNodeType, ParseError> {
        match self.node_catalog.get_type(node_id) {
            Err(e) => Err(e),
            Ok(r) => Ok(r),
        }
    }
}
