use uuid::Uuid;

use crate::ast::ast_tree::private::i32_nodes::node::I32Node;
use crate::ast::ast_tree::private::i32_nodes::node::get_parent_id::I32NodeApiGetParentId;
use crate::ast::ast_tree::private::i32_nodes::node::get_parent_type::I32NodeApiGetParentType;
use crate::ast::ast_tree::private::i32_nodes::node::get_value::I32NodeApiGetValue;
use crate::ast::ast_tree::private::i32_nodes::node::set_parent_id::I32NodeApiSetParentId;
use crate::ast::ast_tree::private::i32_nodes::node::set_parent_type::I32NodeApiSetParentType;
use crate::ast::ast_tree::private::i32_nodes::node::set_value::I32NodeApiSetValue;
use crate::ast::ast_tree::private::i32_nodes::storage::I32NodeStorage;
use crate::parser::parser_error::ParseError;

use super::I32NodeStorageApiUpdate;

impl I32NodeStorageApiUpdate for I32NodeStorage {
    fn update(&mut self, id: Uuid, node: I32Node) -> Result<Uuid, ParseError> {
        self.nodes.entry(id).and_modify(|i32node| {
            if i32node.get_value() != node.get_value() {
                i32node.set_value(node.get_value());
            }
            if i32node.get_parent_type() != node.get_parent_type() {
                i32node.set_parent_type(node.get_parent_type());
            }
            if i32node.get_parent_id() != node.get_parent_id() {
                i32node.set_parent_id(node.get_parent_id());
            }
        });
        Ok(id)
    }
}
