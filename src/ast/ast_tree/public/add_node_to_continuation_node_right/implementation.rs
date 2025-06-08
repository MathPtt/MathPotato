use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::storage::get_id::ContinuationNodeStorageApiGetId;
use crate::ast::ast_tree::private::continuation_node::storage::get_type::ContinutationNodeStorageApiGetType;
use crate::ast::ast_tree::private::node_catalog::storage::get_node_type::NodeCatalogInternalApiGetType;
use crate::ast::ast_tree::MathPotatoAstTree;

use super::AddNodeToContinuationNodeRight;

impl AddNodeToContinuationNodeRight for MathPotatoAstTree {
    fn add_node_to_continuation_node_right(
        &self,
        id: uuid::Uuid,
    ) -> Result<uuid::Uuid, crate::parser::parser_error::ParseError> {
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
                                    .add_node_id_and_type_to_left(self.continuation_node.get_id(), id, node_type)
                                    .unwrap_or_else(|e| {

                                        let cont_node_debug =
                                            self.continuation_node.get_node().unwrap_or_else(|| {
                                            return Err(ParseError::new(
                                                format!(r"Tried to collect continuation node information for error message, but failed. \
                                Continuation node details: {}", self.continuation_node)
                                            ));
                                        });
                                        return Err(ParseError::new(format!(
                                            "Error happened while adding node id: {} and type: {} to {} node.",
                                            id, node_type, cont_node_debug
                                        )));

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
