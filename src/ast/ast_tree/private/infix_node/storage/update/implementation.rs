use std::any::type_name;

use uuid::Uuid;

use super::InfixNodeStorageApiUpdate;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::get_left_type::InfixNodeApiGetLeftType;
use crate::ast::ast_tree::private::infix_node::node::get_operation_type::InfixNodeApiGetOperationType;
use crate::ast::ast_tree::private::infix_node::node::get_right_id::InfixNodeApiGetRightId;
use crate::ast::ast_tree::private::infix_node::node::get_right_type::InfixNodeApiGetRightType;
use crate::ast::ast_tree::private::infix_node::node::set_infix_node_operation_type::InfixNodeApiSetInfixNodeOperationType;
use crate::ast::ast_tree::private::infix_node::node::set_left_id::InfixNodeApiSetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_type::InfixNodeApiSetLeftType;
use crate::ast::ast_tree::private::infix_node::node::set_right_id::InfixNodeApiSetRightId;
use crate::ast::ast_tree::private::infix_node::node::set_right_type::InfixNodeApiSetRightType;
use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::storage::does_node_exist::InfixNodeStorageApiDoesNodeExist;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

impl InfixNodeStorageApiUpdate for InfixNodeStorage {
    fn update(&mut self, id: Uuid, node: InfixNode) -> Result<Uuid, ParseError> {
        match self.does_node_exist(id) {
            false => Err(ParseError::new(format!(
                "There is no {} type node with id: {}.",
                type_name::<AstNodeType>(),
                id
            ))),
            true => {
                self.tree.entry(&id).and_modify(|orig| {
                    if orig.get_infix_node_operation_type() != node.get_infix_node_operation_type()
                    {
                        orig.set_infix_node_operation_type(node.get_infix_node_operation_type())
                    }
                    if orig.get_left_id() != node.get_left_id() {
                        orig.set_left_id(node.get_left_id());
                    }
                    if orig.get_left_type() != node.get_left_type() {
                        orig.set_left_type(node.get_left_type());
                    }
                    if orig.get_right_id() != node.get_right_id() {
                        orig.set_right_id(node.get_right_id());
                    }
                    if orig.get_right_type() != node.get_right_type() {
                        orig.set_right_type(node.get_right_type());
                    }
                });
            }
        }
    }
}
