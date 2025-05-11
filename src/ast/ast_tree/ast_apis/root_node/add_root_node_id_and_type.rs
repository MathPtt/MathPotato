use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree},
    parser::parser_error::ParseError,
};

use super::RootNodeApi;

pub trait RootNodeApiAddRootNodeIdAndType: RootNodeApi {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;
}

impl RootNodeApiAddRootNodeIdAndType for MathPotatoAstTree {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        if self.root_node_id != Uuid::nil() && self.root_node_type != AstNodeType::None {
            Err(ParseError::new(format!(
                "Root node type is not {:#?} and id is not {}.",
                AstNodeType::None,
                Uuid::nil()
            )))
        } else {
            self.root_node_id = id;
            self.root_node_type = node_type.clone();
            Ok((id, node_type))
        }
    }
}
