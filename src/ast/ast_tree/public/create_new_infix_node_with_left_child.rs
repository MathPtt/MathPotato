use anyhow::Result;
use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::infix_operation_type_enum::InfixOperationTypeEnum;

use super::AstApi;
pub mod implementation;

pub trait CreateNewInfixNodeWithLeftChild: AstApi {
    fn create_new_infix_node_with_left_child(
        &mut self,
        operation_type: InfixOperationTypeEnum,
        left_node_id: Uuid,
    ) -> Result<Uuid>;
}
