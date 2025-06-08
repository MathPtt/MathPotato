use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::{
    node::{
        get_left_id::InfixNodeApiGetLeftId, get_left_type::InfixNodeApiGetLeftType,
        get_operation_type::InfixNodeApiGetOperationType, get_right_id::InfixNodeApiGetRightId,
        get_right_type::InfixNodeApiGetRightType, new_with_values::InfixNodeApiNewWithValues,
        InfixNode,
    },
    storage::InfixNodeStorage,
};

use super::InfixNodeStorageApiGetNodeById;

impl InfixNodeStorageApiGetNodeById for InfixNodeStorage {
    fn get_node_by_id(
        &self,
        &id: Uuid,
    ) -> Result<
        crate::ast::ast_tree::private::infix_node::node::InfixNode,
        crate::parser::parser_error::ParseError,
    > {
        match self.nodes.get(&id) {
            Some(r) => Ok(InfixNode::new_with_values(
                *r.get_infix_node_operation_type(),
                *r.get_left_id(),
                *r.get_left_type(),
                *r.get_right_id(),
                *r.get_right_type(),
            )),
            None => todo!(),
        }
    }
}
