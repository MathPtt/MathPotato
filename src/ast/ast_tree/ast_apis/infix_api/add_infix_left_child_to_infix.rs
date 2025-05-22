use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType,
        ast_tree::{ast_nodes::infix_ast_node::InfixAstNode, MathPotatoAstTree},
    },
    parser::parser_error::ParseError,
};

use super::{get_by_id::InfixApiGetNodeById, InfixApi};

pub trait InfixApiAddInfixLeftChildToInfix: InfixApi {
    fn add_infix_left_child_to_infix(
        &mut self,
        child: Uuid,
        parent: Uuid,
    ) -> Result<Uuid, ParseError>;
}
impl InfixApiAddInfixLeftChildToInfix for MathPotatoAstTree {
    fn add_infix_left_child_to_infix(
        &mut self,
        child: Uuid,
        parent: Uuid,
    ) -> Result<Uuid, ParseError> {
        match self.get_infix_node_by_id(parent) {
            None => Err(ParseError::new(format!("There is no {:#?} type in the tree with id: {}. Adding child oepration has been cancelled.",
                AstNodeType::InfixOperationAstNode, parent ))),
            Some(parent_id)=> {
                match self.infix_operation_tree.get(parent) {
            None => Err(ParseError::new(format!("There is no {:#?} type in the tree with id: {}. Adding child oepration has been cancelled.",
                AstNodeType::InfixOperationAstNode, parent ))),
                    Some(parent_internal) => {
                        parent_internal.left_type = AstNodeType::InfixOperationAstNode;
                        parent_internal.left_id = child;
                        self.infix_operation_tree.update(parent, parent_internal);

                Ok(parent)
                    }
        }
            }

        }
    }
}
