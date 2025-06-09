use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::get_left_id::InfixNodeApiGetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_id::InfixNodeApiSetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_type::InfixNodeApiSetLeftType;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApiRemoveLeftAndReturnItsId;

impl InfixNodeStorageApiRemoveLeftAndReturnItsId for InfixNodeStorage {
    fn remove_left_and_return_its_id(&mut self, node_id: Uuid) -> Result<Uuid, ParseError> {
        self.nodes.entry(node_id).and_modify(|infix_node| {
            infix_node.set_left_id(Uuid::nil());
            infix_node.set_left_type(AstNodeType::None);
        });
        Ok(node_id)
    }
}
