use uuid::Uuid;

use crate::parser::parser_error::ParseError;

use self::node::GetInfixNodeByIdResult;

use super::AstApi;

pub mod implementation;
pub mod node;

pub trait GetInfixNodeById: AstApi {
    fn get_infix_node_by_id(&self, id: Uuid) -> Result<GetInfixNodeByIdResult, ParseError>;
}
