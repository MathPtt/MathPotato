use uuid::Uuid;

use crate::ast::ast_tree::global::enums::ast_node_types_enum::AstNodeType;
use crate::ast::ast_tree::private::node_catalog::storage::NodeCatalogStorage;
use crate::parser::parser_error::ParseError;

use super::NodeCatalogApiAddNode;

impl NodeCatalogApiAddNode for NodeCatalogStorage {
    fn add(&mut self, id: Uuid, node_type: AstNodeType) -> Result<(Uuid, AstNodeType), ParseError> {
        if let Some(s) = self.catalog.get(&id) {
            Err(ParseError::new(format!(
                "There is a node in the system with id: {}, and its details are: {:#?}",
                id, s
            )))
        } else {
            match self.catalog.insert(id, node_type.clone()) {
                Some(meh) => Err(ParseError::new(
                                format!("There was a node in the system despite the fact that we already checked this condition. So here are its details: {:#?}", meh))),
                None => Ok((id, node_type))
                }
        }
    }
}
