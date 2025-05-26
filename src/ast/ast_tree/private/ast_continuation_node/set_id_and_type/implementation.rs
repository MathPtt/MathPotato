impl AstContinuationInternalNodeSetIdAndType for AstContinuationNodeApi {
    fn set_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<AstContinuationNodeApi, ParseError> {
        if id == Uuid::nil() || node_type == AstNodeType::None {
            Err(ParseError::new(format!(
                "Invalid input at updating of {} type. Id: {}, type {}.",
                self, id, node_type
            )))
        } else {
            self.id = id.clone();
            self.node_type = node_type.clone();
            Ok(self.clone())
        }
    }
}
