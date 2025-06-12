use std::collections::HashMap;

use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes_api::{entity::I32AstEntity, storage::I32NodeStorage};

use super::I32AstTreeApiPutAll;

impl I32AstTreeApiPutAll for I32NodeStorage {
    fn put_all(&mut self, l: HashMap<Uuid, I32AstEntity>) {
        let _ = l.into_iter().map(|i| self.tree.insert(i.0, i.1));
    }
}
