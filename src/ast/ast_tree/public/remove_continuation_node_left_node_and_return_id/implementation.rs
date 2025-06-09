use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::storage::get_id::ContinuationNodeStorageApiGetId;
use crate::ast::ast_tree::private::continuation_node::storage::get_type::ContinutationNodeStorageApiGetType;
use crate::ast::ast_tree::private::infix_node::storage::remove_left_and_return_its_id::InfixNodeStorageApiRemoveLeftAndReturnItsId;
use crate::parser::parser_error::ParseError;

use super::RemoveContinuationNodeLeftNodeAndReturnId;

impl RemoveContinuationNodeLeftNodeAndReturnId for MathPotatoAstTree {
    fn remove_continuation_node_left_node_and_return_id(&mut self) -> Result<Uuid, ParseError> {
        match self.continuation_node.get_type() {
            AstNodeType::InfixOperationAstNode => {
                let continuation_node_id = self.continuation_node.get_id();
                let id = self.infix_nodes.remove_left_and_return_its_id(continuation_node_id)
                    .unwrap_or_else(|e|panic!("Error happened while removing left node from continuation node. Details: {}", e));
                Ok(id)
            }
            _ => Err(ParseError::new(format!(
                "Continuation node is not a type which has left and right side. Continuation node details: {}",
                self.continuation_node
            ))),
        }
    }
}
