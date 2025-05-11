use uuid::Uuid;

use crate::{
    ast::{
        ast_node_types_enum::AstNodeType, internal::infix_ast_node_internal::InfixAstNodeInternal,
    },
    parser::parser_error::ParseError,
};

use super::MathPotatoAstTree;

/// Root Node Api of the Abstract Syntax Tree
///
/// The root node of the AST must be tracked for the interpreter which starts here and walks the
/// tree and does its magic. At this point I think there will be a single root node.
pub trait RootNodeApi {
    fn add_root_node_id_and_type(
        &mut self,
        id: Uuid,
        node_type: AstNodeType,
    ) -> Result<(Uuid, AstNodeType), ParseError>;

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
