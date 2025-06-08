use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::infix_node::node::set_left_id::InfixNodeApiSetLeftId;
use crate::ast::ast_tree::private::infix_node::node::set_left_type::InfixNodeApiSetLeftType;
use crate::ast::ast_tree::private::infix_node::storage::InfixNodeStorage;
use crate::parser::parser_error::ParseError;

use super::InfixNodeStorageApiAddNodeIdAndTypeToLeft;

impl InfixNodeStorageApiAddNodeIdAndTypeToLeft for InfixNodeStorage {
    fn add_node_id_and_type_to_left(
        &mut self,
        target_node_id: Uuid,
        left_node_id: Uuid,
        left_node_type: AstNodeType,
    ) -> Result<Uuid, ParseError> {
        self.nodes.entry(target_node_id).and_modify(|item| {
            item.set_left_id(left_node_id);
            item.set_left_type(left_node_type);
        });
        Ok(target_node_id)
    }
}
