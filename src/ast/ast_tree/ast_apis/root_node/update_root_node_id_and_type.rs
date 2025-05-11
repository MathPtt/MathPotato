use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree},
    parser::parser_error::ParseError,
};

use super::RootNodeApi;

pub trait RootNodeApiUpdateRootNodeIdAndType: RootNodeApi {
    /// Updates the Root Node id and type
    ///
    /// # Parameters
    /// - `&mut self` - `MathPotatoAstTree`: the AST tree.
    /// - `id` - `Uuid`, the Id of the new Root Node.
    /// - `node_type` - `AstNodeType` - the new type of the Root Node.
    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;
}
impl RootNodeApiUpdateRootNodeIdAndType for MathPotatoAstTree {
    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        self.root_node_id = id;
        self.root_node_type = node_type.clone();
        Ok((id, node_type))
    }
}
