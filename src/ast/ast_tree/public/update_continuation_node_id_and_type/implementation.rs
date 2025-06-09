use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::continuation_node::node::new_from_id_and_type::ContinuationNodeApiNewFromIdAndType;
use crate::ast::ast_tree::private::continuation_node::node::ContinuationNode;
use crate::ast::ast_tree::private::continuation_node::storage::update_continuation_node_id_and_type::ContinuationNodeStorageApiUpdateContinuationNodeIdAndType;
use crate::ast::ast_tree::MathPotatoAstTree;
use crate::parser::parser_error::ParseError;

use super::node::new_from_id_and_type::UpdateContinuationNodeIdAndTypeResultApiNewFromIdAndType;
use super::node::UpdateContinuationNodeIdAndTypeResult;
use super::UpdateContinuationNodeIdAndType;

impl UpdateContinuationNodeIdAndType for MathPotatoAstTree {
    fn update_continuation_node_id_and_type(
        &mut self,
        id: uuid::Uuid,
        node_type: AstNodeType,
    ) -> Result<UpdateContinuationNodeIdAndTypeResult, ParseError> {
        match self.continuation_node.update_continuation_node_id_and_type(
            ContinuationNode::new_from_id_and_type(id, node_type.clone()),
        ) {
            Ok(_) => Ok(UpdateContinuationNodeIdAndTypeResult::new_from_id_and_type(
                id, node_type,
            )),
            Err(e) => Err(ParseError::new(
                format!("Error happened while executing updating continuation node with input: id: {}, node_type: {}. Details: {}", 
                    id,
                    node_type,
                    e)))
        }
    }
}
