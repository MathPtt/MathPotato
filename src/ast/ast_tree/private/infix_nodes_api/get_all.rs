use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use super::{entity::InfixAstNodeInternal, InfixAstTreeApi};

pub mod implementation;

pub trait InfixAstTreeApiGetAll: InfixAstTreeApi {
    fn get_all(&self) -> Result<Vec<(Uuid, InfixAstNodeInternal)>, ParseError>;
}
