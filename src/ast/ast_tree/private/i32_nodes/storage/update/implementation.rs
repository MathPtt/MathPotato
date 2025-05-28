use uuid::Uuid;

use crate::{
    ast::ast_tree::private::i32_nodes_api::{entity::I32AstEntity, storage::I32NodeStorage},
    parser::parser_error::ParseError,
};

use super::I32AstTreeApiUpdate;

impl I32AstTreeApiUpdate for I32NodeStorage {
    fn update(&mut self, id: Uuid, node: I32AstEntity) -> Result<(Uuid, I32AstEntity), ParseError> {
        match self.tree.get(&id) {
            None => Err(ParseError::new(format!(
                "There is no I32AstNode with id {:#?}.",
                id
            ))),
            Some(_) => match self.tree.insert(id, node) {
                None => panic!(
                    "This case should not happen as the previous operation would have caught it."
                ),
                Some(r) => Ok((id, r.clone())),
            },
        }
    }
}
