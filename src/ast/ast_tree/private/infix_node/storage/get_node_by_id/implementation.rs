use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::get_left_type::InfixNodeApiGetLeftType;
use crate::ast::ast_tree::private::infix_node::node::get_operation_type::InfixNodeApiGetOperationType;
use crate::ast::ast_tree::private::infix_node::node::get_parent_id::InfixNodeApiGetParentId;
use crate::ast::ast_tree::private::infix_node::node::get_parent_type::InfixNodeApiGetParentType;
use crate::ast::ast_tree::private::infix_node::node::get_right_id::InfixNodeApiGetRightId;
use crate::ast::ast_tree::private::infix_node::node::get_right_type::InfixNodeApiGetRightType;
use crate::ast::ast_tree::private::infix_node::node::new_with_values::InfixNodeApiNewWithValues;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApiGetNodeById;

impl InfixNodeStorageApiGetNodeById for InfixNodeStorage {
    fn get_node_by_id(&self, id: Uuid) -> Result<InfixNode, ParseError> {
        match self.nodes.get(&id) {
            Some(r) => {
                let rr = r.clone();
                Ok(InfixNode::new_with_values(
                    rr.get_infix_node_operation_type().clone(),
                    rr.get_left_id(),
                    rr.get_left_type().clone(),
                    rr.get_right_id(),
                    rr.get_right_type().clone(),
                    rr.get_parent_id(),
                    rr.get_parent_type().clone(),
                ))
            }
            None => todo!(),
        }
    }
}
