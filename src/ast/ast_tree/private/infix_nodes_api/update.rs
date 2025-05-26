use std::any::type_name;

use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{entity::InfixAstNodeInternal, InfixAstTreeApi};
pub mod implementation;

pub trait InfixAstTreeApiUpdate: InfixAstTreeApi {
    fn update(
        &mut self,
        id: Uuid,
        node: InfixAstNodeInternal,
    ) -> Result<(Uuid, InfixAstNodeInternal), ParseError>;
}
