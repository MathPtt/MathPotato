use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::storage::get_id::ContinuationNodeStorageApiGetId;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::get_left_type::InfixNodeApiGetLeftType;
use crate::ast::ast_tree::private::infix_node::node::get_right_id::InfixNodeApiGetRightId;
use crate::ast::ast_tree::private::infix_node::node::get_right_type::InfixNodeApiGetRightType;
use crate::ast::ast_tree::private::infix_node::storage::get_node_by_id::InfixNodeStorageApiGetNodeById;
use crate::ast::ast_tree::private::node_catalog::storage::get_node_type::NodeCatalogInternalApiGetType;
use crate::parser::parser_error::ParseError;

use super::CheckContinuationNodeConsistency;

impl CheckContinuationNodeConsistency for MathPotatoAstTree {
    fn check_continuation_node_consistency(&self) -> Result<bool, ParseError> {
        let continuation_node_id = self.continuation_node.get_id();
        match self.node_catalog.get_node_type(continuation_node_id) {
            Err(e) => Err(ParseError::new(format!(
                "Error happened while requesting node type of node with id: {}. Details: {}",
                continuation_node_id, e
            ))),
            Ok(t) => match t {
                AstNodeType::InfixOperationAstNode => {
                    let node = self
                        .infix_nodes
                        .get_node_by_id(continuation_node_id)
                        .unwrap_or_else(|e| {
                            panic!(
                                "There is no {} type node with id: {}. Details: {}",
                                AstNodeType::InfixOperationAstNode,
                                continuation_node_id,
                                e
                            )
                        });
                    if node.get_left_id() == Uuid::nil()
                        && node.get_left_type() == &AstNodeType::None
                        && node.get_right_id() != Uuid::nil()
                        && node.get_right_type() != &AstNodeType::None
                    {
                        return Err(ParseError::new(format!(
                            "The {} type node is in inconsistent state, meaning its right side occupied but its left side is empty. Node details: {}",
                            AstNodeType::InfixOperationAstNode,
                            node
                        )));
                    } else {
                        Ok(true)
                    }
                }
                _ => Err(ParseError::new(format!(
                    "The node with id: {} is not type of {}. It is type of: {}",
                    continuation_node_id,
                    AstNodeType::InfixOperationAstNode,
                    t
                ))),
            },
        }
    }
}
