use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{entity::I32AstEntity, I32AstTreeApi};
pub mod implementation;

pub trait I32AstTreeApiUpdate: I32AstTreeApi {
    fn update(&mut self, id: Uuid, node: I32AstEntity) -> Result<(Uuid, I32AstEntity), ParseError>;
}
