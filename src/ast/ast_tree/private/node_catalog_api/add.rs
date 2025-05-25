use uuid::Uuid;

use crate::{
    ast::global::enums::ast_node_types_enum::AstNodeType, parser::parser_error::ParseError,
};

use super::{NodeCatalogApi, NodeCatalogInternalApi};

pub trait NodeCatalogApiAddNode: NodeCatalogInternalApi {
    fn add(&mut self, id: Uuid, node_type: AstNodeType) -> Result<(Uuid, AstNodeType), ParseError>;
}
impl NodeCatalogApiAddNode for NodeCatalogApi {
    fn add(&mut self, id: Uuid, node_type: AstNodeType) -> Result<(Uuid, AstNodeType), ParseError> {
        match self.catalog.get(&id) {
            Some(s) => Err(ParseError::new(format!("There is a node in the system with id: {}, and its details are: {:#?}", id, s))),
            None => {
                match self.catalog.insert(id, node_type.clone()) {
                    Some(meh) => Err(ParseError::new(
                        format!("There was a node in the system despite the fact that we already checked this condition. So here are its details: {:#?}", meh))),
                    None => Ok((id, node_type))
                }
            }

        }
    }
}
