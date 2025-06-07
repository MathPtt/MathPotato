use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::infix_operation_type_enum::InfixOperationTypeEnum;
use crate::parser::parser_error::ParseError;

use super::AstApi;

pub mod implementation;

pub trait CreateNewInfixNodeWithParentNode: AstApi {
    fn create_new_infix_node_with_parent_node(
        self,
        parent_node_id: Uuid,
        infix_operation_type: InfixOperationTypeEnum,
    ) -> Result<Uuid, ParseError>;
}
