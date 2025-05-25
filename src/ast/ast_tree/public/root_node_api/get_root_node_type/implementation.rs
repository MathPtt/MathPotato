impl RootNodeApiGetRootNodeType for MathPotatoAstTree {
    fn get_root_node_type(&self) -> AstNodeType {
        self.root_node_type.clone()
    }
}
