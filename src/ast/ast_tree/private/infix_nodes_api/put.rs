use uuid::Uuid;

use super::{entity::InfixAstNodeInternal, InfixAstTreeApi};

pub mod implementation;

pub trait InfixAstTreeApiPut: InfixAstTreeApi {
    fn put(
        &mut self,
        key: Uuid,
        value: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError>;
}
