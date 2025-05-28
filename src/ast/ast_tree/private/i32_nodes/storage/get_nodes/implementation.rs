use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes_api::{entity::I32AstEntity, storage::I32NodeStorage};

use super::I32AstTreeApiGetNodes;

impl I32AstTreeApiGetNodes for I32NodeStorage {
    fn get_nodes(&self, l: Vec<Uuid>) -> Option<HashMap<Uuid, I32AstEntity>> {
        let res: HashMap<Uuid, I32AstEntity> = l
            .into_iter()
            .filter_map(|uuid| self.tree.get(&uuid).cloned().map(|res| (uuid, res.clone())))
            .collect();

        if !res.is_empty() {
            Some(res)
        } else {
            None
        }
    }
}
