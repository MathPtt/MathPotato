use uuid::Uuid;

use crate::ast::ast_tree::MathPotatoAstTree;
use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::root_node::node::RootNode;
use crate::ast::ast_tree::private::root_node::node::get_id::RootNodeApiGetId;
use crate::ast::ast_tree::private::root_node::node::get_node_type::RootNodeApiGetNodeType;
use crate::ast::ast_tree::private::root_node::node::new_from_id_and_type::RootNodeApiNewFromIdAndType;
use crate::ast::ast_tree::private::root_node::storage::create_or_update_root_node::RootNodeStorageApiCreateOrUpdate;
use crate::parser::parser_error::ParseError;

use super::CreateOrUpdateRootNodeIdAndType;
use super::node::CreateOrUpdateRootNodeIdAndTypeResult;
use super::node::new_from_id_and_type::CreateOrUpdateRootNodeIdAndTypeResultApiNewFromIdAndType;

impl CreateOrUpdateRootNodeIdAndType for MathPotatoAstTree {
    fn create_or_update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<CreateOrUpdateRootNodeIdAndTypeResult, ParseError> {
        match self
            .root_node
            .create_or_update(RootNode::new_from_id_and_type(id, node_type.clone()))
        {
            Ok(r) => Ok(CreateOrUpdateRootNodeIdAndTypeResult::new_from_id_and_type(
                r.get_id(),
                r.get_node_type(),
            )),
            Err(e) => Err(ParseError::new(format!(
                "Error happened while executing create_or_update. Input: id: {}, node_type: {}, further details: {}",
                id, node_type, e
            ))),
        }
    }
}
