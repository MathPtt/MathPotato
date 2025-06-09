use crate::{
    ast::ast_tree::private::root_node::{
        node::{
            RootNode, get_id::RootNodeApiGetId, get_node_type::RootNodeApiGetNodeType,
            new_from_id_and_type::RootNodeApiNewFromIdAndType,
        },
        storage::RootNodeStorage,
    },
    parser::parser_error::ParseError,
};

use super::RootNodeStorageApiCreateOrUpdate;

impl RootNodeStorageApiCreateOrUpdate for RootNodeStorage {
    fn create_or_update(&mut self, node: RootNode) -> Result<RootNode, ParseError> {
        self.id = node.get_id();
        self.node_type = node.get_node_type();
        Ok(RootNode::new_from_id_and_type(
            self.id,
            self.node_type.clone(),
        ))
    }
}
