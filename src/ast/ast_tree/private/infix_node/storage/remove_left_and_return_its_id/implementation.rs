use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_id::InfixNodeApiSetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_type::InfixNodeApiSetLeftType;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApiRemoveLeftAndReturnItsId;

impl InfixNodeStorageApiRemoveLeftAndReturnItsId for InfixNodeStorage {
    fn remove_left_and_return_its_id(
        &self,
        node_id: uuid::Uuid,
    ) -> Result<uuid::Uuid, crate::parser::parser_error::ParseError> {
        self.tree.entry(node_id).and_modify(|infix_node| {
            let id = infix_node.get_left_id();

            infix_node.set_left_id(Uuid::nil()).unwrap_or_else(|e| {
                Err(ParseError::new(format!(
                    "Error happened while updating left id of infix node with id: {}, Details: {}",
                    id, e
                )))
            });

            infix_node.set_left_type(AstNodeType::None).unwrap_or_else(|e| {
                    Err(ParseError::new(format!(
                    "Error happened while updating left type of infix node with id: {}, Details: {}",
                    id, e
                )))
            });

            Ok(id)
        })
    }
}
