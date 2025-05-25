use std::collections::HashMap;

use uuid::Uuid;

use super::{entity::I32AstEntity, I32AstTreeApi};
pub mod implementation;

pub trait I32AstTreeApiPutAll: I32AstTreeApi {
    fn put_all(&mut self, l: HashMap<Uuid, I32AstEntity>);
}
