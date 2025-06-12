use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::i32_nodes::node::set_parent_id::I32NodeApiSetParentId;
use crate::ast::ast_tree::private::i32_nodes::node::set_parent_type::I32NodeApiSetParentType;
use crate::ast::ast_tree::private::i32_nodes::storage::get_node_by_id::I32NodeStorageApiGetNodeById;
use crate::ast::ast_tree::private::i32_nodes::storage::update::I32NodeStorageApiUpdate;
use crate::ast::ast_tree::private::infix_node::node::InfixNode;
use crate::ast::ast_tree::private::infix_node::node::infix_operation_type_enum::InfixOperationTypeEnum;
use crate::ast::ast_tree::private::infix_node::node::new_with_type_and_left_child_node::InfixAstNodeInternalNewWithTypeAndLeftChildNode;
use crate::ast::ast_tree::private::infix_node::storage::put::InfixNodeStorageApiPut;
use crate::ast::ast_tree::private::node_catalog::storage::get_node_type::NodeCatalogInternalApiGetType;

use super::CreateNewInfixNodeWithLeftChild;

impl CreateNewInfixNodeWithLeftChild for MathPotatoAstTree {
    fn create_new_infix_node_with_left_child(
        &mut self,
        operation_type: InfixOperationTypeEnum,
        left_node_id: uuid::Uuid,
    ) -> Result<Uuid> {
        let left_node_type = self
            .node_catalog
            .get_node_type(left_node_id)
            .unwrap_or_else(|e| {
                panic!(
                    "Error happened while retrieving node type of {}. Details: {}",
                    left_node_id, e
                )
            });
        if left_node_type.clone() != AstNodeType::I32AstNode {
            return Err(anyhow::anyhow!(
                "The left node type, {}, is incorrect at this point.",
                left_node_type
            ));
        }
        let new_infix_node_id = self
            .infix_nodes
            .put(InfixNode::new_with_type_and_left_child_node(
                operation_type,
                left_node_type.clone(),
                left_node_id,
            ))
            .unwrap_or_else(|e| {
                panic!(
                    "Error happened while creating new {} node. Details: {}",
                    AstNodeType::InfixOperationAstNode,
                    e
                )
            });
        match left_node_type {
            AstNodeType::I32AstNode => {
                let mut actual_i32_node = self
                    .i32_nodes
                    .get_node_by_id(left_node_id)
                    .with_context(|| {
                        format!(
                            "Didn't receive {} with id {}.",
                            AstNodeType::I32AstNode,
                            left_node_id
                        )
                    })?;
                actual_i32_node.set_parent_id(new_infix_node_id);
                actual_i32_node.set_parent_type(AstNodeType::InfixOperationAstNode);
                self.i32_nodes.update(left_node_id, actual_i32_node).unwrap_or_else(|e|{
                    panic!(
                        "Error happened while updating parent id to {} and parent type to {} for {} node with id {}. Details: {}",
                        new_infix_node_id,
                        AstNodeType::InfixOperationAstNode,
                        AstNodeType::I32AstNode,
                        left_node_id,
                        e
                    );
                });
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Error happened. We are at a point which should have been caught earlier."
                ));
            }
        };

        Ok(new_infix_node_id)
    }
}
