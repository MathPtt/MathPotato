use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;
use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;

use super::I32NodeStorageApiGetNodes;

impl I32NodeStorageApiGetNodes for I32NodeStorage {
    fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32Node>> {
        let res: HashMap<Uuid, I32Node> = l
            .into_iter()
            .filter_map(|uuid| {
                self.nodes
                    .get(&uuid)
                    .cloned()
                    .map(|res| (uuid, res.clone()))
            })
            .collect();

        if !res.is_empty() { Some(res) } else { None }
    }
}
