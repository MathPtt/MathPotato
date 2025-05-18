use uuid::Uuid;

use crate::{
    ast::{ast_node_types_enum::AstNodeType, ast_tree::MathPotatoAstTree},
    parser::parser_error::ParseError,
};

use super::ContNodeApi;

pub trait ContNodeApiUpdateNodeIdAndType: ContNodeApi {
    /// Update Continuation Node and Id
    ///
    /// It updates the continuation node (an Id and node type) in the AST.
    /// If there is no continuation node, meaning the first time continuation node has been
    /// referenced, it will create the reference. It also means that this method blindly overwrites
    /// the values.
    ///
    /// # Parameters
    /// - `id`:`Uuid` - the id of the new continuation node
    /// - `ast_node_type`:`AstNodeType` - the type of the new continuation node
    fn update_continuation_node_id_and_type(
        &mut self,
        id: Uuid,
        ast_node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;
}

impl ContNodeApiUpdateNodeIdAndType for MathPotatoAstTree {
    fn update_continuation_node_id_and_type(
        &mut self,
        id: Uuid,
        ast_node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        self.last_changed_node_type = ast_node_type.clone();
        self.last_changed_node_id = id;
        Ok((self.root_node_id, self.root_node_type.clone()))
    }
}
