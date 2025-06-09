use std::any::type_name;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::node::infix_operation_type_enum::InfixOperationTypeEnum;
use crate::ast::ast_tree::private::infix_node::node::new_with_operation_type::InfixNodeApiNew;
use crate::ast::ast_tree::private::infix_node::node::set_infix_node_operation_type::InfixNodeApiSetInfixNodeOperationType;
use crate::ast::ast_tree::private::infix_node::node::set_parent_id::InfixNodeApisetParentId;
use crate::ast::ast_tree::private::infix_node::storage::get_node_by_id::InfixNodeStorageApiGetNodeById;
use crate::ast::ast_tree::private::infix_node::storage::put::InfixNodeStorageApiPut;
use crate::ast::ast_tree::private::infix_node::storage::update::InfixNodeStorageApiUpdate;

use super::CreateNewInfixNodeWithParentNode;

impl CreateNewInfixNodeWithParentNode for MathPotatoAstTree {
    fn create_new_infix_node_with_parent_node(
        &mut self,
        parent_node_id: uuid::Uuid,
        infix_operation_type: InfixOperationTypeEnum,
    ) -> Result<uuid::Uuid, crate::parser::parser_error::ParseError> {
        let id = self
            .infix_nodes
            .put(InfixNode::new(infix_operation_type.clone()))
            .unwrap_or_else(|e| {
                panic!(
                    "Error happened while creating a new {} node with type: {}. Details: {}",
                    type_name::<InfixNode>(),
                    infix_operation_type,
                    e
                );
            });
        let mut node = self.infix_nodes.get_node_by_id(id).unwrap_or_else(|e| {
            panic!(
                "Error happened while retrieving {} node with id: {}. Details: {}",
                AstNodeType::InfixOperationAstNode,
                id,
                e
            );
        });
        node.set_infix_node_operation_type(infix_operation_type);
        node.set_parent_id(parent_node_id);
        self.infix_nodes.update(id, node).unwrap_or_else(|e| {
            panic!(
                "Error happened while updating {} node with id: {}. Details: {}",
                AstNodeType::InfixOperationAstNode,
                id,
                e
            );
        });
        Ok(id)
    }
}
