use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree},
    parser::parser_error::ParseError,
};

use super::RootNodeApi;

pub trait RootNodeApiAddRootNodeIdAndType: RootNodeApi {
    /// Add Root Node Id and Type
    ///
    /// Adds the provided id and node_type as Root Node to the AST and returns the new Root Node Id
    /// and Type values.
    ///
    /// # Parameters
    /// - `id` - `Uuid` - the new Root Node id.
    /// - `node_type` - `AstNodeType` - the new type of the Root Node.
    ///
    /// # Returns
    /// - `Ok((Uuid, AstNodeType))` - when the operation successful.
    /// - `Err(ParseError)` - The details of the error happened during the operation.
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
