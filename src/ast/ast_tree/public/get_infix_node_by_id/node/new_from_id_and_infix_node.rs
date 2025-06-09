use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::get_left_type::InfixNodeApiGetLeftType;
use crate::ast::ast_tree::private::infix_node::node::get_operation_type::InfixNodeApiGetOperationType;
use crate::ast::ast_tree::private::infix_node::node::get_parent_id::InfixNodeApiGetParentId;
use crate::ast::ast_tree::private::infix_node::node::get_parent_type::InfixNodeApiGetParentType;
use crate::ast::ast_tree::private::infix_node::node::get_right_id::InfixNodeApiGetRightId;
use crate::ast::ast_tree::private::infix_node::node::get_right_type::InfixNodeApiGetRightType;

use super::GetInfixNodeByIdResult;
use super::GetInfixNodeByIdResultApi;

pub trait GetInfixNodeByIdApiNewFromInfixNode: GetInfixNodeByIdResultApi {
    fn new_from_id_and_infix_node(id: Uuid, infix_node: InfixNode) -> Self;
}

impl GetInfixNodeByIdApiNewFromInfixNode for GetInfixNodeByIdResult {
    fn new_from_id_and_infix_node(id: Uuid, infix_node: InfixNode) -> Self {
        GetInfixNodeByIdResult {
            id,
            operation_type: infix_node.get_infix_node_operation_type().clone(),
            left_id: infix_node.get_left_id(),
            left_type: infix_node.get_left_type().clone(),
            right_id: infix_node.get_right_id(),
            right_type: infix_node.get_right_type().clone(),
            parent_id: infix_node.get_parent_id(),
            parent_type: infix_node.get_parent_type().clone(),
        }
    }
}
