use uuid::Uuid;

use crate::{
    ast::ast_tree::{
        global::enums::ast_node_types_enum::AstNodeType,
        private::root_node::{
            node::{
                get_id::RootNodeApiGetId, get_node_type::RootNodeApiGetNodeType,
                new_from_id_and_type::RootNodeApiNewFromIdAndType, RootNode,
            },
            storage::create_or_update_root_node::RootNodeStorageApiCreateOrUpdate,
        },
        MathPotatoAstTree,
    },
    parser::parser_error::ParseError,
};

use super::{
    node::{
        new_from_id_and_type::CreateOrUpdateRootNodeIdAndTypeResultApiNewFromIdAndType,
        CreateOrUpdateRootNodeIdAndTypeResult,
    },
    CreateOrUpdateRootNodeIdAndType,
};

impl CreateOrUpdateRootNodeIdAndType for MathPotatoAstTree {
    fn create_or_update_root_node_id_and_type(
        &self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<CreateOrUpdateRootNodeIdAndTypeResult, ParseError> {
        match self
            .root_node
            .create_or_update(RootNode::new_from_id_and_type(id, node_type))
        {
            Ok(r) => Ok(CreateOrUpdateRootNodeIdAndTypeResult::new_from_id_and_type(
                r.get_id(),
                r.get_node_type(),
            )),
            Err(e) => Err(ParseError::new(
                format!("Error happened while executing create_or_update. Input: id: {}, node_type: {}, further details: {}", 
                    id, 
                    node_type, 
                    e)))
        }
    }
}
