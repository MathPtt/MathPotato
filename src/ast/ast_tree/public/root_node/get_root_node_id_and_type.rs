use crate::ast::ast_tree::public::AstApi;
use crate::parser::parser_error::ParseError;

use super::dto::RootNodeDto;
pub mod implementation;

pub trait GetRootNodeIdAndType: AstApi {
    fn get_root_node_id_and_type(&self) -> Result<RootNodeDto, ParseError>;
}
