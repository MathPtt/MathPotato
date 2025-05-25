use uuid::Uuid;

use crate::{ast::ast_tree::MathPotatoAstTree, parser::parser_error::ParseError};

use super::InfixApiAddInfixLeftChildToInfix;


impl InfixApiAddInfixLeftChildToInfix for MathPotatoAstTree {
    fn add_infix_left_child_to_infix(
        &mut self,
        subject_node_id: Uuid,
        child_node_id: Uuid,
    ) -> Result<Uuid, ParseError> {
                        match self.infix_operation_tree.add_left_child_node_to_node(subject_node_id, child_node_id) {
            Err(e) => Err(ParseError::new(format!("Error happene while adding {:#?} type node with id {} to {:#?} type node with id.", AstNodeType::InfixOperationAstNode, child_node_id, AstNodeType::InfixOperationAstNode, subject_node_id)));
            Ok(result) => Ok(result)
        }
    }
}
