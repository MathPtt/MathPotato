use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::private::root_node::node::get_id::RootNodeApiGetId;
use crate::ast::ast_tree::private::root_node::node::get_node_type::RootNodeApiGetNodeType;
use crate::ast::ast_tree::private::root_node::storage::get::RootNodeStorageApiGet;
use crate::ast::ast_tree::public::root_node::dto::RootNodeDto;
use crate::parser::parser_error::ParseError;

use super::GetRootNodeIdAndType;

impl GetRootNodeIdAndType for MathPotatoAstTree {
    fn get_root_node_id_and_type(&self) -> Result<RootNodeDto, ParseError> {
        let root_node = self.root_node.get();
        Ok(RootNodeDto::new(
            root_node.get_id(),
            root_node.get_node_type(),
        ))
    }
}
