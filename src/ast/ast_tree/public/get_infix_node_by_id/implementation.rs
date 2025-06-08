use std::any::type_name;

use uuid::Uuid;

use crate::ast::ast_tree::private::infix_node::storage::get_node_by_id::InfixNodeStorageApiGetNodeById;
use crate::ast::ast_tree::MathPotatoAstTree;
use crate::parser::parser_error::ParseError;

use super::node::new_from_id_and_infix_node::GetInfixNodeByIdApiNewFromInfixNode;
use super::node::GetInfixNodeByIdResult;
use super::GetInfixNodeById;

impl GetInfixNodeById for MathPotatoAstTree {
    fn get_infix_node_by_id(&self, id: Uuid) -> Result<GetInfixNodeByIdResult, ParseError> {
        let infix_node = self.infix_nodes.get_node_by_id(id).unwrap_or_else(|e| {
            panic!(
                "Error happened while retrieveing {}. Details: {}",
                type_name::<GetInfixNodeByIdResult>(),
                e
            )
        });

        Ok(GetInfixNodeByIdResult::new_from_id_and_infix_node(
            id, infix_node,
        ))
    }
}
