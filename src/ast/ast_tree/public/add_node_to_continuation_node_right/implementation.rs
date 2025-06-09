use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::storage::get_id::ContinuationNodeStorageApiGetId;
use crate::ast::ast_tree::private::continuation_node::storage::get_node::ContinuationNodeStorageApiGetNode;
use crate::ast::ast_tree::private::continuation_node::storage::get_type::ContinutationNodeStorageApiGetType;
use crate::ast::ast_tree::private::infix_node::storage::add_node_id_and_type_to_left::InfixNodeStorageApiAddNodeIdAndTypeToLeft;
use crate::ast::ast_tree::private::node_catalog::storage::get_node_type::NodeCatalogInternalApiGetType;
use crate::ast::ast_tree::MathPotatoAstTree;
use crate::parser::parser_error::ParseError;

use super::AddNodeToContinuationNodeRight;

impl AddNodeToContinuationNodeRight for MathPotatoAstTree {
    fn add_node_to_continuation_node_right(&mut self, id: Uuid) -> Result<Uuid, ParseError> {
        // get the node type
        let node_type = self.node_catalog.get_node_type(id).unwrap_or_else(|e| {
            panic!(
                "There is no node in the NodeCatalog with id: {}. Details: {}",
                id, e
            );
        });
        match node_type {
            AstNodeType::I32AstNode => match self.continuation_node.get_type() {
                AstNodeType::InfixOperationAstNode => {
                    self.infix_nodes
                                    .add_node_id_and_type_to_left(self.continuation_node.get_id(), id, node_type.clone())
                                    .unwrap_or_else(|e| {

                                        let cont_node_debug =
                                            self.continuation_node.get_node().unwrap_or_else(|| {
                                                panic!(r"Tried to collect continuation node information for error message, but failed. \
                                Continuation node details: {}", self.continuation_node)
                                            });
                                panic!(
                                            "Error happened while adding node id: {} and type: {} to {} node.",
                                            id, node_type, cont_node_debug
                                        );

                                    });
                    Ok(id)
                }
                _ => panic!("This path is not covered yet."),
            },
            _ => Err(ParseError::new(format!(
                "The node type of id: {} is {} which is not ok.",
                id, node_type
            ))),
        }
    }
}
