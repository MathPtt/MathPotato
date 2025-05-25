impl RootNodeApiUpdateRootNodeIdAndType for MathPotatoAstTree {
    fn root_node_api_update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        if self.root_node_id != Uuid::nil() && self.root_node_type != AstNodeType::None {
            Err(ParseError::new(format!(
                "Root node type is not {:#?} and id is not {}.",
                AstNodeType::None,
                Uuid::nil()
            )))
        } else {
            self.root_node_id = id;
            self.root_node_type = node_type.clone();
            Ok((id, node_type))
        }
    }
}
