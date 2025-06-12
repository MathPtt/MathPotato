use uuid::Uuid;

use crate::{
    ast::{ast_tree::MathPotatoAstTree, global::enums::ast_node_types_enum::AstNodeType},
    parser::parser_error::ParseError,
};

use super::ContNodeApiCheckIfLeftSideEmptyRightSideOccupied;

impl ContNodeApiCheckIfLeftSideEmptyRightSideOccupied for MathPotatoAstTree {
    fn cont_node_api_check_if_left_empty_right_occupied(
        &self,
        id: Uuid,
    ) -> Result<bool, ParseError> {
        match self.node_catalog.get_type(id) {
            Err(e) => Err(ParseError::new(format!(
                "Error happened while requesting node type of node with id: {}. Details: {}",
                id, e
            ))),
            Ok(t) => match t {
                AstNodeType::InfixOperationAstNode => todo!(),
                _ => Err(ParseError::new(format!(
                    "The node with id: {} is not type of {}. It is type of: {}",
                    id,
                    AstNodeType::InfixOperationAstNode,
                    t
                ))),
            },
        }
    }
}
