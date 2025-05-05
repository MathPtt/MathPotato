use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType, internal::infix_operation_ast_node::InfixAstNodeInternal,
    },
    parser::parser_error::ParseError,
};

use super::MathPotatoAstTree;

/// AST Root node related methods
pub trait RootNodeApi {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;

    /// Returns the root node by id.
    fn get_root_node_infix(&self, id: Uuid) -> Result<(Uuid, InfixAstNodeInternal), ParseError>;

    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;

    fn get_root_node_id(&self) -> Option<Uuid>;

    fn get_root_node_type(&self) -> AstNodeType;
}

impl RootNodeApi for MathPotatoAstTree {
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

    /// Returns the root node by id.
    fn get_root_node_infix(&self, id: Uuid) -> Result<(Uuid, InfixAstNodeInternal), ParseError> {
        match self.infix_operation_tree.get(id) {
            Some(r) => Ok((id, r)),
            None => Err(ParseError::new(format!(
                "There is no InfixOperationAstNode type root node with id {}",
                id
            ))),
        }
    }

    fn update_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError> {
        self.root_node_id = id;
        self.root_node_type = node_type.clone();
        Ok((id, node_type))
    }

    fn get_root_node_id(&self) -> Option<Uuid> {
        if self.root_node_id == Uuid::nil() {
            None
        } else {
            Some(self.root_node_id)
        }
    }

    fn get_root_node_type(&self) -> AstNodeType {
        self.root_node_type.clone()
    }
}
